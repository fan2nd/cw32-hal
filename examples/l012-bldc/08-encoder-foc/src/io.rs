//! 普通 Embassy 线程任务处理按键、LED，并在停机后输出一次锁存故障。
use crate::{
    button::{Action, Button},
    hardware::{LIVE_STATE, LIVE_WORDS, MILLISECONDS, OUTPUT_REQUEST_PERCENT},
};
use core::{
    cell::RefCell,
    sync::atomic::Ordering,
    task::{Poll, Waker},
};
use critical_section::Mutex;
use embassy_cw32::{
    gpio::{Input, Level, Output, Pull},
    peripherals, Peri,
};
static WAKER: Mutex<RefCell<Option<Waker>>> = Mutex::new(RefCell::new(None));

pub fn wake_tasks() {
    let wake = critical_section::with(|cs| WAKER.borrow(cs).borrow_mut().take());
    if let Some(w) = wake {
        w.wake();
    }
}
async fn next_tick(last: &mut u32) {
    core::future::poll_fn(|cx| {
        // clone/drop 放在临界区外；屏蔽区仅检查毫秒和交换一个 waker。
        let mut replacement = Some(cx.waker().clone());
        let (ready, old) = critical_section::with(|cs| {
            let now = MILLISECONDS.load(Ordering::Acquire);
            if now != *last {
                *last = now;
                (true, None)
            } else {
                let mut w = WAKER.borrow(cs).borrow_mut();
                if !w.as_ref().is_some_and(|old| old.will_wake(cx.waker())) {
                    (false, core::mem::replace(&mut *w, replacement.take()))
                } else {
                    (false, None)
                }
            }
        });
        drop(old);
        drop(replacement);
        if ready {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await
}

/// 故障码对应 control::Fault：200 ms 亮、200 ms 灭，组间额外灭 1600 ms。
/// 相对首次看到故障的时刻计时，避免开头从一组中间闪起。
fn fault_led_on(elapsed_ms: u32, code: u8) -> bool {
    if code == 0 {
        return false;
    }
    let pulses_ms = u32::from(code) * 400;
    let phase = elapsed_ms % (pulses_ms + 1600);
    phase < pulses_ms && phase % 400 < 200
}

#[embassy_executor::task]
pub async fn io_task(led: Peri<'static, peripherals::PC13>, key: Peri<'static, peripherals::PA3>) {
    let mut led = Output::new(led, Level::High);
    let key = Input::new(key, Pull::Up);
    let mut previous = 0;
    let mut previous_fault = 0u8;
    let mut fault_since = 0u32;
    let mut button = Button::new();
    let mut live_since = 0u32;
    // 保留最近1.6s的低频趋势，不用于解析电气周期内的波形。
    let mut history = [[0i32; 11]; 16];
    let mut history_next = 0usize;
    let mut history_len = 0usize;
    loop {
        let before = previous;
        next_tick(&mut previous).await;
        crate::hardware::report_fault();
        // defmt-rtt会屏蔽中断：运行时只写内存，故障停PWM/ADC后再打印。
        if LIVE_STATE.load(Ordering::Acquire) == 2 {
            let v =
                core::array::from_fn::<_, 10, _>(|i| LIVE_WORDS[i].load(Ordering::Relaxed) as i32);
            LIVE_STATE.store(0, Ordering::Release);
            // 停机不连续刷屏；状态3/4/5为对齐，6为闭环。
            if (3..=6).contains(&v[0]) {
                history[history_next][0] = previous as i32;
                history[history_next][1..].copy_from_slice(&v);
                history_next = (history_next + 1) % history.len();
                history_len = (history_len + 1).min(history.len());
            }
        }
        if previous.wrapping_sub(live_since) >= 100 && LIVE_STATE.load(Ordering::Acquire) == 0 {
            live_since = previous;
            LIVE_STATE.store(1, Ordering::Release);
        }
        let (running, fault) = crate::hardware::running_and_fault_code();
        if fault != 0 && history_len != 0 {
            // 每毫秒一条，避免非阻塞RTT缓冲被整段历史瞬间填满。
            let index = (history_next + history.len() - history_len) % history.len();
            let v = history[index];
            defmt::info!("RUN_HISTORY ms={} state={} speed/filter={} / {}mHz dq=[{},{}]mV cap={}mV current={}mA valid={} age={} target={}%",
                v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9], v[10]);
            history_len -= 1;
        }
        if fault != 0 {
            OUTPUT_REQUEST_PERCENT.store(0, Ordering::Release);
        }
        match button.update(key.is_low(), fault, previous.wrapping_sub(before)) {
            Action::None => {}
            Action::Output(speed) => OUTPUT_REQUEST_PERCENT.store(speed as u32, Ordering::Release),
            Action::AcknowledgeFault => {
                // fault发布前已关桥、停止ATIM/ADC1；系统复位清理硬件残留，
                // 上电流程重新校准并停在0档。确认键继续按住不具有启动资格。
                OUTPUT_REQUEST_PERCENT.store(0, Ordering::Release);
                cortex_m::peripheral::SCB::sys_reset();
            }
        }
        if fault != previous_fault {
            previous_fault = fault;
            fault_since = previous;
        }
        let on = if fault != 0 {
            fault_led_on(previous.wrapping_sub(fault_since), fault)
        } else {
            running
        };
        if on {
            led.set_low();
        } else {
            led.set_high();
        }
    }
}

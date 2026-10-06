//! 普通 Embassy 线程任务处理按键、LED，并在停机后输出一次锁存故障。
use crate::hardware::{MILLISECONDS, RUN_REQUEST};
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
    let mut held = 0u16;
    let mut previous_fault = 0u8;
    let mut fault_since = 0u32;
    let mut released = false; // PA3 是 VSR/ESC 接口，不是图上的板载按键。
    let mut release_count = 0u16; // 手动开关模式必须先连续释放 60 ms。
    loop {
        let before = previous;
        next_tick(&mut previous).await;
        crate::hardware::report_fault();
        if previous.wrapping_sub(before) > 2 {
            held = 0;
            released = false;
            release_count = 0;
        }
        if key.is_low() {
            release_count = 0;
            held = held.saturating_add(1);
            if held == 60 && released {
                RUN_REQUEST.store(!RUN_REQUEST.load(Ordering::Acquire), Ordering::Release);
                released = false;
            }
        } else {
            held = 0;
            release_count = release_count.saturating_add(1);
            if release_count >= 60 {
                released = true;
            }
        }
        let (running, fault) = crate::hardware::running_and_fault_code();
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

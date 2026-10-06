//! 板级 I/O：按键采样、LED、UART 遥测与普通线程模式下的 RTT 日志。
//! 通过容量有界的消息连接此任务与电机中断域。
//! 外设句柄、寄存器指针和控制器引用均不跨越此边界。

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IoFeedback {
    pub led_on: Option<bool>,
    pub telemetry: Option<[u8; 7]>,
}

impl IoFeedback {
    /// 消费者延迟处理时，保留最新的完整消息。每帧按
    /// 完整七字节复制，绝不与旧帧的未完成部分交错。
    pub fn merge(&mut self, newer: Self) {
        if newer.led_on.is_some() {
            self.led_on = newer.led_on;
        }
        if newer.telemetry.is_some() {
            self.telemetry = newer.telemetry;
        }
    }
    pub fn take(&mut self) -> Self {
        core::mem::take(self)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IoCommand {
    key_pressed: bool,
    age_ms: u16,
}

impl IoCommand {
    pub const fn default_const() -> Self {
        Self {
            key_pressed: false,
            age_ms: 0,
        }
    }
    pub fn update(&mut self, key_pressed: bool) {
        self.key_pressed = key_pressed;
        self.age_ms = 0;
    }
    pub fn tick_1ms(&mut self) {
        self.age_ms = self.age_ms.saturating_add(1);
    }
    // 消抖需要持续获得新采样。单次按下采样之后
    // 若任务停滞，绝不能将其误判为持续按键 60 ms。
    pub const fn key_pressed(&self) -> bool {
        self.key_pressed && self.age_ms < 2
    }
}

use crate::frame_queue::FrameQueue;
use core::cell::RefCell;
use critical_section::Mutex;
use embassy_cw32::{
    gpio::{Input, Level, Output, Pull},
    uart::{ClockSource, Config, Uart},
};

pub async fn io_task(
    led: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PC13>,
    key: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PA3>,
    uart: embassy_cw32::Peri<'static, embassy_cw32::peripherals::UART1>,
    tx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB12>,
    rx_pin: embassy_cw32::Peri<'static, embassy_cw32::peripherals::PB11>,
) {
    let mut led = Output::new(led, Level::High);
    let key = Input::new(key, Pull::Up);
    // 保留原应用的 96 MHz PCLK / (16 * 52 + 1) 分频、8N1 格式、
    // PB12 TX / PB11 RX 引脚映射与轮询节奏。UART2 仍作为电机
    // 软件执行器的中断向量；此处 UART1 不使能任何中断。
    let mut config = Config::default();
    config.baudrate = 115_200;
    config.clock_source = ClockSource::PclkAlt;
    config.rx_pull = Pull::Up;
    let mut uart = Uart::new_blocking(uart, tx_pin, rx_pin, config).unwrap();
    let mut tx = FrameQueue::new();
    let mut last_fault = None;
    loop {
        let (feedback, snapshot) = next_io_tick().await;
        let pressed = key.is_low();
        critical_section::with(|cs| IO_LINK.borrow(cs).borrow_mut().command.update(pressed));
        if let Some(on) = feedback.led_on {
            // PC13 LED 低电平点亮；SET/CLR 操作不会与电机 GPIO 复用配置的读改写竞争。

            if on {
                led.set_low();
            } else {
                led.set_high();
            }
        }
        if let Some(frame) = feedback.telemetry {
            tx.push(frame);
        }
        // 非阻塞发送，每次任务唤醒最多发送一个 UART 字节。
        if uart.is_write_ready() {
            if let Some(byte) = tx.pop_byte() {
                let _ = uart.try_write(byte).unwrap();
            }
        }
        // 快照只保存复制值。RTT 在 IO_LINK 的临界区
        // 之外执行，绝不在 ADC/换相/电机执行器中断内执行。
        if let Some(snapshot) = snapshot {
            snapshot.report(last_fault);
            last_fault = snapshot.fault;
        }
        core::hint::black_box((&led, &key, &uart));
    }
}

struct IoLink {
    command: IoCommand,
    feedback: IoFeedback,
    diagnostics: Option<crate::logging::Snapshot>,
    pending_tick: bool,
    waker: Option<core::task::Waker>,
}
static IO_LINK: Mutex<RefCell<IoLink>> = Mutex::new(RefCell::new(IoLink {
    command: IoCommand::default_const(),
    feedback: IoFeedback {
        led_on: None,
        telemetry: None,
    },
    diagnostics: None,
    pending_tick: false,
    waker: None,
}));

pub fn publish_io_tick() {
    let waker = critical_section::with(|cs| {
        let mut link = IO_LINK.borrow(cs).borrow_mut();
        link.pending_tick = true;
        link.waker.take()
    });
    if let Some(waker) = waker {
        waker.wake();
    }
}

async fn next_io_tick() -> (IoFeedback, Option<crate::logging::Snapshot>) {
    core::future::poll_fn(|cx| {
        critical_section::with(|cs| {
            let mut link = IO_LINK.borrow(cs).borrow_mut();
            if link.pending_tick {
                link.pending_tick = false;
                core::task::Poll::Ready((link.feedback.take(), link.diagnostics.take()))
            } else {
                if !link
                    .waker
                    .as_ref()
                    .is_some_and(|old| old.will_wake(cx.waker()))
                {
                    link.waker = Some(cx.waker().clone());
                }
                core::task::Poll::Pending
            }
        })
    })
    .await
}

pub fn key_sample() -> bool {
    critical_section::with(|cs| {
        let mut link = IO_LINK.borrow(cs).borrow_mut();
        link.command.tick_1ms();
        link.command.key_pressed()
    })
}

pub fn publish_status(feedback: IoFeedback) {
    critical_section::with(|cs| IO_LINK.borrow(cs).borrow_mut().feedback.merge(feedback));
}

/// 与 LED/UART 反馈一样，仅交换最新值；不使用事件队列或指向实时状态的引用。
pub fn publish_diagnostics(snapshot: crate::logging::Snapshot) {
    critical_section::with(|cs| IO_LINK.borrow(cs).borrow_mut().diagnostics = Some(snapshot));
}

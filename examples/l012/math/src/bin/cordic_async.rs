//! Q1.31 waveform and scaled mathematical functions; no pins or DMA required.
#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal, bind_interrupts,
    cordic::{self, AtanScale, Cordic, LogScale, Q1_31, SqrtScale},
    peripherals,
};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
bind_interrupts!(struct Irqs { CORDIC => cordic::InterruptHandler<peripherals::CORDIC>; });
#[embassy_executor::main]
async fn main(_spawner: embassy_executor::Spawner) {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut cordic = Cordic::new(p.CORDIC).unwrap();
    let quarter = Q1_31::from_bits(0x2000_0000);
    let half = Q1_31::from_bits(0x4000_0000);
    let mut angle = 0i32;
    loop {
        let waveform = cordic.sin_cos_async(Q1_31::from_bits(angle), Irqs).await;
        let phase = cordic.phase_async(half, quarter, Irqs).await;
        let magnitude_half = cordic.hypot_half_async(half, quarter, Irqs).await;
        let arctangent = cordic.atan_async(quarter, AtanScale::X4, Irqs).await;
        let hyperbolic_half = cordic.sinh_cosh_half_async(quarter, Irqs).await;
        let inverse_hyperbolic_half = cordic.atanh_half_async(quarter, Irqs).await;
        let log_scaled = cordic.ln_async(quarter, LogScale::X2Over4, Irqs).await;
        let root = cordic.square_root_async(quarter, SqrtScale::X1, Irqs).await;
        let _ = core::hint::black_box((
            waveform,
            phase,
            magnitude_half,
            arctangent,
            hyperbolic_half,
            inverse_hyperbolic_half,
            log_scaled,
            root,
        ));
        if cordic.is_busy() {
            loop {
                cortex_m::asm::wfi();
            }
        }
        angle = angle.wrapping_add(0x0100_0000);
    }
}

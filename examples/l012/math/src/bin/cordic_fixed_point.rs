//! Q1.31 waveform and scaled mathematical functions; no pins or DMA required.
#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::{
    self as hal,
    cordic::{AtanScale, Cordic, LogScale, Q1_31, SqrtScale},
};
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}
#[entry]
fn main() -> ! {
    let p = hal::init(hal::Config::default()).unwrap();
    let mut cordic = Cordic::new(p.CORDIC).unwrap();
    let quarter = Q1_31::from_bits(0x2000_0000);
    let half = Q1_31::from_bits(0x4000_0000);
    let polls = 1024;
    let mut angle = 0i32;
    loop {
        let waveform = cordic.sin_cos(Q1_31::from_bits(angle), polls);
        let phase = cordic.phase(half, quarter, polls);
        let magnitude_half = cordic.hypot_half(half, quarter, polls);
        let arctangent = cordic.atan(quarter, AtanScale::X4, polls);
        let hyperbolic_half = cordic.sinh_cosh_half(quarter, polls);
        let inverse_hyperbolic_half = cordic.atanh_half(quarter, polls);
        let log_scaled = cordic.ln(quarter, LogScale::X2Over4, polls);
        let root = cordic.square_root(quarter, SqrtScale::X1, polls);
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

#![no_std]
#![no_main]

use embassy_cw32::{
    self as hal,
    dma::{self, CopyChannel},
    gpio::{Level, Output, Speed},
    peripherals,
};
use embassy_executor::Spawner;

hal::bind_interrupts!(struct Irqs {
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH23 => dma::InterruptHandler<peripherals::DMA_CH2>, dma::InterruptHandler<peripherals::DMA_CH3>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::init(Default::default()).unwrap();
    let mut indicator = Output::new(p.PB0, Level::Low, Speed::Default);

    // singleton! returns exclusive static SRAM buffers. No caller buffer or
    // channel can be reused while its owned copy is running or forgotten.
    let src16 = cortex_m::singleton!(: [u16; 32] = [0x1357; 32]).unwrap();
    let dst16 = cortex_m::singleton!(: [u16; 32] = [0; 32]).unwrap();
    // Admission comes from normal HAL initialization. Dirty/inherited DMA is
    // rejected before a safe capability is issued; no unsafe copy entry needed.
    let channel1 = match CopyChannel::new(p.DMA_CH1, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA startup not admitted"),
    };
    let copy16 = match channel1.copy(src16, dst16) {
        Ok(copy) => copy,
        Err(_) => panic!("DMA configuration rejected"),
    };
    let completed16 = copy16.blocking_wait().unwrap();
    assert_eq!(completed16.source, completed16.destination);

    let mut channel2 = match CopyChannel::new(p.DMA_CH2, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 2 not admitted"),
    };
    let mut channel3 = match CopyChannel::new(p.DMA_CH3, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 3 not admitted"),
    };
    let mut src8: &'static mut [u8] = cortex_m::singleton!(: [u8; 64] = [0x5a; 64]).unwrap();
    let mut dst8: &'static mut [u8] = cortex_m::singleton!(: [u8; 64] = [0; 64]).unwrap();
    let mut src32: &'static mut [u32] =
        cortex_m::singleton!(: [u32; 32] = [0x1234_5678; 32]).unwrap();
    let mut dst32: &'static mut [u32] = cortex_m::singleton!(: [u32; 32] = [0; 32]).unwrap();

    loop {
        src8[0] = src8[0].wrapping_add(1);
        src32[0] = src32[0].wrapping_add(1);
        // Both capabilities and their disjoint static buffers move into the
        // futures. Successful completion returns the same owners for reuse.
        let copy2 = match channel2.copy(src8, dst8) {
            Ok(copy) => copy,
            Err(_) => panic!("DMA channel 2 configuration rejected"),
        };
        let copy3 = match channel3.copy(src32, dst32) {
            Ok(copy) => copy,
            Err(_) => panic!("DMA channel 3 configuration rejected"),
        };
        // Both transfers are already running. Their real shared IRQ dispatches
        // separate handlers and does not clear the other channel's flags.
        let completed2 = copy2.await.unwrap();
        let completed3 = copy3.await.unwrap();
        assert_eq!(completed2.source, completed2.destination);
        assert_eq!(completed3.source, completed3.destination);
        channel2 = completed2.channel;
        src8 = completed2.source;
        dst8 = completed2.destination;
        channel3 = completed3.channel;
        src32 = completed3.source;
        dst32 = completed3.destination;
        indicator.toggle();
        cortex_m::asm::delay(800_000);
    }
}

// A panic halts with interrupts disabled and never returns quarantined owners.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}

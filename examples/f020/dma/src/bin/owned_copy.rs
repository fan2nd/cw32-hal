#![no_std]
#![no_main]

use cw32_f020_dma_example::{COPY_U8_LEN, COPY_U16_LEN, COPY_U32_LEN};
use embassy_cw32::{
    self as hal,
    dma::{self, CopyChannel},
    peripherals,
};
use embassy_executor::Spawner;

hal::bind_interrupts!(struct Irqs {
    DMACH1 => dma::InterruptHandler<peripherals::DMA_CH1>;
    DMACH2 => dma::InterruptHandler<peripherals::DMA_CH2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // Normal reset/clean-runtime startup admits safe channel ownership.
    let p = hal::init(Default::default()).unwrap();
    let mut channel1 = match CopyChannel::new(p.DMA_CH1, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 1 startup not admitted"),
    };
    let mut channel2 = match CopyChannel::new(p.DMA_CH2, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 2 startup not admitted"),
    };

    // Six exclusive static SRAM allocations occupy 512 bytes in total.
    let mut src8: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0x5a; COPY_U8_LEN]).unwrap();
    let mut dst8: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0; COPY_U8_LEN]).unwrap();
    let mut src16: &'static mut [u16] =
        cortex_m::singleton!(: [u16; COPY_U16_LEN] = [0x1357; COPY_U16_LEN]).unwrap();
    let mut dst16: &'static mut [u16] =
        cortex_m::singleton!(: [u16; COPY_U16_LEN] = [0; COPY_U16_LEN]).unwrap();
    let mut src32: &'static mut [u32] =
        cortex_m::singleton!(: [u32; COPY_U32_LEN] = [0x1234_5678; COPY_U32_LEN]).unwrap();
    let mut dst32: &'static mut [u32] =
        cortex_m::singleton!(: [u32; COPY_U32_LEN] = [0; COPY_U32_LEN]).unwrap();

    loop {
        src16[0] = src16[0].wrapping_add(1);
        src8[0] = src8[0].wrapping_add(1);
        let copy16 = match channel1.copy(src16, dst16) {
            Ok(copy) => copy,
            Err(_) => panic!("u16 copy configuration rejected"),
        };
        let copy8 = match channel2.copy(src8, dst8) {
            Ok(copy) => copy,
            Err(_) => panic!("u8 copy configuration rejected"),
        };
        // Both finite transfers were started before either future is awaited.
        // Each channel has its own real interrupt and terminal flags.
        let completed16 = copy16.await.unwrap();
        let completed8 = copy8.await.unwrap();
        assert_eq!(completed16.source, completed16.destination);
        assert_eq!(completed8.source, completed8.destination);
        channel1 = completed16.channel;
        src16 = completed16.source;
        dst16 = completed16.destination;
        channel2 = completed8.channel;
        src8 = completed8.source;
        dst8 = completed8.destination;

        src32[0] = src32[0].wrapping_add(1);
        src8[0] = src8[0].wrapping_add(1);
        // Reuse the successfully returned capabilities. Channel 1 now copies
        // u32 words; channel 2 reuses its original u8 source and destination.
        let copy32 = match channel1.copy(src32, dst32) {
            Ok(copy) => copy,
            Err(_) => panic!("u32 copy configuration rejected"),
        };
        let copy8 = match channel2.copy(src8, dst8) {
            Ok(copy) => copy,
            Err(_) => panic!("reused u8 copy configuration rejected"),
        };
        let completed32 = copy32.await.unwrap();
        let completed8 = copy8.await.unwrap();
        assert_eq!(completed32.source, completed32.destination);
        assert_eq!(completed8.source, completed8.destination);
        channel1 = completed32.channel;
        src32 = completed32.source;
        dst32 = completed32.destination;
        channel2 = completed8.channel;
        src8 = completed8.source;
        dst8 = completed8.destination;
    }
}

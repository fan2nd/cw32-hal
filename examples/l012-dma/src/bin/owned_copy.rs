#![no_std]
#![no_main]

use cw32_l012_dma_example::{COPY_U8_LEN, COPY_U16_LEN, COPY_U32_LEN};
use embassy_cw32::{
    self as hal,
    dma::{self, CopyChannel},
    peripherals,
};
use embassy_executor::Spawner;

hal::bind_interrupts!(struct Irqs {
    DMACH12 => dma::InterruptHandler<peripherals::DMA_CH1>,
               dma::InterruptHandler<peripherals::DMA_CH2>;
    DMACH34 => dma::InterruptHandler<peripherals::DMA_CH3>,
               dma::InterruptHandler<peripherals::DMA_CH4>;
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
    let mut channel3 = match CopyChannel::new(p.DMA_CH3, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 3 startup not admitted"),
    };
    let mut channel4 = match CopyChannel::new(p.DMA_CH4, Irqs) {
        Ok(channel) => channel,
        Err(_) => panic!("DMA channel 4 startup not admitted"),
    };

    // Eight distinct static SRAM allocations occupy 640 bytes in total.
    let mut source1: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0x5a; COPY_U8_LEN]).unwrap();
    let mut destination1: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0; COPY_U8_LEN]).unwrap();
    let mut source2: &'static mut [u16] =
        cortex_m::singleton!(: [u16; COPY_U16_LEN] = [0x1357; COPY_U16_LEN]).unwrap();
    let mut destination2: &'static mut [u16] =
        cortex_m::singleton!(: [u16; COPY_U16_LEN] = [0; COPY_U16_LEN]).unwrap();
    let mut source3: &'static mut [u32] =
        cortex_m::singleton!(: [u32; COPY_U32_LEN] = [0x1234_5678; COPY_U32_LEN]).unwrap();
    let mut destination3: &'static mut [u32] =
        cortex_m::singleton!(: [u32; COPY_U32_LEN] = [0; COPY_U32_LEN]).unwrap();
    let mut source4: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0xa5; COPY_U8_LEN]).unwrap();
    let mut destination4: &'static mut [u8] =
        cortex_m::singleton!(: [u8; COPY_U8_LEN] = [0; COPY_U8_LEN]).unwrap();

    loop {
        source1[0] = source1[0].wrapping_add(1);
        source2[0] = source2[0].wrapping_add(1);
        source3[0] = source3[0].wrapping_add(1);
        source4[0] = source4[0].wrapping_add(1);
        let copy1 = match channel1.copy(source1, destination1) {
            Ok(copy) => copy,
            Err(_) => panic!("channel 1 u8 copy configuration rejected"),
        };
        let copy2 = match channel2.copy(source2, destination2) {
            Ok(copy) => copy,
            Err(_) => panic!("channel 2 u16 copy configuration rejected"),
        };
        let copy3 = match channel3.copy(source3, destination3) {
            Ok(copy) => copy,
            Err(_) => panic!("channel 3 u32 copy configuration rejected"),
        };
        let copy4 = match channel4.copy(source4, destination4) {
            Ok(copy) => copy,
            Err(_) => panic!("channel 4 u8 copy configuration rejected"),
        };
        // All four finite transfers start before any future is awaited.
        // Each sibling on DMACH12 and DMACH34 retains its own handler and flags.
        let completed1 = copy1.await.unwrap();
        let completed2 = copy2.await.unwrap();
        let completed3 = copy3.await.unwrap();
        let completed4 = copy4.await.unwrap();
        assert_eq!(completed1.source, completed1.destination);
        assert_eq!(completed2.source, completed2.destination);
        assert_eq!(completed3.source, completed3.destination);
        assert_eq!(completed4.source, completed4.destination);

        // Only clean completion returns these capabilities and allocations.
        channel1 = completed1.channel;
        source1 = completed1.source;
        destination1 = completed1.destination;
        channel2 = completed2.channel;
        source2 = completed2.source;
        destination2 = completed2.destination;
        channel3 = completed3.channel;
        source3 = completed3.source;
        destination3 = completed3.destination;
        channel4 = completed4.channel;
        source4 = completed4.source;
        destination4 = completed4.destination;
    }
}

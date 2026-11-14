include!("../metadata_0033.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CW32L083RCS6",
    family: "CW32L0",
    line: "CW32L083",
    memory: &[&[
        MemoryRegion {
            name: "FLASH",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 262144,
            settings: None,
        },
        MemoryRegion {
            name: "RAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 24576,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};

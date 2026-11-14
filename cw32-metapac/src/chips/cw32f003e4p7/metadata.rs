include!("../metadata_0005.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CW32F003E4P7",
    family: "CW32F0",
    line: "CW32F003",
    memory: &[&[
        MemoryRegion {
            name: "FLASH",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 20480,
            settings: None,
        },
        MemoryRegion {
            name: "RAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 3072,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};

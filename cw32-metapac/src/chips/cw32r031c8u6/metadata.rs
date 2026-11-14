include!("../metadata_0036.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CW32R031C8U6",
    family: "CW32R0",
    line: "CW32R031",
    memory: &[&[
        MemoryRegion {
            name: "FLASH",
            kind: MemoryRegionKind::Flash,
            address: 0x0,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "RAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 8192,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};

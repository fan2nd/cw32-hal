include!("../metadata_0020.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "CW32L011K8T6",
    family: "CW32L0",
    line: "CW32L011",
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
            size: 6144,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
};

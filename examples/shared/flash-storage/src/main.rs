#![no_std]
#![no_main]

//! Destructive storage example. Never run without the board/partition review
//! in README.md. Linking this example does not approve a real board envelope.
use cortex_m_rt::entry;
use embassy_cw32 as hal;
use embedded_storage::nor_flash::ReadNorFlash;

unsafe extern "C" {
    static __storage_start: u8;
    static __storage_end: u8;
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        core::hint::spin_loop();
    }
}

#[entry]
fn main() -> ! {
    let mut config = hal::Config::default();
    // Illustrative supply/temperature envelope. Replace it with actual,
    // board-qualified bounds before running. This is not a measurement.
    config.rcc.operating_conditions = hal::rcc::OperatingConditions {
        supply_mv: 3_300,
        min_temperature_c: -20,
        max_temperature_c: 70,
    };
    let clocks = config.rcc.frequencies().unwrap();
    // SAFETY: requires the board qualification described above and in README.
    let conditions = unsafe {
        hal::flash::OperatingConditions::from_rcc(config.rcc.operating_conditions, clocks)
    }
    .unwrap();
    let p = hal::init(config).unwrap();
    // Address-of the nonzero linker boundary symbols does not access FLASH.
    let start = core::ptr::addr_of!(__storage_start) as u32;
    let end = core::ptr::addr_of!(__storage_end) as u32;
    // SAFETY: build.rs excludes this 4 KiB and any SLIB descriptor page from the image.
    // The user must also exclude every external/DMA/debugger/bootloader owner.
    let region = unsafe { hal::flash::ReservedRegion::new(start..end) }.unwrap();
    let mut flash = hal::flash::Flash::new_blocking(p.FLASH, region, conditions).unwrap();
    // Deliberately destructive and not power-fail safe. Only erase the first
    // reserved page; neighboring reserved pages remain untouched.
    flash
        .blocking_erase(0, hal::flash::ERASE_SIZE as u32)
        .unwrap();
    flash.blocking_write(1, b"CW32 storage").unwrap();
    let mut readback = [0u8; 12];
    ReadNorFlash::read(&mut flash, 1, &mut readback).unwrap();
    assert_eq!(&readback, b"CW32 storage");
    loop {
        core::hint::black_box(readback);
        core::hint::spin_loop();
    }
}

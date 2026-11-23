#![no_std]
#![no_main]
use cortex_m_rt::entry;
use embassy_cw32::aes::{Aes, Key};
#[entry]
fn main() -> ! {
    let mut p = embassy_cw32::init(Default::default()).unwrap();
    // Demonstration words only. Do not embed production keys in firmware.
    let key128 = [0x03020100, 0x07060504, 0x0b0a0908, 0x0f0e0d0c];
    let key192 = [1, 2, 3, 4, 5, 6];
    let key256 = [1, 2, 3, 4, 5, 6, 7, 8];
    let input = [0x33221100, 0x77665544, 0xbbaa9988, 0xffeeddcc];
    for key in [
        Key::Bits128(&key128),
        Key::Bits192(&key192),
        Key::Bits256(&key256),
    ] {
        let mut aes = Aes::new(p.AES.reborrow(), key).unwrap();
        let encrypted = aes.encrypt_block(&input, 100_000).unwrap();
        let decrypted = aes.decrypt_block(&encrypted, 100_000).unwrap();
        // A board-visible debugger can inspect this. Round-trip is not an
        // independent known-answer proof of standard byte serialization.
        core::hint::black_box((encrypted, decrypted, decrypted == input));
        drop(aes); // Idle: clear documented key/data registers before gating off.
    }
    loop {
        cortex_m::asm::wfi();
    }
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

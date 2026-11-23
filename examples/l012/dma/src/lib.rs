#![no_std]

pub const COPY_U8_LEN: usize = 64;
pub const COPY_U16_LEN: usize = 32;
pub const COPY_U32_LEN: usize = 32;
pub const STAGING_SIZE: usize = 32;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // Halt without reclaiming or restarting unfinished DMA resources.
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}

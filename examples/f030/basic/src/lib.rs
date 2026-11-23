#![no_std]

// A deliberately minimal panic handler: no RTT probe or semihosting required.
// A panic stops this firmware; attach a debugger to investigate.
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    cortex_m::interrupt::disable();
    loop {
        cortex_m::asm::wfi();
    }
}

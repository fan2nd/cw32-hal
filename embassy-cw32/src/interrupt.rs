//! Audited device interrupts and Embassy's type-level binding contracts.
//!
//! The PAC runtime's generated vector table connects these names to real IRQ
//! slots. Enable `rt` to use [`crate::bind_interrupts!`]. Drivers should require
//! the appropriate [`typelevel::Binding`] proof before enabling their IRQs.

// Use the official Embassy implementation, including its sealed IRQ markers,
// priority handling and NVIC operations. The generated input is only an IRQ list.
#[allow(unsafe_op_in_unsafe_fn)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/_generated_interrupts.rs"));
}
pub use generated::interrupt::*;

// Preserve the earlier spelling of the handler and binding contracts.
pub use typelevel::{Binding, Handler};

/// Install interrupt handlers and construct compile-time binding proofs.
///
/// Every handler in a shared-vector list is called synchronously, in order, on
/// every occurrence of that IRQ. Each handler must check and service its own
/// peripheral's pending flags. IRQ identifiers must exist on the selected chip.
///
/// This macro requires `rt`: its proofs rely on the PAC's generated vector table
/// and cortex-m-rt's `link.x` (which includes the generated `device.x`). A custom
/// bootloader/vector-table replacement must preserve those dispatch contracts.
///
/// Unknown vectors are rejected, including an empty handler list.
///
/// A handler for a different IRQ cannot manufacture a binding proof.
#[cfg(feature = "rt")]
#[macro_export]
macro_rules! bind_interrupts {
    ($(#[$outer:meta])* $vis:vis struct $name:ident {
        $(
            $(#[doc = $doc:literal])*
            $(#[cfg($cond_irq:meta)])?
            $irq:ident => $(
                $(#[cfg($cond_handler:meta)])?
                $handler:ty
            ),*;
        )*
    }) => {
        #[derive(Copy, Clone)]
        $(#[$outer])*
        $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            $(#[cfg($cond_irq)])?
            $(#[doc = $doc])*
            unsafe extern "C" fn $irq() {
                // Reject unknown IRQs even if every handler is cfg-disabled.
                let _ = $crate::interrupt::Interrupt::$irq;
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe {
                        <$handler as $crate::interrupt::typelevel::Handler<
                            $crate::interrupt::typelevel::$irq
                        >>::on_interrupt();
                    }
                )*
            }

            $(#[cfg($cond_irq)])?
            $crate::bind_interrupts!(@inner
                $(
                    $(#[cfg($cond_handler)])?
                    unsafe impl $crate::interrupt::typelevel::Binding<
                        $crate::interrupt::typelevel::$irq, $handler
                    > for $name {}
                )*
            );
        )*
    };
    (@inner $($items:tt)*) => { $($items)* };
}

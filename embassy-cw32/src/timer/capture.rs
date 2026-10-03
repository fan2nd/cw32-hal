//! Typed capture/encoder adapters. Register layouts are never cast across IPs.
use super::{
    input_capture::{Capture, Edge, Filter},
    qei::QeiMode,
    Registers,
};

#[cfg(any(atim_l012, gtim_l012))]
#[macro_use]
#[path = "capture_l012.rs"]
mod l012;
#[cfg(atim_f030)]
#[path = "capture_atim_f030.rs"]
mod atim_f030;
#[cfg(atim_l012)]
#[path = "capture_atim_l012.rs"]
mod atim_l012;
#[cfg(gtim_f030)]
#[path = "capture_gtim_f030.rs"]
mod gtim_f030;
#[cfg(gtim_l012)]
#[path = "capture_gtim_l012.rs"]
mod gtim_l012;

macro_rules! methods {
    ($(fn $name:ident($($arg:ident:$ty:ty),*) $(->$result:ty)?;)+) => {
        trait CaptureRegisters: Copy {
            $(fn $name(self,$($arg:$ty),*) $(->$result)?;)+
        }
        impl Registers {
            $(pub(crate) fn $name(self,$($arg:$ty),*) $(->$result)? {
                match self {
                    #[cfg(atim)] Self::Atim(r) => r.$name($($arg),*),
                    #[cfg(gtim)] Self::Gtim(r) => r.$name($($arg),*),
                }
            })+
        }
    }
}
methods! {
    fn capture_channels()->usize;
    fn capture_filter(filter:Filter)->Option<u8>;
    fn configure_capture(channel:usize, filter:u8);
    fn capture_edge(channel:usize, edge:Option<Edge>);
    fn capture_interrupt(channel:usize, enabled:bool);
    fn capture_interrupt_enabled(channel:usize)->bool;
    fn capture_pending(channel:usize)->bool;
    fn capture_clear(channel:usize);
    fn capture_read(channel:usize)->Capture;
    fn configure_encoder(mode:QeiMode, first_filter:u8, second_filter:u8, invert_first:bool, invert_second:bool);
    fn encoder_downcounting()->bool;
}

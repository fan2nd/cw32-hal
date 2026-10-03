use super::{Capture, CaptureRegisters, Edge, Filter, QeiMode};
use crate::pac::gtim::{regs, vals, Gtim};
impl CaptureRegisters for Gtim {
    capture_l012!(ier, tisel);
}

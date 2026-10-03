use super::{Capture, CaptureRegisters, Edge, Filter, QeiMode};
use crate::pac::atim::{regs, vals, Atim};
impl CaptureRegisters for Atim {
    capture_l012!(dier, tisel1);
}

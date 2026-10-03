//! Independent and window watchdogs with explicit start and no implicit stop.

#[cfg(iwdt)]
mod iwdt;
#[cfg(iwdt)]
pub use iwdt::*;

#[cfg(wwdt)]
mod wwdt;
#[cfg(wwdt)]
pub use wwdt::*;

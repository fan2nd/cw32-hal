#![no_std]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![doc(html_no_source)]
#![doc = include_str!("../README.md")]

/// Package-neutral firmware feature, or the selected chip when it has no group.
pub const FIRMWARE_TARGET: &str = env!("CW32_FIRMWARE_TARGET");

pub mod common;

#[cfg(feature = "pac")]
mod peripherals;

#[cfg(feature = "metadata")]
mod registers;

#[cfg(feature = "pac")]
include!(env!("CW32_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata;

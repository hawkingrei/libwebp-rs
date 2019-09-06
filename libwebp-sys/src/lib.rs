#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![feature(test)]
#![allow(clippy::all)]

use std::default::Default;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const WEBP_ENCODER_ABI_VERSION: i32 = 526;
pub const WEBP_MUX_ABI_VERSION: i32 = 264;

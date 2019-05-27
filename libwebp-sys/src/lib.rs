#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![feature(test)]

extern crate test;

use std::default::Default;

include!("./webp_bindings.rs");

pub const WEBP_ENCODER_ABI_VERSION: i32 = 526;

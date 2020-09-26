// Note: some of these conversions can be done more quickly with
// unsafe conversions if you know that certain preconditions hold;
// these are not currently covered here.

#![allow(clippy::ptr_arg)]

pub mod from_str;
pub mod from_string;

pub mod from_u8_slice;
pub mod from_u8_vec;

pub mod from_path;
pub mod from_path_buf;

pub mod from_os_str;
pub mod from_os_string;

pub mod from_c_str;
pub mod from_c_string;

// The conversion functions use some argument types that you don't
// ordinarly see, such as `&String`. The types are normally implicit,
// for example `String::as_str` takes a `&String`. Since all of our
// conversions are in separate functions, we have to explicitly use
// these types.
#![allow(clippy::ptr_arg)]

pub mod from_c_str;
pub mod from_c_string;
pub mod from_os_str;
pub mod from_os_string;
pub mod from_path;
pub mod from_path_buf;
pub mod from_str;
pub mod from_string;
pub mod from_u8_slice;
pub mod from_u8_vec;

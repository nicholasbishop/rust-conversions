//! Conversions from OsString

use std::ffi::OsString;

// None will be returned if the input is not valid UTF-8.
pub fn os_string_to_str(input: &OsString) -> Option<&str> {
    input.to_str()
}

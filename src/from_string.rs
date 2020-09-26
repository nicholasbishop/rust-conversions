//! Conversions from String

use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};

pub fn string_to_str(input: &String) -> &str {
    input.as_str()
}

pub fn string_to_u8_array(input: &String) -> &[u8] {
    input.as_bytes()
}

pub fn string_to_u8_vec(input: String) -> Vec<u8> {
    input.into_bytes()
}

pub fn string_to_path(input: &String) -> &Path {
    Path::new(input)
}

pub fn string_to_path_buf(input: String) -> PathBuf {
    PathBuf::from(input)
}

pub fn string_to_os_str(input: &String) -> &OsStr {
    OsStr::new(input)
}

pub fn string_to_os_string(input: String) -> OsString {
    OsString::from(input)
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `string_to_c_string`.
pub fn string_to_c_str(input: &String) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// A NulError will be returned if the input contains any nul bytes.
pub fn string_to_c_string(input: String) -> Result<CString, NulError> {
    CString::new(input)
}

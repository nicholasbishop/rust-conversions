//! Conversions from &str

use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};

pub fn str_to_string(input: &str) -> String {
    input.to_string()
}

pub fn str_to_u8_slice(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn str_to_u8_vec(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn str_to_path(input: &str) -> &Path {
    Path::new(input)
}

pub fn str_to_path_buf(input: &str) -> PathBuf {
    PathBuf::from(input)
}

pub fn str_to_os_str(input: &str) -> &OsStr {
    OsStr::new(input)
}

pub fn str_to_os_string(input: &str) -> OsString {
    OsString::from(input)
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `str_to_c_string`.
pub fn str_to_c_str(input: &str) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// A NulError will be returned if the input contains any nul bytes.
pub fn str_to_c_string(input: &str) -> Result<CString, NulError> {
    CString::new(input)
}

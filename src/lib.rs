// Note: some of these conversions can be done more quickly with
// unsafe conversions if you know that certain preconditions hold;
// these are not currently covered here.

#![allow(clippy::ptr_arg)]

// --- Convert from &str ---
use std::ffi::{
    CStr, CString, FromBytesWithNulError, NulError, OsStr, OsString,
};
use std::path::{Path, PathBuf};

pub fn str_to_string(input: &str) -> String {
    input.to_string()
}

pub fn str_to_u8_array(input: &str) -> &[u8] {
    input.as_bytes()
}

pub fn str_to_u8_vec(input: &str) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn str_to_path(input: &str) -> &Path {
    Path::new(input)
}

pub fn str_to_path_buf(input: &str) -> PathBuf {
    Path::new(input).to_path_buf()
}

pub fn str_to_os_str(input: &str) -> &OsStr {
    OsStr::new(input)
}

pub fn str_to_os_string(input: &str) -> OsString {
    OsStr::new(input).to_os_string()
}

// A FromBytesWithNulError will be returned if the input string is not
// nul-terminated or contains any interior nul bytes.
//
// If your input str is not nul-terminated then a conversion without
// allocation is not possible, so consider using `str_to_cstring`.
pub fn str_to_cstr(input: &str) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// A NulError will be returned if the input string contains any nul
// bytes.
pub fn str_to_cstring(input: &str) -> Result<CString, NulError> {
    CString::new(input)
}

// --- Convert from String ---

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

// A FromBytesWithNulError will be returned if the input string is not
// nul-terminated or contains any interior nul bytes.
//
// If your input String is not nul-terminated then a conversion without
// allocation is not possible, so consider using `string_to_cstring`.
pub fn string_to_cstr(input: &String) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

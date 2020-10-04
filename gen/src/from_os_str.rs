use std::ffi::FromBytesWithNulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// Returns None if the input is not valid UTF-8.
pub fn os_str_to_str(input: &OsStr) -> Option<&str> {
    input.to_str()
}

// Returns None if the input is not valid UTF-8.
pub fn os_str_to_string(input: &OsStr) -> Option<String> {
    input.to_str().map(|s| s.to_string())
}

// This conversion is only allowed on Unix.
pub fn os_str_to_u8_slice_unix(input: &OsStr) -> &[u8] {
    input.as_bytes()
}

// This conversion is only allowed on Unix.
pub fn os_str_to_u8_vec_unix(input: &OsStr) -> Vec<u8> {
    input.as_bytes().to_vec()
}

pub fn os_str_to_path(input: &OsStr) -> &Path {
    Path::new(input)
}

pub fn os_str_to_path_buf(input: &OsStr) -> PathBuf {
    PathBuf::from(input)
}

pub fn os_str_to_os_string(input: &OsStr) -> OsString {
    input.to_os_string()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn os_str_to_c_str_unix(
    input: &OsStr,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// This conversion is only allowed on Unix.
pub fn os_str_to_c_string_unix(
    input: &OsStr,
) -> Result<CString, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes()).map(CString::from)
}

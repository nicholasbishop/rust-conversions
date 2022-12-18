use std::ffi::FromBytesWithNulError;
use std::ffi::NulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// Returns None if the input is not valid UTF-8.
pub fn path_to_str(input: &Path) -> Option<&str> {
    input.to_str()
}

// Returns None if the input is not valid UTF-8.
pub fn path_to_string(input: &Path) -> Option<String> {
    input.to_str().map(|s| s.to_string())
}

// This conversion is only allowed on Unix.
pub fn path_to_u8_slice_unix(input: &Path) -> &[u8] {
    input.as_os_str().as_bytes()
}

// This conversion is only allowed on Unix.
pub fn path_to_u8_vec_unix(input: &Path) -> Vec<u8> {
    input.as_os_str().as_bytes().to_vec()
}

pub fn path_to_path_buf(input: &Path) -> PathBuf {
    input.to_path_buf()
}

pub fn path_to_os_str(input: &Path) -> &OsStr {
    input.as_os_str()
}

pub fn path_to_os_string(input: &Path) -> OsString {
    input.as_os_str().to_os_string()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn path_to_c_str_unix(
    input: &Path,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_os_str().as_bytes())
}

// This conversion is only allowed on Unix.
//
// A NulError will be returned if the input contains any nul bytes.
pub fn path_to_c_string_unix(input: &Path) -> Result<CString, NulError> {
    CString::new(input.as_os_str().as_bytes())
}

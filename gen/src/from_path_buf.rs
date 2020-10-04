use std::ffi::FromBytesWithNulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

// Returns None if the input is not valid UTF-8.
pub fn path_buf_to_str(input: &PathBuf) -> Option<&str> {
    input.as_path().to_str()
}

// Returns None if the input is not valid UTF-8.
pub fn path_buf_to_string(input: PathBuf) -> Option<String> {
    input.as_path().to_str().map(|s| s.to_string())
}

// This conversion is only allowed on Unix.
pub fn path_buf_to_u8_slice_unix(input: &PathBuf) -> &[u8] {
    input.as_os_str().as_bytes()
}

// This conversion is only allowed on Unix.
pub fn path_buf_to_u8_vec_unix(input: PathBuf) -> Vec<u8> {
    input.into_os_string().into_vec()
}

pub fn path_buf_to_path(input: &PathBuf) -> &Path {
    input.as_path()
}

pub fn path_buf_to_os_str(input: &PathBuf) -> &OsStr {
    input.as_os_str()
}

pub fn path_buf_to_os_string(input: PathBuf) -> OsString {
    input.into_os_string()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn path_buf_to_c_str_unix(
    input: &PathBuf,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_os_str().as_bytes())
}

// This conversion is only allowed on Unix.
pub fn path_buf_to_c_string_unix(
    input: &PathBuf,
) -> Result<CString, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_os_str().as_bytes()).map(CString::from)
}

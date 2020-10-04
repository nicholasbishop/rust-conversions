use std::ffi::FromBytesWithNulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

// Returns None if the input is not valid UTF-8.
pub fn os_string_to_str(input: &OsString) -> Option<&str> {
    input.to_str()
}

pub fn os_string_to_string(input: OsString) -> Result<String, OsString> {
    input.into_string()
}

// This conversion is only allowed on Unix.
pub fn os_string_to_u8_slice_unix(input: &OsString) -> &[u8] {
    input.as_bytes()
}

// This conversion is only allowed on Unix.
pub fn os_string_to_u8_vec_unix(input: OsString) -> Vec<u8> {
    input.into_vec()
}

pub fn os_string_to_path(input: &OsString) -> &Path {
    Path::new(input)
}

pub fn os_string_to_path_buf(input: OsString) -> PathBuf {
    PathBuf::from(input)
}

pub fn os_string_to_os_str(input: &OsString) -> &OsStr {
    input.as_os_str()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn os_string_to_c_str_unix(
    input: &OsString,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// This conversion is only allowed on Unix.
pub fn os_string_to_c_string_unix(
    input: &OsString,
) -> Result<CString, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes()).map(CString::from)
}

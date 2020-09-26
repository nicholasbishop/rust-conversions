//! Conversions from OsString

use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};

// None will be returned if the input is not valid UTF-8.
pub fn os_string_to_str(input: &OsString) -> Option<&str> {
    input.to_str()
}

// If the input is not valid UTF-8, the input is returned as an error.
pub fn os_string_to_string(input: OsString) -> Result<String, OsString> {
    input.into_string()
}

// This conversion is only allowed on Unix.
pub fn os_string_to_u8_slice_unix(input: &OsString) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    input.as_bytes()
}

// This conversion is only allowed on Unix.
pub fn os_string_to_u8_vec_unix(input: OsString) -> Vec<u8> {
    use std::os::unix::ffi::OsStringExt;
    input.into_vec()
}

pub fn os_string_to_path(input: &OsString) -> &Path {
    Path::new(input)
}

pub fn os_string_to_path_buf(input: OsString) -> PathBuf {
    PathBuf::from(input)
}

pub fn os_string_to_os_str(input: &OsString) -> &OsStr {
    &input
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `os_str_to_c_string`.
pub fn os_string_to_c_str_unix(
    input: &OsString,
) -> Result<&CStr, FromBytesWithNulError> {
    use std::os::unix::ffi::OsStrExt;
    CStr::from_bytes_with_nul(input.as_bytes())
}

// This conversion is only allowed on Unix.
//
// A NulError will be returned if the input contains any nul bytes.
pub fn os_string_to_c_string_unix(
    input: OsString,
) -> Result<CString, NulError> {
    use std::os::unix::ffi::OsStringExt;
    CString::new(input.into_vec())
}

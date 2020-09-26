//! Conversions from Vec<u8>

use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

pub fn u8_vec_to_str(input: &Vec<u8>) -> Result<&str, Utf8Error> {
    std::str::from_utf8(input)
}

pub fn u8_vec_to_string(input: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(input)
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_path_unix(input: &Vec<u8>) -> &Path {
    use std::os::unix::ffi::OsStrExt;
    Path::new(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_path_buf_unix(input: Vec<u8>) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    PathBuf::from(OsString::from_vec(input))
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_os_str_unix(input: &Vec<u8>) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(input)
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_os_string_unix(input: Vec<u8>) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(input)
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `u8_slice_to_c_string`.
pub fn u8_vec_to_c_str(
    input: &Vec<u8>,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input)
}

// A NulError will be returned if the input contains any nul bytes.
pub fn u8_vec_to_c_string(input: Vec<u8>) -> Result<CString, NulError> {
    CString::new(input)
}

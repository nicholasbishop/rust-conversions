use std::ffi::FromBytesWithNulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

pub fn u8_vec_to_str(input: &Vec<u8>) -> Result<&str, Utf8Error> {
    std::str::from_utf8(input)
}

pub fn u8_vec_to_string(input: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(input)
}

pub fn u8_vec_to_u8_slice(input: &Vec<u8>) -> &[u8] {
    input.as_slice()
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_path_unix(input: &Vec<u8>) -> &Path {
    Path::new(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_path_buf_unix(input: Vec<u8>) -> PathBuf {
    PathBuf::from(OsString::from_vec(input))
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_os_str_unix(input: &Vec<u8>) -> &OsStr {
    OsStr::from_bytes(input)
}

// This conversion is only allowed on Unix.
pub fn u8_vec_to_os_string_unix(input: Vec<u8>) -> OsString {
    OsString::from_vec(input)
}

// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn u8_vec_to_c_str(
    input: &Vec<u8>,
) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input)
}

pub fn u8_vec_to_c_string(
    input: &Vec<u8>,
) -> Result<CString, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input).map(CString::from)
}

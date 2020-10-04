use std::ffi::IntoStringError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

pub fn c_string_to_str(input: &CString) -> Result<&str, Utf8Error> {
    input.as_c_str().to_str()
}

pub fn c_string_to_string(input: CString) -> Result<String, IntoStringError> {
    input.into_string()
}

pub fn c_string_to_u8_slice(input: &CString) -> &[u8] {
    input.as_bytes()
}

pub fn c_string_to_u8_vec(input: CString) -> Vec<u8> {
    input.into_bytes()
}

// This conversion is only allowed on Unix.
pub fn c_string_to_path_unix(input: &CString) -> &Path {
    Path::new(OsStr::from_bytes(input.as_bytes()))
}

// This conversion is only allowed on Unix.
pub fn c_string_to_path_buf_unix(input: CString) -> PathBuf {
    PathBuf::from(OsString::from_vec(input.into_bytes()))
}

// This conversion is only allowed on Unix.
pub fn c_string_to_os_str_unix(input: &CString) -> &OsStr {
    OsStr::from_bytes(input.as_bytes())
}

// This conversion is only allowed on Unix.
pub fn c_string_to_os_string_unix(input: CString) -> OsString {
    OsString::from_vec(input.into_bytes())
}

pub fn c_string_to_c_str(input: &CString) -> &CStr {
    input.as_c_str()
}

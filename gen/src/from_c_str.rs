use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::Utf8Error;

pub fn c_str_to_str(input: &CStr) -> Result<&str, Utf8Error> {
    input.to_str()
}

pub fn c_str_to_string(input: &CStr) -> Result<String, Utf8Error> {
    input.to_str().map(|s| s.to_string())
}

pub fn c_str_to_u8_slice(input: &CStr) -> &[u8] {
    input.to_bytes()
}

pub fn c_str_to_u8_vec(input: &CStr) -> Vec<u8> {
    input.to_bytes().to_vec()
}

// This conversion is only allowed on Unix.
pub fn c_str_to_path_unix(input: &CStr) -> &Path {
    Path::new(OsStr::from_bytes(input.to_bytes()))
}

// This conversion is only allowed on Unix.
pub fn c_str_to_path_buf_unix(input: &CStr) -> PathBuf {
    Path::new(OsStr::from_bytes(input.to_bytes())).to_path_buf()
}

// This conversion is only allowed on Unix.
pub fn c_str_to_os_str_unix(input: &CStr) -> &OsStr {
    OsStr::from_bytes(input.to_bytes())
}

// This conversion is only allowed on Unix.
pub fn c_str_to_os_string_unix(input: &CStr) -> OsString {
    OsStr::from_bytes(input.to_bytes()).to_os_string()
}

pub fn c_str_to_c_string(input: &CStr) -> CString {
    CString::from(input)
}

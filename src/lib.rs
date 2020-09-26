// Note: some of these conversions can be done more quickly with
// unsafe conversions if you know that certain preconditions hold;
// these are not currently covered here.

#![allow(clippy::ptr_arg)]

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

// --- Convert from &str ---

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

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `str_to_cstring`.
pub fn str_to_cstr(input: &str) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// A NulError will be returned if the input contains any nul bytes.
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

pub fn string_to_os_str(input: &String) -> &OsStr {
    OsStr::new(input)
}

pub fn string_to_os_string(input: String) -> OsString {
    OsString::from(input)
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `string_to_cstring`.
pub fn string_to_cstr(input: &String) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input.as_bytes())
}

// A NulError will be returned if the input contains any nul bytes.
pub fn string_to_cstring(input: String) -> Result<CString, NulError> {
    CString::new(input)
}

// --- Convert from &[u8] ---

// A Utf8Error will be returned if the input is not valid UTF-8.
pub fn u8_array_to_str(input: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(input)
}

// A FromUtf8Error will be returned if the input is not valid UTF-8.
pub fn u8_array_to_string(input: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(input.to_vec())
}

// This never fails, but invalid UTF-8 sequences will be replaced with
// "�". This returns a `Cow<str>`; call `to_string()` to convert it to
// a `String`.
pub fn u8_array_to_string_lossy(input: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(input)
}

// A direct conversion from bytes to a `Path` is only allowed on Unix.
pub fn u8_array_to_path_unix(input: &[u8]) -> &Path {
    use std::os::unix::ffi::OsStrExt;
    Path::new(OsStr::from_bytes(input))
}

// A direct conversion from bytes to a `PathBuf` is only allowed on
// Unix.
pub fn u8_array_to_path_buf_unix(input: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(OsStr::from_bytes(input))
}

// A direct conversion from bytes to an `&OsStr` is only allowed on
// Unix.
pub fn u8_array_to_os_str_unix(input: &[u8]) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(input)
}

// A direct conversion from bytes to an `OsString` is only allowed on
// Unix.
pub fn u8_array_to_os_string_unix(input: &[u8]) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(input.to_vec())
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `u8_array_to_cstring`.
pub fn u8_array_to_cstr(input: &[u8]) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input)
}

// A NulError will be returned if the input contains any nul bytes.
pub fn u8_array_to_cstring(input: &[u8]) -> Result<CString, NulError> {
    CString::new(input)
}

// --- Convert from Vec<u8> ---

pub fn u8_vec_to_str(input: &Vec<u8>) -> Result<&str, Utf8Error> {
    std::str::from_utf8(input)
}

pub fn u8_vec_to_string(input: Vec<u8>) -> Result<String, FromUtf8Error> {
    String::from_utf8(input)
}

// A direct conversion from bytes to a `Path` is only allowed on Unix.
pub fn u8_vec_to_path_unix(input: &Vec<u8>) -> &Path {
    use std::os::unix::ffi::OsStrExt;
    Path::new(OsStr::from_bytes(input))
}

// A direct conversion from bytes to a `PathBuf` is only allowed on
// Unix.
pub fn u8_vec_to_path_buf_unix(input: Vec<u8>) -> PathBuf {
    use std::os::unix::ffi::OsStringExt;
    PathBuf::from(OsString::from_vec(input))
}

// A direct conversion from bytes to an `OsStr` is only allowed on
// Unix.
pub fn u8_vec_to_os_str_unix(input: &Vec<u8>) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(input)
}

// A direct conversion from bytes to an `OsString` is only allowed on
// Unix.
pub fn u8_vec_to_os_string_unix(input: Vec<u8>) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(input)
}

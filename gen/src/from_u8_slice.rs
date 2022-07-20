use std::borrow::Cow;
use std::ffi::FromBytesWithNulError;
use std::ffi::{CStr, CString};
use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

pub fn u8_slice_to_str(input: &[u8]) -> Result<&str, Utf8Error> {
    std::str::from_utf8(input)
}

pub fn u8_slice_to_string(input: &[u8]) -> Result<String, FromUtf8Error> {
    String::from_utf8(input.to_vec())
}

// This never fails, but invalid UTF-8 sequences will be replaced with
// "�". This returns a `Cow<str>`; call `to_string()` to convert it to
// a `String`.
pub fn u8_slice_to_string_lossy(input: &[u8]) -> Cow<str> {
    String::from_utf8_lossy(input)
}

pub fn u8_slice_to_u8_vec(input: &[u8]) -> Vec<u8> {
    input.to_vec()
}

// This conversion is only allowed on Unix.
pub fn u8_slice_to_path_unix(input: &[u8]) -> &Path {
    Path::new(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_slice_to_path_buf_unix(input: &[u8]) -> PathBuf {
    PathBuf::from(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_slice_to_os_str_unix(input: &[u8]) -> &OsStr {
    OsStr::from_bytes(input)
}

// This conversion is only allowed on Unix.
pub fn u8_slice_to_os_string_unix(input: &[u8]) -> OsString {
    OsString::from_vec(input.to_vec())
}

// A FromBytesWithNulError will be returned if the input is not nul-
// terminated or contains any interior nul bytes. If your input is not nul-
// terminated then a conversion without allocation is not possible, convert
// to a CString instead.
pub fn u8_slice_to_c_str(input: &[u8]) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input)
}

pub fn u8_slice_to_c_string(
    input: &[u8],
) -> Result<CString, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input).map(CString::from)
}

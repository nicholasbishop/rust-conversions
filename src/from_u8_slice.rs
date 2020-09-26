//! Conversions from &[u8]

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};
use std::str::Utf8Error;
use std::string::FromUtf8Error;

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

// This conversion is only allowed on Unix.
pub fn u8_array_to_path_unix(input: &[u8]) -> &Path {
    use std::os::unix::ffi::OsStrExt;
    Path::new(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_array_to_path_buf_unix(input: &[u8]) -> PathBuf {
    use std::os::unix::ffi::OsStrExt;
    PathBuf::from(OsStr::from_bytes(input))
}

// This conversion is only allowed on Unix.
pub fn u8_array_to_os_str_unix(input: &[u8]) -> &OsStr {
    use std::os::unix::ffi::OsStrExt;
    OsStr::from_bytes(input)
}

// This conversion is only allowed on Unix.
pub fn u8_array_to_os_string_unix(input: &[u8]) -> OsString {
    use std::os::unix::ffi::OsStringExt;
    OsString::from_vec(input.to_vec())
}

// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `u8_array_to_c_string`.
pub fn u8_array_to_c_str(input: &[u8]) -> Result<&CStr, FromBytesWithNulError> {
    CStr::from_bytes_with_nul(input)
}

// A NulError will be returned if the input contains any nul bytes.
pub fn u8_array_to_c_string(input: &[u8]) -> Result<CString, NulError> {
    CString::new(input)
}

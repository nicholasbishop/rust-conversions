//! Conversions from &OsStr

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};

// None will be returned if the input is not valid UTF-8.
pub fn os_str_to_str(input: &OsStr) -> Option<&str> {
    input.to_str()
}

// None will be returned if the input is not valid UTF-8.
pub fn os_str_to_string(input: &OsStr) -> Option<String> {
    input.to_str().map(|s| s.to_string())
}

// This never fails, but invalid UTF-8 sequences will be replaced with
// "�". This returns a `Cow<str>`; call `to_string()` to convert it to
// a `String`.
pub fn os_str_to_string_lossy(input: &OsStr) -> Cow<str> {
    input.to_string_lossy()
}

// This conversion is only allowed on Unix.
pub fn os_str_to_u8_slice_unix(input: &OsStr) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    input.as_bytes()
}

// This conversion is only allowed on Unix.
pub fn os_str_to_u8_vec_unix(input: &OsStr) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    input.as_bytes().to_vec()
}

pub fn os_str_to_path(input: &OsStr) -> &Path {
    Path::new(input)
}

pub fn os_str_to_path_buf(input: &OsStr) -> PathBuf {
    PathBuf::from(input)
}

pub fn os_str_to_os_string(input: &OsStr) -> OsString {
    input.to_os_string()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `os_str_to_c_string`.
pub fn os_str_to_c_str_unix(
    input: &OsStr,
) -> Result<&CStr, FromBytesWithNulError> {
    use std::os::unix::ffi::OsStrExt;
    CStr::from_bytes_with_nul(input.as_bytes())
}

// This conversion is only allowed on Unix.
//
// A NulError will be returned if the input contains any nul bytes.
pub fn os_str_to_c_string_unix(input: &OsStr) -> Result<CString, NulError> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(input.as_bytes())
}

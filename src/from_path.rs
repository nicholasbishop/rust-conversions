//! Conversions from &Path

use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr, OsString};
use std::ffi::{FromBytesWithNulError, NulError};
use std::path::{Path, PathBuf};

// Returns `None` if the input is not valid UTF-8.
pub fn path_to_str(input: &Path) -> Option<&str> {
    input.to_str()
}

// This never fails, but invalid UTF-8 sequences will be replaced with
// "�". This returns a `Cow<str>`; call `to_string()` to convert it to
// a `String`.
//
// For printing a `Path`, consider using `Path::display()`.
pub fn path_to_string_lossy(input: &Path) -> Cow<str> {
    input.to_string_lossy()
}

// This conversion is only allowed on Unix.
pub fn path_to_u8_slice_unix(input: &Path) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    input.as_os_str().as_bytes()
}

// This conversion is only allowed on Unix.
pub fn path_to_u8_vec_unix(input: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt;
    input.as_os_str().as_bytes().to_vec()
}

pub fn path_to_path_buf(input: &Path) -> PathBuf {
    input.to_path_buf()
}

pub fn path_to_os_str(input: &Path) -> &OsStr {
    input.as_os_str()
}

pub fn path_to_os_string(input: &Path) -> OsString {
    input.as_os_str().to_os_string()
}

// This conversion is only allowed on Unix.
//
// A FromBytesWithNulError will be returned if the input is not
// nul-terminated or contains any interior nul bytes.
//
// If your input is not nul-terminated then a conversion without
// allocation is not possible, so consider using `path_to_c_string_unix`.
pub fn path_to_c_str_unix(
    input: &Path,
) -> Result<&CStr, FromBytesWithNulError> {
    use std::os::unix::ffi::OsStrExt;
    CStr::from_bytes_with_nul(input.as_os_str().as_bytes())
}

// This conversion is only allowed on Unix.
//
// A NulError will be returned if the input contains any nul bytes.
pub fn path_to_c_string_unix(input: &Path) -> Result<CString, NulError> {
    use std::os::unix::ffi::OsStrExt;
    CString::new(input.as_os_str().as_bytes())
}

use anyhow::Error;
use askama::Template;
use command_run::Command;
use fehler::throws;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use syntect::highlighting::{Color, Theme, ThemeSet};
use syntect::html::highlighted_html_for_string;
use syntect::parsing::{SyntaxReference, SyntaxSet};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    // These are the anchor types; one or more conversions between
    // each of them are generated.
    Str,
    String,
    U8Slice,
    U8Vec,
    Path,
    PathBuf,
    OsStr,
    OsString,
    CStr,
    CString,

    // Ordinarily you never see these types in a function signature,
    // but they often show up as temporary types that you don't
    // explicitly see. For example, `String::as_str` takes a
    // `&String`. Since all of our conversions are in a separate
    // function, we have to explicitly use these types.
    StringRef,
    U8VecRef,
    OsStringRef,
    PathBufRef,
    CStringRef,

    CowStr,
    OptionStr,
    OptionString,
    ResultStrOrUtf8Error,
    ResultStringOrUtf8Error,
    ResultStringOrFromUtf8Error,
    ResultStringOrOsString,
    ResultCStrOrFromBytesWithNulError,
    ResultCStringOrFromBytesWithNulError,
    ResultStringOrIntoStringError,
}

impl Type {
    fn anchors() -> &'static [Type] {
        &[
            Type::Str,
            Type::String,
            Type::U8Slice,
            Type::U8Vec,
            Type::Path,
            Type::PathBuf,
            Type::OsStr,
            Type::OsString,
            Type::CStr,
            Type::CString,
        ]
    }

    fn type_str(&self) -> &'static str {
        match self {
            Type::Str => "&str",
            Type::String => "String",
            Type::U8Slice => "&[u8]",
            Type::U8Vec => "Vec<u8>",
            Type::Path => "&Path",
            Type::PathBuf => "PathBuf",
            Type::OsStr => "&OsStr",
            Type::OsString => "OsString",
            Type::CStr => "&CStr",
            Type::CString => "CString",

            Type::StringRef => "&String",
            Type::U8VecRef => "&Vec<u8>",
            Type::PathBufRef => "&PathBuf",
            Type::OsStringRef => "&OsString",
            Type::CStringRef => "&CString",

            Type::CowStr => "Cow<str>",
            Type::OptionStr => "Option<&str>",
            Type::OptionString => "Option<String>",
            Type::ResultStrOrUtf8Error => "Result<&str, Utf8Error>",
            Type::ResultStringOrUtf8Error => "Result<String, Utf8Error>",
            Type::ResultStringOrFromUtf8Error => {
                "Result<String, FromUtf8Error>"
            }
            Type::ResultStringOrOsString => "Result<String, OsString>",
            Type::ResultCStrOrFromBytesWithNulError => {
                "Result<&CStr, FromBytesWithNulError>"
            }
            Type::ResultCStringOrFromBytesWithNulError => {
                "Result<CString, FromBytesWithNulError>"
            }
            Type::ResultStringOrIntoStringError => {
                "Result<String, IntoStringError>"
            }
        }
    }

    fn html_type_str(&self) -> String {
        self.type_str().replace("<", "&lt;").replace(">", "&gt;")
    }

    fn short_name(&self) -> &'static str {
        match self {
            Type::Str => "str",
            Type::String => "string",
            Type::U8Slice => "u8_slice",
            Type::U8Vec => "u8_vec",
            Type::Path => "path",
            Type::PathBuf => "path_buf",
            Type::OsStr => "os_str",
            Type::OsString => "os_string",
            Type::CStr => "c_str",
            Type::CString => "c_string",

            _ => panic!("no short name for {:?}", self),
        }
    }

    fn uses(&self) -> &'static [&'static str] {
        match self {
            Type::Path => &["std::path::Path"],
            Type::PathBuf => &["std::path::PathBuf"],
            Type::OsStr => &["std::ffi::OsStr"],
            Type::OsString => &["std::ffi::OsString"],
            Type::CStr => &["std::ffi::CStr"],
            Type::CString => &["std::ffi::CString"],

            Type::CowStr => &["std::borrow::Cow"],
            Type::ResultStrOrUtf8Error => &["std::str::Utf8Error"],
            Type::ResultStringOrFromUtf8Error => {
                &["std::string::FromUtf8Error"]
            }
            Type::ResultCStrOrFromBytesWithNulError => {
                &["std::ffi::CStr", "std::ffi::FromBytesWithNulError"]
            }
            Type::ResultCStringOrFromBytesWithNulError => {
                &["std::ffi::CString", "std::ffi::FromBytesWithNulError"]
            }
            Type::ResultStringOrIntoStringError => {
                &["std::ffi::IntoStringError"]
            }
            _ => &[],
        }
    }

    /// Optional comment associated with the type when used as a
    /// return value.
    fn return_comment(&self) -> Option<&'static str> {
        match self {
            Type::CowStr => Some(
                "This never fails, but invalid UTF-8 sequences will be
replaced with \"�\". This returns a `Cow<str>`; call `to_string()` to convert
it to a `String`.",
            ),
            Type::OptionStr | Type::OptionString => {
                Some("Returns None if the input is not valid UTF-8.")
            }
            Type::ResultCStrOrFromBytesWithNulError => Some(
                "A FromBytesWithNulError will be returned if the
input is not nul-terminated or contains any interior nul bytes.

If your input is not nul-terminated then a conversion without allocation
is not possible, convert to a CString instead.",
            ),
            _ => None,
        }
    }
}

#[derive(Default)]
struct Conversion {
    format: &'static str,
    os_str_bytes: bool,
    os_string_bytes: bool,
}

impl Conversion {
    fn format_expr(&self, expr: String) -> String {
        self.format.replace("{}", &expr)
    }

    fn unix_only(&self) -> bool {
        self.os_str_bytes || self.os_string_bytes
    }

    fn uses(&self) -> Vec<&'static str> {
        let mut uses = Vec::new();
        if self.os_str_bytes {
            uses.push("std::os::unix::ffi::OsStrExt");
        }
        if self.os_string_bytes {
            uses.push("std::os::unix::ffi::OsStringExt");
        }
        uses
    }

    fn use_os_str_bytes(mut self) -> Self {
        self.os_str_bytes = true;
        self
    }

    fn use_os_string_bytes(mut self) -> Self {
        self.os_string_bytes = true;
        self
    }
}

fn conversion_chains(t1: Type, t2: Type) -> &'static [&'static [Type]] {
    match (t1, t2) {
        // From &str
        (Type::Str, Type::String) => &[&[Type::Str, Type::String]],
        (Type::Str, Type::U8Slice) => &[&[Type::Str, Type::U8Slice]],
        (Type::Str, Type::U8Vec) => &[&[Type::Str, Type::U8Slice, Type::U8Vec]],
        (Type::Str, Type::Path) => &[&[Type::Str, Type::Path]],
        (Type::Str, Type::PathBuf) => &[&[Type::Str, Type::PathBuf]],
        (Type::Str, Type::OsStr) => &[&[Type::Str, Type::OsStr]],
        (Type::Str, Type::OsString) => &[&[Type::Str, Type::OsString]],
        (Type::Str, Type::CStr) => &[&[
            Type::Str,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::Str, Type::CString) => &[&[
            Type::Str,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From String
        (Type::String, Type::Str) => &[&[Type::StringRef, Type::Str]],
        (Type::String, Type::U8Slice) => &[&[Type::StringRef, Type::U8Slice]],
        (Type::String, Type::U8Vec) => &[&[Type::String, Type::U8Vec]],
        (Type::String, Type::Path) => &[&[Type::StringRef, Type::Path]],
        (Type::String, Type::PathBuf) => &[&[Type::StringRef, Type::PathBuf]],
        (Type::String, Type::OsStr) => &[&[Type::StringRef, Type::OsStr]],
        (Type::String, Type::OsString) => &[&[Type::String, Type::OsString]],
        (Type::String, Type::CStr) => &[&[
            Type::StringRef,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::String, Type::CString) => &[&[
            Type::StringRef,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From &[u8]
        (Type::U8Slice, Type::Str) => {
            &[&[Type::U8Slice, Type::ResultStrOrUtf8Error]]
        }
        (Type::U8Slice, Type::String) => &[
            &[Type::U8Slice, Type::ResultStringOrFromUtf8Error],
            &[Type::U8Slice, Type::CowStr],
        ],
        (Type::U8Slice, Type::U8Vec) => &[&[Type::U8Slice, Type::U8Vec]],
        (Type::U8Slice, Type::Path) => {
            &[&[Type::U8Slice, Type::OsStr, Type::Path]]
        }
        (Type::U8Slice, Type::PathBuf) => {
            &[&[Type::U8Slice, Type::OsStr, Type::PathBuf]]
        }
        (Type::U8Slice, Type::OsStr) => &[&[Type::U8Slice, Type::OsStr]],
        (Type::U8Slice, Type::OsString) => {
            &[&[Type::U8Slice, Type::U8Vec, Type::OsString]]
        }
        (Type::U8Slice, Type::CStr) => {
            &[&[Type::U8Slice, Type::ResultCStrOrFromBytesWithNulError]]
        }
        (Type::U8Slice, Type::CString) => &[&[
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From Vec<u8>
        (Type::U8Vec, Type::Str) => {
            &[&[Type::U8VecRef, Type::ResultStrOrUtf8Error]]
        }
        (Type::U8Vec, Type::String) => {
            &[&[Type::U8Vec, Type::ResultStringOrFromUtf8Error]]
        }
        (Type::U8Vec, Type::U8Slice) => &[&[Type::U8VecRef, Type::U8Slice]],
        (Type::U8Vec, Type::Path) => {
            &[&[Type::U8VecRef, Type::OsStr, Type::Path]]
        }
        (Type::U8Vec, Type::PathBuf) => {
            &[&[Type::U8Vec, Type::OsString, Type::PathBuf]]
        }
        (Type::U8Vec, Type::OsStr) => &[&[Type::U8VecRef, Type::OsStr]],
        (Type::U8Vec, Type::OsString) => &[&[Type::U8Vec, Type::OsString]],
        (Type::U8Vec, Type::CStr) => {
            &[&[Type::U8VecRef, Type::ResultCStrOrFromBytesWithNulError]]
        }
        (Type::U8Vec, Type::CString) => &[&[
            Type::U8VecRef,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From &Path
        (Type::Path, Type::Str) => &[&[Type::Path, Type::OptionStr]],
        (Type::Path, Type::String) => &[&[Type::Path, Type::OptionString]],
        (Type::Path, Type::U8Slice) => {
            &[&[Type::Path, Type::OsStr, Type::U8Slice]]
        }
        (Type::Path, Type::U8Vec) => {
            &[&[Type::Path, Type::OsStr, Type::U8Slice, Type::U8Vec]]
        }
        (Type::Path, Type::PathBuf) => &[&[Type::Path, Type::PathBuf]],
        (Type::Path, Type::OsStr) => &[&[Type::Path, Type::OsStr]],
        (Type::Path, Type::OsString) => {
            &[&[Type::Path, Type::OsStr, Type::OsString]]
        }
        (Type::Path, Type::CStr) => &[&[
            Type::Path,
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::Path, Type::CString) => &[&[
            Type::Path,
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From PathBuf
        (Type::PathBuf, Type::Str) => {
            &[&[Type::PathBufRef, Type::Path, Type::OptionStr]]
        }
        (Type::PathBuf, Type::String) => {
            &[&[Type::PathBuf, Type::Path, Type::OptionString]]
        }
        (Type::PathBuf, Type::U8Slice) => {
            &[&[Type::PathBufRef, Type::OsStr, Type::U8Slice]]
        }
        (Type::PathBuf, Type::U8Vec) => {
            &[&[Type::PathBuf, Type::OsString, Type::U8Vec]]
        }
        (Type::PathBuf, Type::Path) => &[&[Type::PathBufRef, Type::Path]],
        (Type::PathBuf, Type::OsStr) => &[&[Type::PathBufRef, Type::OsStr]],
        (Type::PathBuf, Type::OsString) => &[&[Type::PathBuf, Type::OsString]],
        (Type::PathBuf, Type::CStr) => &[&[
            Type::PathBufRef,
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::PathBuf, Type::CString) => &[&[
            Type::PathBufRef,
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From &OsStr
        (Type::OsStr, Type::Str) => &[&[Type::OsStr, Type::OptionStr]],
        (Type::OsStr, Type::String) => &[&[Type::OsStr, Type::OptionString]],
        (Type::OsStr, Type::U8Slice) => &[&[Type::OsStr, Type::U8Slice]],
        (Type::OsStr, Type::U8Vec) => {
            &[&[Type::OsStr, Type::U8Slice, Type::U8Vec]]
        }
        (Type::OsStr, Type::Path) => &[&[Type::OsStr, Type::Path]],
        (Type::OsStr, Type::PathBuf) => &[&[Type::OsStr, Type::PathBuf]],
        (Type::OsStr, Type::OsString) => &[&[Type::OsStr, Type::OsString]],
        (Type::OsStr, Type::CStr) => &[&[
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::OsStr, Type::CString) => &[&[
            Type::OsStr,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From OsString
        (Type::OsString, Type::Str) => &[&[Type::OsStringRef, Type::OptionStr]],
        (Type::OsString, Type::String) => {
            &[&[Type::OsString, Type::ResultStringOrOsString]]
        }
        (Type::OsString, Type::U8Slice) => {
            &[&[Type::OsStringRef, Type::U8Slice]]
        }
        (Type::OsString, Type::U8Vec) => &[&[Type::OsString, Type::U8Vec]],
        (Type::OsString, Type::Path) => &[&[Type::OsStringRef, Type::Path]],
        (Type::OsString, Type::PathBuf) => &[&[Type::OsString, Type::PathBuf]],
        (Type::OsString, Type::OsStr) => &[&[Type::OsStringRef, Type::OsStr]],
        (Type::OsString, Type::CStr) => &[&[
            Type::OsStringRef,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
        ]],
        (Type::OsString, Type::CString) => &[&[
            Type::OsStringRef,
            Type::U8Slice,
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ]],

        // From &CStr
        (Type::CStr, Type::Str) => &[&[Type::CStr, Type::ResultStrOrUtf8Error]],
        (Type::CStr, Type::String) => &[&[
            Type::CStr,
            Type::ResultStrOrUtf8Error,
            Type::ResultStringOrUtf8Error,
        ]],
        // TODO: add lossy string conversion
        (Type::CStr, Type::U8Slice) => &[&[Type::CStr, Type::U8Slice]],
        (Type::CStr, Type::U8Vec) => {
            &[&[Type::CStr, Type::U8Slice, Type::U8Vec]]
        }
        (Type::CStr, Type::Path) => {
            &[&[Type::CStr, Type::U8Slice, Type::OsStr, Type::Path]]
        }
        (Type::CStr, Type::PathBuf) => &[&[
            Type::CStr,
            Type::U8Slice,
            Type::OsStr,
            Type::Path,
            Type::PathBuf,
        ]],
        (Type::CStr, Type::OsStr) => {
            &[&[Type::CStr, Type::U8Slice, Type::OsStr]]
        }
        (Type::CStr, Type::OsString) => {
            &[&[Type::CStr, Type::U8Slice, Type::OsStr, Type::OsString]]
        }
        (Type::CStr, Type::CString) => &[&[Type::CStr, Type::CString]],

        // From CString
        (Type::CString, Type::Str) => {
            &[&[Type::CStringRef, Type::CStr, Type::ResultStrOrUtf8Error]]
        }
        (Type::CString, Type::String) => {
            &[&[Type::CString, Type::ResultStringOrIntoStringError]]
        }
        // TODO: comment about nul termination variant
        (Type::CString, Type::U8Slice) => &[&[Type::CStringRef, Type::U8Slice]],
        // TODO: comment about nul termination variant
        (Type::CString, Type::U8Vec) => &[&[Type::CString, Type::U8Vec]],
        (Type::CString, Type::Path) => {
            &[&[Type::CStringRef, Type::U8Slice, Type::OsStr, Type::Path]]
        }
        (Type::CString, Type::PathBuf) => {
            &[&[Type::CString, Type::U8Vec, Type::OsString, Type::PathBuf]]
        }
        (Type::CString, Type::OsStr) => {
            &[&[Type::CStringRef, Type::U8Slice, Type::OsStr]]
        }
        (Type::CString, Type::OsString) => {
            &[&[Type::CString, Type::U8Vec, Type::OsString]]
        }
        (Type::CString, Type::CStr) => &[&[Type::CStringRef, Type::CStr]],

        _ => panic!("invalid conversion chain: {:?} -> {:?}", t1, t2),
    }
}

fn direct_conversion(t1: Type, t2: Type) -> Conversion {
    fn mkconv(format: &'static str) -> Conversion {
        Conversion {
            format,
            ..Default::default()
        }
    }

    match (t1, t2) {
        // From &str
        (Type::Str, Type::String) => mkconv("{}.to_string()"),
        (Type::Str, Type::U8Slice) => mkconv("{}.as_bytes()"),
        (Type::Str, Type::Path) => mkconv("Path::new({})"),
        (Type::Str, Type::PathBuf) => mkconv("PathBuf::from({})"),
        (Type::Str, Type::OsStr) => mkconv("OsStr::new({})"),
        (Type::Str, Type::OsString) => mkconv("OsString::from({})"),

        // From String
        (Type::StringRef, Type::Str) => mkconv("{}.as_str()"),
        (Type::StringRef, Type::U8Slice) => mkconv("{}.as_bytes()"),
        (Type::String, Type::U8Vec) => mkconv("{}.into_bytes()"),
        (Type::StringRef, Type::Path) => mkconv("Path::new({})"),
        (Type::StringRef, Type::PathBuf) => mkconv("PathBuf::from({})"),
        (Type::StringRef, Type::OsStr) => mkconv("OsStr::new({})"),
        (Type::String, Type::OsString) => mkconv("OsString::from({})"),

        // From &[u8]
        (Type::U8Slice, Type::ResultStrOrUtf8Error) => {
            mkconv("std::str::from_utf8({})")
        }
        (Type::U8Slice, Type::ResultStringOrFromUtf8Error) => {
            mkconv("String::from_utf8({}.to_vec())")
        }
        (Type::U8Slice, Type::CowStr) => mkconv("String::from_utf8_lossy({})"),
        (Type::U8Slice, Type::U8Vec) => mkconv("{}.to_vec()"),
        (Type::U8Slice, Type::OsStr) => {
            mkconv("OsStr::from_bytes({})").use_os_str_bytes()
        }
        (Type::U8Slice, Type::ResultCStrOrFromBytesWithNulError) => {
            mkconv("CStr::from_bytes_with_nul({})")
        }

        // From Vec<u8>
        (Type::U8VecRef, Type::ResultStrOrUtf8Error) => {
            mkconv("std::str::from_utf8({})")
        }
        (Type::U8Vec, Type::ResultStringOrFromUtf8Error) => {
            mkconv("String::from_utf8({})")
        }
        (Type::U8VecRef, Type::U8Slice) => mkconv("{}.as_slice()"),
        (Type::U8VecRef, Type::OsStr) => {
            mkconv("OsStr::from_bytes({})").use_os_str_bytes()
        }
        (Type::U8Vec, Type::OsString) => {
            mkconv("OsString::from_vec({})").use_os_string_bytes()
        }
        (Type::U8VecRef, Type::ResultCStrOrFromBytesWithNulError) => {
            mkconv("CStr::from_bytes_with_nul({})")
        }

        // From &OsStr
        (Type::OsStr, Type::OptionStr) => mkconv("{}.to_str()"),
        (Type::OsStr, Type::OptionString) => {
            mkconv("{}.to_str().map(|s| s.to_string())")
        }
        (Type::OsStr, Type::U8Slice) => {
            mkconv("{}.as_bytes()").use_os_str_bytes()
        }
        (Type::OsStr, Type::Path) => mkconv("Path::new({})"),
        (Type::OsStr, Type::PathBuf) => mkconv("PathBuf::from({})"),
        (Type::OsStr, Type::OsString) => mkconv("{}.to_os_string()"),

        // From OsString
        (Type::OsStringRef, Type::OptionStr) => mkconv("{}.to_str()"),
        (Type::OsString, Type::ResultStringOrOsString) => {
            mkconv("{}.into_string()")
        }
        (Type::OsStringRef, Type::U8Slice) => {
            mkconv("{}.as_bytes()").use_os_str_bytes()
        }
        (Type::OsString, Type::U8Vec) => {
            mkconv("{}.into_vec()").use_os_string_bytes()
        }
        (Type::OsStringRef, Type::Path) => mkconv("Path::new({})"),
        (Type::OsString, Type::PathBuf) => mkconv("PathBuf::from({})"),
        (Type::OsStringRef, Type::OsStr) => mkconv("{}.as_os_str()"),

        // From &Path
        (Type::Path, Type::OptionStr) => mkconv("{}.to_str()"),
        (Type::Path, Type::OptionString) => {
            mkconv("{}.to_str().map(|s| s.to_string())")
        }
        (Type::Path, Type::PathBuf) => mkconv("{}.to_path_buf()"),
        (Type::Path, Type::OsStr) => mkconv("{}.as_os_str()"),

        // From PathBuf
        (Type::PathBuf, Type::Path) => mkconv("{}.as_path()"),
        (Type::PathBufRef, Type::Path) => mkconv("{}.as_path()"),
        (Type::PathBufRef, Type::OsStr) => mkconv("{}.as_os_str()"),
        (Type::PathBuf, Type::OsString) => mkconv("{}.into_os_string()"),

        // From &CStr
        (Type::CStr, Type::ResultStrOrUtf8Error) => mkconv("{}.to_str()"),
        // TODO: add comment about the with nul option
        (Type::CStr, Type::U8Slice) => mkconv("{}.to_bytes()"),
        (Type::CStr, Type::CString) => mkconv("CString::from({})"),

        // From CString
        (Type::CStringRef, Type::CStr) => mkconv("{}.as_c_str()"),
        (Type::CString, Type::ResultStringOrIntoStringError) => {
            mkconv("{}.into_string()")
        }
        (Type::CStringRef, Type::U8Slice) => mkconv("{}.as_bytes()"),
        (Type::CString, Type::U8Vec) => mkconv("{}.into_bytes()"),

        (Type::ResultStrOrUtf8Error, Type::ResultStringOrUtf8Error) => {
            mkconv("{}.map(|s| s.to_string())")
        }
        (
            Type::ResultCStrOrFromBytesWithNulError,
            Type::ResultCStringOrFromBytesWithNulError,
        ) => mkconv("{}.map(CString::from)"),

        _ => panic!("invalid direct conversion: {:?} -> {:?}", t1, t2),
    }
}

struct Comment(Vec<String>);

impl Comment {
    fn new() -> Comment {
        Comment(Vec::new())
    }

    fn add_paragraph(&mut self, s: &str) {
        // Rewrap the input. The source string may be broken across
        // multiple lines, so first replace any newlines with a
        // space. Then collapse any double spaces.
        let line = s.replace('\n', " ").replace("  ", " ");
        let wrapped = textwrap::fill(&line, 72);

        self.0.push(wrapped);
    }

    fn format(&self) -> String {
        // Join the paragraphs together with a blank line in between
        let all = self.0.join("\n\n");

        // Add "// " to the beginning of each line
        let mut out = String::new();
        for line in all.lines() {
            out.push_str(&format!("// {}\n", line));
        }
        out
    }
}

#[derive(Default)]
struct Code {
    uses: BTreeSet<&'static str>,
    functions: String,
}

impl Code {
    /// Combine some use lines together for brevity.
    fn combine_uses(&self) -> BTreeSet<String> {
        let combos = &[
            ("std::ffi", "CStr", "CString"),
            ("std::ffi", "OsStr", "OsString"),
            ("std::path", "Path", "PathBuf"),
        ];

        // Make a copy of `uses` with `String` instead of `&str`
        let mut uses = self
            .uses
            .iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<_>>();

        for (pre, a, b) in combos {
            let full_a = format!("{}::{}", pre, a);
            let full_b = format!("{}::{}", pre, b);
            if uses.contains(&full_a) && uses.contains(&full_b) {
                uses.remove(&full_a);
                uses.remove(&full_b);
                uses.insert(format!("{}::{{{}, {}}}", pre, a, b));
            }
        }

        uses
    }

    fn gen(&self) -> String {
        let uses = self.combine_uses();

        format!(
            "{}\n\n{}",
            uses.iter()
                .map(|s| format!("use {};", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.functions
        )
    }
}

fn gen_one_conversion(
    anchor1: Type,
    anchor2: Type,
    chain: &'static [Type],
    code: &mut Code,
) {
    let mut expr = "input".to_string();

    let input_type = chain.first().unwrap();
    let output_type = chain.last().unwrap();
    let mut unix_only = false;

    for (t3, t4) in chain.iter().zip(chain.iter().skip(1)) {
        let conv = direct_conversion(*t3, *t4);
        expr = conv.format_expr(expr);
        code.uses.extend(t3.uses());
        code.uses.extend(t4.uses());
        code.uses.extend(conv.uses());
        if conv.unix_only() {
            unix_only = true;
        }
    }

    let mut suffix = String::new();
    if unix_only {
        suffix.push_str("_unix");
    }
    if *output_type == Type::CowStr {
        suffix.push_str("_lossy");
    }

    let func = format!(
        "pub fn {}_to_{}{}(input: {}) -> {} {{\n    {}\n}}",
        anchor1.short_name(),
        anchor2.short_name(),
        suffix,
        input_type.type_str(),
        output_type.type_str(),
        expr
    );

    let mut comment = Comment::new();

    if unix_only {
        comment.add_paragraph("This conversion is only allowed on Unix.");
    }

    if let Some(para) = output_type.return_comment() {
        comment.add_paragraph(para);
    }

    code.functions.push_str(&comment.format());
    code.functions.push_str(&func);
    code.functions.push_str("\n\n");
}

fn gen_code(t1: Type) -> Code {
    let mut code = Code::default();
    for t2 in Type::anchors() {
        if t1 == *t2 {
            continue;
        }

        let chains = conversion_chains(t1, *t2);
        for chain in chains {
            gen_one_conversion(t1, *t2, chain, &mut code);
        }
    }
    code
}

#[throws]
fn run_cargo_cmd(cmd: &str) {
    Command::new("cargo").add_arg(cmd).set_dir("gen").run()?;
}

fn gen_lib_code(mod_names: &[String]) -> String {
    let pub_mods = mod_names
        .iter()
        .map(|s| format!("pub mod {};\n", s))
        .collect::<Vec<_>>()
        .join("");

    format!(
        "
// The conversion functions use some argument types that you don't
// ordinarly see, such as `&String`. The types are normally implicit,
// for example `String::as_str` takes a `&String`. Since all of our
// conversions are in separate functions, we have to explicitly use
// these types.
#![allow(clippy::ptr_arg)]

{}",
        pub_mods
    )
}

/// Generate the Rust files, format them, run clippy, and build.
///
/// Returns a vec mapping from the type being converted from to the
/// path of the generated Rust file.
#[throws]
fn gen_and_build_sources() -> Vec<(Type, PathBuf)> {
    let gen_path = Path::new("gen/src");
    let mut mods = Vec::new();
    let mut out = Vec::new();

    for t1 in Type::anchors() {
        let mod_name = format!("from_{}", t1.short_name());
        mods.push(mod_name.clone());

        let path = gen_path.join(format!("{}.rs", mod_name));
        fs::write(&path, gen_code(*t1).gen())?;
        out.push((*t1, path));
    }

    fs::write(gen_path.join("lib.rs"), gen_lib_code(&mods))?;

    run_cargo_cmd("fmt")?;
    run_cargo_cmd("clippy")?;
    run_cargo_cmd("build")?;

    out
}

#[derive(Template)]
#[template(path = "index.html", escape = "none")]
struct IndexTemplate {
    nav: String,
    content: String,
}

impl IndexTemplate {
    #[throws]
    fn write(&self, path: &Path) {
        fs::write(path, self.render()?)?;
    }
}

struct Highlighter {
    ss: SyntaxSet,
    // TODO
    syntax: SyntaxReference,
    theme: Theme,
}

impl Highlighter {
    fn new() -> Highlighter {
        let ss = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();
        let mut theme = ts.themes["InspiredGitHub"].clone();

        theme.settings.background = Some(Color {
            r: 243,
            g: 246,
            b: 250,
            a: 255,
        });

        let syntax = ss.find_syntax_by_extension("rs").unwrap().clone();

        Highlighter { ss, syntax, theme }
    }

    fn highlight(&self, code: &str) -> String {
        highlighted_html_for_string(code, &self.ss, &self.syntax, &self.theme)
    }
}

#[throws]
fn gen_html_content(gen: &[(Type, PathBuf)]) -> String {
    let mut out = String::new();
    let highlighter = Highlighter::new();

    for (t1, path) in gen {
        let code = fs::read_to_string(path)?;
        let highlighted = highlighter.highlight(&code);

        out.push_str(&format!(
            "<a name={}><h2>From <code>{}</code></h2></a>",
            t1.short_name(),
            t1.html_type_str(),
        ));
        out.push_str(&highlighted);
    }
    out
}

fn gen_html_nav() -> String {
    let mut nav = "<ul>".to_string();
    for a in Type::anchors() {
        nav += &format!(
            "<li><a href=\"#{}\">From <code>{}</code></a></li>",
            a.short_name(),
            a.html_type_str()
        );
    }
    nav += "</ul>";
    nav
}

#[throws]
fn main() {
    let gen = gen_and_build_sources()?;

    IndexTemplate {
        nav: gen_html_nav(),
        content: gen_html_content(&gen)?,
    }
    .write(Path::new("docs/index.html"))?;
}

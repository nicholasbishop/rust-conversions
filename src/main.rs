use command_run::Command;
use std::collections::BTreeSet;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    Str,
    String,
    U8Slice,
    U8Vec,
    Path,
    PathBuf,
    OsStr,
    OsString,
    // TODO: CStr
    // TODO: CString

    // Ordinarily you never see these types in a function signature,
    // but they often show up as temporary types that you don't
    // explicitly see. For example, `String::as_str` takes a
    // `&String`. Since all of our conversions are in a separate
    // function, we have to explicitly use these types.
    StringRef,
    U8VecRef,
    OsStringRef,
    PathBufRef,

    OptionStr,
    OptionString,
    ResultStrOrUtf8Error,
    ResultStringOrFromUtf8Error,
    ResultStringOrOsString,
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

            Type::StringRef => "&String",
            Type::U8VecRef => "&Vec<u8>",
            Type::PathBufRef => "&PathBuf",
            Type::OsStringRef => "&OsString",

            Type::OptionStr => "Option<&str>",
            Type::OptionString => "Option<String>",
            Type::ResultStrOrUtf8Error => "Result<&str, Utf8Error>",
            Type::ResultStringOrFromUtf8Error => {
                "Result<String, FromUtf8Error>"
            }
            Type::ResultStringOrOsString => "Result<String, OsString>",
        }
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

            _ => panic!("no short name for {:?}", self),
        }
    }

    fn uses(&self) -> &'static [&'static str] {
        match self {
            Type::Path => &["std::path::Path"],
            Type::PathBuf => &["std::path::PathBuf"],
            Type::OsStr => &["std::ffi::OsStr"],
            Type::OsString => &["std::ffi::OsString"],
            Type::ResultStrOrUtf8Error => &["std::str::Utf8Error"],
            Type::ResultStringOrFromUtf8Error => {
                &["std::string::FromUtf8Error"]
            }
            _ => &[],
        }
    }
}

fn conversion_chain(t1: Type, t2: Type) -> &'static [Type] {
    match (t1, t2) {
        // From &str
        (Type::Str, Type::String) => &[Type::Str, Type::String],
        (Type::Str, Type::U8Slice) => &[Type::Str, Type::U8Slice],
        (Type::Str, Type::U8Vec) => &[Type::Str, Type::U8Slice, Type::U8Vec],
        (Type::Str, Type::Path) => &[Type::Str, Type::Path],
        (Type::Str, Type::PathBuf) => &[Type::Str, Type::PathBuf],
        (Type::Str, Type::OsStr) => &[Type::Str, Type::OsStr],
        (Type::Str, Type::OsString) => &[Type::Str, Type::OsString],

        // From String
        (Type::String, Type::Str) => &[Type::StringRef, Type::Str],
        (Type::String, Type::U8Slice) => &[Type::StringRef, Type::U8Slice],
        (Type::String, Type::U8Vec) => &[Type::String, Type::U8Vec],
        (Type::String, Type::Path) => &[Type::StringRef, Type::Path],
        (Type::String, Type::PathBuf) => &[Type::StringRef, Type::PathBuf],
        (Type::String, Type::OsStr) => &[Type::StringRef, Type::OsStr],
        (Type::String, Type::OsString) => &[Type::String, Type::OsString],

        // From &[u8]
        (Type::U8Slice, Type::Str) => {
            &[Type::U8Slice, Type::ResultStrOrUtf8Error]
        }
        (Type::U8Slice, Type::String) => {
            &[Type::U8Slice, Type::ResultStringOrFromUtf8Error]
        }
        (Type::U8Slice, Type::U8Vec) => &[Type::U8Slice, Type::U8Vec],
        (Type::U8Slice, Type::Path) => {
            &[Type::U8Slice, Type::OsStr, Type::Path]
        }
        (Type::U8Slice, Type::PathBuf) => {
            &[Type::U8Slice, Type::OsStr, Type::PathBuf]
        }
        (Type::U8Slice, Type::OsStr) => &[Type::U8Slice, Type::OsStr],
        (Type::U8Slice, Type::OsString) => {
            &[Type::U8Slice, Type::U8Vec, Type::OsString]
        }

        // From Vec<u8>
        (Type::U8Vec, Type::Str) => {
            &[Type::U8VecRef, Type::ResultStrOrUtf8Error]
        }
        (Type::U8Vec, Type::String) => {
            &[Type::U8Vec, Type::ResultStringOrFromUtf8Error]
        }
        (Type::U8Vec, Type::U8Slice) => &[Type::U8VecRef, Type::U8Slice],
        (Type::U8Vec, Type::Path) => &[Type::U8VecRef, Type::OsStr, Type::Path],
        (Type::U8Vec, Type::PathBuf) => {
            &[Type::U8Vec, Type::OsString, Type::PathBuf]
        }
        (Type::U8Vec, Type::OsStr) => &[Type::U8VecRef, Type::OsStr],
        (Type::U8Vec, Type::OsString) => &[Type::U8Vec, Type::OsString],

        // From &Path
        (Type::Path, Type::Str) => &[Type::Path, Type::OptionStr],
        (Type::Path, Type::String) => &[Type::Path, Type::OptionString],
        (Type::Path, Type::U8Slice) => {
            &[Type::Path, Type::OsStr, Type::U8Slice]
        }
        (Type::Path, Type::U8Vec) => {
            &[Type::Path, Type::OsStr, Type::U8Slice, Type::U8Vec]
        }
        (Type::Path, Type::PathBuf) => &[Type::Path, Type::PathBuf],
        (Type::Path, Type::OsStr) => &[Type::Path, Type::OsStr],
        (Type::Path, Type::OsString) => {
            &[Type::Path, Type::OsStr, Type::OsString]
        }

        // From PathBuf
        (Type::PathBuf, Type::Str) => {
            &[Type::PathBufRef, Type::Path, Type::OptionStr]
        }
        (Type::PathBuf, Type::String) => {
            &[Type::PathBuf, Type::Path, Type::OptionString]
        }
        (Type::PathBuf, Type::U8Slice) => {
            &[Type::PathBufRef, Type::OsStr, Type::U8Slice]
        }
        (Type::PathBuf, Type::U8Vec) => {
            &[Type::PathBuf, Type::OsString, Type::U8Vec]
        }
        (Type::PathBuf, Type::Path) => &[Type::PathBufRef, Type::Path],
        (Type::PathBuf, Type::OsStr) => &[Type::PathBufRef, Type::OsStr],
        (Type::PathBuf, Type::OsString) => &[Type::PathBuf, Type::OsString],

        // From &OsStr
        (Type::OsStr, Type::Str) => &[Type::OsStr, Type::OptionStr],
        (Type::OsStr, Type::String) => &[Type::OsStr, Type::OptionString],
        (Type::OsStr, Type::U8Slice) => &[Type::OsStr, Type::U8Slice],
        (Type::OsStr, Type::U8Vec) => {
            &[Type::OsStr, Type::U8Slice, Type::U8Vec]
        }
        (Type::OsStr, Type::Path) => &[Type::OsStr, Type::Path],
        (Type::OsStr, Type::PathBuf) => &[Type::OsStr, Type::PathBuf],
        (Type::OsStr, Type::OsString) => &[Type::OsStr, Type::OsString],

        // From OsString
        (Type::OsString, Type::Str) => &[Type::OsStringRef, Type::OptionStr],
        (Type::OsString, Type::String) => {
            &[Type::OsString, Type::ResultStringOrOsString]
        }
        (Type::OsString, Type::U8Slice) => &[Type::OsStringRef, Type::U8Slice],
        (Type::OsString, Type::U8Vec) => &[Type::OsString, Type::U8Vec],
        (Type::OsString, Type::Path) => &[Type::OsStringRef, Type::Path],
        (Type::OsString, Type::PathBuf) => &[Type::OsString, Type::PathBuf],
        (Type::OsString, Type::OsStr) => &[Type::OsStringRef, Type::OsStr],

        _ => panic!("invalid conversion chain: {:?} -> {:?}", t1, t2),
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
        (Type::U8Slice, Type::U8Vec) => mkconv("{}.to_vec()"),
        (Type::U8Slice, Type::OsStr) => {
            mkconv("OsStr::from_bytes({})").use_os_str_bytes()
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

        // From &OsStr
        (Type::OsStr, Type::OptionStr) => mkconv("{}.to_str()"),
        (Type::OsStr, Type::OptionString) => {
            mkconv("{}.to_str().map(|s| s.to_string())")
        }
        (Type::OsStr, Type::U8Slice) => mkconv("{}.as_bytes()"),
        (Type::OsStr, Type::Path) => mkconv("Path::new({})"),
        (Type::OsStr, Type::PathBuf) => mkconv("PathBuf::from({})"),
        (Type::OsStr, Type::OsString) => mkconv("{}.to_os_string()"),

        // From OsString
        (Type::OsStringRef, Type::OptionStr) => mkconv("{}.to_str()"),
        (Type::OsString, Type::ResultStringOrOsString) => {
            mkconv("{}.into_string()")
        }
        (Type::OsStringRef, Type::U8Slice) => mkconv("{}.as_bytes()"),
        (Type::OsString, Type::U8Vec) => mkconv("{}.into_vec()"),
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

        _ => panic!("invalid direct conversion: {:?} -> {:?}", t1, t2),
    }
}

#[derive(Default)]
struct Code {
    uses: BTreeSet<&'static str>,
    functions: String,
}

impl Code {
    fn gen(&self) -> String {
        format!(
            "{}\n\n{}",
            self.uses
                .iter()
                .map(|s| format!("use {};", s))
                .collect::<Vec<_>>()
                .join("\n"),
            self.functions
        )
    }
}

fn gen_one_conversion(anchor1: Type, anchor2: Type, code: &mut Code) {
    let mut expr = "input".to_string();
    let chain = conversion_chain(anchor1, anchor2);

    let input_type = chain.first().unwrap();
    let output_type = chain.last().unwrap();

    for (t3, t4) in chain.iter().zip(chain.iter().skip(1)) {
        let conv = direct_conversion(*t3, *t4);
        expr = conv.format_expr(expr);
        code.uses.extend(t3.uses());
        code.uses.extend(t4.uses());
        code.uses.extend(conv.uses());
    }

    let func = format!(
        "pub fn {}_to_{}(input: {}) -> {} {{\n    {}\n}}",
        anchor1.short_name(),
        anchor2.short_name(),
        input_type.type_str(),
        output_type.type_str(),
        expr
    );

    code.functions.push_str(&func);
    code.functions.push_str("\n\n");
}

fn gen_code() -> Code {
    let mut code = Code::default();
    for t1 in Type::anchors() {
        for t2 in Type::anchors() {
            if t1 == t2 {
                continue;
            }

            gen_one_conversion(*t1, *t2, &mut code);
        }
    }
    code
}

fn main() {
    fs::write("gen/src/lib.rs", gen_code().gen()).unwrap();

    Command::new("cargo")
        .add_arg("fmt")
        .set_dir("gen")
        .run()
        .unwrap();

    Command::new("cargo")
        .add_arg("check")
        .set_dir("gen")
        .run()
        .unwrap();
}

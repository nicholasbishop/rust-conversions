use command_run::Command;
use std::collections::BTreeSet;
use std::fs;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    Str,
    String,
    U8Slice,
    U8Vec,

    StringRef,
    U8VecRef,

    ResultStrOrUtf8Error,
    ResultStringOrFromUtf8Error,
}

impl Type {
    fn anchors() -> &'static [Type] {
        &[Type::Str, Type::String, Type::U8Slice, Type::U8Vec]
    }

    fn type_str(&self) -> &'static str {
        match self {
            Type::Str => "&str",
            Type::String => "String",
            Type::U8Slice => "&[u8]",
            Type::U8Vec => "Vec<u8>",

            Type::StringRef => "&String",
            Type::U8VecRef => "&Vec<u8>",

            Type::ResultStrOrUtf8Error => "Result<&str, Utf8Error>",
            Type::ResultStringOrFromUtf8Error => {
                "Result<String, FromUtf8Error>"
            }
        }
    }

    fn short_name(&self) -> &'static str {
        match self {
            Type::Str => "str",
            Type::String => "string",
            Type::U8Slice => "u8_slice",
            Type::U8Vec => "u8_vec",

            _ => panic!("no short name for {:?}", self),
        }
    }

    fn uses(&self) -> &'static [&'static str] {
        match self {
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
        (Type::Str, Type::String) => &[Type::Str, Type::String],
        (Type::Str, Type::U8Slice) => &[Type::Str, Type::U8Slice],
        (Type::Str, Type::U8Vec) => &[Type::Str, Type::U8Slice, Type::U8Vec],

        (Type::String, Type::Str) => &[Type::StringRef, Type::Str],
        (Type::String, Type::U8Slice) => &[Type::StringRef, Type::U8Slice],
        (Type::String, Type::U8Vec) => &[Type::String, Type::U8Vec],

        (Type::U8Slice, Type::Str) => {
            &[Type::U8Slice, Type::ResultStrOrUtf8Error]
        }
        (Type::U8Slice, Type::String) => {
            &[Type::U8Slice, Type::ResultStringOrFromUtf8Error]
        }
        (Type::U8Slice, Type::U8Vec) => &[Type::U8Slice, Type::U8Vec],

        (Type::U8Vec, Type::Str) => {
            &[Type::U8VecRef, Type::ResultStrOrUtf8Error]
        }
        (Type::U8Vec, Type::String) => {
            &[Type::U8Vec, Type::ResultStringOrFromUtf8Error]
        }
        (Type::U8Vec, Type::U8Slice) => &[Type::U8VecRef, Type::U8Slice],

        _ => panic!("invalid conversion chain: {:?} -> {:?}", t1, t2),
    }
}

fn direct_conversion(expr: &str, t1: Type, t2: Type) -> String {
    match (t1, t2) {
        (Type::Str, Type::String) => format!("{}.to_string()", expr),
        (Type::Str, Type::U8Slice) => format!("{}.as_bytes()", expr),

        (Type::StringRef, Type::Str) => format!("{}.as_str()", expr),
        (Type::StringRef, Type::U8Slice) => format!("{}.as_bytes()", expr),
        (Type::String, Type::U8Vec) => format!("{}.into_bytes()", expr),

        (Type::U8Slice, Type::ResultStrOrUtf8Error) => {
            format!("std::str::from_utf8({})", expr)
        }
        (Type::U8Slice, Type::ResultStringOrFromUtf8Error) => {
            format!("String::from_utf8({}.to_vec())", expr)
        }
        (Type::U8Slice, Type::U8Vec) => format!("{}.to_vec()", expr),

        (Type::U8VecRef, Type::ResultStrOrUtf8Error) => {
            format!("std::str::from_utf8({})", expr)
        }
        (Type::U8Vec, Type::ResultStringOrFromUtf8Error) => {
            format!("String::from_utf8({})", expr)
        }
        (Type::U8VecRef, Type::U8Slice) => format!("{}.as_slice()", expr),

        _ => panic!("invalid direct conversion: {:?} -> {:?}", t1, t2),
    }
}

#[derive(Default)]
struct Code {
    uses: BTreeSet<String>,
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
        expr = direct_conversion(&expr, *t3, *t4);
        code.uses.extend(t3.uses().iter().map(|s| s.to_string()));
        code.uses.extend(t4.uses().iter().map(|s| s.to_string()));
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

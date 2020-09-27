#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    Str,
    String,
    U8Slice,
    U8Vec,

    StringRef,
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
        }
    }

    fn short_name(&self) -> &'static str {
        match self {
            Type::Str => "str",
            Type::String => "string",
            Type::U8Slice => "u8_slice",
            Type::U8Vec => "u8_vec",
            Type::StringRef => panic!("no short name for {:?}", self),
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

        (Type::U8Slice, Type::Str) => &[Type::U8Slice, Type::Str],
        (Type::U8Slice, Type::String) => &[Type::U8Slice, Type::String],
        (Type::U8Slice, Type::U8Vec) => &[Type::U8Slice, Type::U8Vec],

        // TODO
        (Type::U8Vec, Type::Str) => &[Type::U8Vec, Type::Str],
        (Type::U8Vec, Type::String) => &[Type::U8Vec, Type::String],
        (Type::U8Vec, Type::U8Slice) => &[Type::U8Vec, Type::U8Slice],

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

        (Type::U8Slice, Type::Str) => format!("std::str::from_utf8({})", expr),
        (Type::U8Slice, Type::String) => {
            format!("String::from_utf8({}.to_vec())", expr)
        }
        (Type::U8Slice, Type::U8Vec) => format!("{}.to_vec()", expr),

        (Type::U8Vec, Type::Str) => format!("std::str::from_utf8({})", expr),
        (Type::U8Vec, Type::String) => format!("String::from_utf8({})", expr),
        (Type::U8Vec, Type::U8Slice) => format!("{}.as_slice()", expr),

        _ => panic!("invalid direct conversion: {:?} -> {:?}", t1, t2),
    }
}

fn main() {
    for t1 in Type::anchors() {
        for t2 in Type::anchors() {
            if t1 == t2 {
                continue;
            }

            let mut expr = "input".to_string();
            let chain = conversion_chain(*t1, *t2);

            for (t3, t4) in chain.iter().zip(chain.iter().skip(1)) {
                expr = direct_conversion(&expr, *t3, *t4);
            }

            // TODO: func name
            let func = format!(
                "fn {}_to_{}(input: {}) -> {} {{\n    {};\n}}",
                t1.short_name(),
                t2.short_name(),
                t1.type_str(),
                t2.type_str(),
                expr
            );

            println!("{}", func);
        }
    }
}

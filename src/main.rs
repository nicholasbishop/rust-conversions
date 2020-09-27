#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Type {
    Str,
    String,
    U8Slice,

    StringRef,
}

impl Type {
    const fn anchors() -> &'static [Type] {
        &[Type::Str, Type::String, Type::U8Slice]
    }
}

fn conversion_chain(t1: Type, t2: Type) -> &'static [Type] {
    match (t1, t2) {
        (Type::Str, Type::String) => &[Type::Str, Type::String],
        (Type::Str, Type::U8Slice) => &[Type::Str, Type::U8Slice],

        (Type::String, Type::Str) => &[Type::StringRef, Type::Str],
        (Type::String, Type::U8Slice) => &[Type::StringRef, Type::U8Slice],

        (Type::U8Slice, Type::Str) => &[Type::U8Slice, Type::Str],
        (Type::U8Slice, Type::String) => &[Type::U8Slice, Type::String],

        _ => panic!("invalid conversion chain: {:?} -> {:?}", t1, t2),
    }
}

fn direct_conversion(expr: &str, t1: Type, t2: Type) -> String {
    if t1 == Type::Str && t2 == Type::String {
        format!("{}.to_string()", expr)
    } else if t1 == Type::Str && t2 == Type::U8Slice {
        format!("{}.as_bytes()", expr)
    } else if t1 == Type::StringRef && t2 == Type::Str {
        format!("{}.as_str()", expr)
    } else if t1 == Type::StringRef && t2 == Type::U8Slice {
        format!("{}.as_bytes()", expr)
    } else if t1 == Type::U8Slice && t2 == Type::Str {
        format!("std::str::from_utf8({})", expr)
    } else if t1 == Type::U8Slice && t2 == Type::String {
        format!("String::from_utf8({}.to_vec())", expr)
    } else {
        panic!("invalid direct conversion: {:?} -> {:?}", t1, t2);
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

            // TODO can probably clean up this iteration with zip somehow
            for (t3, t4) in chain.iter().zip(chain.iter().skip(1)) {
                expr = direct_conversion(&expr, *t3, *t4);
            }

            println!("{:?} -> {:?}: {}", t1, t2, expr);
        }
    }
}

use pecan_types::google::protobuf::descriptor_pb::*;
use std::io::Write;
use std::path::{Component, Path};
use std::{
    io::Read,
    process::{Command, Stdio},
};

pub fn target_path(proto_path: &str) -> String {
    let parts = rust_module(proto_path);
    let mut p = parts.join("/");
    p.push_str(".rs");
    p
}

pub fn rust_module(proto_path: &str) -> Vec<String> {
    let p = Path::new(&proto_path);
    let mut parts: Vec<_> = p
        .components()
        .flat_map(|p| match p {
            Component::Normal(s) => s.to_str().map(|s| s.replace('.', "_")),
            _ => None,
        })
        .collect();
    let mut file_name = parts.pop().unwrap();
    if let Some(pos) = file_name.rfind('_') {
        file_name.truncate(pos);
    }
    file_name.push_str("_pb");
    parts.push(file_name);
    parts
}

const KEY_WORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "dyn", "union",
];

pub fn escape(name: &str) -> String {
    if KEY_WORDS.contains(&name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

#[derive(Clone, Copy)]
enum LastChar {
    Begin,
    Start,
    Cont(char),
    End,
}

pub fn camel_name(name: &str) -> String {
    let mut s = String::new();
    let mut last_char = LastChar::Begin;
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            match last_char {
                LastChar::Start => {
                    last_char = LastChar::Cont(c);
                }
                LastChar::Cont(l) => {
                    s.push(l.to_ascii_lowercase());
                    last_char = LastChar::Cont(c);
                }
                LastChar::End | LastChar::Begin => {
                    s.push(c);
                    last_char = LastChar::Start;
                }
            }
        } else {
            if let LastChar::Cont(l) = last_char {
                s.push(l);
            }
            if let LastChar::Begin = last_char {
                s.push(c.to_ascii_uppercase());
                if c != '_' {
                    last_char = LastChar::Start;
                }
            } else {
                s.push(c);
                if c == '_' {
                    last_char = LastChar::Begin;
                } else {
                    last_char = LastChar::End;
                }
            }
        }
    }
    if let LastChar::Cont(l) = last_char {
        s.push(l.to_ascii_lowercase());
    }
    s
}

pub fn snake_name(name: &str) -> String {
    let mut b = false;
    let mut s = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_uppercase() {
            if b {
                s.push('_');
            }
            s.push(c.to_ascii_lowercase());
            b = false;
        } else {
            s.push(c);
            b = c != '_';
        }
    }
    s
}

pub fn split<K: Clone, V: Clone>(v: &[(K, V)]) -> (Vec<K>, Vec<V>) {
    (
        v.iter().map(|(k, _)| k.clone()).collect(),
        v.iter().map(|(_, v)| v.clone()).collect(),
    )
}

pub fn package_prefix(f: &FileDescriptorProto) -> String {
    match &f.package {
        Some(n) if !n.is_empty() => format!(".{}.", n),
        _ => ".".to_string(),
    }
}

pub fn type_name(name: &str, prefix: &str) -> String {
    if prefix.is_empty() {
        camel_name(name)
    } else {
        format!("{}_{}", prefix, camel_name(name))
    }
}

pub fn rustfmt(data: &str) -> String {
    let mut child = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let mut input = child.stdin.take().unwrap();
    let mut output = child.stdout.take().unwrap();
    let thread = std::thread::spawn(move || {
        let mut result = String::new();
        output.read_to_string(&mut result).unwrap();
        result
    });
    input.write_all(data.as_bytes()).unwrap();
    drop(input);
    child.wait().unwrap();
    thread.join().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rusty_name() {
        let cases = vec![
            ("abcd", "Abcd"),
            ("aBCd", "AbCd"),
            ("aBcd", "ABcd"),
            ("aBcD", "ABcD"),
            ("ABCD", "Abcd"),
            ("A", "A"),
            ("a", "A"),
            ("a123_45bcd", "A123_45bcd"),
            ("a123_bcd", "A123_Bcd"),
            ("a123_BCD", "A123_Bcd"),
            ("a123_BCD123", "A123_BcD123"),
            ("ABcd", "ABcd"),
            ("ABCd", "AbCd"),
        ];
        for (s, t) in cases {
            let res = super::camel_name(s);
            assert_eq!(res, t, "{}", s);
        }
    }

    #[test]
    fn test_snake_name() {
        let cases = vec![
            ("Abcd", "abcd"),
            ("abcd", "abcd"),
            ("A_B_C_D", "a_b_c_d"),
            ("ABCD", "abcd"),
            ("ABcdEFGh", "abcd_efgh"),
            ("aBcD", "a_bc_d"),
            ("A", "a"),
            ("_", "_"),
            ("_A", "_a"),
            ("A_", "a_"),
            ("A123_45bcd", "a123_45bcd"),
            ("A123_B45bcd", "a123_b45bcd"),
            ("A123_b45bcd", "a123_b45bcd"),
        ];
        for (s, t) in cases {
            let res = super::snake_name(s);
            assert_eq!(res, t, "{}", s);
        }
    }
}

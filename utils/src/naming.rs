pub fn module_name(file_name: &str) -> String {
    let base_name = strip_proto(file_name);
    let mut name = Vec::with_capacity(base_name.len() + 3);
    let mut start = 0;
    for (i, b) in base_name.bytes().enumerate() {
        if b != b'-' && b != b'/' {
            continue;
        }

        if start + 1 < i {
            name.extend_from_slice(&base_name.as_bytes()[start..i]);
        }
        if b == b'-' {
            name.push(b'_');
        }
        if b == b'/' {
            name.push(b':');
            name.push(b':');
        }
        start = i + 1;
    }
    if start < base_name.len() {
        name.extend_from_slice(&base_name.as_bytes()[start..]);
    }
    name.extend_from_slice(b"_pb");
    unsafe { String::from_utf8_unchecked(name) }
}

pub fn alias_name(file_name: &str) -> String {
    let base_name = strip_proto(file_name);
    let mut name = Vec::with_capacity(base_name.len() + 3);
    let mut start = 0;
    for (i, b) in base_name.bytes().enumerate() {
        if b != b'-' && b != b'/' {
            continue;
        }

        if start + 1 < i {
            name.extend_from_slice(&base_name.as_bytes()[start..i]);
        }
        if b == b'-' {
            name.push(b'_');
        }
        if b == b'/' {
            name.push(b'_');
            name.push(b'_');
        }
        start = i + 1;
    }
    if start < base_name.len() {
        name.extend_from_slice(&base_name.as_bytes()[start..]);
    }
    name.extend_from_slice(b"_pb");
    unsafe { String::from_utf8_unchecked(name) }
}

pub fn file_name(module_name: &str) -> String {
    let mut file_name = module_name.replace("::", "/");
    file_name.push_str(".rs");
    file_name
}

pub fn strip_proto(file_name: &str) -> &str {
    if file_name.ends_with(".protodevel") {
        file_name.trim_end_matches(".protodevel")
    } else {
        file_name.trim_end_matches(".proto")
    }
}

fn camel_case_part(bytes: &mut [u8]) {
    let mut upper_count = 0;
    for i in 0..bytes.len() {
        if bytes[i].is_ascii_uppercase() {
            if upper_count > 1 {
                bytes[i - 1] = bytes[i - 1].to_ascii_lowercase();
            } else {
                upper_count += 1;
            }
        } else if bytes[i].is_ascii_lowercase() {
            if i == 0 {
                bytes[i] = bytes[i].to_ascii_uppercase();
            }
            upper_count = 0;
        } else {
            if upper_count > 1 {
                bytes[i - 1] = bytes[i - 1].to_ascii_lowercase();
            }
            upper_count = 0;
        }
    }
    if upper_count > 1 {
        bytes[bytes.len() - 1] = bytes[bytes.len() - 1].to_ascii_lowercase();
    }
}

pub fn camel_case(name: &str) -> String {
    let mut dst_string = Vec::with_capacity(name.len());
    let bytes = name.as_bytes();
    let mut previous = 0;
    let mut cursor = 0;
    for i in 0..bytes.len() {
        if bytes[i] == b'_' {
            if previous != i {
                cursor = dst_string.len();
                dst_string.extend_from_slice(&bytes[previous..i]);
                camel_case_part(&mut dst_string[cursor..]);
            }
            previous = i + 1;
        }
    }
    if previous < bytes.len() {
        cursor = dst_string.len();
        dst_string.extend_from_slice(&bytes[previous..]);
        camel_case_part(&mut dst_string[cursor..]);
    }
    if cursor != 0 {
        camel_case_part(&mut dst_string);
    }
    unsafe { String::from_utf8_unchecked(dst_string) }
}

pub fn snake_case(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut s = Vec::with_capacity(bytes.len());
    let mut previous_byte = b'_';
    for b in bytes {
        if b.is_ascii_uppercase() {
            if previous_byte != b'_' {
                s.push(b'_');
            }
            s.push(b.to_ascii_lowercase());
        } else if *b == b'_' {
            if *b != previous_byte {
                s.push(*b);
            }
        } else {
            s.push(*b)
        }
        previous_byte = *b;
    }

    unsafe { String::from_utf8_unchecked(s) }
}

pub const KEYWORDS: &[&str; 52] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try", "dyn",
];

// TODO: can be faster.
pub fn sanitize_name(s: String) -> String {
    if !KEYWORDS.contains(&s.as_str()) {
        s
    } else {
        format!("r#{}", s)
    }
}

#[inline]
pub fn field_name(name: &str) -> String {
    sanitize_name(snake_case(name))
}

#[inline]
pub fn type_name(parts: &[String]) -> String {
    sanitize_name(camel_case(&parts.join("Nested")))
}

pub fn full_name(pkg: &str, parts: &[String]) -> String {
    let example_len = parts.get(0).map_or(0, |p| p.len());
    let mut abs_name = String::with_capacity(1 + pkg.len() + (example_len + 1) * parts.len());
    if !pkg.is_empty() {
        abs_name.push('.');
        abs_name.push_str(pkg);
    }
    for part in parts {
        abs_name.push('.');
        abs_name.push_str(part);
    }
    abs_name
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_camel_case() {
        let cases = vec![
            ("abcdef", "Abcdef"),
            ("aBCDEF", "ABcdef"),
            ("A_B_C_D_E", "Abcde"),
            ("ABC_DEF_GHK", "AbcDefGhk"),
            ("ABC_DEF_GHK32", "AbcDefGhk32"),
            ("A__B_C__D_E", "Abcde"),
            ("a__b_C__D_E", "Abcde"),
            ("a__bc_D__E_F", "ABcDef"),
            ("abCDE_", "AbCde"),
            ("abCdE_", "AbCdE"),
            ("AbCdEf", "AbCdEf"),
            ("AbCdEf123", "AbCdEf123"),
            ("AbCdEF123", "AbCdEf123"),
            ("AbCdEF123Def", "AbCdEf123Def"),
            ("ABcdEf", "ABcdEf"),
            ("ABcdeF", "ABcdeF"),
            ("a", "A"),
            ("A", "A"),
        ];
        for (src, exp) in cases {
            assert_eq!(super::camel_case(src), exp, "{}", src);
        }
    }

    #[test]
    fn test_snake_case() {
        let cases = vec![
            ("abcdef", "abcdef"),
            ("a", "a"),
            ("A", "a"),
            ("a__B__c__D_", "a_b_c_d_"),
            ("abCDE", "ab_c_d_e"),
            ("ABCDE", "a_b_c_d_e"),
            ("Abc123Def", "abc123_def"),
            ("abc123", "abc123"),
        ];
        for (src, exp) in cases {
            assert_eq!(super::snake_case(src), exp, "{}", src);
        }
    }
}

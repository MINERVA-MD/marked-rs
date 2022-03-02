// Helpers
use std::borrow::{Cow};
use lazy_static::lazy_static;
use fancy_regex::{Captures, Regex};

lazy_static! {
    static ref ESCAPE_TEST: Regex = Regex::new("[&<>\"']").unwrap();
    static ref ESCAPE_TEST_NO_ENCODE: Regex = fancy_regex::Regex::new("[<>\"']|&(?!#?\\w+;)").unwrap();
    static ref UNESCAPE_TEST: Regex = Regex::new("(?i)&(#(?:\\d+)|(?:#x[0-9A-Fa-f]+)|(?:\\w+));?").unwrap();
    static ref CARET: Regex = Regex::new(r"(^|[^\\[])\\^").unwrap();
}

fn match_unescapes(cap: &Captures) -> String {
    let n =  &cap[1].to_lowercase();

    if n == "colon" {
        return String::from(":");
    }

    if n.chars().nth(0).unwrap() == '#' {
        let char_code: u32;
        if n.chars().nth(1).unwrap() == 'x' {
            char_code = u32::from_str_radix(&n[2..], 16).unwrap();
        } else {
            char_code = (&n[1..]).parse::<u32>().unwrap();
        }

        let _char: Option<char> = char::from_u32(char_code);
        if _char.is_some() {

            return _char.unwrap().to_string();
        }
    }
    return String::from("");
}


fn match_escapes(cap: &Captures) -> &'static str {
    match &cap[0] {
        "&"  => "&amp;",
        "<"  => "&lt;",
        ">"  => "&gt;",
        "\"" => "&quot;",
        "'"  => "&#39;",
        _ => panic!("We should never get here"),
    }
}

fn get_escaped_html(html: &str) -> Cow<str> {
    return ESCAPE_TEST.replace_all(html, |cap: &Captures| {
        match_escapes(cap)
    });
}

fn get_unescaped_html(html: &str) -> Cow<str> {
    return ESCAPE_TEST_NO_ENCODE.replace_all(html, |cap: &Captures | {
        match_escapes(cap)
    });
}

pub fn escape(html: &str, encode: bool) -> String {
    if encode {
        if ESCAPE_TEST.is_match(html).unwrap() {
            return get_escaped_html(html).to_string();
        }
    } else {
        if ESCAPE_TEST_NO_ENCODE.is_match(html).unwrap() {
            return get_unescaped_html(html).to_string();
        }
    }
    return html.to_string();
}

pub fn unescape(html: &str) -> String {
    return UNESCAPE_TEST.replace_all(html, |cap: &Captures| {
        match_unescapes(cap)
    }).to_string();
}

pub fn clean_url(sanitize: bool, base: &str, href: &str) -> String {
    todo!()
}

pub fn split_cells(table_row: &str, count: i32) -> String {
    todo!()
}

pub fn resolve_url(base: &str, href: &str) -> String {
    todo!()
}

pub fn rtrim(_str: &str, c: &str, invert: bool) -> String {
    todo!()
}

pub fn find_closing_bracket(_str: &str, b: &str) -> i32 {
    todo!()
}

pub fn repeat_string(pattern: &str, count: i32) -> String {
    todo!()
}

pub fn check_sanitize_deprecation(opt: &str) {
    todo!()
}


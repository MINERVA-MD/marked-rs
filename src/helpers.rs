// Helpers
use onig::*;
use std::borrow::{Cow};
use lazy_static::lazy_static;
use regex::{Captures, Regex};


lazy_static! {
    static ref ESCAPE_TEST: Regex = Regex::new("[&<>\"']").unwrap();
    static ref ESCAPE_TEST_NO_ENCODE: Regex = Regex::new("[<>\"']|&(?!#?\\w+;)").unwrap();
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

pub fn escape(html: &str, encode: bool) -> Cow<str> {
    if encode {
        if ESCAPE_TEST.is_match(html) {
            return get_escaped_html(html);
        }
    } else {
        if ESCAPE_TEST_NO_ENCODE.is_match(html) {
            return get_unescaped_html(html);
        }
    }

    return Cow::Borrowed(html);
}

pub fn unescape(html: &str) -> Cow<str> {
    return UNESCAPE_TEST.replace_all(html, |cap: &Captures| {
        match_unescapes(cap)
    });
}


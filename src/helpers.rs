// Helpers
use std::borrow::{Cow};
use lazy_static::lazy_static;
use fancy_regex::{Captures, Regex};
use crate::lexer::regx;

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

pub fn split_cells(table_row: &str, count: Option<usize>) -> Vec<String> {

    let row = get_row(table_row);
    let mut cells: Vec<String> = regx(r#" \|"#).split(row.as_str())
        .map(|x| x.to_string())
        .collect();


    // First/last cell in a row cannot be empty if it has no leading/trailing pipe
    if cells.get_mut(0).unwrap().trim().is_empty() {
        cells.remove(0);
    }

    let idx = cells.len() - 1;
    if cells.len() > 0 &&
        cells.get_mut(idx).unwrap().trim().is_empty()
    {
        cells.remove(idx);
    }

    if count.is_some() {
        if cells.len() > count.unwrap() {
            cells.drain(0..count.unwrap());
        } else {
            while cells.len() < count.unwrap() {
                cells.push("".to_string());
            }
        }
    }

    for i in 0..cells.len() {
        // leading or trailing whitespace is ignored per the gfm spec
        cells[i] = regx(r#"\\\|"#)
            .replace_all(cells[i].trim(), "|")
            .to_string();
    }
    cells
}

pub fn get_row(a: &str) -> String {
    let row = regex::Regex::new(r#"\|"#).unwrap()
        .replace_all(a, |cap: &regex::Captures| {
            let mut escaped = false;
            let mut curr: i32 = cap.get(0).unwrap().start() as i32;

            loop {
                curr -= 1;
                let str = cap.get(0).map_or("", |m| m.as_str());
                if curr >= 0 && str.is_empty() {
                    if str.chars().nth(curr as usize).unwrap().to_string() == "\\" {
                        escaped = !escaped;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            return if escaped {
                // odd number of slashes means | is escaped
                // so we leave it alone
                "|"
            } else {
                // add space before unescaped |
                " |"
            }
        });

    // println!("{}", row.len());
    // println!("{}", row);

    String::from(row)
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


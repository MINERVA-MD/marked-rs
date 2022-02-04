use std::fs;
use std::env;
use std::path::Path;
use serde_json::Result;
use serde::{Serialize, Deserialize};


use md4rs_src::parser::parse::Parser;

#[derive(Serialize, Deserialize, Debug)]
struct Spec {
    markdown: String,
    html: String,
    example: i64,
    start_line: i64,
    end_line: i64,
    section: String,
}

fn deserialize_specs(path: &str)-> String {
    let data: String = fs::read_to_string(path)
                            .expect("Unable to read file");
    return data;
}

fn get_specs() -> Vec<Spec> {
    let data: String = deserialize_specs("tests/fixtures/commonmark/spec-v3.json");
    let specs: Vec<Spec> = serde_json::from_str(&data).unwrap();
    return specs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_specs() {
        let specs: Vec<Spec> = get_specs();
        for spec in specs.iter() {
            let parse_actual = Parser::parse_markdown(&spec.markdown);
            let parse_expected = &spec.html;
            assert_eq!(parse_expected, parse_expected);
        }
    }
}


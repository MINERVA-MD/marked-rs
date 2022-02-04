use wasm_bindgen::prelude::*;

pub mod parser;

#[wasm_bindgen]
pub fn parse(markdown: &str) -> String  {
    return parser::parse::Parser::parse(markdown);
}

pub fn run()  {
    parser::parse::Parser::parse(&"### Test");
    parser::parse::Parser::parse(&"## Test");
    parser::parse::Parser::parse(&"# Test");
    parser::parse::Parser::parse(&"**Test**");
    parser::parse::Parser::parse(&"*Test*");
}
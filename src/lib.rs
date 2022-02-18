use wasm_bindgen::prelude::*;

pub mod parser;
pub mod lexer;
pub mod helpers;
pub mod defaults;
pub mod rules;


#[wasm_bindgen]
pub fn parse(markdown: &str) -> String  {
    return parser::parse::Parser::parse(markdown);
}

pub fn run()  {
    // parser::parse::Parser::parse(&"### Test");
    // parser::parse::Parser::parse(&"## Test");
    // parser::parse::Parser::parse(&"# Test");
    // parser::parse::Parser::parse(&"**Test**");
    // parser::parse::Parser::parse(&"*Test*");

    // let entity = parser::entity::Entity::entity_lookup("&Abreve;");
    // println!("{:?}", entity.unwrap());
}
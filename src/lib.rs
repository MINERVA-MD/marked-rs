use wasm_bindgen::prelude::*;
// use crate::rules::setup;

pub mod lexer;
pub mod helpers;
pub mod defaults;
pub mod rules;
pub mod tokenizer;
pub mod slugger;
pub mod marked;
pub mod parser;
pub mod renderer;
pub mod text_renderer;


#[wasm_bindgen]
// pub fn parse(markdown: &str) -> String  {
//     return parser::parse::Parser::parse(markdown);
// }

pub fn run()  {
    // parser::parse::Parser::parse(&"### Test");
    // parser::parse::Parser::parse(&"## Test");
    // parser::parse::Parser::parse(&"# Test");
    // parser::parse::Parser::parse(&"**Test**");
    // parser::parse::Parser::parse(&"*Test*");

    // let entity = parser::entity::Entity::entity_lookup("&Abreve;");
    // println!("{:?}", entity.unwrap());
}
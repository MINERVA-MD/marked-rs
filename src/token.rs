use crate::tokenizer::Link;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub _type: &'static str,
    pub raw: String,
    pub href: String,
    pub title: String,
    pub text: String,
    pub tokens: Vec<Token>,
    pub tag: String,
    pub ordered: bool,
    pub start: i32,
    pub lang: String,
    pub loose: bool,
    pub items: Vec<Token>,
    pub depth: usize,
    pub escaped: bool,
    pub pre: bool,
    pub task: bool,
    pub checked: bool,
    pub in_link: bool,
    pub in_raw_block: bool,
    pub links: Vec<Link>,
    pub align: Vec<String>,
    pub rows: Vec<Vec<Token>>,
    pub header: Vec<Token>,
    pub code_block_style: String
}
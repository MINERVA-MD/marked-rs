#![allow(warnings, unused)]
use crate::tokenizer::Token;

pub type StartFn = fn(src: &str) -> i32;
pub type TokenizerFn = fn(src: &str) -> Option<Token>;
pub type RendererFn = fn(token: &mut Token) -> String;

pub struct Extension {
    pub name: String,
    pub level: String,
    pub start: StartFn,
    pub tokenizer: TokenizerFn,
    pub renderer: RendererFn
}
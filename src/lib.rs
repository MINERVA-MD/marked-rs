use crate::marked::Marked;
use wasm_bindgen::prelude::*;

pub mod lexer;
pub mod rules;
pub mod token;
pub mod marked;
pub mod parser;
pub mod slugger;
pub mod helpers;
pub mod defaults;
pub mod renderer;
pub mod tokenizer;
pub mod extension;
pub mod text_renderer;
pub mod regex;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse(md: &str)-> String {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let mut marked = Marked::new(None);
    let html = marked.parse(md, None, None);

    html
}
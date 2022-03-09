use crate::rules::Rules;
use crate::tokenizer::Token;

#[derive(Clone)]
pub struct Options {
    pub base_url: &'static str,
    pub breaks: bool,
    pub extensions: Option<&'static str>,
    pub gfm: bool,
    pub header_ids: bool,
    pub header_prefix: &'static str,
    pub lang_prefix: &'static str,
    pub mangle: bool,
    pub pedantic: bool,
    pub sanitize: bool,
    pub sanitizer: Option<fn(cap: &str)->String>,
    pub silent: bool,
    pub smart_lists: bool,
    pub smartypants: bool,
    pub is_highlight: bool,
    pub tokenizer: Option<&'static str>,
    pub walk_tokens: Option<fn(tokens: &mut Token)>,
    pub xhtml: bool
}

pub enum Default {
    BaseUrl,
    Breaks,
    Extensions,
    Gfm,
    HeaderIds,
    HeaderPrefix,
    Highlight,
    LangPrefix,
    Mangle,
    Pedantic,
    Renderer,
    Sanitize,
    Sanitizer,
    Silent,
    SmartLists,
    Smartypants,
    Tokenizer,
    WalkTokens,
    Xhtml
}

impl Options {
    pub fn new(&self) -> Self {
        Self {
            base_url: "",
            breaks: false,
            extensions: None,
            gfm: true,
            header_ids: false,
            header_prefix: "",
            lang_prefix: "language-",
            mangle: true,
            pedantic: false,
            sanitize: false,
            sanitizer: None,
            silent: false,
            smart_lists: false,
            smartypants: false,
            tokenizer: None,
            is_highlight: false,
            xhtml: false,
            walk_tokens: None
        }
    }


    pub fn highlight(&mut self, code: &str, lang: &str) -> String {
        "".to_string()
    }

    pub fn change_defaults(mut self, new_defaults: Options) {
        self = new_defaults;
    }

    pub fn enable_sanitize(&mut self) {
        self.sanitize = true;
    }

}

impl Copy for Options {

}

pub fn get_default_options() -> Options {
    Options {
        base_url: "",
        breaks: false,
        extensions: None,
        gfm: true,
        header_ids: false,
        header_prefix: "",
        lang_prefix: "",
        mangle: false,
        pedantic: false,
        sanitize: false,
        sanitizer: None,
        silent: false,
        smart_lists: false,
        smartypants: false,
        is_highlight: false,
        tokenizer: None,
        walk_tokens: None,
        xhtml: false
    }
}




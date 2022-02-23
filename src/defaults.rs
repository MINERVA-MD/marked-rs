
#[derive(Clone, Copy)]
pub struct Defaults {
    pub base_url: Option<&'static str>,
    pub breaks: bool,
    pub extensions: Option<&'static str>,
    pub gfm: bool,
    pub header_ids: bool,
    pub header_prefix: &'static str,
    pub highlight: Option<&'static str>,
    pub lang_prefix: &'static str,
    pub mangle: bool,
    pub pedantic: bool,
    pub renderer: Option<&'static str>,
    pub sanitize: bool,
    pub sanitizer: Option<&'static str>,
    pub silent: bool,
    pub smart_lists: bool,
    pub smartypants: bool,
    pub tokenizer: Option<&'static str>,
    pub walk_tokens: Option<&'static str>,
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

impl Defaults {
    pub fn new() -> Self {
        Self {
            base_url: None,
            breaks: false,
            extensions: None,
            gfm: true,
            header_ids: false,
            header_prefix: "",
            highlight: None,
            lang_prefix: "language-",
            mangle: true,
            pedantic: false,
            renderer: None,
            sanitize: false,
            sanitizer: None,
            silent: false,
            smart_lists: false,
            smartypants: false,
            tokenizer: None,
            walk_tokens: None,
            xhtml: false
        }
    }

    pub fn change_defaults(mut self, new_defaults: Defaults) {
        self = new_defaults;
    }

}
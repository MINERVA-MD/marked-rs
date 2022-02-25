use std::rc::Rc;
use regex::{Captures, Regex};
use crate::rules::{MDBlock, Rules};
use crate::defaults::Options;
use crate::helpers::{escape};
use crate::lexer::{ILexer, InlineToken, Lexer, regx};


#[derive(Clone)]
pub struct Token {
    pub _type: &'static str,
    pub raw: String,
    pub href: String,
    pub title: String,
    pub text: String,
    pub tokens: Vec<Token>,
    pub tag: usize,
    pub ordered: String,
    pub start: u32,
    pub lang: String,
    pub loose: bool,
    pub items: Vec<Token>,
    pub depth: usize,
    pub escaped: bool,
    
    pub header: Vec<Token>,
    pub code_block_style: String
}

#[derive(Clone)]
pub struct Link {
    pub href: &'static str,
    pub title: &'static str
}

impl Token {
    
    pub fn new(mut self) -> Self {
        Self {
            _type: "",
            raw: "".to_string(),
            href: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
            tokens: vec![],
            tag: 0,
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            header: vec![],
            code_block_style: "".to_string()
        }
    }

    pub fn append_to_raw(&mut self, to_append: &str) {
        self.raw.push_str(to_append);
    }
    pub fn append_to_text(&mut self, to_append: &str) {
        self.text.push_str(to_append);
    }

}

pub trait ITokenizer {

    // Block
    fn space(&mut self, src: &str) -> Option<Token>;
    fn code(&mut self, src: &str) -> Option<Token>;
    fn fences(&mut self, src: &str) -> Option<Token>;
    fn heading(&mut self, src: &str) -> Option<Token>;
    fn hr(&mut self, src: &str) -> Option<Token>;
    fn blockquote(&mut self, src: &str) -> Option<Token>;
    fn list(&mut self, src: &str) -> Option<Token>;
    fn html(&mut self, src: &str) -> Option<Token>;
    fn def(&mut self, src: &str) -> Option<Token>;
    fn table(&mut self, src: &str) -> Option<Token>;
    fn lheading(&mut self, src: &str) -> Option<Token>;
    fn text(&mut self, src: &str) -> Option<Token>;

    // Inline
    fn escape(&mut self, src: &str) -> Option<Token>;
    fn tag(&mut self, src: &str) -> Option<Token>;
    fn link(&mut self, src: &str) -> Option<Token>;
    fn ref_link(&mut self, src: &str, links: &Vec<String>) -> Option<Token>;
    fn em_strong(&mut self, src: &str, masked_src: &str, prev_char: &str) -> Option<Token>;
    fn code_span(&mut self, src: &str) -> Option<Token>;
    fn br(&mut self, src: &str) -> Option<Token>;
    fn del(&mut self, src: &str) -> Option<Token>;
    fn autolink(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token>;
    fn url(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token>;
    fn inline_text(&mut self, src: &str, smartypants : fn(text: &str) -> String) -> Option<Token>;
}

pub struct Tokenizer {
    pub options: Options,
    pub rules: Option<Rules>,
    pub lexer: Box<Lexer>
}

impl Tokenizer {
    pub fn new(options: Option<Options>) -> Self {
        Self {
            options: options.unwrap(),
            rules: None,
            lexer: Box::from(Lexer::new(Options {
                base_url: "",
                breaks: false,
                extensions: None,
                gfm: false,
                header_ids: false,
                header_prefix: "",
                lang_prefix: "",
                mangle: false,
                pedantic: false,
                sanitize: false,
                sanitizer: None,
                silent: false,
                is_highlight: false,
                smart_lists: false,
                smartypants: false,
                tokenizer: None,
                walk_tokens: None,
                xhtml: false
            })),
        }
    }

    pub fn get_rules(self) -> Rules {
        self.rules.unwrap()
    }
}

impl ITokenizer for Tokenizer {

    fn space(&mut self, src: &str) -> Option<Token> {
        let newline_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Newline);

        if newline_caps.is_some() {
            let caps = newline_caps.unwrap();
            let _raw = caps.get(0).map_or("", |m| m.as_str());

            if _raw.len() > 0 {
                return Some (Token {
                    _type: "space",
                    raw: _raw.to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "".to_string(),
                    tokens: vec![],
                    tag: 0,
                    ordered: "".to_string(),
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }
        }
        None
    }

    fn code(&mut self, src: &str) -> Option<Token> {
        let code_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Code);

        if code_caps.is_some() {
            let caps = code_caps.unwrap();
            let _raw = caps.get(0).map_or("", |m| m.as_str());
            let mut text = regx("(?m)^ {1,4}").replace_all(_raw, "").to_string();
            todo!("Double check reg");
            text = if !self.options.pedantic { regx("\n*$").replace_all(text.as_str(), "").to_string()} else { text.to_string() };

            return Some(Token {
                _type: "code",
                raw: _raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: 0,
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                header: vec![],
                code_block_style: "indented".to_string()
            });
        }
        None
    }

    fn fences(&mut self, src: &str) -> Option<Token> {
        let fences_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Fences);

        if fences_caps.is_some() {
            let caps = fences_caps.unwrap();
            let _raw = caps.get(0).map_or("", |m| m.as_str());
            let cap3 = caps.get(3);
            let mut _text: String = "".to_string();
            if cap3.is_some() {
                _text = cap3.map_or("", |m| m.as_str()).to_string();
            }
            let text = indent_code_compensation(_raw, _text);

            let cap2 = caps.get(2);
            let lang = if cap2.is_some() { cap2.map_or("", |m| m.as_str()).trim().to_string() } else { "".to_string() };

            return Some(Token {
                _type: "code",
                raw: _raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: 0,
                ordered: "".to_string(),
                start: 0,
                lang,
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn heading(&mut self, src: &str) -> Option<Token> {
        let heading_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Heading);

        if heading_caps.is_some() {
            let caps = heading_caps.unwrap();
            let mut text = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();

            if regx("#$").is_match(text.as_str()) {
                let trimmed = regx("#*$").replace_all(text.as_str(), "").to_string();

                if self.options.pedantic {
                    text = trimmed.trim().to_string();
                } else if trimmed == "" || regx(" $").is_match(trimmed.as_str()) {
                    text = trimmed.trim().to_string();
                }
            }

            let _raw = caps.get(0).map_or("", |m| m.as_str());
            let depth = caps.get(1).map_or("", |m| m.as_str()).len();

            let mut token = Token {
                _type: "heading",
                raw: _raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: text.to_string(),
                tokens: vec![],
                tag: 0,
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth,
                escaped: false,
                header: vec![],
                code_block_style: "".to_string()
            };

            // self.lexer.inline(token.text, token.tokens);
            // May need to switch this to reference same token
            return Some(token.clone());
        }
        None
    }

    fn hr(&mut self, src: &str) -> Option<Token> {
        let hr_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Hr);

        if hr_caps.is_some() {
            let caps = hr_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str()).to_string();
            
            return Some(Token{
                _type: "hr",
                raw,
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: 0,
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn blockquote(&mut self, src: &str) -> Option<Token> {
        let blockquote_caps = self.rules.as_ref().unwrap().block.exec(src, MDBlock::Hr);

        if blockquote_caps.is_some() {
            let caps = blockquote_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let text  = regx("(?m)^ *> ?").replace_all(raw, "").to_string();
            let tokens = self.lexer.block_tokens(text.as_str(), vec![]);

            return Some(Token {
                _type: "blockquote",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens,
                tag: 0,
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn list(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn html(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn def(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn table(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn lheading(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn text(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn escape(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn tag(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn link(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn ref_link(&mut self, src: &str, mut links: &Vec<String>) -> Option<Token> {
        todo!()
    }

    fn em_strong(&mut self, src: &str, masked_src: &str, prev_char: &str) -> Option<Token> {
        todo!()
    }

    fn code_span(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn br(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn del(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn autolink(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        todo!()
    }

    fn url(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        todo!()
    }

    fn inline_text(&mut self, src: &str, smartypants: fn(&str) -> String) -> Option<Token> {
        todo!()
    }

}

pub fn output_link(cap: Vec<String>, link: Link, raw: String, mut lexer: Lexer) -> Token {
    let href = link.href.to_string();
    let title = if link.title == "" {escape(link.title, false).to_string()} else {"".to_string()};
    let text = regx(r#"\\([\[\]])"#).replace_all(cap[1].as_str(), "${1}");

    if cap[0].chars().nth(0).unwrap() != '!' {
        lexer.state.in_link = true;
        let token = Token {
            _type: "link",
            raw,
            href,
            title,
            text: text.to_string(),
            tokens: lexer.inline_tokens(text.to_string().as_str(), vec![]),
            tag: 0,
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            header: vec![],
            code_block_style: "".to_string()
        };
        lexer.state.in_link = false;
        token
    } else {
        Token {
            _type: "image",
            raw,
            href,
            title,
            text: escape(text.to_string().as_str(), false).to_string(),
            tokens: vec![],
            tag: 0,
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            header: vec![],
            code_block_style: "".to_string()
        }
    }
}

pub fn indent_code_compensation(raw: &str, text: String) -> String {

    let indent_to_code_caps = regx(r#"^(\s+)(?:```)"#).captures(raw);

    if indent_to_code_caps.is_none() {
        return text;
    }

    let match_indent_to_code = indent_to_code_caps.unwrap();
    let indent_to_code = match_indent_to_code.get(1).map_or("", |m| m.as_str());

    return text
        .split("\n")
        .into_iter()
        .map(|node| {
            let indent_in_node_caps = regx(r#"^\s+"#).captures(node);

            if indent_in_node_caps.is_none() {
                return node.to_string();
            }

            let match_indent_in_node = indent_in_node_caps.unwrap();
            let indent_in_node = match_indent_in_node.get(0).map_or("", |m| m.as_str());

            if indent_in_node.len() >= indent_to_code.len() {
                return String::from(&node[indent_to_code.len()..]);
            }

            return node.to_string();
        })
        .collect::<Vec<String>>()
        .join("\n");
}
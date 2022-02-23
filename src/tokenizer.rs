use regex::Regex;
use crate::rules::Rules;
use crate::defaults::Defaults;
use crate::helpers::escape;
use crate::lexer::{ILexer, InlineToken, Lexer, regx};


#[derive(Clone)]
pub struct Token {
    pub _type: &'static str,
    pub raw: String,
    pub href: String,
    pub title: String,
    pub text: String,
    pub tokens: Vec<Token>,
    pub tag: usize
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
            tag: 0
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
    options: Defaults,
    rules: Option<Rules>
}

impl Tokenizer {
    pub fn new(options: Option<Defaults>) -> Self {
        Self {
            options: options.unwrap(),
            rules: None
        }
    }
}

impl ITokenizer for Tokenizer {
    fn space(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn code(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn fences(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn heading(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn hr(&mut self, src: &str) -> Option<Token> {
        todo!()
    }

    fn blockquote(&mut self, src: &str) -> Option<Token> {
        todo!()
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
            tag: 0
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
            tag: 0
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
use std::fmt;
use std::fmt::Formatter;
use crate::defaults::Options;
use crate::helpers::{escape};
use crate::rules::{MDBlock, MDInline, Rules};
use crate::lexer::{ILexer, Lexer, regx};


#[derive(Clone, PartialEq)]

pub struct Token {
    pub _type: &'static str,
    pub raw: String,
    pub href: String,
    pub title: String,
    pub text: String,
    pub tokens: Vec<Token>,
    pub tag: String,
    pub ordered: String,
    pub start: u32,
    pub lang: String,
    pub loose: bool,
    pub items: Vec<Token>,
    pub depth: usize,
    pub escaped: bool,
    pub pre: bool,
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
            tag: "".to_string(),
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
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

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\tType: {:?} \n\tRaw: {:?} \n\tHref: {:?} \n\tTitle: {:?} \
        \n\tText: {:?} \n\tTokens: {:?} \n\tTag: {:?} \n\tOrdered: {:?} \
        \n\tStart: {:?} \n\tLang: {:?} \n\tLoose: {:?} \n\tItems: {:?} \
        \n\tDepth: {:?} \n\tEscaped: {:?} \n\tPre: {:?} \n\tHeader: {:?} \
        \n\tCode Style: {:?}\n",
            self._type, self.raw, self.href, self.title,
            self.text, self.tokens, self.tag, self.ordered,
            self.start, self.lang, self.loose, self.items,
            self.depth, self.escaped, self.pre, self.header,
            self.code_block_style
        )
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
    fn paragraph(&mut self, src: &str) -> Option<Token>;
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
    pub rules: Rules,
}

impl Tokenizer {
    pub fn new(options: Option<Options>, rules: Rules) -> Self {
        Self {
            options: options.unwrap(),
            rules
        }
    }

    pub fn get_rules(self) -> Rules {
        self.rules
    }
}

impl ITokenizer for Tokenizer {

    //  Block
    fn space(&mut self, src: &str) -> Option<Token> {
        let newline_caps = self.rules.block.exec_fc(src, MDBlock::Newline);

        if newline_caps.is_some() {
            let caps = newline_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());

            if raw.len() > 0 {
                return Some (Token {
                    _type: "space",
                    raw: raw.to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "".to_string(),
                    tokens: vec![],
                    tag: "".to_string(),
                    ordered: "".to_string(),
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }
        }
        None
    }

    fn code(&mut self, src: &str) -> Option<Token> {
        let code_caps = self.rules.block.exec_fc(src, MDBlock::Code);

        if code_caps.is_some() {
            let caps = code_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let mut text = regx("(?m)^ {1,4}").replace_all(raw, "").to_string();
            // todo!("Double check reg");
            text = if !self.options.pedantic { regx("\n*$").replace_all(text.as_str(), "").to_string()} else { text.to_string() };

            return Some(Token {
                _type: "code",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "indented".to_string()
            });
        }
        None
    }

    fn fences(&mut self, src: &str) -> Option<Token> {
        let fences_caps = self.rules.block.exec_fc(src, MDBlock::Fences);

        if fences_caps.is_some() {
            let caps = fences_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap3 = caps.get(3);
            let mut _text: String = "".to_string();
            if cap3.is_some() {
                _text = cap3.map_or("", |m| m.as_str()).to_string();
            }
            let text = indent_code_compensation(raw, _text);

            let cap2 = caps.get(2);
            let lang = if cap2.is_some() { cap2.map_or("", |m| m.as_str()).trim().to_string() } else { "".to_string() };

            return Some(Token {
                _type: "code",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang,
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn heading(&mut self, src: &str) -> Option<Token> {
        let heading_caps = self.rules.block.exec_fc(src, MDBlock::Heading);

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
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth,
                escaped: false,
                pre: false,
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
        let hr_caps = self.rules.block.exec_fc(src, MDBlock::Hr);

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
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn blockquote(&mut self, src: &str) -> Option<Token> {
        let blockquote_caps = self.rules.block.exec_fc(src, MDBlock::Hr);

        if blockquote_caps.is_some() {
            let mut lexer = Lexer::new(self.options);
            let caps = blockquote_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let text  = regx("(?m)^ *> ?").replace_all(raw, "").to_string();

            // Set tokens in caller instead
            // let tokens = lexer.block_tokens(text.as_str(), vec![]);

            return Some(Token {
                _type: "blockquote",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn list(&mut self, src: &str) -> Option<Token> {
        None
    }

    fn html(&mut self, src: &str) -> Option<Token> {
        let html_caps = self.rules.block.exec_fc(src, MDBlock::Html);

        if html_caps.is_some() {
            let mut lexer = Lexer::new(self.options);
            let caps = html_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap_1 = caps.get(1).map_or("", |m| m.as_str());

            let pre = !self.options.sanitizer.is_some() &&
                cap_1 == "pre" ||
                cap_1 == "script" ||
                cap_1 == "style";

            let mut token = Token {
                _type: "html",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: String::from(cap_1),
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre,
                header: vec![],
                code_block_style: "".to_string()
            };

            if self.options.sanitize {
                token._type = "paragraph";
                token.text = if self.options.sanitizer.is_some() {
                    (self.options.sanitizer.unwrap())(raw)
                } else {
                    escape(raw, false)
                };
            }
            return Some(token);
        }
        None
    }

    fn def(&mut self, src: &str) -> Option<Token> {
        let def_caps = self.rules.block.exec_fc(src, MDBlock::Def);

        if def_caps.is_some() {
            let caps = def_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());
            let mut cap3 = caps.get(3).map_or("", |m| m.as_str()).to_string();

            cap3 = (&cap3[1 .. cap3.len() - 1]).to_string();
            let mut tag = cap1.to_lowercase();
            tag = regx(r#"\s+"#).replace_all(tag.as_str(), " ").to_string();

            return Some(Token {
                _type: "def",
                raw: raw.to_string(),
                href: cap2.to_string(),
                title: cap3.to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag,
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn table(&mut self, src: &str) -> Option<Token> {
        None
    }

    fn lheading(&mut self, src: &str) -> Option<Token> {
        let lheading_caps = self.rules.block.exec_fc(src, MDBlock::LHeading);

        if lheading_caps.is_some() {

            let caps = lheading_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());

            let depth = if cap2.chars().nth(0).unwrap() == '=' {
                1 as usize
            } else {
                2 as usize
            };

            let token = Token {
                _type: "heading",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: cap1.to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };

            // TODO: Implement to move tokens
            // lexer.inline(token.text, token.tokens);
            return Some(token);
        }
        None
    }

    fn paragraph(&mut self, src: &str) -> Option<Token> {
        let paragraph_caps = self.rules.block.exec_fc(src, MDBlock::Paragraph);

        if paragraph_caps.is_some() {
            let caps = paragraph_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let text = if cap1.chars().nth(cap1.len() - 1).unwrap().to_string() == "\n" {
                cap1.chars().next_back().unwrap().to_string()
            } else {
                cap1.to_string()
            };

            let token = Token {
                _type: "paragraph",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };
            return Some(token);
        }
        None
    }

    fn text(&mut self, src: &str) -> Option<Token> {
        let text_caps = self.rules.block.exec_fc(src, MDBlock::Text);

        if text_caps.is_some() {
            let caps = text_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());

            let mut token = Token {
                _type: "text",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: raw.to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };
            return Some(token);
        }
        None
    }


    // Inline
    fn escape(&mut self, src: &str) -> Option<Token> {
        let escape_caps = self.rules.inline.exec_fc(src, MDInline::Escape);

        if escape_caps.is_some() {
            let caps = escape_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let text = escape(cap1, false).to_string();

            return Some(Token {
                _type: "escape",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn tag(&mut self, src: &str) -> Option<Token> {
        None
    }

    fn link(&mut self, src: &str) -> Option<Token> {
        None
    }

    fn ref_link(&mut self, src: &str, mut links: &Vec<String>) -> Option<Token> {
        None
    }

    fn em_strong(&mut self, src: &str, masked_src: &str, prev_char: &str) -> Option<Token> {
        None
    }

    fn code_span(&mut self, src: &str) -> Option<Token> {
        None
    }

    fn br(&mut self, src: &str) -> Option<Token> {
        let br_caps = self.rules.inline.exec_fc(src, MDInline::Br);

        if br_caps.is_some() {
            let caps = br_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());

            let token = Token {
                _type: "br",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }

    fn del(&mut self, src: &str) -> Option<Token> {
        let del_caps = self.rules.inline.exec_fc(src, MDInline::Del);

        if del_caps.is_some() &&
            self.rules.inline.del != ""
        {
            // TODO: pass in lexer as arg instead
            let mut lexer = Lexer::new(self.options);
            let caps = del_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let caps_2 = caps.get(2).map_or("", |m| m.as_str());

            // Put this in caller
            let tokens = lexer.inline_tokens(caps_2, vec![]);

            let token = Token {
                _type: "del",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: caps_2.to_string(),
                tokens,
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }

    fn autolink(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        None
    }

    fn url(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        None
    }

    fn inline_text(&mut self, src: &str, smartypants: fn(&str) -> String) -> Option<Token> {
        let inline_caps = self.rules.inline.exec_fc(src, MDInline::Text);

        if inline_caps.is_some() {
            let lexer = Lexer::new(self.options);
            let caps = inline_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());

            let mut text = "".to_string();

            if lexer.state.in_raw_block {
                text = if self.options.sanitize {
                    if self.options.sanitizer.is_some() {
                        (self.options.sanitizer.unwrap())(raw)
                    } else {
                        escape(raw, false)
                    }
                } else {
                    raw.to_string()
                }
            } else {
                let html = if self.options.smartypants {
                    smartypants(raw)
                } else {
                    raw.to_string()
                };

                text = escape(html.as_str(), false);
            }

            let token = Token {
                _type: "text",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: "".to_string(),
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                header: vec![],
                code_block_style: "".to_string()
            };
            println!("{:?}", token);
            return Some(token);
        }
        None
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
            tag: "".to_string(),
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
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
            tag: "".to_string(),
            ordered: "".to_string(),
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
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
    let indent_to_code = match_indent_to_code.get(1).map_or("", |m| <&str>::from(m));

    return text
        .split("\n")
        .into_iter()
        .map(|node| {
            let indent_in_node_caps = regx(r#"^\s+"#).captures(node);

            if indent_in_node_caps.is_none() {
                return node.to_string();
            }

            let match_indent_in_node = indent_in_node_caps.unwrap();
            let indent_in_node = match_indent_in_node.get(0).map_or("", |m| <&str>::from(m));

            if indent_in_node.len() >= indent_to_code.len() {
                return String::from(&node[indent_to_code.len()..]);
            }

            return node.to_string();
        })
        .collect::<Vec<String>>()
        .join("\n");
}
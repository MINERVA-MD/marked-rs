#![allow(warnings, unused)]
use std::rc::Rc;
use std::cmp::min;
use std::{fmt};
use std::ops::Range;
use std::cell::RefCell;
use fancy_regex::Captures;

use crate::defaults::Options;
use crate::lexer::{InlineToken, Lexer, regx};
use crate::rules::{get_rules, MDBlock, MDInline, Rules};
use crate::helpers::{escape, find_closing_bracket, is_divisible, is_not_divisible, is_odd, rtrim, split_cells};


#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub _type: &'static str,
    pub raw: String,
    pub href: String,
    pub title: String,
    pub text: String,
    pub tokens: Vec<Rc<RefCell<Token>>>,
    pub tag: String,
    pub ordered: bool,
    pub start: i32,
    pub lang: String,
    pub loose: bool,
    pub items: Vec<Rc<RefCell<Token>>>,
    pub depth: usize,
    pub escaped: bool,
    pub pre: bool,
    pub task: bool,
    pub checked: bool,
    pub in_link: bool,
    pub in_raw_block: bool,
    pub links: Vec<Link>,
    pub align: Vec<String>,
    pub rows: Vec<Vec<Rc<RefCell<Token>>>>,
    pub header: Vec<Rc<RefCell<Token>>>,
    pub code_block_style: String
}


#[derive(Clone, PartialEq, Debug)]
pub struct Link {
    pub href: String,
    pub title: String,
    pub tag: String
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
            ordered: false,
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
            task: false,
            checked: false,
            in_link: false,
            in_raw_block: false,
            links: vec![],
            align: vec![],
            rows: vec![],
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


impl fmt::Display for Token {
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
    fn table(&mut self, src: &str, tokens: &mut Vec<InlineToken>) -> Option<Token>;
    fn lheading(&mut self, src: &str) -> Option<Token>;
    fn paragraph(&mut self, src: &str) -> Option<Token>;
    fn text(&mut self, src: &str) -> Option<Token>;

    // Inline
    fn escape(&mut self, src: &str) -> Option<Token>;
    fn tag(&mut self, src: &str, in_link: &mut bool, in_raw_block: &mut bool) -> Option<Token>;
    fn link(&mut self, src: &str) -> Option<Token>;
    fn ref_link(&mut self, src: &str, links: &Vec<Link>) -> Option<Token>;
    fn em_strong(&mut self, src: &str, masked_src: &str, prev_char: &str) -> Option<Token>;
    fn code_span(&mut self, src: &str) -> Option<Token>;
    fn br(&mut self, src: &str) -> Option<Token>;
    fn del(&mut self, src: &str) -> Option<Token>;
    fn autolink(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token>;
    fn url(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token>;
    fn inline_text(&mut self, src: &str, in_raw_block: bool, smartypants : fn(text: &str) -> String) -> Option<Token>;
}

type InlineTokenCallback = fn(&mut Lexer, src: String, tokens: Vec<Token>, parent_block_idx: usize);

pub struct Tokenizer {
    pub options: Options,
    pub rules: Rules
}

impl Tokenizer {
    pub fn new(options: Option<Options>) -> Self {
        let rules = get_rules(options.unwrap());
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
        let newline_caps = self.rules.block.exec_fc(src, MDBlock::Newline, None);

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
                    ordered: false,
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    task: false,
                    checked: false,
                    in_link: false,
                    in_raw_block: false,
                    links: vec![],
                    align: vec![],
                    rows: vec![],
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }
        }
        None
    }

    fn code(&mut self, src: &str) -> Option<Token> {
        let code_caps = self.rules.block.exec_fc(src, MDBlock::Code, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "indented".to_string()
            });
        }
        None
    }

    fn fences(&mut self, src: &str) -> Option<Token> {
        let fences_caps = self.rules.block.exec_fc(src, MDBlock::Fences, None);

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
                ordered: false,
                start: 0,
                lang,
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn heading(&mut self, src: &str) -> Option<Token> {
        let heading_caps = self.rules.block.exec_fc(src, MDBlock::Heading, None);

        if heading_caps.is_some() {
            let caps = heading_caps.unwrap();
            let mut text = caps.get(2).map_or("", |m| m.as_str()).trim().to_string();

            if regx("#$").is_match(text.as_str()) {
                let trimmed = rtrim(text.as_str(), "#", false);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
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
        let hr_caps = self.rules.block.exec_fc(src, MDBlock::Hr, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn blockquote(&mut self, src: &str) -> Option<Token> {
        let blockquote_caps = self.rules.block.exec_fc(src, MDBlock::Blockquote, None);

        if blockquote_caps.is_some() {
            let caps = blockquote_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let text  = regx("(?m)^ *> ?").replace_all(raw, "").to_string();

            return Some(Token {
                _type: "blockquote",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            })
        }
        None
    }

    fn list(&mut self, src: &str) -> Option<Token> {

        let mut list_caps = self.rules.block.exec_fc(src, MDBlock::List, None);
        let mut _src: String = String::from(src);


        if list_caps.is_some() {

            let mut is_task = false;
            let mut blank_line = false;
            let mut indent: usize  = 0;
            let mut is_checked = false;
            let mut ends_with_blank_line = false;
            let mut item_contents = String::from("");

            let mut caps = list_caps.unwrap();
            let mut cap1 = caps.get(1).map_or("", |m| m.as_str());

            let mut bull = cap1.trim().to_string();
            let is_ordered = bull.len() > 1;



            let start: i32 = if is_ordered {
                let num = slice(bull.as_str(), 0..bull.len() - 1).to_string();
                num.parse().unwrap()
            } else {
                // TODO: May need to change this to be "" or an Option
                0
            };


            let mut list = Token {
                _type: "list",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: is_ordered,
                start,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };

            if is_ordered {
                // TODO: double check that regex is properly escaped
                let re_ol = format!(r#"\d{{1,9}}\{}"#, bull.chars().last().unwrap());
                bull = String::from(re_ol);
            } else {
                let re_ul = format!(r#"\{}"#, bull);
                bull = String::from(re_ul);
            }

            if self.options.pedantic {
                if !is_ordered {
                    bull = String::from("[*+-]");
                }
            }

            let list_re_str = String::from(format!(r#"^( {{0,3}}{})((?: [^\n]*)?(?:\n|$))"#, bull));

            // Get next list item
            let item_regex = fancy_regex::Regex::new(list_re_str.as_str()).unwrap();

            // Check if current bullet point can start a new List Item
            let mut end_early: bool = false;
            let mut raw: String = String::from("");

            while _src.len() > 0 {

                end_early = false;
                let src_cp = _src.clone();
                let item_caps = item_regex.captures(src_cp.as_str()).unwrap();

                if item_caps.is_none() { break; }

                if self.rules.block.get_grammar_fc_regex(MDBlock::Hr, None)
                    .is_match(_src.as_str()).unwrap()
                {
                    // End list if bullet was actually HR (possibly move into itemRegex?)
                    break;
                }

                let _caps = item_caps.unwrap();

                raw = _caps.get(0).map_or("", |m| m.as_str()).to_string();
                let _cap1 = _caps.get(1).map_or("", |m| m.as_str());
                let _cap2 = _caps.get(2).map_or("", |m| m.as_str());

                _src = slice(_src.as_str(), raw.len().. _src.len());

                let mut lines: Vec<String> = _cap2.splitn(2,"\n")
                    .map(|x| x.to_string())
                    .collect();

                lines = vec![lines[0].clone()];

                let mut next_lines: Vec<String> = _src.splitn(2,"\n")
                    .map(|x| x.to_string())
                    .collect();

                next_lines = vec![next_lines[0].clone()];

                let mut line = String::from(lines.get(0).unwrap());
                let mut next_line = String::from(next_lines.get(0).unwrap());


                if self.options.pedantic {
                    indent = 2;
                    item_contents = line.trim_start().to_string();
                } else {
                    indent = regx(r#"[^ ]"#)
                        .find(_cap2)
                        .unwrap()
                        .start();
                    indent = if indent > 4 { 1 } else { indent };
                    item_contents = slice(line.as_str(), indent..line.len());
                    indent += _cap1.len();
                }

                blank_line = false;
                // Items begin with at most one blank line
                if line == "" && regx(r#"^ *$"#)
                    .is_match(next_line.as_ref())
                {
                    raw = format!("{}{}\n", raw.to_string(), next_line);
                    _src = slice(_src.as_str(), next_line.len() + 1.._src.len());
                    end_early = true;
                }

                if !end_early {
                    let next_bullet_re_str = String::from(format!(r#"^ {{0,{}}}(?:[*+-]|\d{{1,9}}[.)])"#, min(3, indent - 1)));
                    let next_bullet_regex = fancy_regex::Regex::new(next_bullet_re_str.as_str()).unwrap();

                    // Check if following lines should be included in List Item
                    while _src.len() > 0 {
                        let raw_lines : Vec<String> = _src.splitn(2, "\n")
                            .map(|x| x.to_string())
                            .collect();

                        let mut raw_line = raw_lines.get(0).unwrap();

                        line = String::from(raw_line.clone());

                        // Re-align to follow commonmark nesting rules
                        if self.options.pedantic {
                            let re_align = fancy_regex::Regex::new(r#"^ {1,4}(?=( {4})*[^ ])"#).unwrap()
                                .replace_all(line.clone().as_str(), "  ")
                                .to_string();
                            line = String::from(re_align);
                        }

                        // End list item if found start of new bullet
                        if next_bullet_regex.is_match(line.as_str()).unwrap() {
                            break;
                        }

                        let line_search_match = regx(r#"[^ ]"#)
                            .find(line.as_str());

                        let line_search_idx = if line_search_match.is_some() {
                            line_search_match
                            .unwrap().start() as i32
                        } else {
                            -1
                        };

                        if line_search_idx >= indent as i32 ||
                            line.trim().is_empty()
                        { // Dedent if possible
                            item_contents = format!("{}\n{}", item_contents, slice(line.as_str(), indent..line.len()));
                        } else if !blank_line{ // Until blank line, item doesn't need indentation
                            item_contents = format!("{}\n{}", item_contents, line);
                        } else { // Otherwise, improper indentation ends this item
                            break;
                        }


                        if !blank_line &&
                            line.trim().is_empty()
                        { // Check if current line is blank
                            blank_line = true;
                        }

                        raw = format!("{}{}\n", raw, raw_line);

                        // TODO: double check guard, place relevant guards at other substrings/slice
                        if raw_line.len() + 1 < _src.len() {
                            _src = slice(_src.as_str(), raw_line.len() + 1.._src.len());
                        } else {
                            _src = "".to_string();
                        }

                    }
                }

                if !list.loose {
                    // If the previous item ended with a blank line, the list is loose
                    if ends_with_blank_line {
                        list.loose = true
                    } else if regx(r#"\n *\n *$"#).is_match(raw.as_str())
                    {
                        ends_with_blank_line = true;
                    }
                }

                // Check for task list items
                if self.options.gfm {
                    let is_task_caps = regx(r#"^\[[ xX]\] "#).captures(item_contents.as_str());
                    if is_task_caps.is_some() {
                        is_task = true;

                        let task_caps = is_task_caps.unwrap();
                        let is_task0 = task_caps.get(0).map_or("", |m| m.as_str()).to_string();

                        is_checked = is_task0 != "[ ] ";
                        item_contents = regx(r#"^\[[ xX]\] +"#)
                            .replace(item_contents.as_str(), "")
                            .to_string()
                    } else {
                        is_task = false;
                    }
                }


                list.items.push(
                    Rc::new(RefCell::new(
                        Token {
                            _type: "list_item",
                            raw: raw.to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: item_contents.to_string(),
                            tokens: vec![],
                            tag: "".to_string(),
                            ordered: false,
                            start: 0,
                            lang: "".to_string(),
                            loose: false,
                            items: vec![],
                            depth: 0,
                            escaped: false,
                            pre: false,
                            task: is_task,
                            checked: is_checked,
                            in_link: false,
                            in_raw_block: false,
                            links: vec![],
                            align: vec![],
                            rows: vec![],
                            header: vec![],
                            code_block_style: "".to_string()
                        }
                    ))
                );

                list.raw = format!("{}{}", list.raw, raw);
            }

            // Do not consume newlines at end of final item. Alternatively, make itemRegex *start* with any newlines to simplify/speed up endsWithBlankLine logic
            let list_item_idx = list.items.len() - 1;
            let mut last_list_item = list.items.get_mut(list_item_idx).unwrap();

            last_list_item.as_ref().borrow_mut().raw = String::from(raw.trim_end());
            last_list_item.as_ref().borrow_mut().text = String::from(item_contents.trim_end());
            list.raw = String::from(list.raw.trim_end());


            return Some(list);
        }
        None
    }

    fn html(&mut self, src: &str) -> Option<Token> {
        // let html_caps = self.rules.block.exec_fc(src, MDBlock::Html, None);
        // println!("{:#?}", self.rules.inline.url);

        let html_re_str =  self.rules.block.html.as_str();
        let html_re = regress::Regex::with_flags(html_re_str, "i").unwrap();
        let html_caps = html_re.find(src);

        if html_caps.is_some() {
            let caps = html_caps.unwrap();

            let raw = caps.group(0).map_or("", |r| { &src[r] });
            let cap_1 = caps.group(1).map_or("", |r| { &src[r] });

            let pre = !self.options.sanitizer.is_some() &&
                cap_1 == "pre" ||
                cap_1 == "script" ||
                cap_1 == "style";

            let mut token = Token {
                _type: "html",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: raw.to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
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
        let def_caps = self.rules.block.exec_fc(src, MDBlock::Def, None);

        if def_caps.is_some() {
            let caps = def_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());
            let mut cap3 = caps.get(3).map_or("", |m| m.as_str()).to_string();

            if cap3.len() > 0 {
                cap3 = (&cap3[1 .. cap3.len() - 1]).to_string();
            }

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn table(&mut self, src: &str, mut tokens: &mut Vec<InlineToken>) -> Option<Token> {

        if  self.rules.block.table.is_empty() { return None; }

        let table_caps = self.rules.block.exec_fc(src, MDBlock::Table, None);

        if table_caps.is_some() {
            let caps = table_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());
            let cap3 = caps.get(3).map_or("", |m| m.as_str());

            let mut header = split_cells(cap1, None)
                .into_iter()
                .map(|header_val| {
                    Rc::new(RefCell::new(Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: header_val.to_string(),
                        tokens: vec![],
                        tag: "".to_string(),
                        ordered: false,
                        start: 0,
                        lang: "".to_string(),
                        loose: false,
                        items: vec![],
                        depth: 0,
                        escaped: false,
                        pre: false,
                        task: false,
                        checked: false,
                        in_link: false,
                        in_raw_block: false,
                        links: vec![],
                        align: vec![],
                        rows: vec![],
                        header: vec![],
                        code_block_style: "".to_string()
                    }))
                })
                .collect::<Vec<Rc<RefCell<Token>>>>();


            let align_replaced = regx(r#"^ *|\| *$"#)
                .replace_all(cap2, "").to_string();

            let mut align: Vec<String> = regx(r#" *\| *"#)
                .split(align_replaced.as_str())
                .map(|x| x.to_string())
                .collect();

            let rows_= if cap3.trim() != "" {
                regx(r#"\n[ \t]*$"#)
                    .replace_all(cap3, "")
                    .split("\n")
                    .map(|x| x.to_string())
                    .collect()
            } else {
                vec![]
            };

            if header.len() == align.len() {

                let mut l = align.len();
                for i in 0..l {
                    if regx(r#"^ *-+: *$"#).is_match(align.get(i).unwrap().as_str()) {
                        align[i] = "right".to_string();
                    } else if regx(r#"^ *:-+: *$"#).is_match(align.get(i).unwrap().as_str()) {
                        align[i] = "center".to_string();
                    } else if regx(r#"^ *:-+ *$"#).is_match(align.get(i).unwrap().as_str()){
                        align[i] = "left".to_string();
                    } else {
                        align[i] = "".to_string();
                    }
                }

                l = rows_.len();
                let mut rows: Vec<Vec<Rc<RefCell<Token>>>> = vec![];
                for i in 0..l {
                    let rows_i = split_cells(rows_[i].as_str(), Some(header.len()))
                        .into_iter()
                        .map(|text_val| {
                            Rc::new(RefCell::new(
                                Token {
                                    _type: "",
                                    raw: "".to_string(),
                                    href: "".to_string(),
                                    title: "".to_string(),
                                    text: text_val.to_string(),
                                    tokens: vec![],
                                    tag: "".to_string(),
                                    ordered: false,
                                    start: 0,
                                    lang: "".to_string(),
                                    loose: false,
                                    items: vec![],
                                    depth: 0,
                                    escaped: false,
                                    pre: false,
                                    task: false,
                                    checked: false,
                                    in_link: false,
                                    in_raw_block: false,
                                    links: vec![],
                                    align: vec![],
                                    rows: vec![],
                                    header: vec![],
                                    code_block_style: "".to_string()
                                }
                            ))
                        })
                        .collect::<Vec<Rc<RefCell<Token>>>>();
                    rows.push(rows_i);
                }

                // parse child tokens inside headers and cells

                // header child tokens
                l = header.len();
                for j in 0..l {
                    header[j].as_ref().borrow_mut().tokens = vec![];
                    tokens.push(
                        InlineToken {
                            src: String::from(header[j].as_ref().borrow().text.as_str()),
                            token: Rc::clone(&header[j]),
                        }
                    );
                }

                let item = Token {
                    _type: "table",
                    raw: raw.to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "".to_string(),
                    tokens: vec![],
                    tag: "".to_string(),
                    ordered: false,
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    task: false,
                    checked: false,
                    in_link: false,
                    in_raw_block: false,
                    links: vec![],
                    align,
                    rows,
                    header,
                    code_block_style: "".to_string()
                };
                return Some(item);
            }
        }
        None
    }

    fn lheading(&mut self, src: &str) -> Option<Token> {
        let lheading_caps = self.rules.block.exec_fc(src, MDBlock::LHeading, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }

    fn paragraph(&mut self, src: &str) -> Option<Token> {
        let paragraph_caps = self.rules.block.exec_fc(src, MDBlock::Paragraph, None);

        if paragraph_caps.is_some() {
            let caps = paragraph_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let text = if cap1.chars().nth(cap1.len() - 1).is_some() &&
                cap1.chars().nth(cap1.len() - 1).unwrap().to_string() == "\n"
            {
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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };
            return Some(token);
        }
        None
    }

    fn text(&mut self, src: &str) -> Option<Token> {
        let text_caps = self.rules.block.exec_fc(src, MDBlock::Text, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };
            return Some(token);
        }
        None
    }

    // Inline
    fn escape(&mut self, src: &str) -> Option<Token> {
        let escape_caps = self.rules.inline.exec_fc(src, MDInline::Escape, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn tag(&mut self, src: &str, mut in_link: &mut bool, mut in_raw_block: &mut bool) -> Option<Token> {
        let tag_caps = self.rules.inline.exec_fc(src, MDInline::Tag, None);
        if tag_caps.is_some() {
            let caps = tag_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());


            if !*in_link && regx(r#"(?i)^<a "#).is_match(raw) {
                *in_link = true;
            } else if *in_link && regx(r#"(?i)^<\\/a>"#).is_match(raw) {
                *in_link = false;
            }

            if !*in_raw_block && regx(r#"(?i)^<(pre|code|kbd|script)(\s|>)"#).is_match(raw) {
                *in_raw_block = true;
            } else if *in_raw_block && regx(r#"(?i)^<\/(pre|code|kbd|script)(\s|>)"#).is_match(raw) {
                *in_raw_block = false;
            }

            // TODO: Add inLink and inRawBlock to Token
            let token_in_link = in_link.clone();
            let token_in_raw_block = in_raw_block.clone();

            let _type = if self.options.sanitize {"text"} else {"html"};
            let text = if self.options.sanitize {
                if self.options.sanitizer.is_some() {
                    (self.options.sanitizer.unwrap())(raw)
                } else {
                    escape(raw, false)
                }
            } else {
                raw.to_string()
            };

            return Some(Token{
                _type,
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: token_in_link,
                in_raw_block: token_in_raw_block,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn link(&mut self, src: &str) -> Option<Token> {

        let link_caps = self.rules.inline.exec_fc(src, MDInline::Link, None);

        if link_caps.is_some() {

            let caps = link_caps.unwrap();
            let mut raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let mut cap2 = caps.get(2).map_or("", |m| m.as_str());
            let mut cap3 = caps.get(3).map_or("", |m| m.as_str());

            let trimmed_url = cap2.trim();
            if !self.options.pedantic && regx(r#"^<"#).is_match(trimmed_url) {
                // commonmark requires matching angle brackets
                if !regx(r#">$"#).is_match(trimmed_url) {
                    return None;
                }

                let trimmed_url_slice = slice(trimmed_url, 0..trimmed_url.len() - 1);
                let rtrim_slash = rtrim(trimmed_url_slice.as_str(), "\\", false);

                if (trimmed_url.len() - rtrim_slash.len()) % 2 == 0 {
                    return None;
                }
            } else {
                // find closing parenthesis
                let last_paren_idx = find_closing_bracket(cap2, "()");
                if last_paren_idx > -1 {
                    let _start = raw.chars().position(|c| c == '!' );

                    let start = if _start.is_some() {
                        if _start.unwrap() == 9 {
                            5 as usize
                        } else {
                            4 as usize
                        }
                    } else {
                        4 as usize
                    };

                    let link_len = start + cap1.len() + last_paren_idx as usize;
                    cap2 = &cap2[0..last_paren_idx as usize];
                    raw = &raw[0..link_len].trim();
                    cap3 = "";
                }
            }


            let mut _href = cap2;
            let mut _title = "";
            if self.options.pedantic {
                // split pedantic href and title
                let link_captures = regx(r#"^([^'"]*[^\s])\s+(['"])(.*)\2"#).captures(_href);

                if link_captures.is_some() {
                    let caps = link_captures.unwrap();
                    let link1 = caps.get(1).map_or("", |m| m.as_str());
                    let link3 = caps.get(3).map_or("", |m| m.as_str());
                    _href = link1;
                    _title = link3;
                }
            } else {
                    _title = if !cap3.is_empty() {
                        &cap3[1..cap3.len()  - 1]
                    } else {
                        ""
                    };
                }
                _href = _href.trim();

                if regx(r#"^<"#).is_match(_href) {
                    if self.options.pedantic && !(regx(r#">$"#).is_match(trimmed_url)) {
                        _href = &_href[1..];
                    } else {
                        _href = &_href[1.._href.len() - 1];
                    }
                }

                let href = if !_href.is_empty() {
                    self.rules.inline.get_grammar_fc_regex(MDInline::Escapes, None)
                        .replace_all(_href, "${1}")
                        .to_string()
                } else {
                    _href.to_string()
                };

                let title = if !_title.is_empty() {
                    self.rules.inline.get_grammar_fc_regex(MDInline::Escapes, None)
                        .replace_all(_title, "${1}")
                        .to_string()
                } else {
                    _title.to_string()
                };

                let link = Link {
                    href,
                    title,
                    tag: "".to_string()
                };

                let token = output_link(caps, link, raw.to_string());
                return Some(token);
                // Set inline tokens for token.tokens see output_link (original)
        }
        None
    }

    fn ref_link(&mut self, src: &str, mut links: &Vec<Link>) -> Option<Token> {
        let ref_link_caps = self.rules.inline.exec_fc(src, MDInline::RefLink, None);
        let no_link_caps = self.rules.inline.exec_fc(src, MDInline::NoLink, None);

        if ref_link_caps.is_some() ||
            no_link_caps.is_some()
        {
            let caps = if ref_link_caps.is_some() {
                ref_link_caps.unwrap()
            } else {
                no_link_caps.unwrap()
            };

            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());

            let link = if cap2.len() > 0 {
                regx(r#"\s+"#).replace_all(cap2, " ").to_string()
            } else {
                regx(r#"\s+"#).replace_all(cap1, " ").to_string()
            };

            let link_idx = links.iter().position(|l| l.tag == link.to_lowercase());

            if link_idx.is_none()  {
                let text = raw.chars().nth(0).unwrap().to_string();
                return Some(Token {
                    _type: "text",
                    raw: text.clone(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text,
                    tokens: vec![],
                    tag: "".to_string(),
                    ordered: false,
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    task: false,
                    checked: false,
                    in_link: false,
                    in_raw_block: false,
                    links: vec![],
                    align: vec![],
                    rows: vec![],
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }

            let link_ref = links.get(link_idx.unwrap()).unwrap();
            // TODO: may need to change Link to use Options instead
            // TODO: Similarly for aligns in table
            if link_ref.href.is_empty() {
                let text = raw.chars().nth(0).unwrap().to_string();
                return Some(Token {
                    _type: "text",
                    raw: text.clone(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text,
                    tokens: vec![],
                    tag: "".to_string(),
                    ordered: false,
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    task: false,
                    checked: false,
                    in_link: false,
                    in_raw_block: false,
                    links: vec![],
                    align: vec![],
                    rows: vec![],
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }

            let link = Link {
                href: link_ref.href.clone(),
                title: link_ref.title.clone(),
                tag: link_ref.tag.clone()
            };

            let token = output_link(caps, link, raw.to_string());
            return Some(token);
        }
        None
    }

    fn em_strong(&mut self, src: &str, masked_src: &str, prev_char: &str) -> Option<Token> {
        let em_strong_caps = self.rules.inline.exec_fc(src, MDInline::EmStrong, Some("l_delim"));
        let mut _masked_src: String = String::from(masked_src);

        if em_strong_caps.is_none() { return None; }

        let caps = em_strong_caps.unwrap();
        let raw = caps.get(0).map_or("", |m| m.as_str());
        let match1 = caps.get(1).map_or("", |m| m.as_str());
        let match2 = caps.get(2).map_or("", |m| m.as_str());
        let match3 = caps.get(3).map_or("", |m| m.as_str());

        if regx(r#"[\p{L}\p{N}]"#).is_match(prev_char) &&
            match3.len() > 0
        { return None; }



        let next_char = if match1.len() > 0 {
            match1.clone()
        } else if match2.len() > 0 {
            match2.clone()
        } else {
            ""
        };


        let punctuation_caps = self.rules.inline.exec_fc(prev_char, MDInline::Punctuation, None);

        if next_char.is_empty() ||
            (next_char.len() > 0 &&
                (prev_char == "" ||
                    punctuation_caps.is_some()
                )
            )
        {

            let mut r_length: i32 = 0;
            let l_length: i32 = (raw.len() - 1 ) as i32;
            let mut mid_delim_total: i32 = 0;
            let mut r_delim = String::from("");
            let mut delim_total: i32 = l_length.clone() as i32;

            let end_reg_str = if raw.chars().nth(0).unwrap() == '*' {
                self.rules.inline.em_strong.r_delim_ast.clone()
            } else {
                self.rules.inline.em_strong.r_delim_und.clone()
            };

            let elems: i32 = -1 * (src.len() as i32) + (l_length);
            let start_idx: usize = ((_masked_src.len() as i32) + elems) as usize;

            _masked_src = String::from(slice(_masked_src.as_str(), start_idx.._masked_src.len()));
            let end_re = fancy_regex::Regex::new(end_reg_str.as_str()).unwrap();

            for captures_res in end_re.captures_iter(_masked_src.as_str())
            {
                if captures_res.is_err() { break; }

                let end_caps = captures_res.unwrap();
                let raw_match = end_caps.get(0).unwrap();

                let _match1 = end_caps.get(1).map_or("", |m| m.as_str());
                let _match2 = end_caps.get(2).map_or("", |m| m.as_str());
                let _match3 = end_caps.get(3).map_or("", |m| m.as_str());
                let _match4 = end_caps.get(4).map_or("", |m| m.as_str());
                let _match5 = end_caps.get(5).map_or("", |m| m.as_str());
                let _match6 = end_caps.get(6).map_or("", |m| m.as_str());


                r_delim = if !_match1.is_empty() {
                    _match1.to_string()
                } else if !_match2.is_empty() {
                    _match2.to_string()
                } else if !_match3.is_empty() {
                    _match3.to_string()
                } else if !_match4.is_empty() {
                    _match4.to_string()
                } else if !_match5.is_empty() {
                    _match5.to_string()
                } else if !_match6.is_empty(){
                    _match6.to_string()
                } else {
                    "".to_string()
                };

                // skip single * in __abc*abc__
                if r_delim.is_empty() {
                    continue;
                }


                r_length = r_delim.len() as i32;


                // found another Left Delim
                if _match3.len() > 0 ||
                    _match4.len() > 0
                {
                    delim_total += r_length;
                    continue;
                } else if _match5.len() > 0 || // either Left or Right Delim
                    _match6.len() > 0
                {
                    if is_not_divisible(l_length , 3) &&
                        is_divisible(l_length + r_length, 3)
                    {
                        mid_delim_total += r_length;
                        continue; // CommonMark Emphasis Rules 9-10
                    }
                }


                delim_total -= r_length;
                if delim_total > 0 {
                    continue;
                } // Haven't found enough closing delimiters

                // Remove extra characters. *a*** -> *a*
                r_length = (min(r_length, r_length + delim_total + mid_delim_total)).clone();

                // Create `em` if smallest delimiter has odd char count. *a***
                if is_odd(min(l_length, r_length))
                {

                    let text_end_idx = (l_length + (raw_match.start() as i32) + r_length) as usize;
                    let raw_end_idx = (l_length + (raw_match.start() as i32) + r_length + 1) as usize;

                    let text = slice(src, 1..text_end_idx);
                    let raw = slice(src, 0..raw_end_idx);

                    return Some(Token {
                        _type: "em",
                        raw: raw.to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: text.to_string(),
                        tokens: vec![],
                        tag: "".to_string(),
                        ordered: false,
                        start: 0,
                        lang: "".to_string(),
                        loose: false,
                        items: vec![],
                        depth: 0,
                        escaped: false,
                        pre: false,
                        task: false,
                        checked: false,
                        in_link: false,
                        in_raw_block: false,
                        links: vec![],
                        align: vec![],
                        rows: vec![],
                        header: vec![],
                        code_block_style: "".to_string()
                    });
                }

                // Create 'strong' if smallest delimiter has even char count. **a***
                let text_end_idx = (l_length + (raw_match.start() as i32) + r_length - 1) as usize;
                let raw_end_idx = (l_length + (raw_match.start() as i32) + r_length + 1) as usize;

                let text = slice(src, 2..text_end_idx);
                let raw = slice(src, 0..raw_end_idx);


                return Some(Token {
                    _type: "strong",
                    raw: raw.to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: text.to_string(),
                    tokens: vec![],
                    tag: "".to_string(),
                    ordered: false,
                    start: 0,
                    lang: "".to_string(),
                    loose: false,
                    items: vec![],
                    depth: 0,
                    escaped: false,
                    pre: false,
                    task: false,
                    checked: false,
                    in_link: false,
                    in_raw_block: false,
                    links: vec![],
                    align: vec![],
                    rows: vec![],
                    header: vec![],
                    code_block_style: "".to_string()
                });
            }
        }
        None
    }

    fn code_span(&mut self, src: &str) -> Option<Token> {
        let code_span_caps = self.rules.inline.exec_fc(src, MDInline::Code, None);
        if code_span_caps.is_some() {
            let caps = code_span_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());

            let mut text = regx(r#"\n"#).replace_all(cap2, " ").to_string();

            let has_non_space_chars = regx(r#"[^ ]"#).is_match(text.as_str());
            let has_space_chars_on_both_ends = regx(r#"^ "#).is_match(text.as_str()) && regx(r#" $"#).is_match(text.as_str());

            if has_non_space_chars &&
                has_space_chars_on_both_ends
            {
                text = slice(text.as_str(), 1..text.len() - 1);
            }

            text = escape(text.as_str(), true);

            return Some(Token {
                _type: "codespan",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: text.to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn br(&mut self, src: &str) -> Option<Token> {
        let br_caps = self.rules.inline.exec_fc(src, MDInline::Br, None);

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
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }

    fn del(&mut self, src: &str) -> Option<Token> {
        if self.rules.inline.del.is_empty() { return None; }

        let del_caps = self.rules.inline.exec_fc(src, MDInline::Del, None);

        if del_caps.is_some() &&
            self.rules.inline.del != ""
        {
            let caps = del_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let caps_2 = caps.get(2).map_or("", |m| m.as_str());

            let token = Token {
                _type: "del",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: caps_2.to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }

    fn autolink(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        let autolink_caps = self.rules.inline.exec_fc(src, MDInline::Autolink, None);
        if autolink_caps.is_some() {
            let caps = autolink_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());

            let mut text = String::from("");
            let mut href = String::from("");

            if cap2 == "@" {
                text = if self.options.mangle {
                    escape(mangle(cap1).as_str(), false)
                } else {
                    cap1.to_string()
                };
                href = format!("mailto:{}", text);
            } else {
                text = escape(cap1, false);
                href = text.clone();
            }
            return Some(Token {
                _type: "link",
                raw: raw.to_string(),
                href: href.to_string(),
                title: "".to_string(),
                text: text.to_string(),
                tokens: vec![
                    Rc::new(RefCell::new(
                        Token {
                            _type: "text",
                            raw: text.clone(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: text.clone(),
                            tokens: vec![],
                            tag: "".to_string(),
                            ordered: false,
                            start: 0,
                            lang: "".to_string(),
                            loose: false,
                            items: vec![],
                            depth: 0,
                            escaped: false,
                            pre: false,
                            task: false,
                            checked: false,
                            in_link: false,
                            in_raw_block: false,
                            links: vec![],
                            align: vec![],
                            rows: vec![],
                            header: vec![],
                            code_block_style: "".to_string()
                        }
                    ))
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn url(&mut self, src: &str, mangle: fn(text: &str) -> String) -> Option<Token> {
        if  self.rules.inline.url.is_empty() { return None; }

        let url_caps = self.rules.inline.exec_fc(src, MDInline::Url, None);
        if url_caps.is_some() {
            let caps = url_caps.unwrap();
            let mut raw = caps.get(0).map_or("", |m| m.as_str());
            let cap1 = caps.get(1).map_or("", |m| m.as_str());
            let cap2 = caps.get(2).map_or("", |m| m.as_str());

            let mut text = String::from("");
            let mut href = String::from("");

            if cap2 == "@" {
                text = if self.options.mangle {
                    escape(mangle(raw).as_str(), false)
                } else {
                    raw.to_string()
                };
                href = format!("mailto:{}", text);
            } else {
                // do extended autolink path validation
                let mut prev_cap_zero = raw.to_string().clone();

                loop {
                    prev_cap_zero = raw.to_string().clone();
                    let backpedal_caps = self.rules.inline.exec_fc(raw, MDInline::Backpedal, None);
                    if backpedal_caps.is_some() {
                        let caps = backpedal_caps.unwrap();
                        raw = caps.get(0).map_or("", |m| m.as_str());
                    } else {
                        break;
                    }
                    if prev_cap_zero == raw { break; }
                }

                text = escape(raw, false);
                if cap1 == "www." {
                    href = format!("http://{}", text);
                } else {
                    href = text.clone();
                }
            }
            return Some(Token {
                _type: "link",
                raw: raw.to_string(),
                href: href.to_string(),
                title: "".to_string(),
                text: text.to_string(),
                tokens: vec![
                    Rc::new(RefCell::new(Token{
                        _type: "text",
                        raw: text.clone(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: text.clone(),
                        tokens: vec![],
                        tag: "".to_string(),
                        ordered: false,
                        start: 0,
                        lang: "".to_string(),
                        loose: false,
                        items: vec![],
                        depth: 0,
                        escaped: false,
                        pre: false,
                        task: false,
                        checked: false,
                        in_link: false,
                        in_raw_block: false,
                        links: vec![],
                        align: vec![],
                        rows: vec![],
                        header: vec![],
                        code_block_style: "".to_string()
                    }))
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block: false,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            });
        }
        None
    }

    fn inline_text(&mut self, src: &str, in_raw_block: bool, smartypants: fn(&str) -> String) -> Option<Token> {
        let inline_caps = self.rules.inline.exec_fc(src, MDInline::Text, None);

        if inline_caps.is_some() {
            let caps = inline_caps.unwrap();
            let raw = caps.get(0).map_or("", |m| m.as_str());
            let mut text = "".to_string();

            // TODO: Move this to inside lexer
            if in_raw_block {
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

            let token =
                Token {
                _type: "text",
                raw: raw.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text,
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: false,
                task: false,
                checked: false,
                in_link: false,
                in_raw_block,
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            };

            return Some(token);
        }
        None
    }
}

pub fn output_link(caps: Captures, link: Link, raw: String) -> Token {

    let cap0 = caps.get(0).map_or("", |m| m.as_str());
    let cap1=  caps.get(1).map_or("", |m| m.as_str());

    let href = link.href.to_string();
    let title = if link.title.len() > 0 {
        escape(link.title.as_str(), false).to_string()
    } else {
        "".to_string()
    };

    let text = regx(r#"\\([\[\]])"#).replace_all(cap1, "${1}");

    if cap0.chars().nth(0).unwrap() != '!' {
        let token = Token {
            _type: "link",
            raw,
            href,
            title,
            text: text.to_string(),
            tokens: vec![],
            tag: "".to_string(),
            ordered: false,
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
            task: false,
            checked: false,
            in_link: false,
            in_raw_block: false,
            links: vec![],
            align: vec![],
            rows: vec![],
            header: vec![],
            code_block_style: "".to_string()
        };
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
            ordered: false,
            start: 0,
            lang: "".to_string(),
            loose: false,
            items: vec![],
            depth: 0,
            escaped: false,
            pre: false,
            task: false,
            checked: false,
            in_link: false,
            in_raw_block: false,
            links: vec![],
            align: vec![],
            rows: vec![],
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
                return slice(node, indent_to_code.len()..node.len());
            }

            return node.to_string();
        })
        .collect::<Vec<String>>()
        .join("\n");
}

pub fn slice(s: &str, range: Range<usize>) -> String {
    if s.len() > range.start && s.len() >= range.end {
        s.chars().take(range.end).skip(range.start).collect()
    } else {
        "".to_string()
    }
}



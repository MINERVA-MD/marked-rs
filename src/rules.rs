use regex::Regex;
use fancy_regex::*;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use crate::defaults::Options;

lazy_static! {
    static ref CARET: regex::Regex = regex::Regex::new("(^|[^\\[])\\^").unwrap();
}

pub enum MDBlock {
    Newline,
    Code,
    Fences,
    Hr,
    Heading,
    Blockquote,
    List,
    Html,
    Def,
    Table,
    LHeading,
    Paragraph,
    Text,
    Label,
    Title,
    Bullet,
    ListItemStart,
    Tag,
    Comment
}

pub enum MDInline {
    Escape,
    Autolink,
    Url,
    Tag,
    Link,
    RefLink,
    NoLink,
    RefLinkSearch,
    EmStrong,
    Code,
    Br,
    Del,
    Text,
    Punctuation,
    _Punctuation,
    BlockSkip,
    EscapedEmSt,
    Comment,
    Escapes,
    Scheme,
    Email,
    Attribute,
    Label,
    Href,
    Title,
    Breaks,
    Strong,
    Em,
    ExtendedEmail,
    Backpedal
}

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub newline: String,
    pub code: String,
    pub fences: String,
    pub hr: String,
    pub heading: String,
    pub blockquote: String,
    pub list: String,
    pub html: String,
    pub def: String,
    pub table: String,
    pub l_heading: String,
    pub paragraph: String,
    pub text: String,
    pub label: String,
    pub title: String,
    pub bullet: String,
    pub list_item_start: String,
    pub tag: String,
    pub comment: String
}

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Delim {
    pub l_delim: String,
    pub r_delim_ast: String,
    pub r_delim_und: String
}

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Inline {
    pub escape: String,
    pub autolink: String,
    pub url: String,
    pub tag: String,
    pub link: String,
    pub ref_link: String,
    pub no_link: String,
    pub ref_link_search: String,
    pub em_strong: Delim,
    pub code: String,
    pub br: String,
    pub del: String,
    pub text: String,
    pub punctuation: String,
    pub _punctuation: String,
    pub block_skip: String,
    pub escaped_em_st: String,
    pub comment: String,
    pub escapes: String,
    pub scheme: String,
    pub email: String,
    pub attribute: String,
    pub label: String,
    pub href: String,
    pub title: String,
    pub breaks: String,
    pub strong: Bold,
    pub em: Bold,
    pub extended_email: String,
    pub backpedal: String
}

#[derive(Clone)]
#[derive(PartialEq, PartialOrd)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Bold {
    pub start: String,
    pub middle: String,
    pub end_ast: String,
    pub end_und: String
}

pub struct Edit {
    pub regex_str: String,
    pub opt: String
}

impl Block {
    pub fn set_grammar_regex(&mut self, rule: MDBlock, regex_str: &str) {
        match rule {
            MDBlock::Newline        => { self.newline = regex_str.to_string(); }
            MDBlock::Code           => { self.code = regex_str.to_string(); }
            MDBlock::Fences         => { self.fences = regex_str.to_string(); }
            MDBlock::Hr             => { self.hr = regex_str.to_string(); }
            MDBlock::Heading        => { self.heading = regex_str.to_string(); }
            MDBlock::Blockquote     => { self.blockquote = regex_str.to_string(); }
            MDBlock::List           => { self.list = regex_str.to_string(); }
            MDBlock::Html           => { self.html = regex_str.to_string(); }
            MDBlock::Def            => { self.def = regex_str.to_string(); }
            MDBlock::Table          => { self.table = regex_str.to_string(); }
            MDBlock::LHeading       => { self.l_heading = regex_str.to_string(); }
            MDBlock::Paragraph      => { self.paragraph = regex_str.to_string(); }
            MDBlock::Text           => { self.text = regex_str.to_string(); }
            MDBlock::Label          => { self.label = regex_str.to_string(); }
            MDBlock::Title          => { self.title = regex_str.to_string(); }
            MDBlock::Bullet         => { self.bullet = regex_str.to_string(); }
            MDBlock::ListItemStart  => { self.list_item_start = regex_str.to_string(); }
            MDBlock::Tag            => { self.tag = regex_str.to_string(); }
            MDBlock::Comment        => { self.comment = regex_str.to_string(); }
        }
    }

    pub fn get_grammar_regex(&self, rule: MDBlock, opt: Option<&str>) -> regex::Regex {
        match rule {
            MDBlock::Newline        => regex::Regex::new(self.newline.as_str()).unwrap(),
            MDBlock::Code           => regex::Regex::new(self.code.as_str()).unwrap(),
            MDBlock::Fences         => regex::Regex::new(self.fences.as_str()).unwrap(),
            MDBlock::Hr             => regex::Regex::new(self.hr.as_str()).unwrap(),
            MDBlock::Heading        => regex::Regex::new(self.heading.as_str()).unwrap(),
            MDBlock::Blockquote     => regex::Regex::new(self.blockquote.as_str()).unwrap(),
            MDBlock::List           => regex::Regex::new(self.list.as_str()).unwrap(),
            MDBlock::Html           => regex::Regex::new(self.html.as_str()).unwrap(),
            MDBlock::Def            => regex::Regex::new(self.def.as_str()).unwrap(),
            MDBlock::Table          => regex::Regex::new(self.table.as_str()).unwrap(),
            MDBlock::LHeading       => regex::Regex::new(self.l_heading.as_str()).unwrap(),
            MDBlock::Paragraph      => regex::Regex::new(self.paragraph.as_str()).unwrap(),
            MDBlock::Text           => regex::Regex::new(self.text.as_str()).unwrap(),
            MDBlock::Label          => regex::Regex::new(self.label.as_str()).unwrap(),
            MDBlock::Title          => regex::Regex::new(self.title.as_str()).unwrap(),
            MDBlock::Bullet         => regex::Regex::new(self.bullet.as_str()).unwrap(),
            MDBlock::ListItemStart  => regex::Regex::new(self.list_item_start.as_str()).unwrap(),
            MDBlock::Tag            => regex::Regex::new(self.tag.as_str()).unwrap(),
            MDBlock::Comment        => regex::Regex::new(self.comment.as_str()).unwrap(),
        }
    }

    pub fn exec<'a>(&self, src: &'a str, rule: MDBlock, opt: Option<&'a str>) -> Option<regex::Captures<'a>> {
        self.get_grammar_regex(rule, opt).captures(src)
    }


    pub fn get_grammar_fc_regex(&self, rule: MDBlock, opt: Option<&str>) -> fancy_regex::Regex {
        match rule {
            MDBlock::Newline        => fancy_regex::Regex::new(self.newline.as_str()).unwrap(),
            MDBlock::Code           => fancy_regex::Regex::new(self.code.as_str()).unwrap(),
            MDBlock::Fences         => fancy_regex::Regex::new(self.fences.as_str()).unwrap(),
            MDBlock::Hr             => fancy_regex::Regex::new(self.hr.as_str()).unwrap(),
            MDBlock::Heading        => fancy_regex::Regex::new(self.heading.as_str()).unwrap(),
            MDBlock::Blockquote     => fancy_regex::Regex::new(self.blockquote.as_str()).unwrap(),
            MDBlock::List           => fancy_regex::Regex::new(self.list.as_str()).unwrap(),
            MDBlock::Html           => fancy_regex::Regex::new(self.html.as_str()).unwrap(),
            MDBlock::Def            => fancy_regex::Regex::new(self.def.as_str()).unwrap(),
            MDBlock::Table          => fancy_regex::Regex::new(self.table.as_str()).unwrap(),
            MDBlock::LHeading       => fancy_regex::Regex::new(self.l_heading.as_str()).unwrap(),
            MDBlock::Paragraph      => fancy_regex::Regex::new(self.paragraph.as_str()).unwrap(),
            MDBlock::Text           => fancy_regex::Regex::new(self.text.as_str()).unwrap(),
            MDBlock::Label          => fancy_regex::Regex::new(self.label.as_str()).unwrap(),
            MDBlock::Title          => fancy_regex::Regex::new(self.title.as_str()).unwrap(),
            MDBlock::Bullet         => fancy_regex::Regex::new(self.bullet.as_str()).unwrap(),
            MDBlock::ListItemStart  => fancy_regex::Regex::new(self.list_item_start.as_str()).unwrap(),
            MDBlock::Tag            => fancy_regex::Regex::new(self.tag.as_str()).unwrap(),
            MDBlock::Comment        => fancy_regex::Regex::new(self.comment.as_str()).unwrap(),
        }
    }

    pub fn exec_fc<'a>(&self, src: &'a str, rule: MDBlock, opt: Option<&'a str>) -> Option<fancy_regex::Captures<'a>> {
        self.get_grammar_fc_regex(rule, opt).captures(src).unwrap()
    }
}

impl Inline {

    pub fn set_grammar_regex(&mut self, rule: MDInline, regex_str: &str, opt: Option<&str>) {
        match rule {
            MDInline::Escape            => { self.escape = regex_str.to_string(); }
            MDInline::Autolink          => { self.autolink = regex_str.to_string(); }
            MDInline::Url               => { self.url = regex_str.to_string(); }
            MDInline::Tag               => { self.tag = regex_str.to_string(); }
            MDInline::Link              => { self.link = regex_str.to_string(); }
            MDInline::RefLink           => { self.ref_link = regex_str.to_string(); }
            MDInline::NoLink            => { self.no_link = regex_str.to_string(); }
            MDInline::RefLinkSearch     => { self.ref_link_search = regex_str.to_string(); }
            MDInline::EmStrong          => {
                if opt.unwrap() == "l_delim" { self.em_strong.l_delim = regex_str.to_string(); }
                else if opt.unwrap() == "r_delim_ast" { self.em_strong.r_delim_ast = regex_str.to_string(); }
                else { self.em_strong.r_delim_und = regex_str.to_string();}
            }

            MDInline::Code              => { self.code = regex_str.to_string(); }
            MDInline::Br                => { self.br = regex_str.to_string(); }
            MDInline::Del               => { self.del = regex_str.to_string(); }
            MDInline::Text              => { self.text = regex_str.to_string(); }
            MDInline::Punctuation       => { self.punctuation = regex_str.to_string(); }
            MDInline::_Punctuation      => { self._punctuation = regex_str.to_string(); }
            MDInline::BlockSkip         => { self.block_skip = regex_str.to_string(); }
            MDInline::EscapedEmSt       => { self.escaped_em_st = regex_str.to_string(); }
            MDInline::Comment           => { self.comment = regex_str.to_string(); }
            MDInline::Escapes           => { self.escapes = regex_str.to_string(); }
            MDInline::Scheme            => { self.scheme = regex_str.to_string(); }
            MDInline::Email             => { self.email = regex_str.to_string(); }
            MDInline::Attribute         => { self.attribute = regex_str.to_string(); }
            MDInline::Label             => { self.label = regex_str.to_string(); }
            MDInline::Href              => { self.href = regex_str.to_string(); }
            MDInline::Title             => { self.title = regex_str.to_string(); }
            MDInline::Breaks            => { self.breaks = regex_str.to_string(); }
            MDInline::Strong            => {
                if opt.unwrap() == "start"        { self.strong.start = regex_str.to_string(); }
                else if opt.unwrap() == "end_ast" { self.strong.end_ast = regex_str.to_string(); }
                else if opt.unwrap() == "end_und" { self.strong.end_und = regex_str.to_string(); }
                else                              { self.strong.middle = regex_str.to_string(); }
            }
            MDInline::Em                => {
                if opt.unwrap() == "start"        { self.em.start = regex_str.to_string(); }
                else if opt.unwrap() == "end_ast" { self.em.end_ast = regex_str.to_string(); }
                else if opt.unwrap() == "end_und" { self.em.end_und = regex_str.to_string(); }
                else                              { self.em.middle = regex_str.to_string(); }
            }
            MDInline::ExtendedEmail     => { self.extended_email = regex_str.to_string(); }
            MDInline::Backpedal     => { self.backpedal = regex_str.to_string(); }
        }

    }

    pub fn get_grammar_regex(&self, rule: MDInline, opt: Option<&str>) -> regex::Regex {
        match rule {
            MDInline::Escape            => regex::Regex::new(self.escape.as_str()).unwrap(),
            MDInline::Autolink          => regex::Regex::new(self.autolink.as_str()).unwrap(),
            MDInline::Url               => regex::Regex::new(self.url.as_str()).unwrap(),
            MDInline::Tag               => regex::Regex::new(self.tag.as_str()).unwrap(),
            MDInline::Link              => regex::Regex::new(self.link.as_str()).unwrap(),
            MDInline::RefLink           => regex::Regex::new(self.ref_link.as_str()).unwrap(),
            MDInline::NoLink            => regex::Regex::new(self.no_link.as_str()).unwrap(),
            MDInline::RefLinkSearch     => regex::Regex::new(self.ref_link_search.as_str()).unwrap(),
            MDInline::EmStrong          => {
                return if opt.unwrap() == "l_delim"     { regex::Regex::new(self.em_strong.l_delim.as_str()).unwrap() }
                else if opt.unwrap() == "r_delim_ast"   { regex::Regex::new(self.em_strong.r_delim_ast.as_str()).unwrap() }
                else                                    { regex::Regex::new(self.em_strong.r_delim_und.as_str()).unwrap() }
            },
            MDInline::Code              => regex::Regex::new(self.code.as_str()).unwrap(),
            MDInline::Br                => regex::Regex::new(self.br.as_str()).unwrap(),
            MDInline::Del               => regex::Regex::new(self.del.as_str()).unwrap(),
            MDInline::Text              => regex::Regex::new(self.text.as_str()).unwrap(),
            MDInline::Punctuation       => regex::Regex::new(self.punctuation.as_str()).unwrap(),
            MDInline::_Punctuation      => regex::Regex::new(self._punctuation.as_str()).unwrap(),
            MDInline::BlockSkip         => regex::Regex::new(self.block_skip.as_str()).unwrap(),
            MDInline::EscapedEmSt       => regex::Regex::new(self.escaped_em_st.as_str()).unwrap(),
            MDInline::Comment           => regex::Regex::new(self.comment.as_str()).unwrap(),
            MDInline::Escapes           => regex::Regex::new(self.escapes.as_str()).unwrap(),
            MDInline::Scheme            => regex::Regex::new(self.scheme.as_str()).unwrap(),
            MDInline::Email             => regex::Regex::new(self.email.as_str()).unwrap(),
            MDInline::Attribute         => regex::Regex::new(self.attribute.as_str()).unwrap(),
            MDInline::Label             => regex::Regex::new(self.label.as_str()).unwrap(),
            MDInline::Href              => regex::Regex::new(self.href.as_str()).unwrap(),
            MDInline::Title             => regex::Regex::new(self.title.as_str()).unwrap(),
            MDInline::Breaks            => regex::Regex::new(self.breaks.as_str()).unwrap(),
            MDInline::Strong            => {
                return if opt.unwrap() == "start" { regex::Regex::new(self.strong.start.as_str()).unwrap() }
                else if opt.unwrap() == "end_ast" { regex::Regex::new(self.strong.end_ast.as_str()).unwrap() }
                else if opt.unwrap() == "end_und" { regex::Regex::new(self.strong.end_und.as_str()).unwrap() }
                else                              { regex::Regex::new(self.strong.middle.as_str()).unwrap() }
            }
            MDInline::Em                => {
                return if opt.unwrap() == "start" { regex::Regex::new(self.em.start.as_str()).unwrap() }
                else if opt.unwrap() == "end_ast" { regex::Regex::new(self.em.end_ast.as_str()).unwrap() }
                else if opt.unwrap() == "end_und" { regex::Regex::new(self.em.end_und.as_str()).unwrap() }
                else                              { regex::Regex::new(self.em.middle.as_str()).unwrap() }
            }
            MDInline::ExtendedEmail         => regex::Regex::new(self.extended_email.as_str()).unwrap(),
            MDInline::Backpedal             => regex::Regex::new(self.backpedal.as_str()).unwrap()
        }
    }

    pub fn get_grammar_fc_regex(&self, rule: MDInline, opt: Option<&str>) -> fancy_regex::Regex {
        match rule {
            MDInline::Escape            => fancy_regex::Regex::new(self.escape.as_str()).unwrap(),
            MDInline::Autolink          => fancy_regex::Regex::new(self.autolink.as_str()).unwrap(),
            MDInline::Url               => fancy_regex::Regex::new(self.url.as_str()).unwrap(),
            MDInline::Tag               => fancy_regex::Regex::new(self.tag.as_str()).unwrap(),
            MDInline::Link              => fancy_regex::Regex::new(self.link.as_str()).unwrap(),
            MDInline::RefLink           => fancy_regex::Regex::new(self.ref_link.as_str()).unwrap(),
            MDInline::NoLink            => fancy_regex::Regex::new(self.no_link.as_str()).unwrap(),
            MDInline::RefLinkSearch     => fancy_regex::Regex::new(self.ref_link_search.as_str()).unwrap(),
            MDInline::EmStrong          => {
                return if opt.unwrap() == "l_delim"     { fancy_regex::Regex::new(self.em_strong.l_delim.as_str()).unwrap() }
                else if opt.unwrap() == "r_delim_ast"   { fancy_regex::Regex::new(self.em_strong.r_delim_ast.as_str()).unwrap() }
                else                                    { fancy_regex::Regex::new(self.em_strong.r_delim_und.as_str()).unwrap() }
            },
            MDInline::Code              => fancy_regex::Regex::new(self.code.as_str()).unwrap(),
            MDInline::Br                => fancy_regex::Regex::new(self.br.as_str()).unwrap(),
            MDInline::Del               => fancy_regex::Regex::new(self.del.as_str()).unwrap(),
            MDInline::Text              => fancy_regex::Regex::new(self.text.as_str()).unwrap(),
            MDInline::Punctuation       => fancy_regex::Regex::new(self.punctuation.as_str()).unwrap(),
            MDInline::_Punctuation      => fancy_regex::Regex::new(self._punctuation.as_str()).unwrap(),
            MDInline::BlockSkip         => fancy_regex::Regex::new(self.block_skip.as_str()).unwrap(),
            MDInline::EscapedEmSt       => fancy_regex::Regex::new(self.escaped_em_st.as_str()).unwrap(),
            MDInline::Comment           => fancy_regex::Regex::new(self.comment.as_str()).unwrap(),
            MDInline::Escapes           => fancy_regex::Regex::new(self.escapes.as_str()).unwrap(),
            MDInline::Scheme            => fancy_regex::Regex::new(self.scheme.as_str()).unwrap(),
            MDInline::Email             => fancy_regex::Regex::new(self.email.as_str()).unwrap(),
            MDInline::Attribute         => fancy_regex::Regex::new(self.attribute.as_str()).unwrap(),
            MDInline::Label             => fancy_regex::Regex::new(self.label.as_str()).unwrap(),
            MDInline::Href              => fancy_regex::Regex::new(self.href.as_str()).unwrap(),
            MDInline::Title             => fancy_regex::Regex::new(self.title.as_str()).unwrap(),
            MDInline::Breaks            => fancy_regex::Regex::new(self.breaks.as_str()).unwrap(),
            MDInline::Strong            => {
                return if opt.unwrap() == "start" { fancy_regex::Regex::new(self.strong.start.as_str()).unwrap() }
                else if opt.unwrap() == "end_ast" { fancy_regex::Regex::new(self.strong.end_ast.as_str()).unwrap() }
                else if opt.unwrap() == "end_und" { fancy_regex::Regex::new(self.strong.end_und.as_str()).unwrap() }
                else                              { fancy_regex::Regex::new(self.strong.middle.as_str()).unwrap() }
            }
            MDInline::Em                => {
                return if opt.unwrap() == "start" { fancy_regex::Regex::new(self.em.start.as_str()).unwrap() }
                else if opt.unwrap() == "end_ast" { fancy_regex::Regex::new(self.em.end_ast.as_str()).unwrap() }
                else if opt.unwrap() == "end_und" { fancy_regex::Regex::new(self.em.end_und.as_str()).unwrap() }
                else                              { fancy_regex::Regex::new(self.em.middle.as_str()).unwrap() }
            }
            MDInline::ExtendedEmail         => fancy_regex::Regex::new(self.extended_email.as_str()).unwrap(),
            MDInline::Backpedal             => fancy_regex::Regex::new(self.backpedal.as_str()).unwrap()
        }
    }

    pub fn exec<'a>(&self, src: &'a str, rule: MDInline, opt: Option<&'a str>) -> Option<regex::Captures<'a>> {
        self.get_grammar_regex(rule, opt).captures(src)
    }

    pub fn exec_fc<'a>(&self, src: &'a str, rule: MDInline, opt: Option<&'a str>) -> Option<Captures<'a>> {
        self.get_grammar_fc_regex(rule, opt).captures(src).unwrap()
    }
}

impl Edit {
    pub fn new(regex_str: String, opt: &str) -> Self {
        Self {
            regex_str,
            opt: opt.to_string()
        }
    }

    pub fn replace(mut self, name: &str, val: &str) -> Self {
        let _val = CARET.replace_all(val, "${1}");
        self.regex_str = self.regex_str.replacen(name, _val.to_string().as_str(), 1);
        self
    }

    pub fn replacen(mut self, name: &str, val: &str, count: usize) -> Self {
        let _val = CARET.replace_all(val, "${1}");
        self.regex_str = self.regex_str.replacen(name, _val.to_string().as_str(), count);
        self
    }

    pub fn replace_all(mut self, name: &str, val: &str) -> Self {
        let _val = CARET.replace_all(val, "${1}");
        self.regex_str = self.regex_str.replace(name, _val.to_string().as_str());
        self
    }

    pub fn get_regex_str(&mut self) -> String {
        return self.regex_str.to_string()
    }

    pub fn get_regex(&mut self) -> Regex {
        return Regex::new(self.regex_str.as_str()).unwrap();
    }
}

pub struct Rules {
    pub block: Block,
    pub inline: Inline
}

pub fn setup_block_rules() ->  Vec<Block> {

    /* Normal MD */
    let mut normal_block = Block {
        newline: "^(?: *(?:\\n|$))+".to_string(),
        code: "^( {4}[^\\n]+(?:\\n(?: *(?:\\n|$))*)?)+".to_string(),
        fences: "^ {0,3}(`{3,}(?=[^`\\n]*\\n)|~{3,})([^\\n]*)\\n(?:|([\\s\\S]*?)\\n)(?: {0,3}\\1[~`]* *(?=\\n|$)|$)".to_string(),
        hr: "^ {0,3}((?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$)".to_string(),
        heading: "^ {0,3}(#{1,6})(?=\\s|$)(.*)(?:\\n+|$)".to_string(),
        blockquote: "^( {0,3}> ?(paragraph|[^\\n]*)(?:\\n|$))+".to_string(),
        list: "^( {0,3}bull)( [^\\n]+?)?(?:\\n|$)".to_string(),
        html: "^ {0,3}(?:<(script|pre|style|textarea)[\\s>][\\s\\S]*?(?:<\\/\\1>[^\\n]*\\n+|$)|comment[^\\n]*(\\n+|$)|<\\?[\\s\\S]*?(?:\\?>\\n*|$)|<![A-Z][\\s\\S]*?(?:>\\n*|$)|<!\\[CDATA\\[[\\s\\S]*?(?:\\]\\]>\\n*|$)|<\\/?(tag)(?: +|\\n|\\/?>)[\\s\\S]*?(?:(?:\\n *)+\\n|$)|<(?!script|pre|style|textarea)([a-z][\\w-]*)(?:attribute)*? *\\/?>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n *)+\\n|$)|<\\/(?!script|pre|style|textarea)[a-z][\\w-]*\\s*>(?=[ \\t]*(?:\\n|$))[\\s\\S]*?(?:(?:\\n *)+\\n|$))".to_string(),
        def: "^ {0,3}\\[(label)\\]: *(?:\\n *)?<?([^\\s>]+)>?(?:(?: +(?:\\n *)?| *\\n *)(title))? *(?:\\n+|$)".to_string(),
        table: "".to_string(),
        l_heading: "^([^\\n]+)\\n {0,3}(=+|-+) *(?:\\n+|$)".to_string(),
        paragraph: "^([^\\n]+(?:\\n(?!hr|lheading|heading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)".to_string(),
        text: "^[^\\n]+".to_string(),
        label: "(?!\\s*\\])(?:\\\\.|[^\\[\\]\\\\])+".to_string(),
        title: r#"(?:"(?:\\"?|[^"\\])*"|'[^'\n]*(?:\n[^'\n]+)*\n?'|\([^()]*\))"#.to_string(),
        bullet: "(?:[*+-]|\\d{1,9}[.)])".to_string(),
        list_item_start: "".to_string(),
        tag: "address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[1-6]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul".to_string(),
        comment: "<!--(?!-?>)[\\s\\S]*?(?:-->|$)".to_string()
    };

    normal_block.set_grammar_regex(
        MDBlock::Def,
        Edit::new(normal_block.def.to_string(), "")
            .replace("label", normal_block.label.as_str())
            .replace("title", normal_block.title.as_str())
            .get_regex_str().as_str()
    );

    normal_block.set_grammar_regex(
        MDBlock::ListItemStart,
        Edit::new(String::from("^( *)(bull) *"), normal_block.bullet.as_str())
            .replace("bull", normal_block.bullet.as_str())
            .get_regex_str().as_str()
    );


    normal_block.set_grammar_regex(
        MDBlock::List,
        Edit::new(normal_block.list.to_string(), "")
            .replace("bull", normal_block.bullet.as_str())
            .replace("hr", "\\n+(?=\\1?(?:(?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$))")
            .replace("def",  format!("\\n+(?={})", normal_block.def).as_str())
            .get_regex_str().as_str()
    );

    normal_block.set_grammar_regex(
        MDBlock::Html,
        Edit::new(normal_block.html.to_string(), "")
            .replace("comment", normal_block.comment.as_str())
            .replace("tag", normal_block.tag.as_str())
            .replace("attribute", r#" +[a-zA-Z:_][\w.:-]*(?: *= *"[^"\n]*"| *= *'[^'\n]*'| *= *[^\s"'=<>`]+)?"#)
            .get_regex_str().as_str()
    );

    normal_block.set_grammar_regex(
       MDBlock::Paragraph,
        Edit::new(normal_block.paragraph.to_string(), "")
            .replace("hr", normal_block.hr.as_str())
            .replace("|lheading", "")
            .replace("heading", " {0,3}#{1,6} ")
            .replace("|table", "")
            .replace("blockquote", " {0,3}>")
            .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
            .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
            .replace("html", "<\\/?(?:tag)(?: +|\\n|\\/?>)|<(?:script|pre|style|textarea|!--)")
            .replace("tag", normal_block.tag.as_str())
            .get_regex_str().as_str()
    );

    normal_block.set_grammar_regex(
        MDBlock::Blockquote,
        Edit::new(normal_block.blockquote.to_string(), "")
            .replace("paragraph", normal_block.paragraph.as_str())
            .get_regex_str().as_str()
    );


    /* GFM Block Grammar */
    let mut gfm_block = normal_block.clone();

    gfm_block.set_grammar_regex(MDBlock::Table,
                                "^ *([^\\n ].*\\|.*)\\n {0,3}(?:\\| *)?(:?-+:? *(?:\\| *:?-+:? *)*)(?:\\| *)?(?:\\n((?:(?! *\\n|hr|heading|blockquote|code|fences|list|html).*(?:\\n|$))*)\\n*|$)"
    );

    gfm_block.set_grammar_regex(
        MDBlock::Table,
        Edit::new(gfm_block.table.to_string(), "")
            .replace("hr", gfm_block.hr.as_str())
            .replace("heading", " {0,3}#{1,6} ")
            .replace("blockquote", " {0,3}>")
            .replace("code", " {4}[^\\n]")
            .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
            .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
            .replace("html", "<\\/?(?:tag)(?: +|\\n|\\/?>)|<(?:script|pre|style|textarea|!--)")
            .replace("tag", gfm_block.tag.as_str())
            .get_regex_str().as_str()
    );


    gfm_block.set_grammar_regex(
        MDBlock::Paragraph,
        Edit::new("^([^\\n]+(?:\\n(?!hr|heading|lheading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)".to_string(), "")
            .replace("hr", gfm_block.hr.as_str())
            .replace("|lheading", "")
            .replace("heading", " {0,3}#{1,6} ")
            .replace("table", gfm_block.table.as_str())
            .replace("blockquote", " {0,3}>")
            .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
            .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
            .replace("html", "<\\/?(?:tag)(?: +|\\n|\\/?>)|<(?:script|pre|style|textarea|!--)")
            .replace("tag", gfm_block.tag.as_str())
            .get_regex_str().as_str()
    );


    /* Pedantic grammar (original John Gruber's loose markdown specification) */
    let mut pedantic_block = normal_block.clone();

    pedantic_block.set_grammar_regex(
        MDBlock::Html,
        Edit::new(
            r#"^ *(?:comment *(?:\n|\s*$)|<(tag)[\s\S]+?<\/\1> *(?:\n{2,}|\s*$)|<tag(?:"[^"]*"|'[^']*'|\s[^'"/>\s]*)*?\/?> *(?:\n{2,}|\s*$))"#.to_string(),  "")
            .replace("comment", normal_block.comment.as_str())
            .replace_all("tag", "(?!(?:a|em|strong|small|s|cite|q|dfn|abbr|data|time|code|var|samp|kbd|sub|sup|i|b|u|mark|ruby|rt|rp|bdi|bdo|span|br|wbr|ins|del|img)\\b)\\w+(?!:|[^\\w\\s@]*@)\\b")
            .get_regex_str().as_str()
    );

    pedantic_block.set_grammar_regex(
        MDBlock::Def,
        r#"^ *\[([^\]]+)\]: *<?([^\s>]+)>?(?: +(["(][^\n]+[")]))? *(?:\n+|$)"#
    );

    pedantic_block.set_grammar_regex(
        MDBlock::Heading,
        "^(#{1,6})(.*)(?:\\n+|$)"
    );

    pedantic_block.set_grammar_regex(
        MDBlock::Fences,
        ""
    );

    pedantic_block.set_grammar_regex(
        MDBlock::Paragraph,
        Edit::new("^([^\\n]+(?:\\n(?!hr|heading|lheading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)".to_string(), "")
            .replace("hr", pedantic_block.hr.as_str())
            .replace("lheading", pedantic_block.l_heading.as_str())
            .replace("heading", " *#{1,6} *[^\\n]")
            .replace("blockquote", " {0,3}>")
            .replace("|fences", "")
            .replace("|list", "")
            .replace("|html", "")
            .get_regex_str().as_str()
    );

    let mut blocks = vec![normal_block, gfm_block, pedantic_block];

    return blocks;

}

pub fn setup_inline_rules() -> Vec<Inline> {
    /* Inline-Level Grammar */
    let mut normal_inline = Inline {
        escape: r##"^\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{|}~])"##.to_string(),
        autolink: "^<(scheme:[^\\s\\x00-\\x1f<>]*|email)>".to_string(),
        url: "".to_string(),
        tag: "^comment|^<\\/[a-zA-Z][\\w:-]*\\s*>|^<[a-zA-Z][\\w-]*(?:attribute)*?\\s*\\/?>|^<\\?[\\s\\S]*?\\?>|^<![a-zA-Z]+\\s[\\s\\S]*?>|^<!\\[CDATA\\[[\\s\\S]*?\\]\\]>".to_string(),
        link: "^!?\\[(label)\\]\\(\\s*(href)(?:\\s+(title))?\\s*\\)".to_string(),
        ref_link: "^!?\\[(label)\\]\\[(ref)\\]".to_string(),
        no_link: "^!?\\[(ref)\\](?:\\[\\])?".to_string(),
        ref_link_search: "reflink|nolink(?!\\()".to_string(),
        em_strong: Delim {
            l_delim: "^(?:\\*+(?:([punct_])|[^\\s*]))|^_+(?:([punct*])|([^\\s_]))".to_string(),
            r_delim_ast: "^[^_*]*?\\_\\_[^_*]*?\\*[^_*]*?(?=\\_\\_)|[punct_](\\*+)(?=[\\s]|$)|[^punct*_\\s](\\*+)(?=[punct_\\s]|$)|[punct_\\s](\\*+)(?=[^punct*_\\s])|[\\s](\\*+)(?=[punct_])|[punct_](\\*+)(?=[punct_])|[^punct*_\\s](\\*+)(?=[^punct*_\\s])".to_string(),
            r_delim_und: "^[^_*]*?\\*\\*[^_*]*?\\_[^_*]*?(?=\\*\\*)|[punct*](\\_+)(?=[\\s]|$)|[^punct*_\\s](\\_+)(?=[punct*\\s]|$)|[punct*\\s](\\_+)(?=[^punct*_\\s])|[\\s](\\_+)(?=[punct*])|[punct*](\\_+)(?=[punct*])".to_string()
        },
        code: "^(`+)([^`]|[^`][\\s\\S]*?[^`])\\1(?!`)".to_string(),
        br: "^( {2,}|\\\\)\\n(?!\\s*$)".to_string(),
        del: "".to_string(),
        text: "^(`+|[^`])(?:(?= {2,}\\n)|[\\s\\S]*?(?:(?=[\\\\<!\\[`*_]|\\b_|$)|[^ ](?= {2,}\\n)))".to_string(),
        punctuation: "^([\\spunctuation])".to_string(),
        _punctuation: r##"!"#$%&'()+\-.,/:;<=>?@\[\]`^{|}~"##.to_string(),
        block_skip: "\\[[^\\]]*?\\]\\([^\\)]*?\\)|`[^`]*?`|<[^>]*?>".to_string(),
        escaped_em_st: "\\\\\\*|\\\\_".to_string(),
        comment: "<!--(?!-?>)[\\s\\S]*?(?:-->|$)".to_string(),
        escapes: r##"\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{|}~])"##.to_string(),
        scheme: "[a-zA-Z][a-zA-Z0-9+.-]{1,31}".to_string(),
        email: "[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+(@)[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(?![-_])".to_string(),
        attribute: r#"\s+[a-zA-Z:_][\w.:-]*(?:\s*=\s*"[^"]*"|\s*=\s*'[^']*'|\s*=\s*[^\s"'=<>`]+)?"#.to_string(),
        label: r#"(?:\[(?:\\.|[^\[\]\\])*\]|\\.|`[^`]*`|[^\[\]\\`])*?"#.to_string(),
        href: "<(?:\\\\.|[^\\n<>\\\\])+>|[^\\s\\x00-\\x1f]*".to_string(),
        title: r#""(?:\\"?|[^"\\])*"|'(?:\\'?|[^'\\])*'|\((?:\\\)?|[^)\\])*\)"#.to_string(),
        breaks: "".to_string(),
        strong: Bold {
            start: "".to_string(),
            middle: "".to_string(),
            end_ast: "".to_string(),
            end_und: "".to_string()
        },
        em: Bold {
            start: "".to_string(),
            middle: "".to_string(),
            end_ast: "".to_string(),
            end_und: "".to_string()
        },
        extended_email: "".to_string(),
        backpedal: "".to_string(),
    };

    normal_inline.set_grammar_regex(
        MDInline::Punctuation,
        Edit::new(normal_inline.punctuation.to_string(), "")
            .replace_all("punctuation", normal_inline._punctuation.as_str())
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::Comment,
        Edit::new(normal_inline.comment.to_string(), "")
            .replace("(?:-->|$)", "-->")
            .get_regex_str().as_str(),
        None

    );

    normal_inline.set_grammar_regex(
        MDInline::EmStrong,
        Edit::new(normal_inline.em_strong.l_delim.to_string(), "")
            .replace_all("punct", normal_inline._punctuation.as_str())
            .get_regex_str().as_str(),
        Some("l_delim")
    );


    normal_inline.set_grammar_regex(
        MDInline::EmStrong,
        Edit::new(normal_inline.em_strong.r_delim_ast.to_string(), "")
            .replace_all("punct", normal_inline._punctuation.as_str())
            .get_regex_str().as_str(),
        Some("r_delim_ast")
    );

    normal_inline.set_grammar_regex(
        MDInline::EmStrong,
        Edit::new(normal_inline.em_strong.r_delim_und.to_string(), "")
            .replace_all("punct", normal_inline._punctuation.as_str())
            .get_regex_str().as_str(),
        Some("r_delim_und")
    );

    normal_inline.set_grammar_regex(
        MDInline::Autolink,
        Edit::new(normal_inline.autolink.to_string(), "")
            .replace("scheme", normal_inline.scheme.as_str())
            .replace("email", normal_inline.email.as_str())
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::Tag,
        Edit::new(normal_inline.tag.to_string(), "")
            .replace("comment", normal_inline.comment.as_str())
            .replace("attribute", normal_inline.attribute.as_str())
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::Link,
        Edit::new(normal_inline.link.to_string(), "")
            .replace("label", normal_inline.label.as_str())
            .replace("href", normal_inline.href.as_str())
            .replace("title", normal_inline.title.as_str())
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::RefLink,
        Edit::new(normal_inline.ref_link.to_string(), "")
            .replace("label", normal_inline.label.as_str())
            .replace("ref", "(?!\\s*\\])(?:\\\\.|[^\\[\\]\\\\])+")
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::NoLink,
        Edit::new(normal_inline.no_link.to_string(), "")
            .replace("ref", "(?!\\s*\\])(?:\\\\.|[^\\[\\]\\\\])+")
            .get_regex_str().as_str(),
        None
    );

    normal_inline.set_grammar_regex(
        MDInline::RefLinkSearch,
        Edit::new(normal_inline.ref_link_search.to_string(), "")
            .replace("reflink", normal_inline.ref_link.as_str())
            .replace("nolink", normal_inline.no_link.as_str())
            .get_regex_str().as_str(),
        None
    );


    /* Pedantic Inline Grammar */
    let mut pedantic_inline = normal_inline.clone();

    pedantic_inline.set_grammar_regex(
        MDInline::Strong,
        Edit::new("^__|\\*\\*".to_string(), "")
            .get_regex_str().as_str(),
        Some("start")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Strong,
        Edit::new("^__(?=\\S)([\\s\\S]*?\\S)__(?!_)|^\\*\\*(?=\\S)([\\s\\S]*?\\S)\\*\\*(?!\\*)".to_string(), "")
            .get_regex_str().as_str(),
        Some("middle")
    );


    pedantic_inline.set_grammar_regex(
        MDInline::Strong,
        Edit::new("\\*\\*(?!\\*)".to_string(), "")
            .get_regex_str().as_str(),
        Some("end_ast")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Strong,
        Edit::new("__(?!_)".to_string(), "")
            .get_regex_str().as_str(),
        Some("end_und")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Em,
        Edit::new("^_|\\*".to_string(), "")
            .get_regex_str().as_str(),
        Some("start")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Em,
        Edit::new("^()\\*(?=\\S)([\\s\\S]*?\\S)\\*(?!\\*)|^_(?=\\S)([\\s\\S]*?\\S)_(?!_)".to_string(), "")
            .get_regex_str().as_str(),
        Some("middle")
    );


    pedantic_inline.set_grammar_regex(
        MDInline::Em,
        Edit::new("\\*(?!\\*)".to_string(), "")
            .get_regex_str().as_str(),
        Some("end_ast")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Em,
        Edit::new("_(?!_)".to_string(), "")
            .get_regex_str().as_str(),
        Some("end_und")
    );

    pedantic_inline.set_grammar_regex(
        MDInline::Link,
        Edit::new("^!?\\[(label)\\]\\((.*?)\\)".to_string(), "")
            .replace("label", pedantic_inline.label.as_str())
            .get_regex_str().as_str(),
        None
    );

    pedantic_inline.set_grammar_regex(
        MDInline::RefLink,
        Edit::new("^!?\\[(label)\\]\\s*\\[([^\\]]*)\\]".to_string(), "")
            .replace("label", pedantic_inline.label.as_str())
            .get_regex_str().as_str(),
        None
    );

    /**
     * GFM Inline Grammar
     */
    let mut gfm_inline = normal_inline.clone();


    gfm_inline.set_grammar_regex(
        MDInline::Escape,
        Edit::new(gfm_inline.escape.to_string(), "")
            .replace("])", "~|])")
            .get_regex_str().as_str(),
        None
    );

    gfm_inline.set_grammar_regex(
        MDInline::ExtendedEmail,
        "[A-Za-z0-9._+-]+(@)[a-zA-Z0-9-_]+(?:\\.[a-zA-Z0-9-_]*[a-zA-Z0-9])+(?![-_])",
        None
    );

    gfm_inline.set_grammar_regex(
        MDInline::Url,
        "^((?:ftp|https?):\\/\\/|www\\.)(?:[a-zA-Z0-9\\-]+\\.?)+[^\\s<]*|^email",
        None
    );

    gfm_inline.set_grammar_regex(
        MDInline::Backpedal,
        "(?:[^?!.,:;*_~()&]+|\\([^)]*\\)|&(?![a-zA-Z0-9]+;$)|[?!.,:;*_~)]+(?!$))+",
        None
    );

    gfm_inline.set_grammar_regex(
        MDInline::Del,
        "^(~~?)(?=[^\\s~])([\\s\\S]*?[^\\s~])\\1(?=[^~]|$)",
        None
    );

    gfm_inline.set_grammar_regex(
        MDInline::Text,
        "^([`~]+|[^`~])(?:(?= {2,}\\n)|(?=[a-zA-Z0-9.!#$%&'*+\\/=?_`{\\|}~-]+@)|[\\s\\S]*?(?:(?=[\\\\<!\\[`*~_]|\\b_|https?:\\/\\/|ftp:\\/\\/|www\\.|$)|[^ ](?= {2,}\\n)|[^a-zA-Z0-9.!#$%&'*+\\/=?_`{\\|}~-](?=[a-zA-Z0-9.!#$%&'*+\\/=?_`{\\|}~-]+@)))",
    None
    );

    gfm_inline.set_grammar_regex(
        MDInline::Url,
        Edit::new(gfm_inline.url.to_string(), "i")
            .replace("email", gfm_inline.extended_email.as_str())
            .get_regex_str().as_str(),
        None
    );

    let mut gfm_with_breaks_inline = gfm_inline.clone();

    gfm_with_breaks_inline.set_grammar_regex(
        MDInline::Br,
        Edit::new(normal_inline.br.to_string(), "")
            .replace("{2,}", "*")
            .get_regex_str().as_str(),
        None
    );

    gfm_with_breaks_inline.set_grammar_regex(
        MDInline::Text,
        Edit::new(gfm_inline.text.to_string(), "")
            .replace("\\b_", "\\b_| {2,}\\n")
            .replace_all("{2,}", "*")
            .get_regex_str().as_str(),
        None
    );

    let inlines = vec![normal_inline, pedantic_inline, gfm_inline, gfm_with_breaks_inline];
    inlines
}

pub fn get_default_rules() -> Rules {
    let blocks = setup_block_rules();
    let block = blocks.get(0).unwrap();
    let inlines = setup_inline_rules();
    let inline = inlines.get(0).unwrap();
    Rules {
        block: Block {
            newline: block.newline.to_string(),
            code: block.code.to_string(),
            fences: block.fences.to_string(),
            hr: block.hr.to_string(),
            heading: block.heading.to_string(),
            blockquote: block.blockquote.to_string(),
            list: block.list.to_string(),
            html: block.html.to_string(),
            def: block.def.to_string(),
            table: block.table.to_string(),
            l_heading: block.l_heading.to_string(),
            paragraph: block.paragraph.to_string(),
            text: block.paragraph.to_string(),
            label: block.label.to_string(),
            title: block.title.to_string(),
            bullet: block.bullet.to_string(),
            list_item_start: block.list_item_start.to_string(),
            tag: block.tag.to_string(),
            comment: block.comment.to_string()
        },
        inline: Inline {
            escape: inline.escape.to_string(),
            autolink: inline.autolink.to_string(),
            url: inline.url.to_string(),
            tag: inline.tag.to_string(),
            link: inline.link.to_string(),
            ref_link: inline.ref_link.to_string(),
            no_link: inline.no_link.to_string(),
            ref_link_search: inline.ref_link_search.to_string(),
            em_strong: Delim {
                l_delim: inline.em_strong.l_delim.to_string(),
                r_delim_ast: inline.em_strong.r_delim_ast.to_string(),
                r_delim_und: inline.em_strong.r_delim_und.to_string()
            },
            code: inline.code.to_string(),
            br: inline.br.to_string(),
            del: inline.del.to_string(),
            text: inline.text.to_string(),
            punctuation: inline.punctuation.to_string(),
            _punctuation: inline._punctuation.to_string(),
            block_skip: inline.block_skip.to_string(),
            escaped_em_st: inline.escaped_em_st.to_string(),
            comment: inline.comment.to_string(),
            escapes: inline.escapes.to_string(),
            scheme: inline.scheme.to_string(),
            email: inline.email.to_string(),
            attribute: inline.attribute.to_string(),
            label: inline.label.to_string(),
            href: inline.href.to_string(),
            title: inline.href.to_string(),
            breaks: inline.breaks.to_string(),
            strong: Bold {
                start: inline.strong.start.to_string(),
                middle: inline.strong.middle.to_string(),
                end_ast: inline.strong.end_ast.to_string(),
                end_und: inline.strong.end_und.to_string()
            },
            em: Bold {
                start: inline.em.start.to_string(),
                middle: inline.em.middle.to_string(),
                end_ast: inline.em.end_ast.to_string(),
                end_und: inline.em.end_und.to_string()
            },
            extended_email: inline.extended_email.to_string(),
            backpedal: inline.backpedal.to_string()
        }
    }
}

pub fn get_rules(options: Options) -> Rules {

    let blocks = setup_block_rules();
    let inlines = setup_inline_rules();
    let mut block = blocks.get(0).unwrap();
    let mut inline = inlines.get(0).unwrap();

    // Block: [normal_block, gfm_block, pedantic_block]
    // Inline: [normal_inline, pedantic_inline, gfm_inline, gfm_with_breaks_inline]
    if options.pedantic {
        block = blocks.get(2).unwrap();
        inline = inlines.get(1).unwrap();
    } else if options.gfm {
        block = blocks.get(1).unwrap();
        if options.breaks {
            inline = inlines.get(3).unwrap();
        } else {
            inline = inlines.get(2).unwrap();
        }
    }


    Rules {
        block: Block {
            newline: block.newline.to_string(),
            code: block.code.to_string(),
            fences: block.fences.to_string(),
            hr: block.hr.to_string(),
            heading: block.heading.to_string(),
            blockquote: block.blockquote.to_string(),
            list: block.list.to_string(),
            html: block.html.to_string(),
            def: block.def.to_string(),
            table: block.table.to_string(),
            l_heading: block.l_heading.to_string(),
            paragraph: block.paragraph.to_string(),
            text: block.paragraph.to_string(),
            label: block.label.to_string(),
            title: block.title.to_string(),
            bullet: block.bullet.to_string(),
            list_item_start: block.list_item_start.to_string(),
            tag: block.tag.to_string(),
            comment: block.comment.to_string()
        },
        inline: Inline {
            escape: inline.escape.to_string(),
            autolink: inline.autolink.to_string(),
            url: inline.url.to_string(),
            tag: inline.tag.to_string(),
            link: inline.link.to_string(),
            ref_link: inline.ref_link.to_string(),
            no_link: inline.no_link.to_string(),
            ref_link_search: inline.ref_link_search.to_string(),
            em_strong: Delim {
                l_delim: inline.em_strong.l_delim.to_string(),
                r_delim_ast: inline.em_strong.r_delim_ast.to_string(),
                r_delim_und: inline.em_strong.r_delim_und.to_string()
            },
            code: inline.code.to_string(),
            br: inline.br.to_string(),
            del: inline.del.to_string(),
            text: inline.text.to_string(),
            punctuation: inline.punctuation.to_string(),
            _punctuation: inline._punctuation.to_string(),
            block_skip: inline.block_skip.to_string(),
            escaped_em_st: inline.escaped_em_st.to_string(),
            comment: inline.comment.to_string(),
            escapes: inline.escapes.to_string(),
            scheme: inline.scheme.to_string(),
            email: inline.email.to_string(),
            attribute: inline.attribute.to_string(),
            label: inline.label.to_string(),
            href: inline.href.to_string(),
            title: inline.href.to_string(),
            breaks: inline.breaks.to_string(),
            strong: Bold {
                start: inline.strong.start.to_string(),
                middle: inline.strong.middle.to_string(),
                end_ast: inline.strong.end_ast.to_string(),
                end_und: inline.strong.end_und.to_string()
            },
            em: Bold {
                start: inline.em.start.to_string(),
                middle: inline.em.middle.to_string(),
                end_ast: inline.em.end_ast.to_string(),
                end_und: inline.em.end_und.to_string()
            },
            extended_email: inline.extended_email.to_string(),
            backpedal: inline.backpedal.to_string()
        }
    }
}

pub fn test() {
    // let info_str = "html=====   ";
    // let lang_caps = regx(r#"\S*"#).captures(info_str).unwrap();
    // let lang = lang_caps.get(0).map_or("", |m| m.as_str());
    // println!("{:?}", lang);
    // let inline = &setup_inline_rules()[3];
    // fs::write("helpers.txt", inline.text.as_str()).expect("Unable to write file");
}
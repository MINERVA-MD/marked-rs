use std::fs;
use onig::*;
use std::borrow::Borrow;
use serde_json::to_string;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};

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
    BlockSkip,
    EscapedEmSt,
    Comment,
    Escapes,
    Scheme,
    Email,
    Attribute,
    Label
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

pub struct Delim {
    pub l_delim: String,
    pub r_delim_ast: String,
    pub r_delim_und: String
}

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
    pub block_skip: String,
    pub escaped_em_st: String,
    pub comment: String,
    pub escapes: String,
    pub scheme: String,
    pub email: String,
    pub attribute: String,
    pub label: String
}

pub struct Bold {
    start: String,
    middle: String,
    end_ast: String,
    end_und: String
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

    pub fn get_grammar_regex(&self, rule: MDBlock) -> Regex {
        match rule {
            MDBlock::Newline        => Regex::new(self.newline.as_str()).unwrap(),
            MDBlock::Code           => Regex::new(self.code.as_str()).unwrap(),
            MDBlock::Fences         => Regex::new(self.fences.as_str()).unwrap(),
            MDBlock::Hr             => Regex::new(self.hr.as_str()).unwrap(),
            MDBlock::Heading        => Regex::new(self.heading.as_str()).unwrap(),
            MDBlock::Blockquote     => Regex::new(self.blockquote.as_str()).unwrap(),
            MDBlock::List           => Regex::new(self.list.as_str()).unwrap(),
            MDBlock::Html           => Regex::new(self.html.as_str()).unwrap(),
            MDBlock::Def            => Regex::new(self.def.as_str()).unwrap(),
            MDBlock::Table          => Regex::new(self.table.as_str()).unwrap(),
            MDBlock::LHeading       => Regex::new(self.l_heading.as_str()).unwrap(),
            MDBlock::Paragraph      => Regex::new(self.paragraph.as_str()).unwrap(),
            MDBlock::Text           => Regex::new(self.text.as_str()).unwrap(),
            MDBlock::Label          => Regex::new(self.label.as_str()).unwrap(),
            MDBlock::Title          => Regex::new(self.title.as_str()).unwrap(),
            MDBlock::Bullet         => Regex::new(self.bullet.as_str()).unwrap(),
            MDBlock::ListItemStart  => Regex::new(self.list_item_start.as_str()).unwrap(),
            MDBlock::Tag            => Regex::new(self.tag.as_str()).unwrap(),
            MDBlock::Comment        => Regex::new(self.comment.as_str()).unwrap(),
        }
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


pub fn setup() ->  Vec<&Block> {
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

    // let mut gfm_block = normal_block.clone();
    // let mut gfm_block = Block {
    //     newline: normal_block.newline.to_string(),
    //     code: normal_block.code.to_string(),
    //     fences: normal_block.fences.to_string(),
    //     hr: normal_block.hr.to_string(),
    //     heading: normal_block.heading.to_string(),
    //     blockquote: normal_block.blockquote.to_string(),
    //     list: normal_block.list.to_string(),
    //     html: normal_block.html.to_string(),
    //     def: normal_block.def.to_string(),
    //     table: normal_block.table.to_string(),
    //     l_heading: normal_block.l_heading.to_string(),
    //     paragraph: "".to_string(),
    //     text: normal_block.text.to_string(),
    //     label: normal_block.label.to_string(),
    //     title: normal_block.title.to_string(),
    //     bullet: normal_block.bullet.to_string(),
    //     list_item_start: normal_block.list_item_start.to_string(),
    //     tag: normal_block.tag.to_string(),
    //     comment: normal_block.comment.to_string()
    // };

    let mut block2 = Block {
        newline: "".to_string(),
        code: "".to_string(),
        fences: "".to_string(),
        hr: "".to_string(),
        heading: "".to_string(),
        blockquote: "".to_string(),
        list: "".to_string(),
        html: "".to_string(),
        def: "".to_string(),
        table: "".to_string(),
        l_heading: "".to_string(),
        paragraph: "".to_string(),
        text: "".to_string(),
        label: "".to_string(),
        title: "".to_string(),
        bullet: "".to_string(),
        list_item_start: "".to_string(),
        tag: "".to_string(),
        comment: "".to_string()
    };


    // gfm_block.set_grammar_regex(MDBlock::Table,
    //                             "^ *([^\\n ].*\\|.*)\\n {0,3}(?:\\| *)?(:?-+:? *(?:\\| *:?-+:? *)*)(?:\\| *)?(?:\\n((?:(?! *\\n|hr|heading|blockquote|code|fences|list|html).*(?:\\n|$))*)\\n*|$)"
    // );
    //
    // gfm_block.set_grammar_regex(
    //     MDBlock::Table,
    //     Edit::new(gfm_block.table.to_string(), "")
    //         .replace("hr", gfm_block.hr.as_str())
    //         .replace("heading", " {0,3}#{1,6} ")
    //         .replace("blockquote", " {0,3}>")
    //         .replace("code", " {4}[^\\n]")
    //         .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
    //         .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
    //         .replace("html", "<\\/?(?:tag)(?: +|\\n|\\/?>)|<(?:script|pre|style|textarea|!--)")
    //         .replace("tag", gfm_block.tag.as_str())
    //         .get_regex_str().as_str()
    // );
    //
    // gfm_block.paragraph = "".to_string();

    // println!("Paragraph: {}.......................................\n\n\n", gfm_block.paragraph.to_string());

    // gfm_block.set_grammar_regex(
    //     MDBlock::Paragraph,
    //     Edit::new(gfm_block.paragraph.to_string(), "")
    //         .replace("hr", normal_block.hr.as_str())
    //         .replace("|lheading", "")
    //         .replace("heading", " {0,3}#{1,6} ")
    //         .replace("table", gfm_block.table.as_str())
    //         .replace("blockquote", " {0,3}>")
    //         .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
    //         .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
    //         .replace("html", "<\\/?(?:tag)(?: +|\\n|/?>)|<(?:script|pre|style|textarea|!--)")
    //         .replace("tag", gfm_block.tag.as_str())
    //         .get_regex_str().as_str()
    // );



    // let pedantic_block = normal_block.clone();



    return vec![&normal_block, &gfm_block];

}

pub fn test() {

    let block = &setup()[1];
    fs::write("helpers.txt", block.paragraph.as_str()).expect("Unable to write file");
}
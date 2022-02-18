use std::fs;
use onig::*;
use lazy_static::lazy_static;

lazy_static! {
    static ref CARET: Regex = Regex::new("(^|[^\\[])\\^").unwrap();
}

pub struct Block {
    newline: String,
    code: String,
    fences: String,
    hr: String,
    heading: String,
    blockquote: String,
    list: String,
    html: String,
    def: String,
    table: String,
    l_heading: String,
    _paragraph: String,
    text: String,
    _label: String,
    _title: String,
    bullet: String,
    list_item_start: String,
    _tag: String,
    _comment: String
}

pub struct Delim {
    l_delim: String,
    r_delim_ast: String,
    r_delim_und: String
}

pub struct Inline {
    escape: String,
    autolink: String,
    url: String,
    tag: String,
    link: String,
    ref_link: String,
    no_link: String,
    ref_link_search: String,
    em_strong: Delim,
    code: String,
    br: String,
    del: String,
    text: String,
    punctuation: String,
    block_skip: String,
    escaped_em_st: String,
    _comment: String,
    _escapes: String,
    scheme: String,
    _email: String,
    _attribute: String,
    _label: String
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

impl Edit {
    pub fn new(regex_str: &str, opt: &str) -> Self {
        Self {
            regex_str: regex_str.to_string(),
            opt: opt.to_string()
        }
    }

    pub fn replace(mut self, name: &str, val: &str) -> Self {
        let _val = CARET.replace_all(val, "");
        self.regex_str = self.regex_str.replace(name, _val.as_str());
        self
    }

    pub fn get_regex(&mut self) -> Regex {
        return Regex::new(self.regex_str.as_str()).unwrap();
    }
}

pub fn test() {
    let paragraph = "^([^\\n]+(?:\\n(?!hr|lheading|heading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)";
    let tag = "address|article|aside|base|basefont|blockquote|body|caption|center|col|colgroup|dd|details|dialog|dir|div|dl|dt|fieldset|figcaption|figure|footer|form|frame|frameset|h[1-6]|head|header|hr|html|iframe|legend|li|link|main|menu|menuitem|meta|nav|noframes|ol|optgroup|option|p|param|section|source|summary|table|tbody|td|tfoot|th|thead|title|tr|track|ul";
    let hr = "^ {0,3}((?:- *){3,}|(?:_ *){3,}|(?:\\* *){3,})(?:\\n+|$)";
    let mut edit = Edit::new(paragraph,  "")
        .replace("hr", hr)
        .replace("|lheading", "")
        .replace("heading", " {0,3}#{1,6} ")
        .replace("|table", "")
        .replace("blockquote", " {0,3}>")
        .replace("fences", " {0,3}(?:`{3,}(?=[^`\\n]*\\n)|~{3,})[^\\n]*\\n")
        .replace("list", " {0,3}(?:[*+-]|1[.)]) ")
        .replace("html", "<\\/?(?:tag)(?: +|\\n|\\/?>)|<(?:script|pre|style|textarea|!--)")
        .replace("tag", tag);

    println!("{}", edit.regex_str);

    fs::write("helpers.txt", edit.regex_str).expect("Unable to write file");

}
use std::char;

use crate::parser::md4rs::md_parse;
use crate::parser::entity::Entity;

static NEED_HTML_ESC_FLAG: u32                  = 0x1;
static NEED_URL_ESC_FLAG: u32                   = 0x2;

static MD_HTML_FLAG_DEBUG: u32                  = 0x0001;
static MD_HTML_FLAG_VERBATIM_ENTITIES: u32      = 0x0002;
static MD_HTML_FLAG_SKIP_UTF8_BOM: u32          = 0x0004;
static MD_HTML_FLAG_XHTML: u32                  = 0x0008;

pub struct MDHTMLTag {
    pub process_output: fn(text: &str, userdata: fn()),
    pub flags: u32,
    pub userdata: fn(),
    pub escape_map: [char; 256],
    pub image_nesting_level: i32
}

pub struct MDParser {
    pub abi_version: u32,
    pub flags: u32,
    pub enter_block: fn(_type: MD_BLOCKTYPE, detail: MDBlock, r: &MDHTMLTag) -> u32,
    pub leave_block: fn(_type: MD_BLOCKTYPE, detail: MDBlock, r: &MDHTMLTag) -> u32,
    pub enter_span: fn(_type: MD_SPANTYPE, detail: MDSpan, r: &mut MDHTMLTag) -> u32,
    pub leave_span: fn(_type: MD_SPANTYPE, detail: MDSpan, r: &mut MDHTMLTag) -> u32,
    pub text: fn(_type: &MD_TEXTTYPE, text: &str, r: &mut MDHTMLTag)-> u32,
    pub debug_log: fn(msg: &str, r: MDHTMLTag),
    pub syntax: u32
}

pub static MD_TEXTTYPE2: [&'static str; 8] = [
    "MD_TEXT_NORMAL",
    "MD_TEXT_NULLCHAR",
    "MD_TEXT_BR",
    "MD_TEXT_SOFTBR",
    "MD_TEXT_ENTITY",
    "MD_TEXT_CODE",
    "MD_TEXT_HTML",
    "MD_TEXT_LATEXMATH",
];

#[derive(PartialEq)]
pub enum MD_ALIGN {
    MD_ALIGN_DEFAULT,  /* 0 When unspecified. */
    MD_ALIGN_LEFT,
    MD_ALIGN_CENTER,
    MD_ALIGN_RIGHT
}

#[derive(PartialEq)]
pub enum MD_BLOCKTYPE {
    /* <body>...</body> */
    MD_BLOCK_DOC = 0,

    /* <blockquote>...</blockquote> */
    MD_BLOCK_QUOTE,

    /* <ul>...</ul>
     * Detail: Structure MD_BLOCK_UL_DETAIL. */
    MD_BLOCK_UL,

    /* <ol>...</ol>
     * Detail: Structure MD_BLOCK_OL_DETAIL. */
    MD_BLOCK_OL,

    /* <li>...</li>
     * Detail: Structure MD_BLOCK_LI_DETAIL. */
    MD_BLOCK_LI,

    /* <hr> */
    MD_BLOCK_HR,

    /* <h1>...</h1> (for levels up to 6)
     * Detail: Structure MdBlockHDetail. */
    MD_BLOCK_H,

    /* <pre><code>...</code></pre>
     * Note the text lines within code blocks are terminated with '\n'
     * instead of explicit MD_TEXT_BR. */
    MD_BLOCK_CODE,

    /* Raw HTML block. This itself does not correspond to any particular HTML
     * tag. The contents of it _is_ raw HTML source intended to be put
     * in verbatim form to the HTML output. */
    MD_BLOCK_HTML,

    /* <p>...</p> */
    MD_BLOCK_P,

    /* <table>...</table> and its contents.
     * Detail: Structure MD_BLOCK_TABLE_DETAIL (for MD_BLOCK_TABLE),
     *         structure MD_BLOCK_TD_DETAIL (for MD_BLOCK_TH and MD_BLOCK_TD)
     * Note all of these are used only if extension MD_FLAG_TABLES is enabled. */
    MD_BLOCK_TABLE,
    MD_BLOCK_THEAD,
    MD_BLOCK_TBODY,
    MD_BLOCK_TR,
    MD_BLOCK_TH,
    MD_BLOCK_TD
}

#[derive(PartialEq)]
pub enum MD_SPANTYPE {
    /* <em>...</em> */
    MD_SPAN_EM,

    /* <strong>...</strong> */
    MD_SPAN_STRONG,

    /* <a href="xxx">...</a>
     * Detail: Structure MD_SPAN_A_DETAIL. */
    MD_SPAN_A,

    /* <img src="xxx">...</a>
     * Detail: Structure MD_SPAN_IMG_DETAIL.
     * Note: Image text can contain nested spans and even nested images.
     * If rendered into ALT attribute of HTML <IMG> tag, it's responsibility
     * of the parser to deal with it.
     */
    MD_SPAN_IMG,

    /* <code>...</code> */
    MD_SPAN_CODE,

    /* <del>...</del>
     * Note: Recognized only when MD_FLAG_STRIKETHROUGH is enabled.
     */
    MD_SPAN_DEL,

    /* For recognizing inline ($) and display ($$) equations
     * Note: Recognized only when MD_FLAG_LATEXMATHSPANS is enabled.
     */
    MD_SPAN_LATEXMATH,
    MD_SPAN_LATEXMATH_DISPLAY,

    /* Wiki links
     * Note: Recognized only when MD_FLAG_WIKILINKS is enabled.
     */
    MD_SPAN_WIKILINK,

    /* <u>...</u>
     * Note: Recognized only when MD_FLAG_UNDERLINE is enabled. */
    MD_SPAN_U
}

#[derive(PartialEq)]
pub enum MD_TEXTTYPE {
    /* Normal text. */
    MD_TEXT_NORMAL = 0,

    /* NULL character. CommonMark requires replacing NULL character with
     * the replacement char U+FFFD, so this allows caller to do that easily. */
    MD_TEXT_NULLCHAR,

    /* Line breaks.
     * Note these are not sent from blocks with verbatim output (MD_BLOCK_CODE
     * or MD_BLOCK_HTML). In such cases, '\n' is part of the text itself. */
    MD_TEXT_BR,         /* <br> (hard break) */
    MD_TEXT_SOFTBR,     /* '\n' in source text where it is not semantically meaningful (soft break) */

    /* Entity.
     * (a) Named entity, e.g. &nbsp;
     *     (Note MD4C does not have a list of known entities.
     *     Anything matching the regexp /&[A-Za-z][A-Za-z0-9]{1,47};/ is
     *     treated as a named entity.)
     * (b) Numerical entity, e.g. &#1234;
     * (c) Hexadecimal entity, e.g. &#x12AB;
     *
     * As MD4C is mostly encoding agnostic, application gets the verbatim
     * entity text into the MD_PARSER::text_callback(). */
    MD_TEXT_ENTITY,

    /* Text in a code block (inside MD_BLOCK_CODE) or inlined code (`code`).
     * If it is inside MD_BLOCK_CODE, it includes spaces for indentation and
     * '\n' for new lines. MD_TEXT_BR and MD_TEXT_SOFTBR are not sent for this
     * kind of text. */
    MD_TEXT_CODE,

    /* Text is a raw HTML. If it is contents of a raw HTML block (i.e. not
     * an inline raw HTML), then MD_TEXT_BR and MD_TEXT_SOFTBR are not used.
     * The text contains verbatim '\n' for the new lines. */
    MD_TEXT_HTML,

    /* Text is inside an equation. This is processed the same way as inlined code
     * spans (`code`). */
    MD_TEXT_LATEXMATH
}

pub struct MDAttribute {
    text: &'static str,
    size: u32,
    substr_offsets: Vec<u32>,
}

pub struct MDBlockULDetail {
    is_tight: u32,              /* Non-zero if tight list, zero if loose. */
    mark: char                  /* Item bullet character in MarkDown source of the list, e.g. '-', '+', '*'. */
}

pub struct MDBlockOLDetail {
    pub start: u32,              /* Start index of the ordered list. */
    pub is_tight: u32,           /* Non-zero if tight list, zero if loose. */
    pub mark_delimiter: char     /* Character delimiting the item marks in MarkDown source, e.g. '.' or ')' */
}

pub struct MDBlockLIDetail {
    is_task: u32,                /* Can be non-zero only with MD_FLAG_TASKLISTS */
    task_mark: char,             /* If is_task, then one of 'x', 'X' or ' '. Undefined otherwise. */
    task_mark_offset: u32        /* If is_task, then offset in the input of the char between '[' and ']'. */
}

pub struct MDBlockCodeDetail {
    info: MDAttribute,
    lang: MDAttribute,
    fence_char: char     /* The character used for fenced code block; or zero for indented code block. */
}

pub struct MDBlockTDDetail {
    align: MD_ALIGN
}

pub struct MDSpanADetail {
    href: MDAttribute,
    title: MDAttribute
}

pub struct MDSpanImgDetail {
    src: MDAttribute,
    title: MDAttribute
}

pub struct MDSpanWikiDetail {
    target: MDAttribute
}

pub struct MDBlockHDetail {
    level: u32         /* Header level (1 - 6) */
}

pub struct MDBlock {
    pub h: MDBlockHDetail,
    pub ol: MDBlockOLDetail,
    pub li: MDBlockLIDetail,
    pub code: MDBlockCodeDetail,
    pub th: MDBlockTDDetail,
    pub td: MDBlockTDDetail
}

pub struct MDSpan {
    pub a: MDSpanADetail,
    pub img: MDSpanImgDetail,
    pub wiki: MDSpanWikiDetail
}

    /*****************************************
     ***  HTML rendering helper functions  ***
     *****************************************/
pub fn render_verbatim(r: &MDHTMLTag, text: &str) {
    println!("render_verbatim: {}", text);
    (r.process_output)(text, r.userdata);
}

pub fn get_char_as_u32(ch: &char) -> u32 {
    return *ch as u32;
}

pub fn get_char_as_usize(ch: &char) -> usize {
    return get_char_as_u32(ch) as usize;
}


pub fn get_escaped_char_as_u32(r: &MDHTMLTag, ch: &char)-> u32 {
    let idx: usize = get_char_as_usize(ch);
    let _char: u32 = r.escape_map[idx] as u32;
    return  _char;
}

pub fn get_replaced_hex_char(ch: &char, shift: u32) -> usize {
    return (((get_char_as_u32(ch)) >> shift) & 0xf) as usize;
}

pub fn need_html_esc(r: &MDHTMLTag, ch: &char) -> bool {
    // println!("need_html_esc: {}", ch);
    let result: u32 = get_escaped_char_as_u32(r, ch) & NEED_HTML_ESC_FLAG;
    return if result == 0 { false } else { true };
}

pub fn need_url_esc(r: &MDHTMLTag, ch: &char) -> bool {
    // println!("need_url_esc: {}", ch);
    let result: u32 = get_escaped_char_as_u32(r, ch) & NEED_URL_ESC_FLAG;
    return if result == 0 { false } else { true };
}

pub fn render_html_escaped(r: &MDHTMLTag, text: &str){
    let mut beg: u32 = 0;
    let mut off: u32 = 0;
    let size: u32 = text.chars().count() as u32;

    loop {
        while (off + 3) < size &&
            !need_html_esc(r, &text.chars().nth((off + 0) as usize).unwrap()) &&
            !need_html_esc(r, &text.chars().nth((off + 1) as usize).unwrap()) &&
            !need_html_esc(r, &text.chars().nth((off + 2) as usize).unwrap()) &&
            !need_html_esc(r, &text.chars().nth((off + 3) as usize).unwrap())
        {
            off += 4;
        }
        while off < size &&
            !need_html_esc(r, &text.chars().nth(off as usize).unwrap())
        {
            off += 1;
        }

        if off > beg {
            render_verbatim(r, &text[(beg as usize)..])
        }

        if off < size {
            (|| {
                match &text.chars().nth((off) as usize).unwrap() {
                    '&' => {
                        render_verbatim(r, "&amp;");
                        return;
                    }
                    '<' => {
                        render_verbatim(r, "&lt;");
                        return;
                    }
                    '>' => {
                        render_verbatim(r, "&gt;");
                        return;
                    }
                    '"' => {
                        render_verbatim(r, "&quot;");
                        return;
                    }
                    _ => {
                        return;
                    }
                }
            })();
            off += 1;
        } else {
            break;
        }
        beg = off;
    }
}



pub fn render_url_escaped(r: &MDHTMLTag, text: &str) {

    let mut beg: u32 = 0;
    let mut off: u32 = 0;
    const HEX_CHARS: [&str; 16] = [
        "0", "1", "2", "3",
        "4", "5", "6", "7",
        "8", "9", "A","B",
        "C", "D", "E", "F"
    ];
    let size: u32 = text.chars().count() as u32;

    loop {
        while off < size &&
            !need_url_esc(r, &text.chars().nth((off) as usize).unwrap())
        {
            off += 1;
        }

        if off > beg {
            render_verbatim(r, &text[(beg as usize)..])
        }

        if off < size {
            let mut hex: String = String::from("xxx");
            (|| {
                match &text.chars().nth((off) as usize).unwrap() {
                    '&' => {
                        render_verbatim(r, "&amp;");
                        return;
                    }
                    _ => {
                        let hex1_idx: usize = get_replaced_hex_char(&text.chars().nth((off) as usize).unwrap(), 4);
                        let hex2_idx: usize = get_replaced_hex_char(&text.chars().nth((off) as usize).unwrap(), 0);

                        hex.replace_range(0..1,"%");
                        hex.replace_range(1..2, HEX_CHARS[hex1_idx]);
                        hex.replace_range(2..3, HEX_CHARS[hex2_idx]);
                        render_verbatim(r, &hex);
                        return;
                    }
                }
            })();
            off += 1;
        } else {
            break;
        }
        beg = off;
    }
}


pub fn render_utf8_codepoint(r: &MDHTMLTag, codepoint: u32, fn_append: fn(r: &MDHTMLTag, text: &str)) {
    let n: usize;
    let mut utf8: Vec<u8> = vec![0, 0, 0, 0];
    let utf8_replacement_char: Vec<u8> = vec![0xef, 0xbf, 0xbd];

    if codepoint <= 0x7f {
        n = 1;
        utf8[0] = codepoint as u8;
    } else if codepoint <= 0x7ff {
        n = 2;
        utf8[0] = (0xc0 | ((codepoint >>  6) & 0x1f)) as u8;
        utf8[1] = (0x80 + ((codepoint >>  0) & 0x3f)) as u8;
    } else if codepoint <= 0xffff {
        n = 3;
        utf8[0] = (0xe0 | ((codepoint >> 12) & 0xf)) as u8;
        utf8[1] = (0x80 + ((codepoint >>  6) & 0x3f)) as u8;
        utf8[2] = (0x80 + ((codepoint >>  0) & 0x3f)) as u8;
    } else {
        n = 4;
        utf8[0] = (0xf0 | ((codepoint >> 18) & 0x7)) as u8;
        utf8[1] = (0x80 + ((codepoint >> 12) & 0x3f)) as u8;
        utf8[2] = (0x80 + ((codepoint >>  6) & 0x3f)) as u8;
        utf8[3] = (0x80 + ((codepoint >>  0) & 0x3f)) as u8;
    }

    if 0 < codepoint  &&  codepoint <= 0x10ffff {
        let text= String::from_utf8(Vec::from(&utf8[0..n])).unwrap();
        fn_append(r, &text);
    } else {
        let text= String::from_utf8(utf8_replacement_char).unwrap();
        fn_append(r, &text);
    }

}

pub fn render_entity(r: &MDHTMLTag, text: &str, fn_append: fn(r: &MDHTMLTag, text: &str)) {
    if r.flags & MD_HTML_FLAG_VERBATIM_ENTITIES > 0 {
        render_verbatim(r, text);
        return;
    }

    let size: u32 = text.chars().count() as u32;

    /* We assume UTF-8 output is what is desired. */
    if size > 3 && text.chars().nth((1) as usize).unwrap() == '#' {
        let mut codepoint: u32 = 0;
        if text.chars().nth((2) as usize).unwrap() == 'x' ||
            text.chars().nth((2) as usize).unwrap() == 'X'
        {
            /* Hexadecimal entity (e.g. "&#x1234abcd;")). */
            for i in 3..size {
                let idx = i as usize;
                let hex_value = get_char_as_u32(&text.chars().nth((i) as usize).unwrap());
                codepoint = 16 * codepoint + hex_value;
            }

        } else {
            /* Decimal entity (e.g. "&1234;") */
            for i in 2..size {
                let idx = i as usize;
                let hex_value = get_char_as_u32(&text.chars().nth((i) as usize).unwrap()) - get_char_as_u32(&'0');
                codepoint = 10 * codepoint + hex_value;
            }
        }
        render_utf8_codepoint(r, codepoint, fn_append);
        return;
    } else {
        let entity: Option<&Entity> = Entity::entity_lookup(text);

        if entity.is_some() {
            render_utf8_codepoint(r, entity.unwrap().codepoints[0], fn_append);
            if entity.unwrap().codepoints[1] > 0 {
                render_utf8_codepoint(r, entity.unwrap().codepoints[1], fn_append);
            }
            return;
        }
        fn_append(r, text);
    }
}

pub fn render_attribute(r: &MDHTMLTag, attr: MDAttribute, fn_append: fn(r: &MDHTMLTag, text: &str)) {
    let mut i: usize = 0;
    let mut offset = attr.substr_offsets[i];

    while offset < attr.size {

        let _type: &'static str = MD_TEXTTYPE2[i];
        let off: u32 = attr.substr_offsets[i];
        let text: &str = &attr.text[(off as usize)..];

        (|| {
            match _type {
                "MD_TEXT_NULLCHAR" => {
                    render_utf8_codepoint(r, 0x0000, render_verbatim);
                    return;
                }
                "MD_TEXT_ENTITY" => {
                    render_entity(r, text, fn_append);
                    return;
                }
                _ => {
                    fn_append(r, text);
                    return;
                }
            }
        })();

        i += 1;
        offset = attr.substr_offsets[i];
    }
}

pub fn render_open_ol_block(r: &MDHTMLTag, det: MDBlockOLDetail) {
    let text = format!("<ol start=\"{}\">\n", det.start);

    if det.start == 0 {
        render_verbatim(r, "<ol>\n");
        return;
    }

    render_verbatim(r, text.as_str());
}

pub fn render_open_li_block(r: &MDHTMLTag, det: MDBlockLIDetail) {
    if det.is_task > 0 {
        render_verbatim(r, "<li class=\"task-list-item\">\
                                <input type=\"checkbox\" class=\"task-list-item-checkbox\" disabled");

        if det.task_mark == 'x' ||
            det.task_mark == 'X' {
            render_verbatim(r, " checked");
        }
        render_verbatim(r, ">");
    } else {
        render_verbatim(r, "<li>")
    }
}

pub fn render_open_code_block(r: &MDHTMLTag, det: MDBlockCodeDetail) {
    render_verbatim(r, "<pre><code");

    if !det.lang.text.is_empty() {
        render_verbatim(r, " class=\"language-");
        render_attribute(r, det.lang, render_html_escaped);
        render_verbatim(r, "\"");
    }

    render_verbatim(r, "\"");
}

pub fn render_open_td_block(r: &MDHTMLTag, cell_type: &str, det: MDBlockTDDetail) {
    render_verbatim(r, "<");
    render_verbatim(r, cell_type);

    (|| {
        match det.align {
            MD_ALIGN::MD_ALIGN_LEFT => {
                render_verbatim(r, " align=\"left\">");
                return;
            }
            MD_ALIGN::MD_ALIGN_CENTER => {
                render_verbatim(r, " align=\"center\">");
                return;
            }

            MD_ALIGN::MD_ALIGN_RIGHT => {
                render_verbatim(r, " align=\"right\">");
                return;
            }

            _ => {
                render_verbatim(r, ">");
                return;
            }
        }
    })();
}

pub fn render_open_a_span(r: &MDHTMLTag, det: MDSpanADetail) {
    render_verbatim(r, "<a href=\"");
    render_attribute(r, det.href, render_url_escaped);

    if !det.title.text.is_empty() {
        render_verbatim(r, "\" title=\"");
        render_attribute(r, det.title, render_html_escaped);
    }

    render_verbatim(r, "\">");
}

pub fn render_open_img_span(r: &mut MDHTMLTag, det: MDSpanImgDetail) {
    render_verbatim(r, "<img src=\"");
    render_attribute(r, det.src, render_url_escaped);
    render_verbatim(r, "\" alt=\"");
    r.image_nesting_level += 1;
}

pub fn render_close_img_span(r: &mut MDHTMLTag, det: MDSpanImgDetail){
    if !det.title.text.is_empty() {
        render_verbatim(r, "\" title=\"");
        render_attribute(r, det.title, render_html_escaped);
    }

    let text = if r.flags & MD_HTML_FLAG_XHTML > 0 { "\" />" } else { "\">" };
    render_verbatim(r, text);

    r.image_nesting_level -= 1;
}

pub fn render_open_wikilink_span(r: &MDHTMLTag, det: MDSpanWikiDetail){
    render_verbatim(r, "<x-wikilink data-target=\"");
    render_attribute(r, det.target, render_html_escaped);
    render_verbatim(r, "\">");
}


/**************************************
 ***  HTML renderer implementation  ***
 **************************************/
pub fn enter_block_callback(_type: MD_BLOCKTYPE, detail: MDBlock, r: &MDHTMLTag) -> u32 {
    static HEADINGS: [&str; 6] = [ "<h1>", "<h2>", "<h3>", "<h4>", "<h5>", "<h6>" ];

    (|| {
        match _type {
            MD_BLOCKTYPE::MD_BLOCK_DOC => {
                /* noop */
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_QUOTE => {
                render_verbatim(r, "<blockquote>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_UL => {
                render_verbatim(r, "<ul>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_OL => {
                render_open_ol_block(r, detail.ol);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_LI => {
                render_open_li_block(r, detail.li);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_HR => {
                let text =  if r.flags & MD_HTML_FLAG_XHTML > 0 { "<hr />\n" } else { "<hr>\n" };
                render_verbatim(r, text);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_H => {
                let heading = HEADINGS[(detail.h.level - 1) as usize];
                render_verbatim(r, heading);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_CODE => {
                render_open_code_block(r, detail.code);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_HTML => {
                /* noop */
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_P => {
                render_verbatim(r, "<p>");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TABLE => {
                render_verbatim(r, "<table>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_THEAD => {
                render_verbatim(r, "<thead>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TBODY => {
                render_verbatim(r, "<tbody>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TR => {
                render_verbatim(r, "<tr>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TH => {
                render_open_td_block(r, "th", detail.th);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TD => {
                render_open_td_block(r, "td", detail.td);
                return;
            }
        }
    })();

    return 0;
}


pub fn leave_block_callback(_type: MD_BLOCKTYPE, detail: MDBlock, r: &MDHTMLTag) -> u32 {
    static HEADINGS: [&str; 6] = [ "</h1>\n", "</h2>\n", "</h3>\n", "</h4>\n", "</h5>\n", "</h6>\n" ];

    (|| {
        match _type {
            MD_BLOCKTYPE::MD_BLOCK_DOC => {
                /* noop */
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_QUOTE => {
                render_verbatim(r, "</blockquote>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_UL => {
                render_verbatim(r, "</ul>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_OL => {
                render_verbatim(r, "</ol>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_LI => {
                render_verbatim(r, "</li>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_HR => {
                /* noop */
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_H => {
                let text = HEADINGS[(detail.h.level - 1) as usize];
                render_verbatim(r, text);
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_CODE => {
                render_verbatim(r, "</code></pre>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_HTML => {
                /* noop */
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_P => {
                render_verbatim(r, "</p>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TABLE => {
                render_verbatim(r, "</table>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_THEAD => {
                render_verbatim(r, "</thead>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TBODY => {
                render_verbatim(r, "</tbody>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TR => {
                render_verbatim(r, "</tr>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TH => {
                render_verbatim(r, "</th>\n");
                return;
            }
            MD_BLOCKTYPE::MD_BLOCK_TD => {
                render_verbatim(r, "</td>\n");
                return;
            }
        }
    })();

    return 0;
}


pub fn enter_span_callback(_type: MD_SPANTYPE, detail: MDSpan, r: &mut MDHTMLTag) -> u32 {

    if r.image_nesting_level > 0 {
        /* We are inside a Markdown image label. Markdown allows to use any
         * emphasis and other rich contents in that context similarly as in
         * any link label.
         *
         * However, unlike in the case of links (where that contents becomes
         * contents of the <a>...</a> tag), in the case of images the contents
         * is supposed to fall into the attribute alt: <img alt="...">.
         *
         * In that context we naturally cannot output nested HTML tags. So lets
         * suppress them and only output the plain text (i.e. what falls into
         * text() callback).
         *
         * This make-it-a-plain-text approach is the recommended practice by
         * CommonMark specification (for HTML output).
         */
        return 0;
    }

    (|| {
        match _type {
            MD_SPANTYPE::MD_SPAN_EM => {
                render_verbatim(r, "<em>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_STRONG => {
                render_verbatim(r, "<strong>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_U => {
                render_verbatim(r, "<u>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_A => {
                render_open_a_span(r, detail.a);
                return;
            }
            MD_SPANTYPE::MD_SPAN_IMG => {
                render_open_img_span(r, detail.img);
                return;
            }
            MD_SPANTYPE::MD_SPAN_CODE => {
                render_verbatim(r, "<code>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_DEL => {
                render_verbatim(r, "<del>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_LATEXMATH => {
                render_verbatim(r, "<x-equation>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_LATEXMATH_DISPLAY => {
                render_verbatim(r,  "<x-equation type=\"display\">");
                return;
            }
            MD_SPANTYPE::MD_SPAN_WIKILINK => {
                render_open_wikilink_span(r, detail.wiki);
                return;
            }
        }
    })();

    return 0;
}


pub fn leave_span_callback(_type: MD_SPANTYPE, detail: MDSpan, r: &mut MDHTMLTag) -> u32 {

    if r.image_nesting_level > 0 {
        /* Ditto as in enter_span_callback(), except we have to allow the
         * end of the <img> tag. */
        if r.image_nesting_level == 1  &&  _type == MD_SPANTYPE::MD_SPAN_IMG {
            render_close_img_span(r, detail.img);
        }
        return 0;
    }

    (|| {
        match _type {
            MD_SPANTYPE::MD_SPAN_EM => {
                render_verbatim(r, "</em>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_STRONG => {
                render_verbatim(r, "</strong>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_U => {
                render_verbatim(r, "</u>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_A => {
                render_verbatim(r, "</a>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_IMG => {
                /*noop, handled above*/
                return;
            }
            MD_SPANTYPE::MD_SPAN_CODE => {
                render_verbatim(r, "</code>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_DEL => {
                render_verbatim(r, "</del>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_LATEXMATH => {
                /*fall through*/
                return;
            }
            MD_SPANTYPE::MD_SPAN_LATEXMATH_DISPLAY => {
                render_verbatim(r, "</x-equation>");
                return;
            }
            MD_SPANTYPE::MD_SPAN_WIKILINK => {
                render_verbatim(r, "</x-wikilink>");
                return;
            }
        }
    })();

    return 0;
}

pub fn text_callback(_type: &MD_TEXTTYPE, text: &str, r: &mut MDHTMLTag) -> u32{

    (|| {
        match _type {
            MD_TEXTTYPE::MD_TEXT_NULLCHAR => {
                render_utf8_codepoint(r, 0x0000, render_verbatim);
                return;
            }
            MD_TEXTTYPE::MD_TEXT_BR => {
                let br = if r.image_nesting_level == 0 { if r.flags & MD_HTML_FLAG_XHTML > 0 {"<br />\n"} else {"<br>\n"}} else {" "};
                render_verbatim(r, br);
                return;
            }
            MD_TEXTTYPE::MD_TEXT_SOFTBR => {
                let sbr = if r.image_nesting_level == 0 {"\n" } else {" "};
                render_verbatim(r, sbr);
                return;
            }
            MD_TEXTTYPE::MD_TEXT_HTML => {
                render_html_escaped(r, text);
                return;
            }
            MD_TEXTTYPE::MD_TEXT_ENTITY => {
                render_entity(r, text, render_html_escaped);
                return;
            }
            _ => {
                render_html_escaped(r, text);
                return;
            }
        }
    })();

    return 0;
}

pub fn debug_log_callback(msg: &str, r: MDHTMLTag){
    if r.flags & MD_HTML_FLAG_DEBUG > 0 {
        println!("MD4C: {}\n", msg)
    }
}

pub fn md_html(input: &str, process_output: fn(text: &str, _: fn()),
               userdata: fn(), parser_flags: u32, renderer_flags: u32
) -> u32 {
    let mut render: MDHTMLTag = MDHTMLTag {
        process_output,
        flags: renderer_flags,
        userdata,
        escape_map: ['0'; 256],
        image_nesting_level: 0
    };

    let parser: MDParser = MDParser {
        abi_version: 0,
        flags: parser_flags,
        enter_block: enter_block_callback,
        leave_block: leave_block_callback,
        enter_span: enter_span_callback,
        leave_span: leave_span_callback,
        text: text_callback,
        debug_log: debug_log_callback,
        syntax: 0
    };

    for i in 0..256 {
        let ch = char::from_u32(i as u32).unwrap();
        let mut escape_map = 0;

        if "\"&<>".contains(ch) {
            escape_map |= 0x1;
        }

        if !ch.is_ascii_alphanumeric() && !"~-_.+!*(),%#@?=;:/,+$".contains(ch){
            escape_map |= 0x2;
        }

        render.escape_map[i as usize] = char::from_u32(escape_map as u32).unwrap();
    }

    /* Consider skipping UTF-8 byte order mark (BOM). */

    return md_parse(input, &parser, &render);
}
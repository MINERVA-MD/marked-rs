#![allow(warnings, unused)]
use regex::Regex;
use fancy_regex::*;
use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use crate::defaults::Options;

lazy_static! {
    static ref CARET: regex::Regex = regex::Regex::new("(^|[^\\[])\\^").unwrap();

    static ref BLOCK: BlockRegex<fancy_regex::Regex> = get_global_block_rules();
    static ref INLINE: InlineRegex<fancy_regex::Regex> = get_global_inline_rules();

    static ref BLOCK_REGRESS: BlockRegex<regress::Regex> = get_global_block_regress_rules();
    static ref INLINE_REGRESS: InlineRegex<regress::Regex> = get_global_inline_regress_rules();
}

//  Structs for Global FR Regex
pub struct BlockRegex <R> {
    pub normal: BlockRegexGroup<R>,
    pub pedantic: BlockRegexGroup<R>,
    pub gfm: BlockRegexGroup<R>
}

pub struct InlineRegex<R>  {
    pub normal: InlineRegexGroup<R>,
    pub pedantic: InlineRegexGroup<R>,
    pub gfm: InlineRegexGroup<R>,
    pub gfm_with_breaks: InlineRegexGroup<R>,
}

pub struct BlockRegexGroup<R> {
    pub newline: R,
    pub code: R,
    pub fences: R,
    pub hr: R,
    pub heading: R,
    pub blockquote: R,
    pub list: R,
    pub html: R,
    pub def: R,
    pub table: R,
    pub l_heading: R,
    pub paragraph: R,
    pub text: R,
    pub label: R,
    pub title: R,
    pub bullet: R,
    pub list_item_start: R,
    pub tag: R,
    pub comment: R
}

pub struct InlineRegexGroup<R> {
    pub escape: R,
    pub autolink: R,
    pub url: R,
    pub tag: R,
    pub link: R,
    pub ref_link: R,
    pub no_link: R,
    pub ref_link_search: R,
    pub em_strong: DelimRegex<R>,
    pub code: R,
    pub br: R,
    pub del: R,
    pub text: R,
    pub punctuation: R,
    pub _punctuation: R,
    pub block_skip: R,
    pub escaped_em_st: R,
    pub comment: R,
    pub escapes: R,
    pub scheme: R,
    pub email: R,
    pub attribute: R,
    pub label: R,
    pub href: R,
    pub title: R,
    pub breaks: R,
    pub strong: BoldRegex<R>,
    pub em: BoldRegex<R>,
    pub extended_email: R,
    pub backpedal: R
}

pub struct DelimRegex<R>  {
    pub l_delim: R,
    pub r_delim_ast: R,
    pub r_delim_und: R
}

pub struct BoldRegex<R> {
    pub start: R,
    pub middle: R,
    pub end_ast: R,
    pub end_und: R
}




// Access point to globally compiled regexes
pub fn exec_block<'a>(
    src: &'a str,
    rule: MDBlock,
    options: &Options,
    opt: &'a str,
) -> Option<fancy_regex::Captures<'a>> {

    return if options.pedantic {
        exec_pedantic_block(src, opt, rule)
    } else if options.gfm {
        exec_gfm_block(src, opt, rule)
    } else {
        exec_normal_block(src, opt, rule)
    };
}

pub fn exec_inline<'a>(
    src: &'a str,
    rule: MDInline,
    options: &Options,
    opt: &'a str,
) -> Option<fancy_regex::Captures<'a>> {
    return if options.pedantic {
        exec_pedantic_inline(src, opt, rule)
    } else if options.gfm {
        if options.breaks {
            exec_gfm_breaks_inline(src, opt, rule)
        } else {
            exec_gfm_inline(src, opt, rule)
        }
    } else {
        exec_normal_inline(src, opt, rule)
    };
}


// Get direct access to global compiled regexes
pub fn get_block<'a>(
    rule: MDBlock,
    options: &Options,
    opt: &'a str,
) -> &'a fancy_regex::Regex {

    return if options.pedantic {
        get_pedantic_block(opt, rule)
    } else if options.gfm {
        get_gfm_block(opt, rule)
    } else {
        get_normal_block(opt, rule)
    };
}

pub fn get_inline<'a>(
    rule: MDInline,
    options: &Options,
    opt: &'a str,
) -> &'a fancy_regex::Regex {
    return if options.pedantic {
        get_pedantic_inline(opt, rule)
    } else if options.gfm {
        if options.breaks {
            get_gfm_breaks_inline(opt, rule)
        } else {
            get_gfm_inline(opt, rule)
        }
    } else {
        get_normal_inline(opt, rule)
    };
}


pub fn exec_block_regress<'a>(
    src: &'a str,
    rule: MDBlock,
    options: &Options,
    opt: &'a str,
) -> Option<regress::Match> {

    return if options.pedantic {
        exec_pedantic_block_regress(src, opt, rule)
    } else if options.gfm {
        exec_gfm_block_regress(src, opt, rule)
    } else {
        exec_normal_block_regress(src, opt, rule)
    };
}

pub fn exec_inline_regress<'a>(
    src: &'a str,
    rule: MDInline,
    options: &Options,
    opt: &'a str,
) -> Option<fancy_regex::Captures<'a>> {
    return if options.pedantic {
        exec_pedantic_inline(src, opt, rule)
    } else if options.gfm {
        if options.breaks {
            exec_gfm_breaks_inline(src, opt, rule)
        } else {
            exec_gfm_inline(src, opt, rule)
        }
    } else {
        exec_normal_inline(src, opt, rule)
    };
}

fn get_global_block_rules() -> BlockRegex<fancy_regex::Regex> {
    // normal_block, gfm_block, pedantic_block
    let (
        normal_block,
        gfm_block,
        pedantic_block
    ) = get_block_rules();

    BlockRegex {
        normal: BlockRegexGroup {
            newline: build_regex(normal_block.newline),
            code: build_regex(normal_block.code),
            fences: build_regex(normal_block.fences),
            hr: build_regex(normal_block.hr),
            heading: build_regex(normal_block.heading),
            blockquote: build_regex(normal_block.blockquote),
            list: build_regex(normal_block.list),
            html: build_regex(("(?i)".to_owned() + normal_block.html.as_str())),
            def: build_regex(normal_block.def),
            table: build_regex(normal_block.table),
            l_heading: build_regex(normal_block.l_heading),
            paragraph: build_regex(normal_block.paragraph),
            text: build_regex(normal_block.text),
            label: build_regex(normal_block.label),
            title: build_regex(normal_block.title),
            bullet: build_regex(normal_block.bullet),
            list_item_start: build_regex(normal_block.list_item_start),
            tag: build_regex(normal_block.tag),
            comment: build_regex(normal_block.comment),
        },
        gfm: BlockRegexGroup {
            newline: build_regex(gfm_block.newline),
            code: build_regex(gfm_block.code),
            fences: build_regex(gfm_block.fences),
            hr: build_regex(gfm_block.hr),
            heading: build_regex(gfm_block.heading),
            blockquote: build_regex(gfm_block.blockquote),
            list: build_regex(gfm_block.list),
            html: build_regex(("(?i)".to_owned() + gfm_block.html.as_str())),
            def: build_regex(gfm_block.def),
            table: build_regex(gfm_block.table),
            l_heading: build_regex(gfm_block.l_heading),
            paragraph: build_regex(gfm_block.paragraph),
            text: build_regex(gfm_block.text),
            label: build_regex(gfm_block.label),
            title: build_regex(gfm_block.title),
            bullet: build_regex(gfm_block.bullet),
            list_item_start: build_regex(gfm_block.list_item_start),
            tag: build_regex(gfm_block.tag),
            comment: build_regex(gfm_block.comment),
        },
        pedantic: BlockRegexGroup {
            newline: build_regex(pedantic_block.newline),
            code: build_regex(pedantic_block.code),
            fences: build_regex(pedantic_block.fences),
            hr: build_regex(pedantic_block.hr),
            heading: build_regex(pedantic_block.heading),
            blockquote: build_regex(pedantic_block.blockquote),
            list: build_regex(pedantic_block.list),
            html: build_regex(("(?i)".to_owned() + pedantic_block.html.as_str())),
            def: build_regex(pedantic_block.def),
            table: build_regex(pedantic_block.table),
            l_heading: build_regex(pedantic_block.l_heading),
            paragraph: build_regex(pedantic_block.paragraph),
            text: build_regex(pedantic_block.text),
            label: build_regex(pedantic_block.label),
            title: build_regex(pedantic_block.title),
            bullet: build_regex(pedantic_block.bullet),
            list_item_start: build_regex(pedantic_block.list_item_start),
            tag: build_regex(pedantic_block.tag),
            comment: build_regex(pedantic_block.comment),
        },
    }
}

fn get_global_inline_rules() -> InlineRegex<fancy_regex::Regex> {
    let (
        normal_inline,
        pedantic_inline,
        gfm_inline,
        gfm_with_breaks_inline
    ) = get_inline_rules();

    InlineRegex {
        normal: InlineRegexGroup {
            escape: build_regex(normal_inline.escape),
            autolink: build_regex(normal_inline.autolink),
            url: build_regex("(?i)".to_owned() + normal_inline.url.as_str()),
            tag: build_regex(normal_inline.tag),
            link: build_regex(normal_inline.link),
            ref_link: build_regex(normal_inline.ref_link),
            no_link: build_regex(normal_inline.no_link),
            ref_link_search: build_regex(normal_inline.ref_link_search),
            em_strong: DelimRegex {
                l_delim: build_regex(normal_inline.em_strong.l_delim),
                r_delim_ast: build_regex(normal_inline.em_strong.r_delim_ast),
                r_delim_und: build_regex(normal_inline.em_strong.r_delim_und),
            },
            code: build_regex(normal_inline.code),
            br: build_regex(normal_inline.br),
            del: build_regex(normal_inline.del),
            text: build_regex(normal_inline.text),
            punctuation: build_regex(normal_inline.punctuation),
            _punctuation: build_regex(normal_inline._punctuation),
            block_skip: build_regex(normal_inline.block_skip),
            escaped_em_st: build_regex(normal_inline.escaped_em_st),
            comment: build_regex(normal_inline.comment),
            escapes: build_regex(normal_inline.escapes),
            scheme: build_regex(normal_inline.scheme),
            email: build_regex(normal_inline.email),
            attribute: build_regex(normal_inline.attribute),
            label: build_regex(normal_inline.label),
            href: build_regex(normal_inline.href),
            title: build_regex(normal_inline.title),
            breaks: build_regex(normal_inline.breaks),
            strong: BoldRegex {
                start: build_regex(normal_inline.strong.start),
                middle: build_regex(normal_inline.strong.middle),
                end_ast: build_regex(normal_inline.strong.end_ast),
                end_und: build_regex(normal_inline.strong.end_und),
            },
            em: BoldRegex {
                start: build_regex(normal_inline.em.start),
                middle: build_regex(normal_inline.em.middle),
                end_ast: build_regex(normal_inline.em.end_ast),
                end_und: build_regex(normal_inline.em.end_und),
            },
            extended_email: build_regex(normal_inline.extended_email),
            backpedal: build_regex(normal_inline.backpedal),
        },
        pedantic: InlineRegexGroup {
            escape: build_regex(pedantic_inline.escape),
            autolink: build_regex(pedantic_inline.autolink),
            url: build_regex("(?i)".to_owned() + pedantic_inline.url.as_str()),
            tag: build_regex(pedantic_inline.tag),
            link: build_regex(pedantic_inline.link),
            ref_link: build_regex(pedantic_inline.ref_link),
            no_link: build_regex(pedantic_inline.no_link),
            ref_link_search: build_regex(pedantic_inline.ref_link_search),
            em_strong: DelimRegex {
                l_delim: build_regex(pedantic_inline.em_strong.l_delim),
                r_delim_ast: build_regex(pedantic_inline.em_strong.r_delim_ast),
                r_delim_und: build_regex(pedantic_inline.em_strong.r_delim_und),
            },
            code: build_regex(pedantic_inline.code),
            br: build_regex(pedantic_inline.br),
            del: build_regex(pedantic_inline.del),
            text: build_regex(pedantic_inline.text),
            punctuation: build_regex(pedantic_inline.punctuation),
            _punctuation: build_regex(pedantic_inline._punctuation),
            block_skip: build_regex(pedantic_inline.block_skip),
            escaped_em_st: build_regex(pedantic_inline.escaped_em_st),
            comment: build_regex(pedantic_inline.comment),
            escapes: build_regex(pedantic_inline.escapes),
            scheme: build_regex(pedantic_inline.scheme),
            email: build_regex(pedantic_inline.email),
            attribute: build_regex(pedantic_inline.attribute),
            label: build_regex(pedantic_inline.label),
            href: build_regex(pedantic_inline.href),
            title: build_regex(pedantic_inline.title),
            breaks: build_regex(pedantic_inline.breaks),
            strong: BoldRegex {
                start: build_regex(pedantic_inline.strong.start),
                middle: build_regex(pedantic_inline.strong.middle),
                end_ast: build_regex(pedantic_inline.strong.end_ast),
                end_und: build_regex(pedantic_inline.strong.end_und),
            },
            em: BoldRegex {
                start: build_regex(pedantic_inline.em.start),
                middle: build_regex(pedantic_inline.em.middle),
                end_ast: build_regex(pedantic_inline.em.end_ast),
                end_und: build_regex(pedantic_inline.em.end_und),
            },
            extended_email: build_regex(pedantic_inline.extended_email),
            backpedal: build_regex(pedantic_inline.backpedal),
        },
        gfm: InlineRegexGroup {
            escape: build_regex(gfm_inline.escape),
            autolink: build_regex(gfm_inline.autolink),
            url: build_regex("(?i)".to_owned() + gfm_inline.url.as_str()),
            tag: build_regex(gfm_inline.tag),
            link: build_regex(gfm_inline.link),
            ref_link: build_regex(gfm_inline.ref_link),
            no_link: build_regex(gfm_inline.no_link),
            ref_link_search: build_regex(gfm_inline.ref_link_search),
            em_strong: DelimRegex {
                l_delim: build_regex(gfm_inline.em_strong.l_delim),
                r_delim_ast: build_regex(gfm_inline.em_strong.r_delim_ast),
                r_delim_und: build_regex(gfm_inline.em_strong.r_delim_und),
            },
            code: build_regex(gfm_inline.code),
            br: build_regex(gfm_inline.br),
            del: build_regex(gfm_inline.del),
            text: build_regex(gfm_inline.text),
            punctuation: build_regex(gfm_inline.punctuation),
            _punctuation: build_regex(gfm_inline._punctuation),
            block_skip: build_regex(gfm_inline.block_skip),
            escaped_em_st: build_regex(gfm_inline.escaped_em_st),
            comment: build_regex(gfm_inline.comment),
            escapes: build_regex(gfm_inline.escapes),
            scheme: build_regex(gfm_inline.scheme),
            email: build_regex(gfm_inline.email),
            attribute: build_regex(gfm_inline.attribute),
            label: build_regex(gfm_inline.label),
            href: build_regex(gfm_inline.href),
            title: build_regex(gfm_inline.title),
            breaks: build_regex(gfm_inline.breaks),
            strong: BoldRegex {
                start: build_regex(gfm_inline.strong.start),
                middle: build_regex(gfm_inline.strong.middle),
                end_ast: build_regex(gfm_inline.strong.end_ast),
                end_und: build_regex(gfm_inline.strong.end_und),
            },
            em: BoldRegex {
                start: build_regex(gfm_inline.em.start),
                middle: build_regex(gfm_inline.em.middle),
                end_ast: build_regex(gfm_inline.em.end_ast),
                end_und: build_regex(gfm_inline.em.end_und),
            },
            extended_email: build_regex(gfm_inline.extended_email),
            backpedal: build_regex(gfm_inline.backpedal),
        },
        gfm_with_breaks: InlineRegexGroup {
            escape: build_regex(gfm_with_breaks_inline.escape),
            autolink: build_regex(gfm_with_breaks_inline.autolink),
            url: build_regex("(?i)".to_owned() + gfm_with_breaks_inline.url.as_str()),
            tag: build_regex(gfm_with_breaks_inline.tag),
            link: build_regex(gfm_with_breaks_inline.link),
            ref_link: build_regex(gfm_with_breaks_inline.ref_link),
            no_link: build_regex(gfm_with_breaks_inline.no_link),
            ref_link_search: build_regex(gfm_with_breaks_inline.ref_link_search),
            em_strong: DelimRegex {
                l_delim: build_regex(gfm_with_breaks_inline.em_strong.l_delim),
                r_delim_ast: build_regex(gfm_with_breaks_inline.em_strong.r_delim_ast),
                r_delim_und: build_regex(gfm_with_breaks_inline.em_strong.r_delim_und),
            },
            code: build_regex(gfm_with_breaks_inline.code),
            br: build_regex(gfm_with_breaks_inline.br),
            del: build_regex(gfm_with_breaks_inline.del),
            text: build_regex(gfm_with_breaks_inline.text),
            punctuation: build_regex(gfm_with_breaks_inline.punctuation),
            _punctuation: build_regex(gfm_with_breaks_inline._punctuation),
            block_skip: build_regex(gfm_with_breaks_inline.block_skip),
            escaped_em_st: build_regex(gfm_with_breaks_inline.escaped_em_st),
            comment: build_regex(gfm_with_breaks_inline.comment),
            escapes: build_regex(gfm_with_breaks_inline.escapes),
            scheme: build_regex(gfm_with_breaks_inline.scheme),
            email: build_regex(gfm_with_breaks_inline.email),
            attribute: build_regex(gfm_with_breaks_inline.attribute),
            label: build_regex(gfm_with_breaks_inline.label),
            href: build_regex(gfm_with_breaks_inline.href),
            title: build_regex(gfm_with_breaks_inline.title),
            breaks: build_regex(gfm_with_breaks_inline.breaks),
            strong: BoldRegex {
                start: build_regex(gfm_with_breaks_inline.strong.start),
                middle: build_regex(gfm_with_breaks_inline.strong.middle),
                end_ast: build_regex(gfm_with_breaks_inline.strong.end_ast),
                end_und: build_regex(gfm_with_breaks_inline.strong.end_und),
            },
            em: BoldRegex {
                start: build_regex(gfm_with_breaks_inline.em.start),
                middle: build_regex(gfm_with_breaks_inline.em.middle),
                end_ast: build_regex(gfm_with_breaks_inline.em.end_ast),
                end_und: build_regex(gfm_with_breaks_inline.em.end_und),
            },
            extended_email: build_regex(gfm_with_breaks_inline.extended_email),
            backpedal: build_regex(gfm_with_breaks_inline.backpedal),
        }
    }
}

fn get_global_block_regress_rules() -> BlockRegex<regress::Regex> {
    // normal_block, gfm_block, pedantic_block
    let (
        normal_block,
        gfm_block,
        pedantic_block
    ) = get_block_rules();

    BlockRegex {
        normal: BlockRegexGroup {
            newline: build_regress(normal_block.newline, ""),
            code: build_regress(normal_block.code, ""),
            fences: build_regress(normal_block.fences, ""),
            hr: build_regress(normal_block.hr, ""),
            heading: build_regress(normal_block.heading, ""),
            blockquote: build_regress(normal_block.blockquote, ""),
            list: build_regress(normal_block.list, ""),
            html: build_regress(normal_block.html, "i"),
            def: build_regress(normal_block.def, ""),
            table: build_regress(normal_block.table, ""),
            l_heading: build_regress(normal_block.l_heading, ""),
            paragraph: build_regress(normal_block.paragraph, ""),
            text: build_regress(normal_block.text, ""),
            label: build_regress(normal_block.label, ""),
            title: build_regress(normal_block.title, ""),
            bullet: build_regress(normal_block.bullet, ""),
            list_item_start: build_regress(normal_block.list_item_start, ""),
            tag: build_regress(normal_block.tag, ""),
            comment: build_regress(normal_block.comment, ""),
        },
        gfm: BlockRegexGroup {
            newline: build_regress(gfm_block.newline, ""),
            code: build_regress(gfm_block.code, ""),
            fences: build_regress(gfm_block.fences, ""),
            hr: build_regress(gfm_block.hr, ""),
            heading: build_regress(gfm_block.heading, ""),
            blockquote: build_regress(gfm_block.blockquote, ""),
            list: build_regress(gfm_block.list, ""),
            html: build_regress(gfm_block.html, "i"),
            def: build_regress(gfm_block.def, ""),
            table: build_regress(gfm_block.table, ""),
            l_heading: build_regress(gfm_block.l_heading, ""),
            paragraph: build_regress(gfm_block.paragraph, ""),
            text: build_regress(gfm_block.text, ""),
            label: build_regress(gfm_block.label, ""),
            title: build_regress(gfm_block.title, ""),
            bullet: build_regress(gfm_block.bullet, ""),
            list_item_start: build_regress(gfm_block.list_item_start, ""),
            tag: build_regress(gfm_block.tag, ""),
            comment: build_regress(gfm_block.comment, ""),
        },
        pedantic: BlockRegexGroup {
            newline: build_regress(pedantic_block.newline, ""),
            code: build_regress(pedantic_block.code, ""),
            fences: build_regress(pedantic_block.fences, ""),
            hr: build_regress(pedantic_block.hr, ""),
            heading: build_regress(pedantic_block.heading, ""),
            blockquote: build_regress(pedantic_block.blockquote, ""),
            list: build_regress(pedantic_block.list, ""),
            html: build_regress(pedantic_block.html, "i"),
            def: build_regress(pedantic_block.def, ""),
            table: build_regress(pedantic_block.table, ""),
            l_heading: build_regress(pedantic_block.l_heading, ""),
            paragraph: build_regress(pedantic_block.paragraph, ""),
            text: build_regress(pedantic_block.text, ""),
            label: build_regress(pedantic_block.label, ""),
            title: build_regress(pedantic_block.title, ""),
            bullet: build_regress(pedantic_block.bullet, ""),
            list_item_start: build_regress(pedantic_block.list_item_start, ""),
            tag: build_regress(pedantic_block.tag, ""),
            comment: build_regress(pedantic_block.comment, ""),
        },
    }
}

fn get_global_inline_regress_rules() -> InlineRegex<regress::Regex> {
    let (
        normal_inline,
        pedantic_inline,
        gfm_inline,
        gfm_with_breaks_inline
    ) = get_inline_rules();

    InlineRegex {
        normal: InlineRegexGroup {
            escape: build_regress(normal_inline.escape, ""),
            autolink: build_regress(normal_inline.autolink, ""),
            url: build_regress(normal_inline.url, "i"),
            tag: build_regress(normal_inline.tag, ""),
            link: build_regress(normal_inline.link, ""),
            ref_link: build_regress(normal_inline.ref_link, ""),
            no_link: build_regress(normal_inline.no_link, ""),
            ref_link_search: build_regress(normal_inline.ref_link_search, ""),
            em_strong: DelimRegex {
                l_delim: build_regress(normal_inline.em_strong.l_delim, ""),
                r_delim_ast: build_regress(normal_inline.em_strong.r_delim_ast, ""),
                r_delim_und: build_regress(normal_inline.em_strong.r_delim_und, ""),
            },
            code: build_regress(normal_inline.code, ""),
            br: build_regress(normal_inline.br, ""),
            del: build_regress(normal_inline.del, ""),
            text: build_regress(normal_inline.text, ""),
            punctuation: build_regress(normal_inline.punctuation, ""),
            _punctuation: build_regress(normal_inline._punctuation, ""),
            block_skip: build_regress(normal_inline.block_skip, ""),
            escaped_em_st: build_regress(normal_inline.escaped_em_st, ""),
            comment: build_regress(normal_inline.comment, ""),
            escapes: build_regress(normal_inline.escapes, ""),
            scheme: build_regress(normal_inline.scheme, ""),
            email: build_regress(normal_inline.email, ""),
            attribute: build_regress(normal_inline.attribute, ""),
            label: build_regress(normal_inline.label, ""),
            href: build_regress(normal_inline.href, ""),
            title: build_regress(normal_inline.title, ""),
            breaks: build_regress(normal_inline.breaks, ""),
            strong: BoldRegex {
                start: build_regress(normal_inline.strong.start, ""),
                middle: build_regress(normal_inline.strong.middle, ""),
                end_ast: build_regress(normal_inline.strong.end_ast, ""),
                end_und: build_regress(normal_inline.strong.end_und, ""),
            },
            em: BoldRegex {
                start: build_regress(normal_inline.em.start, ""),
                middle: build_regress(normal_inline.em.middle, ""),
                end_ast: build_regress(normal_inline.em.end_ast, ""),
                end_und: build_regress(normal_inline.em.end_und, ""),
            },
            extended_email: build_regress(normal_inline.extended_email, ""),
            backpedal: build_regress(normal_inline.backpedal, ""),
        },
        pedantic: InlineRegexGroup {
            escape: build_regress(pedantic_inline.escape, ""),
            autolink: build_regress(pedantic_inline.autolink, ""),
            url: build_regress(pedantic_inline.url, "i"),
            tag: build_regress(pedantic_inline.tag, ""),
            link: build_regress(pedantic_inline.link, ""),
            ref_link: build_regress(pedantic_inline.ref_link, ""),
            no_link: build_regress(pedantic_inline.no_link, ""),
            ref_link_search: build_regress(pedantic_inline.ref_link_search, ""),
            em_strong: DelimRegex {
                l_delim: build_regress(pedantic_inline.em_strong.l_delim, ""),
                r_delim_ast: build_regress(pedantic_inline.em_strong.r_delim_ast, ""),
                r_delim_und: build_regress(pedantic_inline.em_strong.r_delim_und, ""),
            },
            code: build_regress(pedantic_inline.code, ""),
            br: build_regress(pedantic_inline.br, ""),
            del: build_regress(pedantic_inline.del, ""),
            text: build_regress(pedantic_inline.text, ""),
            punctuation: build_regress(pedantic_inline.punctuation, ""),
            _punctuation: build_regress(pedantic_inline._punctuation, ""),
            block_skip: build_regress(pedantic_inline.block_skip, ""),
            escaped_em_st: build_regress(pedantic_inline.escaped_em_st, ""),
            comment: build_regress(pedantic_inline.comment, ""),
            escapes: build_regress(pedantic_inline.escapes, ""),
            scheme: build_regress(pedantic_inline.scheme, ""),
            email: build_regress(pedantic_inline.email, ""),
            attribute: build_regress(pedantic_inline.attribute, ""),
            label: build_regress(pedantic_inline.label, ""),
            href: build_regress(pedantic_inline.href, ""),
            title: build_regress(pedantic_inline.title, ""),
            breaks: build_regress(pedantic_inline.breaks, ""),
            strong: BoldRegex {
                start: build_regress(pedantic_inline.strong.start, ""),
                middle: build_regress(pedantic_inline.strong.middle, ""),
                end_ast: build_regress(pedantic_inline.strong.end_ast, ""),
                end_und: build_regress(pedantic_inline.strong.end_und, ""),
            },
            em: BoldRegex {
                start: build_regress(pedantic_inline.em.start, ""),
                middle: build_regress(pedantic_inline.em.middle, ""),
                end_ast: build_regress(pedantic_inline.em.end_ast, ""),
                end_und: build_regress(pedantic_inline.em.end_und, ""),
            },
            extended_email: build_regress(pedantic_inline.extended_email, ""),
            backpedal: build_regress(pedantic_inline.backpedal, ""),
        },
        gfm: InlineRegexGroup {
            escape: build_regress(gfm_inline.escape, ""),
            autolink: build_regress(gfm_inline.autolink, ""),
            url: build_regress(gfm_inline.url, "i"),
            tag: build_regress(gfm_inline.tag, ""),
            link: build_regress(gfm_inline.link, ""),
            ref_link: build_regress(gfm_inline.ref_link, ""),
            no_link: build_regress(gfm_inline.no_link, ""),
            ref_link_search: build_regress(gfm_inline.ref_link_search, ""),
            em_strong: DelimRegex {
                l_delim: build_regress(gfm_inline.em_strong.l_delim, ""),
                r_delim_ast: build_regress(gfm_inline.em_strong.r_delim_ast, ""),
                r_delim_und: build_regress(gfm_inline.em_strong.r_delim_und, ""),
            },
            code: build_regress(gfm_inline.code, ""),
            br: build_regress(gfm_inline.br, ""),
            del: build_regress(gfm_inline.del, ""),
            text: build_regress(gfm_inline.text, ""),
            punctuation: build_regress(gfm_inline.punctuation, ""),
            _punctuation: build_regress(gfm_inline._punctuation, ""),
            block_skip: build_regress(gfm_inline.block_skip, ""),
            escaped_em_st: build_regress(gfm_inline.escaped_em_st, ""),
            comment: build_regress(gfm_inline.comment, ""),
            escapes: build_regress(gfm_inline.escapes, ""),
            scheme: build_regress(gfm_inline.scheme, ""),
            email: build_regress(gfm_inline.email, ""),
            attribute: build_regress(gfm_inline.attribute, ""),
            label: build_regress(gfm_inline.label, ""),
            href: build_regress(gfm_inline.href, ""),
            title: build_regress(gfm_inline.title, ""),
            breaks: build_regress(gfm_inline.breaks, ""),
            strong: BoldRegex {
                start: build_regress(gfm_inline.strong.start, ""),
                middle: build_regress(gfm_inline.strong.middle, ""),
                end_ast: build_regress(gfm_inline.strong.end_ast, ""),
                end_und: build_regress(gfm_inline.strong.end_und, ""),
            },
            em: BoldRegex {
                start: build_regress(gfm_inline.em.start, ""),
                middle: build_regress(gfm_inline.em.middle, ""),
                end_ast: build_regress(gfm_inline.em.end_ast, ""),
                end_und: build_regress(gfm_inline.em.end_und, ""),
            },
            extended_email: build_regress(gfm_inline.extended_email, ""),
            backpedal: build_regress(gfm_inline.backpedal, ""),
        },
        gfm_with_breaks: InlineRegexGroup {
            escape: build_regress(gfm_with_breaks_inline.escape, ""),
            autolink: build_regress(gfm_with_breaks_inline.autolink, ""),
            url: build_regress(gfm_with_breaks_inline.url, "i"),
            tag: build_regress(gfm_with_breaks_inline.tag, ""),
            link: build_regress(gfm_with_breaks_inline.link, ""),
            ref_link: build_regress(gfm_with_breaks_inline.ref_link, ""),
            no_link: build_regress(gfm_with_breaks_inline.no_link, ""),
            ref_link_search: build_regress(gfm_with_breaks_inline.ref_link_search, ""),
            em_strong: DelimRegex {
                l_delim: build_regress(gfm_with_breaks_inline.em_strong.l_delim, ""),
                r_delim_ast: build_regress(gfm_with_breaks_inline.em_strong.r_delim_ast, ""),
                r_delim_und: build_regress(gfm_with_breaks_inline.em_strong.r_delim_und, ""),
            },
            code: build_regress(gfm_with_breaks_inline.code, ""),
            br: build_regress(gfm_with_breaks_inline.br, ""),
            del: build_regress(gfm_with_breaks_inline.del, ""),
            text: build_regress(gfm_with_breaks_inline.text, ""),
            punctuation: build_regress(gfm_with_breaks_inline.punctuation, ""),
            _punctuation: build_regress(gfm_with_breaks_inline._punctuation, ""),
            block_skip: build_regress(gfm_with_breaks_inline.block_skip, ""),
            escaped_em_st: build_regress(gfm_with_breaks_inline.escaped_em_st, ""),
            comment: build_regress(gfm_with_breaks_inline.comment, ""),
            escapes: build_regress(gfm_with_breaks_inline.escapes, ""),
            scheme: build_regress(gfm_with_breaks_inline.scheme, ""),
            email: build_regress(gfm_with_breaks_inline.email, ""),
            attribute: build_regress(gfm_with_breaks_inline.attribute, ""),
            label: build_regress(gfm_with_breaks_inline.label, ""),
            href: build_regress(gfm_with_breaks_inline.href, ""),
            title: build_regress(gfm_with_breaks_inline.title, ""),
            breaks: build_regress(gfm_with_breaks_inline.breaks, ""),
            strong: BoldRegex {
                start: build_regress(gfm_with_breaks_inline.strong.start, ""),
                middle: build_regress(gfm_with_breaks_inline.strong.middle, ""),
                end_ast: build_regress(gfm_with_breaks_inline.strong.end_ast, ""),
                end_und: build_regress(gfm_with_breaks_inline.strong.end_und, ""),
            },
            em: BoldRegex {
                start: build_regress(gfm_with_breaks_inline.em.start, ""),
                middle: build_regress(gfm_with_breaks_inline.em.middle, ""),
                end_ast: build_regress(gfm_with_breaks_inline.em.end_ast, ""),
                end_und: build_regress(gfm_with_breaks_inline.em.end_und, ""),
            },
            extended_email: build_regress(gfm_with_breaks_inline.extended_email, ""),
            backpedal: build_regress(gfm_with_breaks_inline.backpedal, ""),
        }
    }
}



fn get_block_rules() ->  (Block, Block, Block) {

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
        table: "a^".to_string(),
        l_heading: "^([^\\n]+)\\n {0,3}(=+|-+) *(?:\\n+|$)".to_string(),
        paragraph: "^([^\\n]+(?:\\n(?!hr|lheading|heading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)".to_string(),
        text: "^[^\\n]+".to_string(),
        label: "(?!\\s*\\])(?:\\\\.|[^\\[\\]\\\\])+".to_string(),
        title: r#"(?:"(?:\\"?|[^"\\])*"|'[^'\n]*(?:\n[^'\n]+)*\n?'|\([^()]*\))"#.to_string(),
        bullet: "(?:[*+-]|\\d{1,9}[.)])".to_string(),
        list_item_start: "a^".to_string(),
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
        "a^"
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

    (normal_block, gfm_block, pedantic_block)
}

fn get_inline_rules() -> (Inline, Inline, Inline, Inline) {
    /* Inline-Level Grammar */
    let mut normal_inline = Inline {
        // Escaped the following: (| => \|) and (~ => \~)
        escape: r##"^\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{\|}\~])"##.to_string(),
        autolink: "^<(scheme:[^\\s\\x00-\\x1f<>]*|email)>".to_string(),
        url: "a^".to_string(),
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
        del: "a^".to_string(),
        text: "^(`+|[^`])(?:(?= {2,}\\n)|[\\s\\S]*?(?:(?=[\\\\<!\\[`*_]|\\b_|$)|[^ ](?= {2,}\\n)))".to_string(),
        punctuation: "^([\\spunctuation])".to_string(),
        _punctuation: r##"!"#$%&'()+\-.,/:;<=>?@\[\]`^{|}~"##.to_string(),
        block_skip: "\\[[^\\]]*?\\]\\([^\\)]*?\\)|`[^`]*?`|<[^>]*?>".to_string(),
        escaped_em_st: "\\\\\\*|\\\\_".to_string(),
        comment: "<!--(?!-?>)[\\s\\S]*?(?:-->|$)".to_string(),
        // Escaped the following: (| => \|) and (~ => \~)
        escapes: r##"\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{\|}\~])"##.to_string(),
        scheme: "[a-zA-Z][a-zA-Z0-9+.-]{1,31}".to_string(),
        email: "[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+(@)[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(?![-_])".to_string(),
        attribute: r#"\s+[a-zA-Z:_][\w.:-]*(?:\s*=\s*"[^"]*"|\s*=\s*'[^']*'|\s*=\s*[^\s"'=<>`]+)?"#.to_string(),
        label: r#"(?:\[(?:\\.|[^\[\]\\])*\]|\\.|`[^`]*`|[^\[\]\\`])*?"#.to_string(),
        href: "<(?:\\\\.|[^\\n<>\\\\])+>|[^\\s\\x00-\\x1f]*".to_string(),
        title: r#""(?:\\"?|[^"\\])*"|'(?:\\'?|[^'\\])*'|\((?:\\\)?|[^)\\])*\)"#.to_string(),
        breaks: "a^".to_string(),
        strong: Bold {
            start: "a^".to_string(),
            middle: "a^".to_string(),
            end_ast: "a^".to_string(),
            end_und: "a^".to_string()
        },
        em: Bold {
            start: "a^".to_string(),
            middle: "a^".to_string(),
            end_ast: "a^".to_string(),
            end_und: "a^".to_string()
        },
        extended_email: "a^".to_string(),
        backpedal: "a^".to_string(),
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

    // GFM Inline Grammar
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

    (normal_inline, pedantic_inline, gfm_inline, gfm_with_breaks_inline)
}

fn build_regex(pattern: String) -> fancy_regex::Regex {
    // fancy_regex::Regex::new(pattern.as_str()).unwrap()
    fancy_regex::RegexBuilder::new(pattern.as_str())
        .backtrack_limit(3000000)
        .build()
        .unwrap()
}

fn build_regress(pattern: String, flag: &str) -> regress::Regex {
    return if flag == "i" || flag == "m" {
        regress::Regex::with_flags(pattern.as_str(), flag).unwrap()
    } else {
        regress::Regex::new(pattern.as_str()).unwrap()
    }
}



fn exec_normal_block<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDBlock::Newline => { BLOCK.normal.newline.captures(src).unwrap() }
        MDBlock::Code => { BLOCK.normal.code.captures(src).unwrap() }
        MDBlock::Fences => { BLOCK.normal.fences.captures(src).unwrap() }
        MDBlock::Hr => { BLOCK.normal.hr.captures(src).unwrap() }
        MDBlock::Heading => { BLOCK.normal.heading.captures(src).unwrap() }
        MDBlock::Blockquote => { BLOCK.normal.blockquote.captures(src).unwrap() }
        MDBlock::List => { BLOCK.normal.list.captures(src).unwrap() }
        MDBlock::Html => { BLOCK.normal.html.captures(src).unwrap() }
        MDBlock::Def => { BLOCK.normal.def.captures(src).unwrap() }
        MDBlock::Table => { BLOCK.normal.table.captures(src).unwrap() }
        MDBlock::LHeading => { BLOCK.normal.l_heading.captures(src).unwrap() }
        MDBlock::Paragraph => { BLOCK.normal.paragraph.captures(src).unwrap() }
        MDBlock::Text => { BLOCK.normal.text.captures(src).unwrap() }
        MDBlock::Label => { BLOCK.normal.label.captures(src).unwrap() }
        MDBlock::Title => { BLOCK.normal.title.captures(src).unwrap() }
        MDBlock::Bullet => { BLOCK.normal.bullet.captures(src).unwrap() }
        MDBlock::ListItemStart => { BLOCK.normal.list_item_start.captures(src).unwrap() }
        MDBlock::Tag => { BLOCK.normal.tag.captures(src).unwrap() }
        MDBlock::Comment => { BLOCK.normal.comment.captures(src).unwrap() }
    }
}

fn exec_pedantic_block<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDBlock::Newline => { BLOCK.pedantic.newline.captures(src).unwrap() }
        MDBlock::Code => { BLOCK.pedantic.code.captures(src).unwrap() }
        MDBlock::Fences => { BLOCK.pedantic.fences.captures(src).unwrap() }
        MDBlock::Hr => { BLOCK.pedantic.hr.captures(src).unwrap() }
        MDBlock::Heading => { BLOCK.pedantic.heading.captures(src).unwrap() }
        MDBlock::Blockquote => { BLOCK.pedantic.blockquote.captures(src).unwrap() }
        MDBlock::List => { BLOCK.pedantic.list.captures(src).unwrap() }
        MDBlock::Html => { BLOCK.pedantic.html.captures(src).unwrap() }
        MDBlock::Def => { BLOCK.pedantic.def.captures(src).unwrap() }
        MDBlock::Table => { BLOCK.pedantic.table.captures(src).unwrap() }
        MDBlock::LHeading => { BLOCK.pedantic.l_heading.captures(src).unwrap() }
        MDBlock::Paragraph => { BLOCK.pedantic.paragraph.captures(src).unwrap() }
        MDBlock::Text => { BLOCK.pedantic.text.captures(src).unwrap() }
        MDBlock::Label => { BLOCK.pedantic.label.captures(src).unwrap() }
        MDBlock::Title => { BLOCK.pedantic.title.captures(src).unwrap() }
        MDBlock::Bullet => { BLOCK.pedantic.bullet.captures(src).unwrap() }
        MDBlock::ListItemStart => { BLOCK.pedantic.list_item_start.captures(src).unwrap() }
        MDBlock::Tag => { BLOCK.pedantic.tag.captures(src).unwrap() }
        MDBlock::Comment => { BLOCK.pedantic.comment.captures(src).unwrap() }
    }
}

fn exec_gfm_block<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDBlock::Newline => { BLOCK.gfm.newline.captures(src).unwrap() }
        MDBlock::Code => { BLOCK.gfm.code.captures(src).unwrap() }
        MDBlock::Fences => { BLOCK.gfm.fences.captures(src).unwrap() }
        MDBlock::Hr => { BLOCK.gfm.hr.captures(src).unwrap() }
        MDBlock::Heading => { BLOCK.gfm.heading.captures(src).unwrap() }
        MDBlock::Blockquote => { BLOCK.gfm.blockquote.captures(src).unwrap() }
        MDBlock::List => { BLOCK.gfm.list.captures(src).unwrap() }
        MDBlock::Html => { BLOCK.gfm.html.captures(src).unwrap() }
        MDBlock::Def => { BLOCK.gfm.def.captures(src).unwrap() }
        MDBlock::Table => { BLOCK.gfm.table.captures(src).unwrap() }
        MDBlock::LHeading => { BLOCK.gfm.l_heading.captures(src).unwrap() }
        MDBlock::Paragraph => { BLOCK.gfm.paragraph.captures(src).unwrap() }
        MDBlock::Text => { BLOCK.gfm.text.captures(src).unwrap() }
        MDBlock::Label => { BLOCK.gfm.label.captures(src).unwrap() }
        MDBlock::Title => { BLOCK.gfm.title.captures(src).unwrap() }
        MDBlock::Bullet => { BLOCK.gfm.bullet.captures(src).unwrap() }
        MDBlock::ListItemStart => { BLOCK.gfm.list_item_start.captures(src).unwrap() }
        MDBlock::Tag => { BLOCK.gfm.tag.captures(src).unwrap() }
        MDBlock::Comment => { BLOCK.gfm.comment.captures(src).unwrap() }
    }
}

fn exec_normal_inline<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDInline::Escape            => { INLINE.normal.escape.captures(src).unwrap() }
        MDInline::Autolink          => { INLINE.normal.autolink.captures(src).unwrap() }
        MDInline::Url               => { INLINE.normal.url.captures(src).unwrap() }
        MDInline::Tag               => { INLINE.normal.tag.captures(src).unwrap() }
        MDInline::Link              => { INLINE.normal.link.captures(src).unwrap() }
        MDInline::RefLink           => { INLINE.normal.ref_link.captures(src).unwrap() }
        MDInline::NoLink            => { INLINE.normal.no_link.captures(src).unwrap() }
        MDInline::RefLinkSearch     => { INLINE.normal.ref_link_search.captures(src).unwrap() }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE.normal.em_strong.l_delim.captures(src).unwrap() }
            else if opt == "r_delim_ast" { INLINE.normal.em_strong.r_delim_ast.captures(src).unwrap() }
            else { INLINE.normal.em_strong.r_delim_und.captures(src).unwrap() }
        }
        MDInline::Code              => { INLINE.normal.code.captures(src).unwrap() }
        MDInline::Br                => { INLINE.normal.br.captures(src).unwrap() }
        MDInline::Del               => { INLINE.normal.del.captures(src).unwrap() }
        MDInline::Text              => { INLINE.normal.text.captures(src).unwrap() }
        MDInline::Punctuation       => { INLINE.normal.punctuation.captures(src).unwrap() }
        MDInline::_Punctuation      => { INLINE.normal._punctuation.captures(src).unwrap() }
        MDInline::BlockSkip         => { INLINE.normal.block_skip.captures(src).unwrap() }
        MDInline::EscapedEmSt       => { INLINE.normal.escaped_em_st.captures(src).unwrap() }
        MDInline::Comment           => { INLINE.normal.comment.captures(src).unwrap() }
        MDInline::Escapes           => { INLINE.normal.escapes.captures(src).unwrap() }
        MDInline::Scheme            => { INLINE.normal.scheme.captures(src).unwrap() }
        MDInline::Email             => { INLINE.normal.email.captures(src).unwrap() }
        MDInline::Attribute         => { INLINE.normal.attribute.captures(src).unwrap() }
        MDInline::Label             => { INLINE.normal.label.captures(src).unwrap() }
        MDInline::Href              => { INLINE.normal.href.captures(src).unwrap() }
        MDInline::Title             => { INLINE.normal.title.captures(src).unwrap() }
        MDInline::Breaks            => { INLINE.normal.breaks.captures(src).unwrap() }
        MDInline::Strong            => {
            if opt == "start"           { INLINE.normal.strong.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.normal.strong.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.normal.strong.end_und.captures(src).unwrap() }
            else                        { INLINE.normal.strong.middle.captures(src).unwrap() }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE.normal.em.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.normal.em.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.normal.em.end_und.captures(src).unwrap() }
            else                        { INLINE.normal.em.middle.captures(src).unwrap() }
        }
        MDInline::ExtendedEmail     => { INLINE.normal.extended_email.captures(src).unwrap() }
        MDInline::Backpedal         => { INLINE.normal.backpedal.captures(src).unwrap() }
    }
}

fn exec_pedantic_inline<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDInline::Escape            => { INLINE.pedantic.escape.captures(src).unwrap() }
        MDInline::Autolink          => { INLINE.pedantic.autolink.captures(src).unwrap() }
        MDInline::Url               => { INLINE.pedantic.url.captures(src).unwrap() }
        MDInline::Tag               => { INLINE.pedantic.tag.captures(src).unwrap() }
        MDInline::Link              => { INLINE.pedantic.link.captures(src).unwrap() }
        MDInline::RefLink           => { INLINE.pedantic.ref_link.captures(src).unwrap() }
        MDInline::NoLink            => { INLINE.pedantic.no_link.captures(src).unwrap() }
        MDInline::RefLinkSearch     => { INLINE.pedantic.ref_link_search.captures(src).unwrap() }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE.pedantic.em_strong.l_delim.captures(src).unwrap() }
            else if opt == "r_delim_ast" { INLINE.pedantic.em_strong.r_delim_ast.captures(src).unwrap() }
            else { INLINE.pedantic.em_strong.r_delim_und.captures(src).unwrap() }
        }
        MDInline::Code              => { INLINE.pedantic.code.captures(src).unwrap() }
        MDInline::Br                => { INLINE.pedantic.br.captures(src).unwrap() }
        MDInline::Del               => { INLINE.pedantic.del.captures(src).unwrap() }
        MDInline::Text              => { INLINE.pedantic.text.captures(src).unwrap() }
        MDInline::Punctuation       => { INLINE.pedantic.punctuation.captures(src).unwrap() }
        MDInline::_Punctuation      => { INLINE.pedantic._punctuation.captures(src).unwrap() }
        MDInline::BlockSkip         => { INLINE.pedantic.block_skip.captures(src).unwrap() }
        MDInline::EscapedEmSt       => { INLINE.pedantic.escaped_em_st.captures(src).unwrap() }
        MDInline::Comment           => { INLINE.pedantic.comment.captures(src).unwrap() }
        MDInline::Escapes           => { INLINE.pedantic.escapes.captures(src).unwrap() }
        MDInline::Scheme            => { INLINE.pedantic.scheme.captures(src).unwrap() }
        MDInline::Email             => { INLINE.pedantic.email.captures(src).unwrap() }
        MDInline::Attribute         => { INLINE.pedantic.attribute.captures(src).unwrap() }
        MDInline::Label             => { INLINE.pedantic.label.captures(src).unwrap() }
        MDInline::Href              => { INLINE.pedantic.href.captures(src).unwrap() }
        MDInline::Title             => { INLINE.pedantic.title.captures(src).unwrap() }
        MDInline::Breaks            => { INLINE.pedantic.breaks.captures(src).unwrap() }
        MDInline::Strong            => {
            if opt == "start"           { INLINE.pedantic.strong.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.pedantic.strong.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.pedantic.strong.end_und.captures(src).unwrap() }
            else                        { INLINE.pedantic.strong.middle.captures(src).unwrap() }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE.pedantic.em.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.pedantic.em.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.pedantic.em.end_und.captures(src).unwrap() }
            else                        { INLINE.pedantic.em.middle.captures(src).unwrap() }
        }
        MDInline::ExtendedEmail     => { INLINE.pedantic.extended_email.captures(src).unwrap() }
        MDInline::Backpedal         => { INLINE.pedantic.backpedal.captures(src).unwrap() }
    }
}

fn exec_gfm_inline<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDInline::Escape            => { INLINE.gfm.escape.captures(src).unwrap() }
        MDInline::Autolink          => { INLINE.gfm.autolink.captures(src).unwrap() }
        MDInline::Url               => { INLINE.gfm.url.captures(src).unwrap() }
        MDInline::Tag               => { INLINE.gfm.tag.captures(src).unwrap() }
        MDInline::Link              => { INLINE.gfm.link.captures(src).unwrap() }
        MDInline::RefLink           => { INLINE.gfm.ref_link.captures(src).unwrap() }
        MDInline::NoLink            => { INLINE.gfm.no_link.captures(src).unwrap() }
        MDInline::RefLinkSearch     => { INLINE.gfm.ref_link_search.captures(src).unwrap() }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE.gfm.em_strong.l_delim.captures(src).unwrap() }
            else if opt == "r_delim_ast" { INLINE.gfm.em_strong.r_delim_ast.captures(src).unwrap() }
            else { INLINE.gfm.em_strong.r_delim_und.captures(src).unwrap() }
        }
        MDInline::Code              => { INLINE.gfm.code.captures(src).unwrap() }
        MDInline::Br                => { INLINE.gfm.br.captures(src).unwrap() }
        MDInline::Del               => { INLINE.gfm.del.captures(src).unwrap() }
        MDInline::Text              => { INLINE.gfm.text.captures(src).unwrap() }
        MDInline::Punctuation       => { INLINE.gfm.punctuation.captures(src).unwrap() }
        MDInline::_Punctuation      => { INLINE.gfm._punctuation.captures(src).unwrap() }
        MDInline::BlockSkip         => { INLINE.gfm.block_skip.captures(src).unwrap() }
        MDInline::EscapedEmSt       => { INLINE.gfm.escaped_em_st.captures(src).unwrap() }
        MDInline::Comment           => { INLINE.gfm.comment.captures(src).unwrap() }
        MDInline::Escapes           => { INLINE.gfm.escapes.captures(src).unwrap() }
        MDInline::Scheme            => { INLINE.gfm.scheme.captures(src).unwrap() }
        MDInline::Email             => { INLINE.gfm.email.captures(src).unwrap() }
        MDInline::Attribute         => { INLINE.gfm.attribute.captures(src).unwrap() }
        MDInline::Label             => { INLINE.gfm.label.captures(src).unwrap() }
        MDInline::Href              => { INLINE.gfm.href.captures(src).unwrap() }
        MDInline::Title             => { INLINE.gfm.title.captures(src).unwrap() }
        MDInline::Breaks            => { INLINE.gfm.breaks.captures(src).unwrap() }
        MDInline::Strong            => {
            if opt == "start"           { INLINE.gfm.strong.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.gfm.strong.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.gfm.strong.end_und.captures(src).unwrap() }
            else                        { INLINE.gfm.strong.middle.captures(src).unwrap() }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE.gfm.em.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.gfm.em.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.gfm.em.end_und.captures(src).unwrap() }
            else                        { INLINE.gfm.em.middle.captures(src).unwrap() }
        }
        MDInline::ExtendedEmail     => { INLINE.gfm.extended_email.captures(src).unwrap() }
        MDInline::Backpedal         => { INLINE.gfm.backpedal.captures(src).unwrap() }
    }
}

fn exec_gfm_breaks_inline<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<fancy_regex::Captures<'a>> {
    match rule {
        MDInline::Escape            => { INLINE.gfm_with_breaks.escape.captures(src).unwrap() }
        MDInline::Autolink          => { INLINE.gfm_with_breaks.autolink.captures(src).unwrap() }
        MDInline::Url               => { INLINE.gfm_with_breaks.url.captures(src).unwrap() }
        MDInline::Tag               => { INLINE.gfm_with_breaks.tag.captures(src).unwrap() }
        MDInline::Link              => { INLINE.gfm_with_breaks.link.captures(src).unwrap() }
        MDInline::RefLink           => { INLINE.gfm_with_breaks.ref_link.captures(src).unwrap() }
        MDInline::NoLink            => { INLINE.gfm_with_breaks.no_link.captures(src).unwrap() }
        MDInline::RefLinkSearch     => { INLINE.gfm_with_breaks.ref_link_search.captures(src).unwrap() }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE.gfm_with_breaks.em_strong.l_delim.captures(src).unwrap() }
            else if opt == "r_delim_ast" { INLINE.gfm_with_breaks.em_strong.r_delim_ast.captures(src).unwrap() }
            else { INLINE.gfm_with_breaks.em_strong.r_delim_und.captures(src).unwrap() }
        }
        MDInline::Code              => { INLINE.gfm_with_breaks.code.captures(src).unwrap() }
        MDInline::Br                => { INLINE.gfm_with_breaks.br.captures(src).unwrap() }
        MDInline::Del               => { INLINE.gfm_with_breaks.del.captures(src).unwrap() }
        MDInline::Text              => { INLINE.gfm_with_breaks.text.captures(src).unwrap() }
        MDInline::Punctuation       => { INLINE.gfm_with_breaks.punctuation.captures(src).unwrap() }
        MDInline::_Punctuation      => { INLINE.gfm_with_breaks._punctuation.captures(src).unwrap() }
        MDInline::BlockSkip         => { INLINE.gfm_with_breaks.block_skip.captures(src).unwrap() }
        MDInline::EscapedEmSt       => { INLINE.gfm_with_breaks.escaped_em_st.captures(src).unwrap() }
        MDInline::Comment           => { INLINE.gfm_with_breaks.comment.captures(src).unwrap() }
        MDInline::Escapes           => { INLINE.gfm_with_breaks.escapes.captures(src).unwrap() }
        MDInline::Scheme            => { INLINE.gfm_with_breaks.scheme.captures(src).unwrap() }
        MDInline::Email             => { INLINE.gfm_with_breaks.email.captures(src).unwrap() }
        MDInline::Attribute         => { INLINE.gfm_with_breaks.attribute.captures(src).unwrap() }
        MDInline::Label             => { INLINE.gfm_with_breaks.label.captures(src).unwrap() }
        MDInline::Href              => { INLINE.gfm_with_breaks.href.captures(src).unwrap() }
        MDInline::Title             => { INLINE.gfm_with_breaks.title.captures(src).unwrap() }
        MDInline::Breaks            => { INLINE.gfm_with_breaks.breaks.captures(src).unwrap() }
        MDInline::Strong            => {
            if opt == "start"           { INLINE.gfm_with_breaks.strong.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.gfm_with_breaks.strong.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.gfm_with_breaks.strong.end_und.captures(src).unwrap() }
            else                        { INLINE.gfm_with_breaks.strong.middle.captures(src).unwrap() }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE.gfm_with_breaks.em.start.captures(src).unwrap() }
            else if opt == "end_ast"    { INLINE.gfm_with_breaks.em.end_ast.captures(src).unwrap() }
            else if opt == "end_und"    { INLINE.gfm_with_breaks.em.end_und.captures(src).unwrap() }
            else                        { INLINE.gfm_with_breaks.em.middle.captures(src).unwrap() }
        }
        MDInline::ExtendedEmail     => { INLINE.gfm_with_breaks.extended_email.captures(src).unwrap() }
        MDInline::Backpedal         => { INLINE.gfm_with_breaks.backpedal.captures(src).unwrap() }
    }
}



fn get_normal_block<'a>(
    opt: &'a str,
    rule: MDBlock
) -> &'a fancy_regex::Regex {
    match rule {
        MDBlock::Newline => { &BLOCK.normal.newline }
        MDBlock::Code => { &BLOCK.normal.code }
        MDBlock::Fences => { &BLOCK.normal.fences }
        MDBlock::Hr => { &BLOCK.normal.hr }
        MDBlock::Heading => { &BLOCK.normal.heading }
        MDBlock::Blockquote => { &BLOCK.normal.blockquote }
        MDBlock::List => { &BLOCK.normal.list }
        MDBlock::Html => { &BLOCK.normal.html }
        MDBlock::Def => { &BLOCK.normal.def }
        MDBlock::Table => { &BLOCK.normal.table }
        MDBlock::LHeading => { &BLOCK.normal.l_heading }
        MDBlock::Paragraph => { &BLOCK.normal.paragraph }
        MDBlock::Text => { &BLOCK.normal.text }
        MDBlock::Label => { &BLOCK.normal.label }
        MDBlock::Title => { &BLOCK.normal.title }
        MDBlock::Bullet => { &BLOCK.normal.bullet }
        MDBlock::ListItemStart => { &BLOCK.normal.list_item_start }
        MDBlock::Tag => { &BLOCK.normal.tag }
        MDBlock::Comment => { &BLOCK.normal.comment }
    }
}

fn get_pedantic_block<'a>(
    opt: &'a str,
    rule: MDBlock
) -> &'a fancy_regex::Regex {
    match rule {
        MDBlock::Newline => { &BLOCK.pedantic.newline }
        MDBlock::Code => { &BLOCK.pedantic.code }
        MDBlock::Fences => { &BLOCK.pedantic.fences }
        MDBlock::Hr => { &BLOCK.pedantic.hr }
        MDBlock::Heading => { &BLOCK.pedantic.heading }
        MDBlock::Blockquote => { &BLOCK.pedantic.blockquote }
        MDBlock::List => { &BLOCK.pedantic.list }
        MDBlock::Html => { &BLOCK.pedantic.html }
        MDBlock::Def => { &BLOCK.pedantic.def }
        MDBlock::Table => { &BLOCK.pedantic.table }
        MDBlock::LHeading => { &BLOCK.pedantic.l_heading }
        MDBlock::Paragraph => { &BLOCK.pedantic.paragraph }
        MDBlock::Text => { &BLOCK.pedantic.text }
        MDBlock::Label => { &BLOCK.pedantic.label }
        MDBlock::Title => { &BLOCK.pedantic.title }
        MDBlock::Bullet => { &BLOCK.pedantic.bullet }
        MDBlock::ListItemStart => { &BLOCK.pedantic.list_item_start }
        MDBlock::Tag => { &BLOCK.pedantic.tag }
        MDBlock::Comment => { &BLOCK.pedantic.comment }
    }
}

fn get_gfm_block<'a>(
    opt: &'a str,
    rule: MDBlock
) -> &'a fancy_regex::Regex {
    match rule {
        MDBlock::Newline => { &BLOCK.gfm.newline }
        MDBlock::Code => { &BLOCK.gfm.code }
        MDBlock::Fences => { &BLOCK.gfm.fences }
        MDBlock::Hr => { &BLOCK.gfm.hr }
        MDBlock::Heading => { &BLOCK.gfm.heading }
        MDBlock::Blockquote => { &BLOCK.gfm.blockquote }
        MDBlock::List => { &BLOCK.gfm.list }
        MDBlock::Html => { &BLOCK.gfm.html }
        MDBlock::Def => { &BLOCK.gfm.def }
        MDBlock::Table => { &BLOCK.gfm.table }
        MDBlock::LHeading => { &BLOCK.gfm.l_heading }
        MDBlock::Paragraph => { &BLOCK.gfm.paragraph }
        MDBlock::Text => { &BLOCK.gfm.text }
        MDBlock::Label => { &BLOCK.gfm.label }
        MDBlock::Title => { &BLOCK.gfm.title }
        MDBlock::Bullet => { &BLOCK.gfm.bullet }
        MDBlock::ListItemStart => { &BLOCK.gfm.list_item_start }
        MDBlock::Tag => { &BLOCK.gfm.tag }
        MDBlock::Comment => { &BLOCK.gfm.comment }
    }
}

fn get_normal_inline<'a>(
    opt: &'a str,
    rule: MDInline
) -> &'a fancy_regex::Regex {
    match rule {
        MDInline::Escape            => { &INLINE.normal.escape }
        MDInline::Autolink          => { &INLINE.normal.autolink }
        MDInline::Url               => { &INLINE.normal.url }
        MDInline::Tag               => { &INLINE.normal.tag }
        MDInline::Link              => { &INLINE.normal.link }
        MDInline::RefLink           => { &INLINE.normal.ref_link }
        MDInline::NoLink            => { &INLINE.normal.no_link }
        MDInline::RefLinkSearch     => { &INLINE.normal.ref_link_search }
        MDInline::EmStrong          => {
            if opt == "l_delim" { &INLINE.normal.em_strong.l_delim }
            else if opt == "r_delim_ast" { &INLINE.normal.em_strong.r_delim_ast }
            else { &INLINE.normal.em_strong.r_delim_und }
        }
        MDInline::Code              => { &INLINE.normal.code }
        MDInline::Br                => { &INLINE.normal.br }
        MDInline::Del               => { &INLINE.normal.del }
        MDInline::Text              => { &INLINE.normal.text }
        MDInline::Punctuation       => { &INLINE.normal.punctuation }
        MDInline::_Punctuation      => { &INLINE.normal._punctuation }
        MDInline::BlockSkip         => { &INLINE.normal.block_skip }
        MDInline::EscapedEmSt       => { &INLINE.normal.escaped_em_st }
        MDInline::Comment           => { &INLINE.normal.comment }
        MDInline::Escapes           => { &INLINE.normal.escapes }
        MDInline::Scheme            => { &INLINE.normal.scheme }
        MDInline::Email             => { &INLINE.normal.email }
        MDInline::Attribute         => { &INLINE.normal.attribute }
        MDInline::Label             => { &INLINE.normal.label }
        MDInline::Href              => { &INLINE.normal.href }
        MDInline::Title             => { &INLINE.normal.title }
        MDInline::Breaks            => { &INLINE.normal.breaks }
        MDInline::Strong            => {
            if opt == "start"           { &INLINE.normal.strong.start }
            else if opt == "end_ast"    { &INLINE.normal.strong.end_ast }
            else if opt == "end_und"    { &INLINE.normal.strong.end_und }
            else                        { &INLINE.normal.strong.middle }
        }
        MDInline::Em                => {
            if opt == "start"           { &INLINE.normal.em.start }
            else if opt == "end_ast"    { &INLINE.normal.em.end_ast }
            else if opt == "end_und"    { &INLINE.normal.em.end_und }
            else                        { &INLINE.normal.em.middle }
        }
        MDInline::ExtendedEmail     => { &INLINE.normal.extended_email }
        MDInline::Backpedal         => { &INLINE.normal.backpedal }
    }
}

fn get_pedantic_inline<'a>(
    opt: &'a str,
    rule: MDInline
) -> &'a fancy_regex::Regex {
    match rule {
        MDInline::Escape            => { &INLINE.pedantic.escape }
        MDInline::Autolink          => { &INLINE.pedantic.autolink }
        MDInline::Url               => { &INLINE.pedantic.url }
        MDInline::Tag               => { &INLINE.pedantic.tag }
        MDInline::Link              => { &INLINE.pedantic.link }
        MDInline::RefLink           => { &INLINE.pedantic.ref_link }
        MDInline::NoLink            => { &INLINE.pedantic.no_link }
        MDInline::RefLinkSearch     => { &INLINE.pedantic.ref_link_search }
        MDInline::EmStrong          => {
            if opt == "l_delim" { &INLINE.pedantic.em_strong.l_delim }
            else if opt == "r_delim_ast" { &INLINE.pedantic.em_strong.r_delim_ast }
            else { &INLINE.pedantic.em_strong.r_delim_und }
        }
        MDInline::Code              => { &INLINE.pedantic.code }
        MDInline::Br                => { &INLINE.pedantic.br }
        MDInline::Del               => { &INLINE.pedantic.del }
        MDInline::Text              => { &INLINE.pedantic.text }
        MDInline::Punctuation       => { &INLINE.pedantic.punctuation }
        MDInline::_Punctuation      => { &INLINE.pedantic._punctuation }
        MDInline::BlockSkip         => { &INLINE.pedantic.block_skip }
        MDInline::EscapedEmSt       => { &INLINE.pedantic.escaped_em_st }
        MDInline::Comment           => { &INLINE.pedantic.comment }
        MDInline::Escapes           => { &INLINE.pedantic.escapes }
        MDInline::Scheme            => { &INLINE.pedantic.scheme }
        MDInline::Email             => { &INLINE.pedantic.email }
        MDInline::Attribute         => { &INLINE.pedantic.attribute }
        MDInline::Label             => { &INLINE.pedantic.label }
        MDInline::Href              => { &INLINE.pedantic.href }
        MDInline::Title             => { &INLINE.pedantic.title }
        MDInline::Breaks            => { &INLINE.pedantic.breaks }
        MDInline::Strong            => {
            if opt == "start"           { &INLINE.pedantic.strong.start }
            else if opt == "end_ast"    { &INLINE.pedantic.strong.end_ast }
            else if opt == "end_und"    { &INLINE.pedantic.strong.end_und }
            else                        { &INLINE.pedantic.strong.middle }
        }
        MDInline::Em                => {
            if opt == "start"           { &INLINE.pedantic.em.start }
            else if opt == "end_ast"    { &INLINE.pedantic.em.end_ast }
            else if opt == "end_und"    { &INLINE.pedantic.em.end_und }
            else                        { &INLINE.pedantic.em.middle }
        }
        MDInline::ExtendedEmail     => { &INLINE.pedantic.extended_email }
        MDInline::Backpedal         => { &INLINE.pedantic.backpedal }
    }
}

fn get_gfm_inline<'a>(
    opt: &'a str,
    rule: MDInline
) -> &'a fancy_regex::Regex {
    match rule {
        MDInline::Escape            => { &INLINE.gfm.escape }
        MDInline::Autolink          => { &INLINE.gfm.autolink }
        MDInline::Url               => { &INLINE.gfm.url }
        MDInline::Tag               => { &INLINE.gfm.tag }
        MDInline::Link              => { &INLINE.gfm.link }
        MDInline::RefLink           => { &INLINE.gfm.ref_link }
        MDInline::NoLink            => { &INLINE.gfm.no_link }
        MDInline::RefLinkSearch     => { &INLINE.gfm.ref_link_search }
        MDInline::EmStrong          => {
            if opt == "l_delim" { &INLINE.gfm.em_strong.l_delim }
            else if opt == "r_delim_ast" { &INLINE.gfm.em_strong.r_delim_ast }
            else { &INLINE.gfm.em_strong.r_delim_und }
        }
        MDInline::Code              => { &INLINE.gfm.code }
        MDInline::Br                => { &INLINE.gfm.br }
        MDInline::Del               => { &INLINE.gfm.del }
        MDInline::Text              => { &INLINE.gfm.text }
        MDInline::Punctuation       => { &INLINE.gfm.punctuation }
        MDInline::_Punctuation      => { &INLINE.gfm._punctuation }
        MDInline::BlockSkip         => { &INLINE.gfm.block_skip }
        MDInline::EscapedEmSt       => { &INLINE.gfm.escaped_em_st }
        MDInline::Comment           => { &INLINE.gfm.comment }
        MDInline::Escapes           => { &INLINE.gfm.escapes }
        MDInline::Scheme            => { &INLINE.gfm.scheme }
        MDInline::Email             => { &INLINE.gfm.email }
        MDInline::Attribute         => { &INLINE.gfm.attribute }
        MDInline::Label             => { &INLINE.gfm.label }
        MDInline::Href              => { &INLINE.gfm.href }
        MDInline::Title             => { &INLINE.gfm.title }
        MDInline::Breaks            => { &INLINE.gfm.breaks }
        MDInline::Strong            => {
            if opt == "start"           { &INLINE.gfm.strong.start }
            else if opt == "end_ast"    { &INLINE.gfm.strong.end_ast }
            else if opt == "end_und"    { &INLINE.gfm.strong.end_und }
            else                        { &INLINE.gfm.strong.middle }
        }
        MDInline::Em                => {
            if opt == "start"           { &INLINE.gfm.em.start }
            else if opt == "end_ast"    { &INLINE.gfm.em.end_ast }
            else if opt == "end_und"    { &INLINE.gfm.em.end_und }
            else                        { &INLINE.gfm.em.middle }
        }
        MDInline::ExtendedEmail     => { &INLINE.gfm.extended_email }
        MDInline::Backpedal         => { &INLINE.gfm.backpedal }
    }
}

fn get_gfm_breaks_inline<'a>(
    opt: &'a str,
    rule: MDInline
) -> &'a fancy_regex::Regex {
    match rule {
        MDInline::Escape            => { &INLINE.gfm_with_breaks.escape }
        MDInline::Autolink          => { &INLINE.gfm_with_breaks.autolink }
        MDInline::Url               => { &INLINE.gfm_with_breaks.url }
        MDInline::Tag               => { &INLINE.gfm_with_breaks.tag }
        MDInline::Link              => { &INLINE.gfm_with_breaks.link }
        MDInline::RefLink           => { &INLINE.gfm_with_breaks.ref_link }
        MDInline::NoLink            => { &INLINE.gfm_with_breaks.no_link }
        MDInline::RefLinkSearch     => { &INLINE.gfm_with_breaks.ref_link_search }
        MDInline::EmStrong          => {
            if opt == "l_delim" { &INLINE.gfm_with_breaks.em_strong.l_delim }
            else if opt == "r_delim_ast" { &INLINE.gfm_with_breaks.em_strong.r_delim_ast }
            else { &INLINE.gfm_with_breaks.em_strong.r_delim_und }
        }
        MDInline::Code              => { &INLINE.gfm_with_breaks.code }
        MDInline::Br                => { &INLINE.gfm_with_breaks.br }
        MDInline::Del               => { &INLINE.gfm_with_breaks.del }
        MDInline::Text              => { &INLINE.gfm_with_breaks.text }
        MDInline::Punctuation       => { &INLINE.gfm_with_breaks.punctuation }
        MDInline::_Punctuation      => { &INLINE.gfm_with_breaks._punctuation }
        MDInline::BlockSkip         => { &INLINE.gfm_with_breaks.block_skip }
        MDInline::EscapedEmSt       => { &INLINE.gfm_with_breaks.escaped_em_st }
        MDInline::Comment           => { &INLINE.gfm_with_breaks.comment }
        MDInline::Escapes           => { &INLINE.gfm_with_breaks.escapes }
        MDInline::Scheme            => { &INLINE.gfm_with_breaks.scheme }
        MDInline::Email             => { &INLINE.gfm_with_breaks.email }
        MDInline::Attribute         => { &INLINE.gfm_with_breaks.attribute }
        MDInline::Label             => { &INLINE.gfm_with_breaks.label }
        MDInline::Href              => { &INLINE.gfm_with_breaks.href }
        MDInline::Title             => { &INLINE.gfm_with_breaks.title }
        MDInline::Breaks            => { &INLINE.gfm_with_breaks.breaks }
        MDInline::Strong            => {
            if opt == "start"           { &INLINE.gfm_with_breaks.strong.start }
            else if opt == "end_ast"    { &INLINE.gfm_with_breaks.strong.end_ast }
            else if opt == "end_und"    { &INLINE.gfm_with_breaks.strong.end_und }
            else                        { &INLINE.gfm_with_breaks.strong.middle }
        }
        MDInline::Em                => {
            if opt == "start"           { &INLINE.gfm_with_breaks.em.start }
            else if opt == "end_ast"    { &INLINE.gfm_with_breaks.em.end_ast }
            else if opt == "end_und"    { &INLINE.gfm_with_breaks.em.end_und }
            else                        { &INLINE.gfm_with_breaks.em.middle }
        }
        MDInline::ExtendedEmail     => { &INLINE.gfm_with_breaks.extended_email }
        MDInline::Backpedal         => { &INLINE.gfm_with_breaks.backpedal }
    }
}


fn exec_normal_block_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<regress::Match> {
    match rule {
        MDBlock::Newline => { BLOCK_REGRESS.normal.newline.find(src) }
        MDBlock::Code => { BLOCK_REGRESS.normal.code.find(src) }
        MDBlock::Fences => { BLOCK_REGRESS.normal.fences.find(src) }
        MDBlock::Hr => { BLOCK_REGRESS.normal.hr.find(src) }
        MDBlock::Heading => { BLOCK_REGRESS.normal.heading.find(src) }
        MDBlock::Blockquote => { BLOCK_REGRESS.normal.blockquote.find(src) }
        MDBlock::List => { BLOCK_REGRESS.normal.list.find(src) }
        MDBlock::Html => { BLOCK_REGRESS.normal.html.find(src) }
        MDBlock::Def => { BLOCK_REGRESS.normal.def.find(src) }
        MDBlock::Table => { BLOCK_REGRESS.normal.table.find(src)}
        MDBlock::LHeading => { BLOCK_REGRESS.normal.l_heading.find(src) }
        MDBlock::Paragraph => { BLOCK_REGRESS.normal.paragraph.find(src) }
        MDBlock::Text => { BLOCK_REGRESS.normal.text.find(src) }
        MDBlock::Label => { BLOCK_REGRESS.normal.label.find(src) }
        MDBlock::Title => { BLOCK_REGRESS.normal.title.find(src) }
        MDBlock::Bullet => { BLOCK_REGRESS.normal.bullet.find(src) }
        MDBlock::ListItemStart => { BLOCK_REGRESS.normal.list_item_start.find(src) }
        MDBlock::Tag => { BLOCK_REGRESS.normal.tag.find(src) }
        MDBlock::Comment => { BLOCK_REGRESS.normal.comment.find(src) }
    }
}

fn exec_pedantic_block_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<regress::Match> {
    match rule {
        MDBlock::Newline => { BLOCK_REGRESS.pedantic.newline.find(src) }
        MDBlock::Code => { BLOCK_REGRESS.pedantic.code.find(src) }
        MDBlock::Fences => { BLOCK_REGRESS.pedantic.fences.find(src) }
        MDBlock::Hr => { BLOCK_REGRESS.pedantic.hr.find(src) }
        MDBlock::Heading => { BLOCK_REGRESS.pedantic.heading.find(src) }
        MDBlock::Blockquote => { BLOCK_REGRESS.pedantic.blockquote.find(src)}
        MDBlock::List => { BLOCK_REGRESS.pedantic.list.find(src) }
        MDBlock::Html => { BLOCK_REGRESS.pedantic.html.find(src) }
        MDBlock::Def => { BLOCK_REGRESS.pedantic.def.find(src) }
        MDBlock::Table => { BLOCK_REGRESS.pedantic.table.find(src) }
        MDBlock::LHeading => { BLOCK_REGRESS.pedantic.l_heading.find(src) }
        MDBlock::Paragraph => { BLOCK_REGRESS.pedantic.paragraph.find(src) }
        MDBlock::Text => { BLOCK_REGRESS.pedantic.text.find(src) }
        MDBlock::Label => { BLOCK_REGRESS.pedantic.label.find(src) }
        MDBlock::Title => { BLOCK_REGRESS.pedantic.title.find(src) }
        MDBlock::Bullet => { BLOCK_REGRESS.pedantic.bullet.find(src) }
        MDBlock::ListItemStart => { BLOCK_REGRESS.pedantic.list_item_start.find(src) }
        MDBlock::Tag => { BLOCK_REGRESS.pedantic.tag.find(src) }
        MDBlock::Comment => { BLOCK_REGRESS.pedantic.comment.find(src) }
    }
}

fn exec_gfm_block_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDBlock
) -> Option<regress::Match> {
    match rule {
        MDBlock::Newline => { BLOCK_REGRESS.gfm.newline.find(src) }
        MDBlock::Code => { BLOCK_REGRESS.gfm.code.find(src) }
        MDBlock::Fences => { BLOCK_REGRESS.gfm.fences.find(src) }
        MDBlock::Hr => { BLOCK_REGRESS.gfm.hr.find(src) }
        MDBlock::Heading => { BLOCK_REGRESS.gfm.heading.find(src)}
        MDBlock::Blockquote => { BLOCK_REGRESS.gfm.blockquote.find(src) }
        MDBlock::List => { BLOCK_REGRESS.gfm.list.find(src) }
        MDBlock::Html => { BLOCK_REGRESS.gfm.html.find(src) }
        MDBlock::Def => { BLOCK_REGRESS.gfm.def.find(src) }
        MDBlock::Table => { BLOCK_REGRESS.gfm.table.find(src) }
        MDBlock::LHeading => { BLOCK_REGRESS.gfm.l_heading.find(src) }
        MDBlock::Paragraph => { BLOCK_REGRESS.gfm.paragraph.find(src) }
        MDBlock::Text => { BLOCK_REGRESS.gfm.text.find(src) }
        MDBlock::Label => { BLOCK_REGRESS.gfm.label.find(src)}
        MDBlock::Title => { BLOCK_REGRESS.gfm.title.find(src) }
        MDBlock::Bullet => { BLOCK_REGRESS.gfm.bullet.find(src) }
        MDBlock::ListItemStart => { BLOCK_REGRESS.gfm.list_item_start.find(src) }
        MDBlock::Tag => { BLOCK_REGRESS.gfm.tag.find(src) }
        MDBlock::Comment => { BLOCK_REGRESS.gfm.comment.find(src) }
    }
}

fn exec_normal_inline_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<regress::Match> {
    match rule {
        MDInline::Escape            => { INLINE_REGRESS.normal.escape.find(src)}
        MDInline::Autolink          => { INLINE_REGRESS.normal.autolink.find(src) }
        MDInline::Url               => { INLINE_REGRESS.normal.url.find(src) }
        MDInline::Tag               => { INLINE_REGRESS.normal.tag.find(src) }
        MDInline::Link              => { INLINE_REGRESS.normal.link.find(src)}
        MDInline::RefLink           => { INLINE_REGRESS.normal.ref_link.find(src) }
        MDInline::NoLink            => { INLINE_REGRESS.normal.no_link.find(src) }
        MDInline::RefLinkSearch     => { INLINE_REGRESS.normal.ref_link_search.find(src) }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE_REGRESS.normal.em_strong.l_delim.find(src) }
            else if opt == "r_delim_ast" { INLINE_REGRESS.normal.em_strong.r_delim_ast.find(src) }
            else { INLINE_REGRESS.normal.em_strong.r_delim_und.find(src) }
        }
        MDInline::Code              => { INLINE_REGRESS.normal.code.find(src) }
        MDInline::Br                => { INLINE_REGRESS.normal.br.find(src) }
        MDInline::Del               => { INLINE_REGRESS.normal.del.find(src) }
        MDInline::Text              => { INLINE_REGRESS.normal.text.find(src) }
        MDInline::Punctuation       => { INLINE_REGRESS.normal.punctuation.find(src) }
        MDInline::_Punctuation      => { INLINE_REGRESS.normal._punctuation.find(src) }
        MDInline::BlockSkip         => { INLINE_REGRESS.normal.block_skip.find(src) }
        MDInline::EscapedEmSt       => { INLINE_REGRESS.normal.escaped_em_st.find(src) }
        MDInline::Comment           => { INLINE_REGRESS.normal.comment.find(src) }
        MDInline::Escapes           => { INLINE_REGRESS.normal.escapes.find(src) }
        MDInline::Scheme            => { INLINE_REGRESS.normal.scheme.find(src) }
        MDInline::Email             => { INLINE_REGRESS.normal.email.find(src) }
        MDInline::Attribute         => { INLINE_REGRESS.normal.attribute.find(src) }
        MDInline::Label             => { INLINE_REGRESS.normal.label.find(src) }
        MDInline::Href              => { INLINE_REGRESS.normal.href.find(src) }
        MDInline::Title             => { INLINE_REGRESS.normal.title.find(src) }
        MDInline::Breaks            => { INLINE_REGRESS.normal.breaks.find(src) }
        MDInline::Strong            => {
            if opt == "start"           { INLINE_REGRESS.normal.strong.start.find(src) }
            else if opt == "end_ast"    { INLINE_REGRESS.normal.strong.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.normal.strong.end_und.find(src)}
            else                        { INLINE_REGRESS.normal.strong.middle.find(src) }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE_REGRESS.normal.em.start.find(src)}
            else if opt == "end_ast"    { INLINE_REGRESS.normal.em.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.normal.em.end_und.find(src) }
            else                        { INLINE_REGRESS.normal.em.middle.find(src) }
        }
        MDInline::ExtendedEmail     => { INLINE_REGRESS.normal.extended_email.find(src) }
        MDInline::Backpedal         => { INLINE_REGRESS.normal.backpedal.find(src) }
    }
}

fn exec_pedantic_inline_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<regress::Match> {
    match rule {
        MDInline::Escape            => { INLINE_REGRESS.pedantic.escape.find(src)}
        MDInline::Autolink          => { INLINE_REGRESS.pedantic.autolink.find(src) }
        MDInline::Url               => { INLINE_REGRESS.pedantic.url.find(src) }
        MDInline::Tag               => { INLINE_REGRESS.pedantic.tag.find(src) }
        MDInline::Link              => { INLINE_REGRESS.pedantic.link.find(src)}
        MDInline::RefLink           => { INLINE_REGRESS.pedantic.ref_link.find(src) }
        MDInline::NoLink            => { INLINE_REGRESS.pedantic.no_link.find(src) }
        MDInline::RefLinkSearch     => { INLINE_REGRESS.pedantic.ref_link_search.find(src) }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE_REGRESS.pedantic.em_strong.l_delim.find(src)}
            else if opt == "r_delim_ast" { INLINE_REGRESS.pedantic.em_strong.r_delim_ast.find(src) }
            else { INLINE_REGRESS.pedantic.em_strong.r_delim_und.find(src) }
        }
        MDInline::Code              => { INLINE_REGRESS.pedantic.code.find(src) }
        MDInline::Br                => { INLINE_REGRESS.pedantic.br.find(src) }
        MDInline::Del               => { INLINE_REGRESS.pedantic.del.find(src) }
        MDInline::Text              => { INLINE_REGRESS.pedantic.text.find(src) }
        MDInline::Punctuation       => { INLINE_REGRESS.pedantic.punctuation.find(src) }
        MDInline::_Punctuation      => { INLINE_REGRESS.pedantic._punctuation.find(src) }
        MDInline::BlockSkip         => { INLINE_REGRESS.pedantic.block_skip.find(src) }
        MDInline::EscapedEmSt       => { INLINE_REGRESS.pedantic.escaped_em_st.find(src) }
        MDInline::Comment           => { INLINE_REGRESS.pedantic.comment.find(src) }
        MDInline::Escapes           => { INLINE_REGRESS.pedantic.escapes.find(src) }
        MDInline::Scheme            => { INLINE_REGRESS.pedantic.scheme.find(src) }
        MDInline::Email             => { INLINE_REGRESS.pedantic.email.find(src) }
        MDInline::Attribute         => { INLINE_REGRESS.pedantic.attribute.find(src) }
        MDInline::Label             => { INLINE_REGRESS.pedantic.label.find(src) }
        MDInline::Href              => { INLINE_REGRESS.pedantic.href.find(src) }
        MDInline::Title             => { INLINE_REGRESS.pedantic.title.find(src) }
        MDInline::Breaks            => { INLINE_REGRESS.pedantic.breaks.find(src) }
        MDInline::Strong            => {
            if opt == "start"           { INLINE_REGRESS.pedantic.strong.start.find(src) }
            else if opt == "end_ast"    { INLINE_REGRESS.pedantic.strong.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.pedantic.strong.end_und.find(src)}
            else                        { INLINE_REGRESS.pedantic.strong.middle.find(src) }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE_REGRESS.pedantic.em.start.find(src)}
            else if opt == "end_ast"    { INLINE_REGRESS.pedantic.em.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.pedantic.em.end_und.find(src) }
            else                        { INLINE_REGRESS.pedantic.em.middle.find(src) }
        }
        MDInline::ExtendedEmail     => { INLINE_REGRESS.pedantic.extended_email.find(src) }
        MDInline::Backpedal         => { INLINE_REGRESS.pedantic.backpedal.find(src) }
    }
}

fn exec_gfm_inline_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<regress::Match> {
    match rule {
        MDInline::Escape            => { INLINE_REGRESS.gfm.escape.find(src)}
        MDInline::Autolink          => { INLINE_REGRESS.gfm.autolink.find(src) }
        MDInline::Url               => { INLINE_REGRESS.gfm.url.find(src) }
        MDInline::Tag               => { INLINE_REGRESS.gfm.tag.find(src) }
        MDInline::Link              => { INLINE_REGRESS.gfm.link.find(src)}
        MDInline::RefLink           => { INLINE_REGRESS.gfm.ref_link.find(src) }
        MDInline::NoLink            => { INLINE_REGRESS.gfm.no_link.find(src) }
        MDInline::RefLinkSearch     => { INLINE_REGRESS.gfm.ref_link_search.find(src) }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE_REGRESS.gfm.em_strong.l_delim.find(src) }
            else if opt == "r_delim_ast" { INLINE_REGRESS.gfm.em_strong.r_delim_ast.find(src) }
            else { INLINE_REGRESS.gfm.em_strong.r_delim_und.find(src) }
        }
        MDInline::Code              => { INLINE_REGRESS.gfm.code.find(src) }
        MDInline::Br                => { INLINE_REGRESS.gfm.br.find(src) }
        MDInline::Del               => { INLINE_REGRESS.gfm.del.find(src) }
        MDInline::Text              => { INLINE_REGRESS.gfm.text.find(src) }
        MDInline::Punctuation       => { INLINE_REGRESS.gfm.punctuation.find(src) }
        MDInline::_Punctuation      => { INLINE_REGRESS.gfm._punctuation.find(src) }
        MDInline::BlockSkip         => { INLINE_REGRESS.gfm.block_skip.find(src) }
        MDInline::EscapedEmSt       => { INLINE_REGRESS.gfm.escaped_em_st.find(src) }
        MDInline::Comment           => { INLINE_REGRESS.gfm.comment.find(src) }
        MDInline::Escapes           => { INLINE_REGRESS.gfm.escapes.find(src) }
        MDInline::Scheme            => { INLINE_REGRESS.gfm.scheme.find(src) }
        MDInline::Email             => { INLINE_REGRESS.gfm.email.find(src) }
        MDInline::Attribute         => { INLINE_REGRESS.gfm.attribute.find(src) }
        MDInline::Label             => { INLINE_REGRESS.gfm.label.find(src) }
        MDInline::Href              => { INLINE_REGRESS.gfm.href.find(src) }
        MDInline::Title             => { INLINE_REGRESS.gfm.title.find(src) }
        MDInline::Breaks            => { INLINE_REGRESS.gfm.breaks.find(src) }
        MDInline::Strong            => {
            if opt == "start"           { INLINE_REGRESS.gfm.strong.start.find(src) }
            else if opt == "end_ast"    { INLINE_REGRESS.gfm.strong.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.gfm.strong.end_und.find(src)}
            else                        { INLINE_REGRESS.gfm.strong.middle.find(src) }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE_REGRESS.gfm.em.start.find(src)}
            else if opt == "end_ast"    { INLINE_REGRESS.gfm.em.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.gfm.em.end_und.find(src) }
            else                        { INLINE_REGRESS.gfm.em.middle.find(src) }
        }
        MDInline::ExtendedEmail     => { INLINE_REGRESS.gfm.extended_email.find(src) }
        MDInline::Backpedal         => { INLINE_REGRESS.gfm.backpedal.find(src) }
    }
}

fn exec_gfm_breaks_inline_regress<'a>(
    src: &'a str,
    opt: &'a str,
    rule: MDInline
) -> Option<regress::Match> {
    match rule {
        MDInline::Escape            => { INLINE_REGRESS.gfm_with_breaks.escape.find(src)}
        MDInline::Autolink          => { INLINE_REGRESS.gfm_with_breaks.autolink.find(src) }
        MDInline::Url               => { INLINE_REGRESS.gfm_with_breaks.url.find(src) }
        MDInline::Tag               => { INLINE_REGRESS.gfm_with_breaks.tag.find(src) }
        MDInline::Link              => { INLINE_REGRESS.gfm_with_breaks.link.find(src)}
        MDInline::RefLink           => { INLINE_REGRESS.gfm_with_breaks.ref_link.find(src) }
        MDInline::NoLink            => { INLINE_REGRESS.gfm_with_breaks.no_link.find(src) }
        MDInline::RefLinkSearch     => { INLINE_REGRESS.gfm_with_breaks.ref_link_search.find(src) }
        MDInline::EmStrong          => {
            if opt == "l_delim" { INLINE_REGRESS.gfm_with_breaks.em_strong.l_delim.find(src)}
            else if opt == "r_delim_ast" { INLINE_REGRESS.gfm_with_breaks.em_strong.r_delim_ast.find(src) }
            else { INLINE_REGRESS.gfm_with_breaks.em_strong.r_delim_und.find(src) }
        }
        MDInline::Code              => { INLINE_REGRESS.gfm_with_breaks.code.find(src) }
        MDInline::Br                => { INLINE_REGRESS.gfm_with_breaks.br.find(src) }
        MDInline::Del               => { INLINE_REGRESS.gfm_with_breaks.del.find(src) }
        MDInline::Text              => { INLINE_REGRESS.gfm_with_breaks.text.find(src) }
        MDInline::Punctuation       => { INLINE_REGRESS.gfm_with_breaks.punctuation.find(src) }
        MDInline::_Punctuation      => { INLINE_REGRESS.gfm_with_breaks._punctuation.find(src) }
        MDInline::BlockSkip         => { INLINE_REGRESS.gfm_with_breaks.block_skip.find(src) }
        MDInline::EscapedEmSt       => { INLINE_REGRESS.gfm_with_breaks.escaped_em_st.find(src) }
        MDInline::Comment           => { INLINE_REGRESS.gfm_with_breaks.comment.find(src) }
        MDInline::Escapes           => { INLINE_REGRESS.gfm_with_breaks.escapes.find(src) }
        MDInline::Scheme            => { INLINE_REGRESS.gfm_with_breaks.scheme.find(src) }
        MDInline::Email             => { INLINE_REGRESS.gfm_with_breaks.email.find(src) }
        MDInline::Attribute         => { INLINE_REGRESS.gfm_with_breaks.attribute.find(src) }
        MDInline::Label             => { INLINE_REGRESS.gfm_with_breaks.label.find(src) }
        MDInline::Href              => { INLINE_REGRESS.gfm_with_breaks.href.find(src) }
        MDInline::Title             => { INLINE_REGRESS.gfm_with_breaks.title.find(src) }
        MDInline::Breaks            => { INLINE_REGRESS.gfm_with_breaks.breaks.find(src) }
        MDInline::Strong            => {
            if opt == "start"           { INLINE_REGRESS.gfm_with_breaks.strong.start.find(src) }
            else if opt == "end_ast"    { INLINE_REGRESS.gfm_with_breaks.strong.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.gfm_with_breaks.strong.end_und.find(src)}
            else                        { INLINE_REGRESS.gfm_with_breaks.strong.middle.find(src) }
        }
        MDInline::Em                => {
            if opt == "start"           { INLINE_REGRESS.gfm_with_breaks.em.start.find(src)}
            else if opt == "end_ast"    { INLINE_REGRESS.gfm_with_breaks.em.end_ast.find(src) }
            else if opt == "end_und"    { INLINE_REGRESS.gfm_with_breaks.em.end_und.find(src) }
            else                        { INLINE_REGRESS.gfm_with_breaks.em.middle.find(src) }
        }
        MDInline::ExtendedEmail     => { INLINE_REGRESS.gfm_with_breaks.extended_email.find(src) }
        MDInline::Backpedal         => { INLINE_REGRESS.gfm_with_breaks.backpedal.find(src) }
    }
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

pub enum RegexGlobalOpt {
    CaseInsensitive,
    MultiLine
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

    pub fn get_grammar_regex(&self, rule: MDBlock, _opt: Option<&str>) -> regex::Regex {
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
        // fancy_regex::RegexBuilder::new(src).backtrack_limit(100000000).build().unwrap()
        self.get_grammar_regex(rule, opt).captures(src)
    }


    pub fn get_grammar_fc_regex(&self, rule: MDBlock, _opt: Option<RegexGlobalOpt>) -> fancy_regex::Regex {
        match rule {
            MDBlock::Newline        => fancy_regex::Regex::new(self.newline.as_str()).unwrap(),
            MDBlock::Code           => fancy_regex::Regex::new(self.code.as_str()).unwrap(),
            MDBlock::Fences         => fancy_regex::Regex::new(self.fences.as_str()).unwrap(),
            MDBlock::Hr             => fancy_regex::Regex::new(self.hr.as_str()).unwrap(),
            MDBlock::Heading        => fancy_regex::Regex::new(self.heading.as_str()).unwrap(),
            MDBlock::Blockquote     => fancy_regex::Regex::new(self.blockquote.as_str()).unwrap(),
            MDBlock::List           => fancy_regex::Regex::new(self.list.as_str()).unwrap(),
            MDBlock::Html           => {
                let html = format!("{}{}", "(?i)", self.html);
                fancy_regex::Regex::new(html.as_str()).unwrap()
                // fancy_regex::Regex::new(self.html.as_str()).unwrap()
            },
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

    pub fn exec_fc<'a>(&self, src: &'a str, rule: MDBlock, opt: Option<RegexGlobalOpt>) -> Option<fancy_regex::Captures<'a>> {
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
        table: "a^".to_string(),
        l_heading: "^([^\\n]+)\\n {0,3}(=+|-+) *(?:\\n+|$)".to_string(),
        paragraph: "^([^\\n]+(?:\\n(?!hr|lheading|heading|blockquote|fences|list|html|table| +\\n)[^\\n]+)*)".to_string(),
        text: "^[^\\n]+".to_string(),
        label: "(?!\\s*\\])(?:\\\\.|[^\\[\\]\\\\])+".to_string(),
        title: r#"(?:"(?:\\"?|[^"\\])*"|'[^'\n]*(?:\n[^'\n]+)*\n?'|\([^()]*\))"#.to_string(),
        bullet: "(?:[*+-]|\\d{1,9}[.)])".to_string(),
        list_item_start: "a^".to_string(),
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
        // Escaped the following: (| => \|) and (~ => \~)
        escape: r##"^\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{\|}\~])"##.to_string(),
        autolink: "^<(scheme:[^\\s\\x00-\\x1f<>]*|email)>".to_string(),
        url: "a^".to_string(),
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
        del: "a^".to_string(),
        text: "^(`+|[^`])(?:(?= {2,}\\n)|[\\s\\S]*?(?:(?=[\\\\<!\\[`*_]|\\b_|$)|[^ ](?= {2,}\\n)))".to_string(),
        punctuation: "^([\\spunctuation])".to_string(),
        _punctuation: r##"!"#$%&'()+\-.,/:;<=>?@\[\]`^{|}~"##.to_string(),
        block_skip: "\\[[^\\]]*?\\]\\([^\\)]*?\\)|`[^`]*?`|<[^>]*?>".to_string(),
        escaped_em_st: "\\\\\\*|\\\\_".to_string(),
        comment: "<!--(?!-?>)[\\s\\S]*?(?:-->|$)".to_string(),
        // Escaped the following: (| => \|) and (~ => \~)
        escapes: r##"\\([!"#$%&'()*+,\-./:;<=>?@\[\]\\^_`{\|}\~])"##.to_string(),
        scheme: "[a-zA-Z][a-zA-Z0-9+.-]{1,31}".to_string(),
        email: "[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+(@)[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)+(?![-_])".to_string(),
        attribute: r#"\s+[a-zA-Z:_][\w.:-]*(?:\s*=\s*"[^"]*"|\s*=\s*'[^']*'|\s*=\s*[^\s"'=<>`]+)?"#.to_string(),
        label: r#"(?:\[(?:\\.|[^\[\]\\])*\]|\\.|`[^`]*`|[^\[\]\\`])*?"#.to_string(),
        href: "<(?:\\\\.|[^\\n<>\\\\])+>|[^\\s\\x00-\\x1f]*".to_string(),
        title: r#""(?:\\"?|[^"\\])*"|'(?:\\'?|[^'\\])*'|\((?:\\\)?|[^)\\])*\)"#.to_string(),
        breaks: "a^".to_string(),
        strong: Bold {
            start: "a^".to_string(),
            middle: "a^".to_string(),
            end_ast: "a^".to_string(),
            end_und: "a^".to_string()
        },
        em: Bold {
            start: "a^".to_string(),
            middle: "a^".to_string(),
            end_ast: "a^".to_string(),
            end_und: "a^".to_string()
        },
        extended_email: "a^".to_string(),
        backpedal: "a^".to_string(),
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

    // GFM Inline Grammar
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
            text: block.text.to_string(),
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
            text: block.text.to_string(),
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

pub fn test() {}


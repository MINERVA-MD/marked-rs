#![allow(warnings, unused)]
use marked_rs::rules::test;

use std::fs;
use std::env;
use seq_macro::seq;
use std::path::Path;
use serde_json::Result;
use test_case::test_case;
use serde::{Serialize, Deserialize};
use marked_rs::rules::{Block, MDBlock};

#[cfg(test)]
mod tests {
    use marked_rs::lexer::mangle;
    use marked_rs::rules::{Inline, setup_block_rules, setup_inline_rules};
    use super::*;

    fn deserialize_specs(path: &str)-> String {
        let data: String = fs::read_to_string(path)
            .expect("Unable to read file");
        return data;
    }

    fn get_block_specs(path: &str) -> Block {
        let data: String = deserialize_specs(path);
        let spec: Block = serde_json::from_str(&data).unwrap();
        return spec;
    }

    fn get_inline_specs(path: &str) -> Inline {
        let data: String = deserialize_specs(path);
        let spec: Inline = serde_json::from_str(&data).unwrap();
        return spec;
    }

    #[test]
    fn validate_normal_block_rules() {
        test();

        let block = &setup_block_rules()[0];
        let specs = get_block_specs("tests/fixtures/marked-regex/block/normal-block.json");

        assert_eq!(block.newline.as_str(), specs.newline.as_str());
        assert_eq!(block.code.as_str(), specs.code.as_str());
        assert_eq!(block.fences.as_str(), specs.fences.as_str());
        assert_eq!(block.hr.as_str(), specs.hr.as_str());
        assert_eq!(block.heading.as_str(), specs.heading.as_str());
        assert_eq!(block.blockquote.as_str(), specs.blockquote.as_str());
        assert_eq!(block.list.as_str(), specs.list.as_str());
        assert_eq!(block.html.as_str(), specs.html.as_str());
        assert_eq!(block.def.as_str(), specs.def.as_str());
        assert_eq!(block.table.as_str(), specs.table.as_str());
        assert_eq!(block.l_heading.as_str(), specs.l_heading.as_str());
        assert_eq!(block.paragraph.as_str(), specs.paragraph.as_str());
        assert_eq!(block.text.as_str(), specs.text.as_str());
        assert_eq!(block.label.as_str(), specs.label.as_str());
        assert_eq!(block.title.as_str(), specs.title.as_str());
        assert_eq!(block.bullet.as_str(), specs.bullet.as_str());
        assert_eq!(block.list_item_start.as_str(), specs.list_item_start.as_str());
        assert_eq!(block.tag.as_str(), specs.tag.as_str());
        assert_eq!(block.comment.as_str(), specs.comment.as_str());
    }

    #[test]
    fn validate_gfm_block_rules() {
        let block = &setup_block_rules()[1];
        let specs = get_block_specs("tests/fixtures/marked-regex/block/gfm-block.json");

        assert_eq!(block.newline.as_str(), specs.newline.as_str());
        assert_eq!(block.code.as_str(), specs.code.as_str());
        assert_eq!(block.fences.as_str(), specs.fences.as_str());
        assert_eq!(block.hr.as_str(), specs.hr.as_str());
        assert_eq!(block.heading.as_str(), specs.heading.as_str());
        assert_eq!(block.blockquote.as_str(), specs.blockquote.as_str());
        assert_eq!(block.list.as_str(), specs.list.as_str());
        assert_eq!(block.html.as_str(), specs.html.as_str());
        assert_eq!(block.def.as_str(), specs.def.as_str());
        assert_eq!(block.table.as_str(), specs.table.as_str());
        assert_eq!(block.l_heading.as_str(), specs.l_heading.as_str());
        assert_eq!(block.paragraph.as_str(), specs.paragraph.as_str());
        assert_eq!(block.text.as_str(), specs.text.as_str());
        assert_eq!(block.label.as_str(), specs.label.as_str());
        assert_eq!(block.title.as_str(), specs.title.as_str());
        assert_eq!(block.bullet.as_str(), specs.bullet.as_str());
        assert_eq!(block.list_item_start.as_str(), specs.list_item_start.as_str());
        assert_eq!(block.tag.as_str(), specs.tag.as_str());
        assert_eq!(block.comment.as_str(), specs.comment.as_str());
    }

    #[test]
    fn validate_pedantic_block_rules() {
        let block = &setup_block_rules()[2];
        let specs = get_block_specs("tests/fixtures/marked-regex/block/pedantic-block.json");

        assert_eq!(block.newline.as_str(), specs.newline.as_str());
        assert_eq!(block.code.as_str(), specs.code.as_str());
        assert_eq!(block.fences.as_str(), specs.fences.as_str());
        assert_eq!(block.hr.as_str(), specs.hr.as_str());
        assert_eq!(block.heading.as_str(), specs.heading.as_str());
        assert_eq!(block.blockquote.as_str(), specs.blockquote.as_str());
        assert_eq!(block.list.as_str(), specs.list.as_str());
        assert_eq!(block.html.as_str(), specs.html.as_str());
        assert_eq!(block.def.as_str(), specs.def.as_str());
        assert_eq!(block.table.as_str(), specs.table.as_str());
        assert_eq!(block.l_heading.as_str(), specs.l_heading.as_str());
        assert_eq!(block.paragraph.as_str(), specs.paragraph.as_str());
        assert_eq!(block.text.as_str(), specs.text.as_str());
        assert_eq!(block.label.as_str(), specs.label.as_str());
        assert_eq!(block.title.as_str(), specs.title.as_str());
        assert_eq!(block.bullet.as_str(), specs.bullet.as_str());
        assert_eq!(block.list_item_start.as_str(), specs.list_item_start.as_str());
        assert_eq!(block.tag.as_str(), specs.tag.as_str());
        assert_eq!(block.comment.as_str(), specs.comment.as_str());
    }

    #[test]
    fn validate_normal_inline_rules() {
        let inline = &setup_inline_rules()[0];
        let specs = get_inline_specs("tests/fixtures/marked-regex/inline/normal-inline.json");

        assert_eq!(inline.escape.as_str(), specs.escape.as_str());
        assert_eq!(inline.autolink.as_str(), specs.autolink.as_str());
        assert_eq!(inline.url.as_str(), specs.url.as_str());
        assert_eq!(inline.tag.as_str(), specs.tag.as_str());
        assert_eq!(inline.link.as_str(), specs.link.as_str());
        assert_eq!(inline.ref_link.as_str(), specs.ref_link.as_str());
        assert_eq!(inline.no_link.as_str(), specs.no_link.as_str());
        assert_eq!(inline.ref_link_search.as_str(), specs.ref_link_search.as_str());
        assert_eq!(inline.em_strong.l_delim.as_str(), specs.em_strong.l_delim.as_str());
        assert_eq!(inline.em_strong.r_delim_ast.as_str(), specs.em_strong.r_delim_ast.as_str());
        assert_eq!(inline.em_strong.r_delim_und.as_str(), specs.em_strong.r_delim_und.as_str());
        assert_eq!(inline.code.as_str(), specs.code.as_str());
        assert_eq!(inline.br.as_str(), specs.br.as_str());
        assert_eq!(inline.del.as_str(), specs.del.as_str());
        assert_eq!(inline.text.as_str(), specs.text.as_str());
        assert_eq!(inline.punctuation.as_str(), specs.punctuation.as_str());
        assert_eq!(inline._punctuation.as_str(), specs._punctuation.as_str());
        assert_eq!(inline.block_skip.as_str(), specs.block_skip.as_str());
        assert_eq!(inline.escaped_em_st.as_str(), specs.escaped_em_st.as_str());
        assert_eq!(inline.comment.as_str(), specs.comment.as_str());
        assert_eq!(inline.escapes.as_str(), specs.escapes.as_str());
        assert_eq!(inline.scheme.as_str(), specs.scheme.as_str());
        assert_eq!(inline.email.as_str(), specs.email.as_str());
        assert_eq!(inline.attribute.as_str(), specs.attribute.as_str());
        assert_eq!(inline.label.as_str(), specs.label.as_str());
        assert_eq!(inline.href.as_str(), specs.href.as_str());
        assert_eq!(inline.title.as_str(), specs.title.as_str());
        assert_eq!(inline.breaks.as_str(), specs.breaks.as_str());
        assert_eq!(inline.strong.start.as_str(), specs.strong.start.as_str());
        assert_eq!(inline.strong.middle.as_str(), specs.strong.middle.as_str());
        assert_eq!(inline.strong.end_ast.as_str(), specs.strong.end_ast.as_str());
        assert_eq!(inline.strong.end_und.as_str(), specs.strong.end_und.as_str());
        assert_eq!(inline.em.start.as_str(), specs.em.start.as_str());
        assert_eq!(inline.em.middle.as_str(), specs.em.middle.as_str());
        assert_eq!(inline.em.end_ast.as_str(), specs.em.end_ast.as_str());
        assert_eq!(inline.em.end_und.as_str(), specs.em.end_und.as_str());
        assert_eq!(inline.extended_email.as_str(), specs.extended_email.as_str());
        assert_eq!(inline.backpedal.as_str(), specs.backpedal.as_str());
    }

    #[test]
    fn validate_pedantic_inline_rules() {
        let inline = &setup_inline_rules()[1];
        let specs = get_inline_specs("tests/fixtures/marked-regex/inline/pedantic-inline.json");

        assert_eq!(inline.escape.as_str(), specs.escape.as_str());
        assert_eq!(inline.autolink.as_str(), specs.autolink.as_str());
        assert_eq!(inline.url.as_str(), specs.url.as_str());
        assert_eq!(inline.tag.as_str(), specs.tag.as_str());
        assert_eq!(inline.link.as_str(), specs.link.as_str());
        assert_eq!(inline.ref_link.as_str(), specs.ref_link.as_str());
        assert_eq!(inline.no_link.as_str(), specs.no_link.as_str());
        assert_eq!(inline.ref_link_search.as_str(), specs.ref_link_search.as_str());
        assert_eq!(inline.em_strong.l_delim.as_str(), specs.em_strong.l_delim.as_str());
        assert_eq!(inline.em_strong.r_delim_ast.as_str(), specs.em_strong.r_delim_ast.as_str());
        assert_eq!(inline.em_strong.r_delim_und.as_str(), specs.em_strong.r_delim_und.as_str());
        assert_eq!(inline.code.as_str(), specs.code.as_str());
        assert_eq!(inline.br.as_str(), specs.br.as_str());
        assert_eq!(inline.del.as_str(), specs.del.as_str());
        assert_eq!(inline.text.as_str(), specs.text.as_str());
        assert_eq!(inline.punctuation.as_str(), specs.punctuation.as_str());
        assert_eq!(inline._punctuation.as_str(), specs._punctuation.as_str());
        assert_eq!(inline.block_skip.as_str(), specs.block_skip.as_str());
        assert_eq!(inline.escaped_em_st.as_str(), specs.escaped_em_st.as_str());
        assert_eq!(inline.comment.as_str(), specs.comment.as_str());
        assert_eq!(inline.escapes.as_str(), specs.escapes.as_str());
        assert_eq!(inline.scheme.as_str(), specs.scheme.as_str());
        assert_eq!(inline.email.as_str(), specs.email.as_str());
        assert_eq!(inline.attribute.as_str(), specs.attribute.as_str());
        assert_eq!(inline.label.as_str(), specs.label.as_str());
        assert_eq!(inline.href.as_str(), specs.href.as_str());
        assert_eq!(inline.title.as_str(), specs.title.as_str());
        assert_eq!(inline.breaks.as_str(), specs.breaks.as_str());
        assert_eq!(inline.strong.start.as_str(), specs.strong.start.as_str());
        assert_eq!(inline.strong.middle.as_str(), specs.strong.middle.as_str());
        assert_eq!(inline.strong.end_ast.as_str(), specs.strong.end_ast.as_str());
        assert_eq!(inline.strong.end_und.as_str(), specs.strong.end_und.as_str());
        assert_eq!(inline.em.start.as_str(), specs.em.start.as_str());
        assert_eq!(inline.em.middle.as_str(), specs.em.middle.as_str());
        assert_eq!(inline.em.end_ast.as_str(), specs.em.end_ast.as_str());
        assert_eq!(inline.em.end_und.as_str(), specs.em.end_und.as_str());
        assert_eq!(inline.extended_email.as_str(), specs.extended_email.as_str());
        assert_eq!(inline.backpedal.as_str(), specs.backpedal.as_str());
    }

    #[test]
    fn validate_gfm_inline_rules() {
        let inline = &setup_inline_rules()[2];
        let specs = get_inline_specs("tests/fixtures/marked-regex/inline/gfm-inline.json");

        assert_eq!(inline.escape.as_str(), specs.escape.as_str());
        assert_eq!(inline.autolink.as_str(), specs.autolink.as_str());
        assert_eq!(inline.url.as_str(), specs.url.as_str());
        assert_eq!(inline.tag.as_str(), specs.tag.as_str());
        assert_eq!(inline.link.as_str(), specs.link.as_str());
        assert_eq!(inline.ref_link.as_str(), specs.ref_link.as_str());
        assert_eq!(inline.no_link.as_str(), specs.no_link.as_str());
        assert_eq!(inline.ref_link_search.as_str(), specs.ref_link_search.as_str());
        assert_eq!(inline.em_strong.l_delim.as_str(), specs.em_strong.l_delim.as_str());
        assert_eq!(inline.em_strong.r_delim_ast.as_str(), specs.em_strong.r_delim_ast.as_str());
        assert_eq!(inline.em_strong.r_delim_und.as_str(), specs.em_strong.r_delim_und.as_str());
        assert_eq!(inline.code.as_str(), specs.code.as_str());
        assert_eq!(inline.br.as_str(), specs.br.as_str());
        assert_eq!(inline.del.as_str(), specs.del.as_str());
        assert_eq!(inline.text.as_str(), specs.text.as_str());
        assert_eq!(inline.punctuation.as_str(), specs.punctuation.as_str());
        assert_eq!(inline._punctuation.as_str(), specs._punctuation.as_str());
        assert_eq!(inline.block_skip.as_str(), specs.block_skip.as_str());
        assert_eq!(inline.escaped_em_st.as_str(), specs.escaped_em_st.as_str());
        assert_eq!(inline.comment.as_str(), specs.comment.as_str());
        assert_eq!(inline.escapes.as_str(), specs.escapes.as_str());
        assert_eq!(inline.scheme.as_str(), specs.scheme.as_str());
        assert_eq!(inline.email.as_str(), specs.email.as_str());
        assert_eq!(inline.attribute.as_str(), specs.attribute.as_str());
        assert_eq!(inline.label.as_str(), specs.label.as_str());
        assert_eq!(inline.href.as_str(), specs.href.as_str());
        assert_eq!(inline.title.as_str(), specs.title.as_str());
        assert_eq!(inline.breaks.as_str(), specs.breaks.as_str());
        assert_eq!(inline.strong.start.as_str(), specs.strong.start.as_str());
        assert_eq!(inline.strong.middle.as_str(), specs.strong.middle.as_str());
        assert_eq!(inline.strong.end_ast.as_str(), specs.strong.end_ast.as_str());
        assert_eq!(inline.strong.end_und.as_str(), specs.strong.end_und.as_str());
        assert_eq!(inline.em.start.as_str(), specs.em.start.as_str());
        assert_eq!(inline.em.middle.as_str(), specs.em.middle.as_str());
        assert_eq!(inline.em.end_ast.as_str(), specs.em.end_ast.as_str());
        assert_eq!(inline.em.end_und.as_str(), specs.em.end_und.as_str());
        assert_eq!(inline.extended_email.as_str(), specs.extended_email.as_str());
        assert_eq!(inline.backpedal.as_str(), specs.backpedal.as_str());
    }

    #[test]
    fn validate_gfm_with_breaks_inline_rules() {
        let inline = &setup_inline_rules()[3];
        let specs = get_inline_specs("tests/fixtures/marked-regex/inline/gfm-breaks-inline.json");

        assert_eq!(inline.escape.as_str(), specs.escape.as_str());
        assert_eq!(inline.autolink.as_str(), specs.autolink.as_str());
        assert_eq!(inline.url.as_str(), specs.url.as_str());
        assert_eq!(inline.tag.as_str(), specs.tag.as_str());
        assert_eq!(inline.link.as_str(), specs.link.as_str());
        assert_eq!(inline.ref_link.as_str(), specs.ref_link.as_str());
        assert_eq!(inline.no_link.as_str(), specs.no_link.as_str());
        assert_eq!(inline.ref_link_search.as_str(), specs.ref_link_search.as_str());
        assert_eq!(inline.em_strong.l_delim.as_str(), specs.em_strong.l_delim.as_str());
        assert_eq!(inline.em_strong.r_delim_ast.as_str(), specs.em_strong.r_delim_ast.as_str());
        assert_eq!(inline.em_strong.r_delim_und.as_str(), specs.em_strong.r_delim_und.as_str());
        assert_eq!(inline.code.as_str(), specs.code.as_str());
        assert_eq!(inline.br.as_str(), specs.br.as_str());
        assert_eq!(inline.del.as_str(), specs.del.as_str());
        assert_eq!(inline.text.as_str(), specs.text.as_str());
        assert_eq!(inline.punctuation.as_str(), specs.punctuation.as_str());
        assert_eq!(inline._punctuation.as_str(), specs._punctuation.as_str());
        assert_eq!(inline.block_skip.as_str(), specs.block_skip.as_str());
        assert_eq!(inline.escaped_em_st.as_str(), specs.escaped_em_st.as_str());
        assert_eq!(inline.comment.as_str(), specs.comment.as_str());
        assert_eq!(inline.escapes.as_str(), specs.escapes.as_str());
        assert_eq!(inline.scheme.as_str(), specs.scheme.as_str());
        assert_eq!(inline.email.as_str(), specs.email.as_str());
        assert_eq!(inline.attribute.as_str(), specs.attribute.as_str());
        assert_eq!(inline.label.as_str(), specs.label.as_str());
        assert_eq!(inline.href.as_str(), specs.href.as_str());
        assert_eq!(inline.title.as_str(), specs.title.as_str());
        assert_eq!(inline.breaks.as_str(), specs.breaks.as_str());
        assert_eq!(inline.strong.start.as_str(), specs.strong.start.as_str());
        assert_eq!(inline.strong.middle.as_str(), specs.strong.middle.as_str());
        assert_eq!(inline.strong.end_ast.as_str(), specs.strong.end_ast.as_str());
        assert_eq!(inline.strong.end_und.as_str(), specs.strong.end_und.as_str());
        assert_eq!(inline.em.start.as_str(), specs.em.start.as_str());
        assert_eq!(inline.em.middle.as_str(), specs.em.middle.as_str());
        assert_eq!(inline.em.end_ast.as_str(), specs.em.end_ast.as_str());
        assert_eq!(inline.em.end_und.as_str(), specs.em.end_und.as_str());
        assert_eq!(inline.extended_email.as_str(), specs.extended_email.as_str());
        assert_eq!(inline.backpedal.as_str(), specs.backpedal.as_str());
    }
}
use md4rs_src::rules::test;

use std::fs;
use std::env;
use seq_macro::seq;
use std::path::Path;
use serde_json::Result;
use test_case::test_case;
use serde::{Serialize, Deserialize};
use md4rs_src::rules::{Block, MDBlock, setup};

#[cfg(test)]
mod tests {
    use super::*;

    fn deserialize_specs(path: &str)-> String {
        let data: String = fs::read_to_string(path)
            .expect("Unable to read file");
        return data;
    }

    fn get_specs(path: &str) -> Block {
        let data: String = deserialize_specs(path);
        let spec: Block = serde_json::from_str(&data).unwrap();
        return spec;
    }

    #[test]
    fn validate_normal_block_rules() {
        let block = &setup()[0];
        let specs = get_specs("tests/fixtures/marked-regex/normal-block.json");


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
        let block = &setup()[1];
        let specs = get_specs("tests/fixtures/marked-regex/gfm-block.json");

        println!("{}\n\n", block.paragraph.to_string());

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

}
#![allow(warnings, unused)]
use std::fs;
use test_case::test_case;
use marked_rs::defaults::Options;
use pretty_assertions::{assert_eq, assert_ne};

#[cfg(test)]
mod marked {
    use std::panic;
    use marked_rs::slugger::Slugger;
    use marked_rs::defaults::get_default_options;
    use marked_rs::lexer::{ILexer, Lexer};
    use marked_rs::marked::Marked;
    use marked_rs::renderer::{IRenderer, Renderer};
    use marked_rs::rules::test;
    use super::*;

    #[test]
    fn should_add_id_attr_by_default() {

        let mut slugger = Slugger::new();
        let options = get_default_options();
        let mut renderer = Renderer::new(options);

        let expected_header = "<h1 id=\"test\">test</h1>\n";
        let actual_header = renderer.heading("test", 1, "test", &mut slugger);

        pretty_assertions::assert_eq!(actual_header, expected_header);
    }

    #[test]
    fn should_not_add_id_attr_when_option_is_false() {

        let mut slugger = Slugger::new();
        let mut options = get_default_options();
        options.header_ids = false;

        let mut renderer = Renderer::new(options);

        let expected_header = "<h1>test</h1>\n";
        let actual_header = renderer.heading("test", 1, "test", &mut slugger);

        pretty_assertions::assert_eq!(actual_header, expected_header);
    }

    #[test]
    fn should_use_lowercase_slug() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("Test", false), "test");
    }

    #[test]
    fn should_be_unique_to_avoid_collisions_1280() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("test", false), "test");
        pretty_assertions::assert_eq!(slugger.slug("test", false), "test-1");
        pretty_assertions::assert_eq!(slugger.slug("test", false), "test-2");
    }

    #[test]
    fn should_be_unique_when_slug_ends_with_number() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("test 1", false), "test-1");
        pretty_assertions::assert_eq!(slugger.slug("test", false), "test");
        pretty_assertions::assert_eq!(slugger.slug("test", false), "test-2");
    }

    #[test]
    fn should_be_unique_when_slug_ends_with_hyphen_number() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo");
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo-1");
        pretty_assertions::assert_eq!(slugger.slug("foo-1", false), "foo-1-1");
        pretty_assertions::assert_eq!(slugger.slug("foo-1", false), "foo-1-2");
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo-2");
    }

    #[test]
    fn should_allow_non_latin_chars() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("привет", false), "привет");
    }

    #[test]
    fn should_remove_ampersands_857() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("This & That Section", false), "this--that-section");
    }

    #[test]
    fn should_remove_periods() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("file.txt", false), "filetxt");
    }

    #[test]
    fn should_remove_html_tags() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("<em>html</em>", false), "html");
    }

    #[test]
    fn should_not_increment_seen_when_dryrun_is_set() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("<h1>This Section</h1>", true), "this-section");
        pretty_assertions::assert_eq!(slugger.slug("<h1>This Section</h1>", false), "this-section");
    }

    #[test]
    fn should_still_return_next_unique_id_when_using_dryrun() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("<h1>This Section</h1>", false), "this-section");
        pretty_assertions::assert_eq!(slugger.slug("<h1>This Section</h1>", true), "this-section-1");
    }

    #[test]
    fn should_be_repeatable_in_a_sequence() {
        let mut slugger = Slugger::new();
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo");
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo-1");
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo-2");
        pretty_assertions::assert_eq!(slugger.slug("foo", true), "foo-3");
        pretty_assertions::assert_eq!(slugger.slug("foo", true), "foo-3");
        pretty_assertions::assert_eq!(slugger.slug("foo", false), "foo-3");
        pretty_assertions::assert_eq!(slugger.slug("foo", true), "foo-4");
    }

    #[test]
    fn should_use_paragraph_type_on_top_level() {

        let options = get_default_options();
        let mut lexer = Lexer::new(options);
        let md = "A Paragraph.\n\n> A blockquote\n\n- list item\n";

        let mut p_tokens = lexer.lex(md);
        let tokens = Lexer::capture_tokens_ac(&mut p_tokens);

        pretty_assertions::assert_eq!(tokens[0]._type, "paragraph");
        pretty_assertions::assert_eq!(tokens[2].tokens[0]._type, "paragraph");
        pretty_assertions::assert_eq!(tokens[3].items[0].tokens[0]._type, "text");
    }

    #[ignore]
    fn should_change_defaults() {
        // TODO: implement
        let result = panic::catch_unwind(|| {
            test();
        });

        // if result.is_ok() {
        //     println!("Should not occur");
        // } else {
        //     println!("Caught panic");
        // }
    }

    #[test]
    fn should_send_html_to_renderer() {
        // TODO: implement (or not?)
    }

    #[test]
    fn should_parse_inline_tokens() {

        let md = "**strong** _em_";
        let mut marked = Marked::new(None);
        let actual_html = marked.parse_inline(md, None);

        pretty_assertions::assert_eq!(actual_html, "<strong>strong</strong> <em>em</em>");
    }

    #[test]
    fn should_not_parse_block_tokens() {

        let md = "# header\n\n_em_";
        let mut marked = Marked::new(None);
        let actual_html = marked.parse_inline(md, None);

        pretty_assertions::assert_eq!(actual_html, "# header\n\n<em>em</em>");
    }
}


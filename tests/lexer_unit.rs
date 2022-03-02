use test_case::test_case;
use marked_rs::defaults::Options;
use marked_rs::lexer::{ILexer, Lexer};
use marked_rs::tokenizer::{Link, Token};

pub fn expect_tokens(md: &str, options: Options, mut tokens: &mut Vec<Token>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    let actual_tokens = lexer.lex(md);
    let expected_tokens = tokens;

    assert_eq!(actual_tokens, expected_tokens);
}


#[cfg(test)]
mod paragraph {
    use marked_rs::defaults::get_default_options;
    use marked_rs::rules::test;
    use super::*;

    #[test]
    fn space_between_paragraphs() {
        let md = "paragraph 1\n\nparagraph 2";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "paragraph 1".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 1".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "paragraph 1".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "paragraph 1".to_string(),
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
                    },
                ],
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
            },
            Token {
                _type: "space",
                raw: "\n\n".to_string(),
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
            },
            Token {
                _type: "paragraph",
                raw: "paragraph 2".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 2".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "paragraph 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "paragraph 2".to_string(),
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
                    },
                ],
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
        ];
        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
    }

    #[test]
    fn indent_code() {

    }

    #[test]
    fn fenced_code() {
        assert_eq!(true, false);
    }

    #[test]
    fn fenced_code_lang() {

    }

    #[test]
    fn headings_depth() {
        assert_eq!(true, false);
    }

    #[test]
    fn no_heading_if_depth_greater_than_six() {
        assert_eq!(true, false);
    }

    #[test]
    fn pipe_table() {
        assert_eq!(true, false);
    }

    #[test]
    fn table_after_paragraph() {
        assert_eq!(true, false);
    }

    #[test]
    fn align_table() {
        assert_eq!(true, false);
    }

    #[test]
    fn no_pipe_table() {
        assert_eq!(true, false);
    }

    #[test]
    fn hr_default() {
        assert_eq!(true, false);
    }

    #[test]
    fn hr_after_line_break_does_not_consume_raw() {
        assert_eq!(true, false);
    }

    #[test]
    fn blockquote_start_inner_end() {
        assert_eq!(true, false);
    }

    #[test]
    fn unordered_list() {
        assert_eq!(true, false);
    }

    #[test]
    fn ordered_list() {
        assert_eq!(true, false);
    }

    #[test]
    fn unordered_list_with_parenthesis() {
        assert_eq!(true, false);
    }

    #[test]
    fn space_after_list() {
        assert_eq!(true, false);
    }

    #[test]
    fn list_start() {
        assert_eq!(true, false);
    }

    #[test]
    fn loose_list() {
        assert_eq!(true, false);
    }


    #[test]
    fn non_loose_list_with_spaces() {
        assert_eq!(true, false);
    }

    #[test]
    fn task_list() {
        assert_eq!(true, false);
    }

    #[test]
    fn html_div() {
        assert_eq!(true, false);
    }

    #[test]
    fn html_pre() {
        assert_eq!(true, false);
    }

    #[test]
    fn html_sanitize() {
        assert_eq!(true, false);
    }

    #[test]
    fn link_def() {
        assert_eq!(true, false);
    }

    #[test]
    fn link_title() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_escape_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_html_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn sanitize_inline_html_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_link_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_title_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_image_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_image_title_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_relink_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_no_link_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_no_def_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_strong_tokens() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_em_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_codespan_tokens() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_only_spaces_not_stripped() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_only_end_spaces_not_stripped() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_begin_and_end_spaces_stripped() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_begin_and_end_newlines_stripped() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_begin_and_end_tabs_not_stripped() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_begin_and_end_newlines() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_beginning_and_end_multiple_spaces_only_one_stripped() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_br() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_del() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_url_autolink() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_url_autolink_email() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_url_autolink_mangle_email() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_url() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_url_email() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_url_mangle_email() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_url_text() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_smartypants_single_quotes() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_smartypants_ellipses() {
        assert_eq!(true, false);
    }


    #[test]
    fn inline_smartypants_en_dash() {
        assert_eq!(true, false);
    }

    #[test]
    fn inline_smartypants_em_dash() {
        assert_eq!(true, false);
    }

}


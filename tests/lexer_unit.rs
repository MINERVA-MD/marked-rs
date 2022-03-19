use std::fs;
use test_case::test_case;

use marked_rs::token::Token;
use marked_rs::tokenizer::{Link};
use marked_rs::defaults::Options;
use marked_rs::lexer::{ILexer, Lexer};
use pretty_assertions::{assert_eq, assert_ne};


pub fn expect_tokens(md: &str, options: Options, mut tokens: &mut Vec<Token>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    lexer.links = links;

    let mut actual_tokens = lexer.lex_ac(md);
    let expected_tokens = tokens;

    pretty_assertions::assert_eq!(&mut actual_tokens, expected_tokens);
}

pub fn expect_inline_tokens(md: &str, options: Options, mut tokens: Vec<Token>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    lexer.links = links;

    let mut inline_tokens = vec![];
     lexer.inline_tokens(md, &mut inline_tokens);

    // println!("Logging inline_tokens ======= {:#?}", inline_tokens);

    let expected_inline_tokens = tokens;
    let actual_inline_tokens_ac = Lexer::capture_tokens_ac(&mut inline_tokens);

    pretty_assertions::assert_eq!(actual_inline_tokens_ac, expected_inline_tokens);
}

pub fn expect_mangle_email(md: &str, options: Options, mut tokens: Vec<Token>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    lexer.links = links;


    let mut inline_tokens = vec![];
    lexer.inline_tokens(md, &mut inline_tokens);

    let expected_inline_tokens = tokens;
    let actual_inline_tokens_ac = Lexer::capture_tokens_ac(&mut inline_tokens);

    let actual_token = actual_inline_tokens_ac.get(0).unwrap();
    let expected_token =  expected_inline_tokens.get(0).unwrap();

    let text_re = fancy_regex::Regex::new(r#"^(&#x?[0-9a-f]+;)+$"#).unwrap();
    let href_re = fancy_regex::Regex::new(r#"^mailto:(&#x?[0-9a-f]+;)+$"#).unwrap();

    pretty_assertions::assert_eq!(actual_token.raw, expected_token.raw);
    pretty_assertions::assert_eq!(actual_token._type, expected_token._type);
    pretty_assertions::assert_eq!(actual_token.tokens[0]._type, expected_token.tokens[0]._type);
    pretty_assertions::assert_eq!(text_re.is_match(actual_token.text.as_str()).unwrap(), true);
    pretty_assertions::assert_eq!(href_re.is_match(actual_token.href.as_str()).unwrap(), true);
    pretty_assertions::assert_eq!(text_re.is_match(actual_token.tokens[0].raw.as_str()).unwrap(), true);
    pretty_assertions::assert_eq!(text_re.is_match(actual_token.tokens[0].text.as_str()).unwrap(), true);
}

pub fn expect_links(md: &str, options: Options, expected_links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    lexer.lex(md);
    let actual_links = lexer.get_links();

    pretty_assertions::assert_eq!(actual_links, expected_links);
}


#[cfg(test)]
mod lexer {
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
                    },
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
            },
            Token {
                _type: "space",
                raw: "\n\n".to_string(),
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
                    },
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
            }
        ];
        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn indent_code() {
        let md = "    code";
        let mut tokens = vec![
            Token {
                _type: "code",
                raw: "    code".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "code".to_string(),
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
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn fenced_code() {
        let md = "```\ncode\n```";
        let mut tokens = vec![
            Token {
                _type: "code",
                raw: "```\ncode\n```".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "code".to_string(),
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn fenced_code_lang() {
        let md = "```text\ncode\n```";
        let mut tokens = vec![
            Token {
                _type: "code",
                raw: "```text\ncode\n```".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "code".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "text".to_string(),
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn headings_depth() {
        let md = "
# heading 1

## heading 2

### heading 3

#### heading 4

##### heading 5

###### heading 6

lheading 1
==========

lheading 2
----------
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "heading",
                raw: "# heading 1\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 1".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 1".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 1".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 1,
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
            },
            Token {
                _type: "heading",
                raw: "## heading 2\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 2".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 2".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 2,
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
            },
            Token {
                _type: "heading",
                raw: "### heading 3\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 3".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 3".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 3".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 3,
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
            },
            Token {
                _type: "heading",
                raw: "#### heading 4\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 4".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 4".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 4".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 4,
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
            },
            Token {
                _type: "heading",
                raw: "##### heading 5\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 5".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 5".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 5".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 5,
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
            },
            Token {
                _type: "heading",
                raw: "###### heading 6\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading 6".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "heading 6".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading 6".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 6,
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
            },
            Token {
                _type: "heading",
                raw: "lheading 1\n==========\n\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "lheading 1".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "lheading 1".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "lheading 1".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 1,
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
            },
            Token {
                _type: "heading",
                raw: "lheading 2\n----------\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "lheading 2".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "lheading 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "lheading 2".to_string(),
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
                ],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 2,
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn no_heading_if_depth_greater_than_six() {
        let md = "####### heading 7";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "####### heading 7".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "####### heading 7".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "####### heading 7".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "####### heading 7".to_string(),
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
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn pipe_table() {
        let md = "
| a | b |
|---|---|
| 1 | 2 |
";

        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            } ,
            Token {
                _type: "table",
                raw: "| a | b |\n|---|---|\n| 1 | 2 |\n".to_string(),
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
                align: vec!["".to_string(), "".to_string()],
                rows: vec![
                    vec![
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "1".to_string(),
                            tokens: vec![
                                Token {
                                    _type: "text",
                                    raw: "1".to_string(),
                                    href: "".to_string(),
                                    title: "".to_string(),
                                    text: "1".to_string(),
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
                        },
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "2".to_string(),
                            tokens: vec![
                                Token {
                                    _type: "text",
                                    raw: "2".to_string(),
                                    href: "".to_string(),
                                    title: "".to_string(),
                                    text: "2".to_string(),
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
                        }
                    ]
                ],
                header: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "a".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "a".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "a".to_string(),
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
                    },
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "b".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "b".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "b".to_string(),
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
                    }
                ],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn table_after_paragraph() {

        let md = "
paragraph 1
| a | b |
|---|---|
| 1 | 2 |
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "paragraph",
                raw: "paragraph 1\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 1".to_string(),
                tokens: vec![ Token {
                    _type: "text",
                    raw: "paragraph 1".to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "paragraph 1".to_string(),
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
                }],
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
            },
            Token {
                _type: "table",
                raw: "| a | b |\n|---|---|\n| 1 | 2 |\n".to_string(),
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
                align: vec!["".to_string(), "".to_string()],
                rows: vec![
                    vec![
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "1".to_string(),
                            tokens: vec![
                                Token {
                                    _type: "text",
                                    raw: "1".to_string(),
                                    href: "".to_string(),
                                    title: "".to_string(),
                                    text: "1".to_string(),
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
                        },
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "2".to_string(),
                            tokens: vec![
                                Token {
                                    _type: "text",
                                    raw: "2".to_string(),
                                    href: "".to_string(),
                                    title: "".to_string(),
                                    text: "2".to_string(),
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
                        }
                    ]
                ],
                header: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "a".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "a".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "a".to_string(),
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
                    },
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "b".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "b".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "b".to_string(),
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
                    }
                ],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn align_table() {
        let md = "
| a | b | c |
|:--|:-:|--:|
| 1 | 2 | 3 |
";

        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "table",
                raw: "| a | b | c |\n|:--|:-:|--:|\n| 1 | 2 | 3 |\n".to_string(),
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
                align: vec!["left".to_string(), "center".to_string(), "right".to_string()],
                rows: vec![
                    vec![
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "1".to_string(),
                            tokens: vec![ Token {
                                _type: "text",
                                raw: "1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "1".to_string(),
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
                            }],
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
                        },
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "2".to_string(),
                            tokens: vec![ Token {
                                _type: "text",
                                raw: "2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "2".to_string(),
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
                        
                            }],
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
                        },
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "3".to_string(),
                            tokens: vec![ Token {
                                _type: "text",
                                raw: "3".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "3".to_string(),
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
                            }],
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
                    ]
                ],
                header: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "a".to_string(),
                        tokens: vec![ Token {
                            _type: "text",
                            raw: "a".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "a".to_string(),
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
                        }],
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
                    },
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "b".to_string(),
                        tokens: vec![ Token {
                            _type: "text",
                            raw: "b".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "b".to_string(),
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
                        }],
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
                    },
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "c".to_string(),
                        tokens: vec![ Token {
                            _type: "text",
                            raw: "c".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "c".to_string(),
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
                        }],
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
                ],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn no_pipe_table() {
        let md = "
a | b
--|--
1 | 2
";

        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "table",
                raw: "a | b\n--|--\n1 | 2\n".to_string(),
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
                align: vec!["".to_string(), "".to_string()],
                rows: vec![
                    vec![
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "1".to_string(),
                            tokens: vec![ Token {
                                _type: "text",
                                raw: "1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "1".to_string(),
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
                            }],
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
                        },
                        Token {
                            _type: "",
                            raw: "".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "2".to_string(),
                            tokens: vec![ Token {
                                _type: "text",
                                raw: "2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "2".to_string(),
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
                            }],
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
                    ]
                ],
                header: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "a".to_string(),
                        tokens: vec![ Token {
                            _type: "text",
                            raw: "a".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "a".to_string(),
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
                        }],
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
                    },
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "b".to_string(),
                        tokens: vec![ Token {
                            _type: "text",
                            raw: "b".to_string(),
                            href: "".to_string(),
                            title: "".to_string(),
                            text: "b".to_string(),
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
                        }],
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
                ],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn hr_default() {
        let md = "---";
        let mut tokens = vec![
            Token {
                _type: "hr",
                raw: "---".to_string(),
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn hr_after_line_break_does_not_consume_raw() {
        let md = "T\nh\n---";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "T\nh\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "T\nh".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "T\nh".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "T\nh".to_string(),
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
            },
            Token {
                _type: "hr",
                raw: "---".to_string(),
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn blockquote_start_inner_end() {
        let md = "> blockquote";
        let mut tokens = vec![
            Token {
                _type: "blockquote",
                raw: "> blockquote".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "blockquote".to_string(),
                tokens: vec![
                    Token {
                        _type: "paragraph",
                        raw: "blockquote".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "blockquote".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "blockquote".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "blockquote".to_string(),
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
                    }
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
            }
        ];

        let options = get_default_options();


        
        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn unordered_list() {

        let md = "
- item 1
- item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "- item 1\n- item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "- item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "- item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }
                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn ordered_list() {
        let md = "
1. item 1
2. item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "1. item 1\n2. item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: true,
                start: 1,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "1. item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "2. item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }

                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn unordered_list_with_parenthesis() {
        let md = "
1) item 1
2) item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "1) item 1\n2) item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: true,
                start: 1,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "1) item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "2) item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }
                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn space_after_list() {
        let md = "
- item 1
- item 2

paragraph
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "- item 1\n- item 2".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "- item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "- item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }
                ],
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
            },
            Token {
                _type: "space",
                raw: "\n\n".to_string(),
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
            },
            Token {
                _type: "paragraph",
                raw: "paragraph\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "paragraph".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "paragraph".to_string(),
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
            }
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn list_start() {
        let md = "
2. item 1
3. item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "2. item 1\n3. item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: true,
                start: 2,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "2. item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "3. item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }

                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn loose_list() {
        let md = "
- item 1

- item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "- item 1\n\n- item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: true,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "- item 1\n\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1\n".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1\n".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                    },
                    Token {
                        _type: "list_item",
                        raw: "- item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                    }
                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn non_loose_list_with_spaces() {
        let md = "
- item 1
  - item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "- item 1\n  - item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "- item 1\n  - item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1\n- item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1\n".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            },
                            Token {
                                _type: "list",
                                raw: "- item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "".to_string(),
                                tokens: vec![],
                                tag: "".to_string(),
                                ordered: false,
                                start: 0,
                                lang: "".to_string(),
                                loose: false,
                                items: vec![
                                    Token {
                                        _type: "list_item",
                                        raw: "- item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
                                        tokens: vec![
                                            Token {
                                                _type: "text",
                                                raw: "item 2".to_string(),
                                                href: "".to_string(),
                                                title: "".to_string(),
                                                text: "item 2".to_string(),
                                                tokens: vec![
                                                    Token {
                                                        _type: "text",
                                                        raw: "item 2".to_string(),
                                                        href: "".to_string(),
                                                        title: "".to_string(),
                                                        text: "item 2".to_string(),
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
                                            }
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
                                    }
                                ],
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
                    }
                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn task_list() {
        let md = "
- [ ] item 1
- [x] item 2
";
        let mut tokens = vec![
            Token {
                _type: "space",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "list",
                raw: "- [ ] item 1\n- [x] item 2\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![
                    Token {
                        _type: "list_item",
                        raw: "- [ ] item 1\n".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 1".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 1".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 1".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 1".to_string(),
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
                            }
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
                        task: true,
                        checked: false,
                        in_link: false,
                        in_raw_block: false,
                        links: vec![],
                        align: vec![],
                        rows: vec![],
                        header: vec![],
                        code_block_style: "".to_string()
                    },
                    Token {
                        _type: "list_item",
                        raw: "- [x] item 2".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "item 2".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "item 2".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "item 2".to_string(),
                                        href: "".to_string(),
                                        title: "".to_string(),
                                        text: "item 2".to_string(),
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
                            }
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
                        task: true,
                        checked: true,
                        in_link: false,
                        in_raw_block: false,
                        links: vec![],
                        align: vec![],
                        rows: vec![],
                        header: vec![],
                        code_block_style: "".to_string()
                    }
                ],
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn html_div() {
        let md = "<div>html</div>";
        let mut tokens = vec![
            Token {
                _type: "html",
                raw: "<div>html</div>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "<div>html</div>".to_string(),
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn html_pre() {
        let md = "<pre>html</pre>";
        let mut tokens = vec![
            Token 
            {
                _type: "html",
                raw: "<pre>html</pre>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "<pre>html</pre>".to_string(),
                tokens: vec![],
                tag: "".to_string(),
                ordered: false,
                start: 0,
                lang: "".to_string(),
                loose: false,
                items: vec![],
                depth: 0,
                escaped: false,
                pre: true,
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
        ];

        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn html_sanitize() {
        let md = "<div>html</div>";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "<div>html</div>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "&lt;div&gt;html&lt;/div&gt;".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "&lt;div&gt;html&lt;/div&gt;".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "&lt;div&gt;html&lt;/div&gt;".to_string(),
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
            }
        ];

        let mut options: Options = get_default_options();
        options.sanitize = true;



        expect_tokens(md, options, &mut tokens, vec![]);
    }

    #[test]
    fn link_def() {
        let md = "[link]: https://example.com";
        let mut options: Options = get_default_options();
        let links = vec![
            Link {
                href: "https://example.com".to_string(),
                title: "".to_string(),
                tag: "link".to_string()
            }
        ];

        expect_links(md, options,  links);
    }

    #[test]
    fn link_title() {
        let md = r#"[link]: https://example.com "title""#;
        let mut options: Options = get_default_options();
        let links = vec![
            Link {
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                tag: "link".to_string()
            }
        ];

        expect_links(md, options,  links);
    }

    #[test]
    fn inline_escape_tokens() {
        let md = "\\>";
        let mut tokens = vec![
            Token {
                _type: "escape",
                raw: "\\>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "&gt;".to_string(),
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
        ];

        let options = get_default_options();


        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_html_tokens() {
        let md = "<div>html</div>";
        let mut tokens = vec![
            Token {
                _type: "html",
                raw: "<div>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "<div>".to_string(),
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
            },
            Token {
                _type: "text",
                raw: "html".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "html".to_string(),
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
            },
            Token {
                _type: "html",
                raw: "</div>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "</div>".to_string(),
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
        ];

        let options = get_default_options();


        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn sanitize_inline_html_tokens() {
        let md = "<div>html</div>";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "<div>html</div>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "&lt;div&gt;html&lt;/div&gt;".to_string(),
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
        ];

        let mut options = get_default_options();
        options.sanitize = true;
        


        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_link_tokens() {
        let md = "[link](https://example.com)";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "[link](https://example.com)".to_string(),
                href: "https://example.com".to_string(),
                title: "".to_string(),
                text: "link".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "link".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "link".to_string(),
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
            }
        ];

        let mut options = get_default_options();


        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_title_tokens() {
        let md = r#"[link](https://example.com "title")"#;
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: r#"[link](https://example.com "title")"#.to_string(),
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                text: "link".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "link".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "link".to_string(),
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
            }
        ];

        let mut options = get_default_options();
        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_image_tokens() {
        let md = r#"![image](https://example.com/image.png)"#;
        let mut tokens = vec![
            Token {
                _type: "image",
                raw: "![image](https://example.com/image.png)".to_string(),
                href: "https://example.com/image.png".to_string(),
                title: "".to_string(),
                text: "image".to_string(),
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
        ];

        let mut options = get_default_options();
        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_image_title_tokens() {
        let md = r#"![image](https://example.com/image.png "title")"#;
        let mut tokens = vec![
            Token {
                _type: "image",
                raw: r#"![image](https://example.com/image.png "title")"#.to_string(),
                href: "https://example.com/image.png".to_string(),
                title: "title".to_string(),
                text: "image".to_string(),
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
        ];

        let mut options = get_default_options();

        expect_inline_tokens(md, options, tokens, vec![]);
    }

    #[test]
    fn inline_relink_tokens() {
        let md = "[link][]";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "[link][]".to_string(),
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                text: "link".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "link".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "link".to_string(),
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
            }
        ];

        let mut options = get_default_options();
        let links = vec![
            Link {
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                tag: "link".to_string()
            }
        ];
        
        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_no_link_tokens() {
        let md = "[link]";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "[link]".to_string(),
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                text: "link".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "link".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "link".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![
            Link {
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                tag: "link".to_string()
            }
        ];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_no_def_tokens() {
        let md = "[link]";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "[link]".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "[link]".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_strong_tokens() {
        let md = "**strong**";
        let mut tokens = vec![
            Token {
                _type: "strong",
                raw: "**strong**".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "strong".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "strong".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "strong".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_em_tokens() {
        let md = "*em*";
        let mut tokens = vec![
            Token {
                _type: "em",
                raw: "*em*".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "em".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "em".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "em".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_codespan_tokens() {
        let md = "`code`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`code`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "code".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_only_spaces_not_stripped() {
        let md = "`   `";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`   `".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "   ".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_only_beginning_spaces_not_stripped() {
        let md = "` a`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "` a`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: " a".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_only_end_spaces_not_stripped() {
        let md = "`a `";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`a `".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a ".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }


    #[test]
    fn inline_begin_and_end_spaces_stripped() {
        let md = "` a `";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "` a `".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_begin_and_end_newlines_stripped() {
        let md = "`\na\n`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`\na\n`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_begin_and_end_tabs_not_stripped() {
        let md = "`\ta\t`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`\ta\t`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "\ta\t".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_begin_and_end_newlines() {
        let md = "`\na\n`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`\na\n`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_beginning_and_end_multiple_spaces_only_one_stripped() {
        let md = "`  a  `";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`  a  `".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: " a ".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_newline_to_space() {
        let md = "`a\nb`";
        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "`a\nb`".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a b".to_string(),
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
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_br() {
        let md = "a\nb";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "a".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "a".to_string(),
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
            },
            Token {
                _type: "br",
                raw: "\n".to_string(),
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
            },
            Token {
                _type: "text",
                raw: "b".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "b".to_string(),
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
        ];

        let mut options = get_default_options();
        options.gfm = true;
        options.breaks = true;

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_del() {
        let md = "~~del~~";
        let mut tokens = vec![
            Token {
                _type: "del",
                raw: "~~del~~".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "del".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "del".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "del".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_url_autolink() {
        let md = "<https://example.com>";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "<https://example.com>".to_string(),
                href: "https://example.com".to_string(),
                title: "".to_string(),
                text: "https://example.com".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "https://example.com".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "https://example.com".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_url_autolink_email() {
        let md = "<test@example.com>";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "<test@example.com>".to_string(),
                href: "mailto:test@example.com".to_string(),
                title: "".to_string(),
                text: "test@example.com".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "test@example.com".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "test@example.com".to_string(),
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
            }
        ];

        let mut options = get_default_options();
        options.mangle = false;

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }


    #[test]
    fn inline_url_autolink_mangle_email() {
        let md = "<test@example.com>";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "<test@example.com>".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
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
            }
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.mangle = true;

        expect_mangle_email(md, options, tokens, links);
    }

    #[test]
    fn inline_url() {
        let md = "https://example.com";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "https://example.com".to_string(),
                href: "https://example.com".to_string(),
                title: "".to_string(),
                text: "https://example.com".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "https://example.com".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "https://example.com".to_string(),
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
            }
        ];

        let mut options = get_default_options();

        let links = vec![];

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_url_email() {
        let md = "test@example.com";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "test@example.com".to_string(),
                href: "mailto:test@example.com".to_string(),
                title: "".to_string(),
                text: "test@example.com".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "test@example.com".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "test@example.com".to_string(),
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
            }
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.gfm = true;
        options.mangle = false;

        expect_inline_tokens(md, options, tokens, links);
    }


    #[test]
    fn inline_url_mangle_email() {
        let md = "test@example.com";
        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "test@example.com".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
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
            }
        ];

        let links = vec![];
        let mut options = get_default_options();

        options.gfm = true;
        options.mangle = true;

        expect_mangle_email(md, options, tokens, links);
    }


    #[test]
    fn inline_url_text() {
        let md = "text";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "text".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "text".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_smartypants_single_quotes() {
        let md = "'single quotes'";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "'single quotes'".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "‘single quotes’".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.smartypants = true;

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_smartypants_double_quotes() {
        let md = r#""double quotes""#;
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: r#""double quotes""#.to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "“double quotes”".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.smartypants = true;

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_smartypants_ellipses() {
        let md = "ellipses...";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "ellipses...".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "ellipses…".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.smartypants = true;

        expect_inline_tokens(md, options, tokens, links);
    }


    #[test]
    fn inline_smartypants_en_dash() {
        let md = "en--dash";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "en--dash".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "en–dash".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.smartypants = true;

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn inline_smartypants_em_dash() {
        let md = "em---dash";
        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "em---dash".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "em—dash".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();
        options.smartypants = true;

        expect_inline_tokens(md, options, tokens, links);
    }

    #[test]
    fn check_cm_spec_example_18() {
        let md = "<http://example.com?find=\\*>\n";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "<http://example.com?find=\\*>\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "<http://example.com?find=\\*>".to_string(),
                tokens: vec![
                    Token {
                        _type: "link",
                        raw:  "<http://example.com?find=\\*>".to_string(),
                        href: "http://example.com?find=\\*".to_string(),
                        title: "".to_string(),
                        text: "http://example.com?find=\\*".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "http://example.com?find=\\*".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "http://example.com?find=\\*".to_string(),
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
                    }
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
            }
        ];

        let links = vec![];
        let mut options = get_default_options();

        expect_tokens(md, options, &mut tokens, links);
    }

    #[test]
    fn check_cm_spec_example_23() {
        let md = "[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"\n";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "[foo]".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "[foo]".to_string(),
                tokens: vec![
                    Token {
                        _type: "link",
                        raw: "[foo]".to_string(),
                        href: "/bar\\*".to_string(),
                        title: "ti\\*tle".to_string(),
                        text: "foo".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "foo".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "foo".to_string(),
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
                    }
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
            },
            Token {
                _type: "space",
                raw: "\n\n".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();

        expect_tokens(md, options, &mut tokens, links);
    }

    #[test]
    fn check_cm_spec_example_22() {
        let md = "[foo](/bar\\* \"ti\\*tle\")\n";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "[foo](/bar\\* \"ti\\*tle\")\n".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "[foo](/bar\\* \"ti\\*tle\")".to_string(),
                tokens: vec![
                    Token {
                        _type: "link",
                        raw: "[foo](/bar\\* \"ti\\*tle\")".to_string(),
                        href: "/bar*".to_string(),
                        title: "ti*tle".to_string(),
                        text: "foo".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "foo".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "foo".to_string(),
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
                    }
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
            },
            Token {
                _type: "space",
                raw: "\n\n".to_string(),
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
        ];

        let links = vec![];
        let mut options = get_default_options();

        expect_tokens(md, options, &mut tokens, links);
    }

}


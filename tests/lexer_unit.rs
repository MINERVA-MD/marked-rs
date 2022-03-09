use test_case::test_case;
use marked_rs::defaults::Options;
use marked_rs::lexer::{ILexer, Lexer};
use marked_rs::tokenizer::{Link, Token};

pub fn expect_tokens(md: &str, options: Options, mut tokens: &mut Vec<Token>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    let actual_tokens = lexer.lex(md);
    let expected_tokens = tokens;

    // if md == "[link]: https://example.com" {
    //     println!("{:?}", actual_tokens);
    // }

    assert_eq!(actual_tokens, expected_tokens);
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
                links: vec![],
                align: vec![],
                rows: vec![],
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "indented".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
    }

    #[ignore]
    fn ordered_list() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn unordered_list_with_parenthesis() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn space_after_list() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn list_start() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn loose_list() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn non_loose_list_with_spaces() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn task_list() {
        assert_eq!(true, false);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
    }

    #[test]
    fn html_pre() {
        let md = "<pre>html</pre>";
        let mut tokens = vec![
            Token {
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let options = get_default_options();
        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
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
                links: vec![],
                align: vec![],
                rows: vec![],
                header: vec![],
                code_block_style: "".to_string()
            }
        ];

        let mut options: Options = get_default_options();
        options.sanitize = true;

        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
    }

    #[ignore]
    fn link_def() {
        let md = "[link]: https://example.com";
        let mut tokens = vec![
        ];

        let mut options: Options = get_default_options();

        let links = vec![];

        expect_tokens(md, options, &mut tokens, links);
    }

    #[ignore]
    fn link_title() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_escape_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_html_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn sanitize_inline_html_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_link_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_title_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_image_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_image_title_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_relink_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_no_link_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_no_def_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_strong_tokens() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_em_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_codespan_tokens() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_only_spaces_not_stripped() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_only_end_spaces_not_stripped() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_begin_and_end_spaces_stripped() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_begin_and_end_newlines_stripped() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_begin_and_end_tabs_not_stripped() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_begin_and_end_newlines() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_beginning_and_end_multiple_spaces_only_one_stripped() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_br() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_del() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_url_autolink() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_url_autolink_email() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_url_autolink_mangle_email() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_url() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_url_email() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_url_mangle_email() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_url_text() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_smartypants_single_quotes() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_smartypants_ellipses() {
        assert_eq!(true, false);
    }


    #[ignore]
    fn inline_smartypants_en_dash() {
        assert_eq!(true, false);
    }

    #[ignore]
    fn inline_smartypants_em_dash() {
        assert_eq!(true, false);
    }

}


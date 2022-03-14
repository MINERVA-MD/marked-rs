use std::fs;
use test_case::test_case;
use marked_rs::defaults::Options;
use marked_rs::renderer::Renderer;
use marked_rs::lexer::{ILexer, Lexer};
use marked_rs::tokenizer::{Link, Token};
use marked_rs::parser::{IParser, Parser};
use pretty_assertions::{assert_eq, assert_ne};

pub fn expect_html(mut tokens: &mut Vec<Token>, options: Options, html: &str, inline: bool) {
    let mut parser = Parser::new(options);
    let actual_html = if inline {
        parser.parse_inline(tokens, Renderer::new(options))
    } else {
        parser.parse(tokens, true)
    };

    let expected_html = html.to_string();
    pretty_assertions::assert_eq!(actual_html, expected_html);
}


#[cfg(test)]
mod parser {
    use marked_rs::defaults::get_default_options;
    use marked_rs::rules::MDBlock::Html;
    use marked_rs::rules::test;
    use super::*;


    // BLOCK

    #[test]
    fn block_space_between_paragraphs() {

        let html = "<p>paragraph 1</p>\n<p>paragraph 2</p>\n";
        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 1".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
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
            },
            Token {
                _type: "paragraph",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 2".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
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
        ];
        let options = get_default_options();

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_hr() {

        let html = "<hr>\n";
        let mut tokens = vec![
            Token {
                _type: "hr",
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
        ];

        let options = get_default_options();

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_heading() {

        let html = r#"<h1 id="heading">heading</h1>\n"#;
        let mut tokens = vec![
            Token {
                _type: "heading",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "heading".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "heading".to_string(),
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
            }
        ];

        let options = get_default_options();

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_code() {

        let html = "<pre><code>code\n</code></pre>\n";
        let mut tokens = vec![
            Token {
                _type: "code",
                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_table() {

        let html = "<table>\n<thead>\n<tr>\n<th align=\"left\">a</th>\n<th align=\"right\">b</th>\n</tr>\n</thead>\n<tbody><tr>\n<td align=\"left\">1</td>\n<td align=\"right\">2</td>\n</tr>\n</tbody></table>\n";

        let mut tokens = vec![
            Token {
                _type: "table",
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
                align: vec!["left".to_string(), "right".to_string()],
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_blockquote() {

        let html = "<blockquote>\n<p>blockquote</p>\n</blockquote>\n";

        let mut tokens = vec![
            Token {
                _type: "blockquote",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "".to_string(),
                tokens: vec![
                    Token {
                        _type: "paragraph",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "blockquote".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_list_unordered() {

        let html = "<ul>\n<li>item 1</li>\n<li>item 2</li>\n</ul>\n";

        let mut tokens = vec![
            Token {
                _type: "list",
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
                items: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_list_ordered() {

        let html = "<ol start=\"2\">\n<li>item 1</li>\n<li>item 2</li>\n</ol>\n";

        let mut tokens = vec![
            Token {
                _type: "list",
                raw: "".to_string(),
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
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_list_tasks() {

        let html = "<ul>\n<li><input disabled=\"\" type=\"checkbox\"> item 1</li>\n<li><input checked=\"\" disabled=\"\" type=\"checkbox\"> item 2</li>\n</ul>\n";

        let mut tokens = vec![
            Token {
                _type: "list",
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
                items: vec![
                    Token {
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 1".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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
                        _type: "",
                        raw: "".to_string(),
                        href: "".to_string(),
                        title: "".to_string(),
                        text: "".to_string(),
                        tokens: vec![
                            Token {
                                _type: "text",
                                raw: "".to_string(),
                                href: "".to_string(),
                                title: "".to_string(),
                                text: "item 2".to_string(),
                                tokens: vec![
                                    Token {
                                        _type: "text",
                                        raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_list_loose() {

        let html = "<ul>\n<li><p>item 1</p>\n</li>\n<li><p>item 2</p>\n</li>\n</ul>\n";

        let mut tokens = vec![
           Token {
               _type: "list",
               raw: "".to_string(),
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
                       _type: "",
                       raw: "".to_string(),
                       href: "".to_string(),
                       title: "".to_string(),
                       text: "".to_string(),
                       tokens: vec![
                           Token {
                               _type: "text",
                               raw: "".to_string(),
                               href: "".to_string(),
                               title: "".to_string(),
                               text: "item 1".to_string(),
                               tokens: vec![
                                   Token {
                                       _type: "text",
                                       raw: "".to_string(),
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
                       _type: "",
                       raw: "".to_string(),
                       href: "".to_string(),
                       title: "".to_string(),
                       text: "".to_string(),
                       tokens: vec![
                           Token {
                               _type: "text",
                               raw: "".to_string(),
                               href: "".to_string(),
                               title: "".to_string(),
                               text: "item 2".to_string(),
                               tokens: vec![
                                   Token {
                                       _type: "text",
                                       raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_html() {

        let html = "<div>html</div>";

        let mut tokens = vec![
            Token {
                _type: "html",
                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_paragraph() {

        let html = "<p>paragraph 1</p>\n";

        let mut tokens = vec![
            Token {
                _type: "paragraph",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "paragraph 1".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    #[test]
    fn block_text() {

        let html = "<p>text 1\ntext 2</p>\n";

        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "text 1".to_string(),
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
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "text 2".to_string(),
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

        expect_html(&mut tokens, options, html, false);
    }

    // INLINE

    #[test]
    fn inline_escape() {

        let html = "&gt;";

        let mut tokens = vec![
            Token {
                _type: "escape",
                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_html() {

        let html = "<div>html</div>";

        let mut tokens = vec![
            Token {
                _type: "html",
                raw: "".to_string(),
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
                raw: "".to_string(),
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
                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_link() {

        // TODO: Implement helpers
        let html = "<a href=\"https://example.com\" title=\"title\">link</a>";

        let mut tokens = vec![
            Token {
                _type: "link",
                raw: "".to_string(),
                href: "https://example.com".to_string(),
                title: "title".to_string(),
                text: "link".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
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

        let options = get_default_options();

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_image() {

        let html = "<img src=\"image.png\" alt=\"image\" title=\"title\">";

        let mut tokens = vec![
            Token {
                _type: "image",
                raw: "".to_string(),
                href: "image.png".to_string(),
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

        let options = get_default_options();

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_strong() {

        let html = "<strong>strong</strong>";

        let mut tokens = vec![
            Token {
                _type: "strong",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "strong".to_string(),
                tokens: vec![
                    Token {
                        _type: "text",
                        raw: "".to_string(),
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

        let options = get_default_options();

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_codespan() {

        let html = "<code>code</code>";

        let mut tokens = vec![
            Token {
                _type: "codespan",
                raw: "".to_string(),
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

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_br() {

        let html = "<br>";

        let mut tokens = vec![
            Token {
                _type: "br",
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
        ];

        let options = get_default_options();

        expect_html(&mut tokens, options, html, true);
    }

    #[test]
    fn inline_del() {

    let html = "<del>del</del>";

    let mut tokens = vec![
        Token {
            _type: "del",
            raw: "".to_string(),
            href: "".to_string(),
            title: "".to_string(),
            text: "del".to_string(),
            tokens: vec![
                Token {
                    _type: "text",
                    raw: "".to_string(),
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

    let options = get_default_options();

    expect_html(&mut tokens, options, html, true);
}

    #[test]
    fn inline_text() {

        let html = "text 1text 2";

        let mut tokens = vec![
            Token {
                _type: "text",
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "text 1".to_string(),
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
                raw: "".to_string(),
                href: "".to_string(),
                title: "".to_string(),
                text: "text 2".to_string(),
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

        expect_html(&mut tokens, options, html, true);
    }

}


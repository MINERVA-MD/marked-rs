use std::cell::RefCell;
use std::fs;
use std::rc::Rc;
use test_case::test_case;
use marked_rs::defaults::Options;
use marked_rs::lexer::{ILexer, Lexer};
use marked_rs::tokenizer::{Link, Token};
use pretty_assertions::{assert_eq, assert_ne};

pub fn expect_tokens(md: &str, options: Options, mut tokens: &mut Vec<Rc<RefCell<Token>>>, links: Vec<Link>) {
    let mut lexer = Lexer::new(options);
    lexer.links = links;

    let mut actual_tokens = lexer.lex(md);
    let expected_tokens = tokens;

    // println!("Actual {:#?}\n\n", actual_tokens);
    // println!("Expected {:#?}\n\n", expected_tokens);

    // pretty_assertions::assert_eq!(&mut actual_tokens, expected_tokens);
}


#[cfg(test)]
mod lexer2 {
    use std::cell::RefCell;
    use std::rc::Rc;
    use marked_rs::defaults::get_default_options;
    use marked_rs::rules::test;
    use super::*;

    #[test]
    fn space_between_paragraphs() {
        let md = "paragraph 1\n\nparagraph 2";
        let mut tokens = vec![
            Rc::new(RefCell::new(
                Token {
                    _type: "paragraph",
                    raw: "paragraph 1".to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "paragraph 1".to_string(),
                    tokens: vec![
                        Rc::new(RefCell::new(
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
                            }
                        )),
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
            )),
            Rc::new(RefCell::new(
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
            )),
            Rc::new(RefCell::new(
                Token {
                    _type: "paragraph",
                    raw: "paragraph 2".to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "paragraph 2".to_string(),
                    tokens: vec![
                        Rc::new(RefCell::new(
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
                            }
                        )),
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
            ))
        ];
        let options = get_default_options();


        expect_tokens(md, options, &mut tokens, vec![]);
    }


    #[test]
    fn check_cm_spec_example_23() {
        let md = "[foo]\n\n[foo]: /bar\\* \"ti\\*tle\"\n";
        let mut tokens = vec![
            Rc::new(RefCell::new(
                Token {
                    _type: "paragraph",
                    raw: "[foo]".to_string(),
                    href: "".to_string(),
                    title: "".to_string(),
                    text: "[foo]".to_string(),
                    tokens: vec![
                        Rc::new(RefCell::new(
                            Token {
                                _type: "link",
                                raw: "[foo]".to_string(),
                                href: "/bar\\*".to_string(),
                                title: "ti\\*tle".to_string(),
                                text: "foo".to_string(),
                                tokens: vec![
                                    Rc::new(RefCell::new(
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
                                    ))
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
                        ))
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
            )),
            Rc::new(RefCell::new(
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
            ))
        ];

        let links = vec![];
        let mut options = get_default_options();

        expect_tokens(md, options, &mut tokens, links);
    }
}


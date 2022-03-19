use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

use crate::tokenizer::Token;
use crate::slugger::Slugger;
use crate::defaults::Options;
use crate::helpers::unescape;
use crate::renderer::{Flags, IRenderer, Renderer};
use crate::text_renderer::{ITextRenderer, TextRenderer};

pub struct Parser {
    pub options: Options,
    pub renderer: Renderer,
    pub text_renderer: TextRenderer,
    pub slugger: Slugger
}

pub trait IParser {
    fn parse(&mut self, tokens: &mut Vec<Rc<RefCell<Token>>>, top: bool) -> String;
    fn parse_inline(&mut self,  tokens:  &mut Vec<Rc<RefCell<Token>>>, renderer: Renderer) -> String;
    fn parse_inline_tr(&mut self, tokens:  &mut Vec<Rc<RefCell<Token>>>, renderer: TextRenderer) -> String;
}

impl Parser {
    pub fn new(options: Options) -> Self {
        Self {
            options,
            renderer: Renderer::new(options),
            text_renderer: TextRenderer::new(),
            slugger: Slugger::new()
        }
    }
}

impl IParser for Parser {
    fn parse(&mut self, mut tokens: &mut Vec<Rc<RefCell<Token>>>, top: bool) -> String {
        let mut token: &mut Rc<RefCell<Token>>;
        let mut out = String::from("");

        let mut item: &mut Rc<RefCell<Token>>;
        let mut cell: String = "".to_string();
        let mut header: String = "".to_string();
        let mut item_body: String = "".to_string();
        let mut row: &mut Vec<Rc<RefCell<Token>>> = &mut vec![];

        let mut start: i32 = 0;
        let mut task: bool = false;
        let mut loose: bool = false;
        let mut checked: bool = false;
        let mut ordered: bool = false;
        let mut body: String= "".to_string();
        let mut checkbox: String =  "".to_string();

        let l = tokens.len();
        let (mut l2, mut l3) = (0 as usize, 0 as usize);


        let mut i = 0;
        while i < tokens.len() {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                todo!("Implement Extensions")
            }

            token = tokens.get_mut(i).unwrap();

            let _type = token.as_ref().borrow()._type.clone();

            match _type {
                "space"         => {
                    i += 1;
                    continue;
                }

                "hr"            => {
                    out.push_str(self.renderer.hr().as_str());
                    i += 1;
                    continue;
                }

                "heading"       => {
                    let text = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, Renderer::new(self.options));
                    let level = token.as_ref().borrow().depth;
                    let _raw = self.parse_inline_tr(&mut token.as_ref().borrow_mut().tokens, TextRenderer::new());
                    let raw = unescape(_raw.as_str());
                    let _out = self.renderer.heading(text.as_str(), level, raw.as_str(), &mut self.slugger);

                    out.push_str(_out.as_str());
                    i += 1;
                    continue;
                }

                "code"          => {
                    out.push_str(self.renderer.code(
                        token.as_ref().borrow_mut().text.as_str(),
                        token.as_ref().borrow().lang.as_str(),
                        token.as_ref().borrow_mut().escaped
                    ).as_str());
                    i += 1;
                    continue;
                }

                "table"         => {
                    header = "".to_string();

                    // header
                    cell = "".to_string();
                    l2 = token.as_ref().borrow().header.len();

                    for j in 0..l2{
                        let mut _curr_token = token.as_ref().borrow_mut();
                        let curr_token = _curr_token.header.get_mut(j).unwrap();
                        let header_tokens = &mut curr_token.as_ref().borrow_mut().tokens;
                        let flags = Flags {
                            header: true,
                            align: token.as_ref().borrow().align[j].clone()
                        };

                        let content = self.parse_inline(header_tokens, self.renderer);
                        let cells = self.renderer.tablecell(content.as_str(), flags);

                        cell = format!("{}{}",
                                       cell,
                                       cells
                        );
                    }

                    header = format!("{}{}",
                                     header,
                                     self.renderer.tablerow(cell.as_str())
                    );

                    body = "".to_string();
                    l2 = token.as_ref().borrow().rows.len();

                    for j in 0..l2 {
                        let mut _row = token.as_ref().borrow_mut();
                        row = _row.rows.get_mut(j).unwrap();

                        cell = "".to_string();
                        l3 = row.len();

                        for k in 0..l3 {
                            let curr_token = row.get_mut(k).unwrap();
                            let row_tokens = &mut curr_token.as_ref().borrow_mut().tokens;
                            let flags = Flags {
                                header: false,
                                align: token.as_ref().borrow().align[k].clone()
                            };

                            let content = self.parse_inline(row_tokens, self.renderer);
                            let cells = self.renderer.tablecell(content.as_str(), flags);

                            cell = format!("{}{}",
                                           cell,
                                           cells
                            );
                        }
                        body = format!("{}{}",
                                       body,
                                       self.renderer.tablerow(cell.as_str())
                        );
                    }
                    out = format!("{}{}",
                                  out,
                                  self.renderer.table(header.as_str(), body.as_str())
                    );
                    i += 1;
                    continue;
                }

                "blockquote"    => {
                    body = self.parse(&mut token.as_ref().borrow_mut().tokens, true);
                    out.push_str(self.renderer.blockquote(body.as_str()).as_str());
                    i += 1;
                    continue;
                }

                "list"          => {
                    ordered = token.as_ref().borrow().ordered.clone();
                    start = token.as_ref().borrow().start.clone();
                    loose = token.as_ref().borrow().loose.clone();
                    l2 = token.as_ref().borrow().items.len();

                    body = "".to_string();
                    for j in 0..l2 {
                        let mut _item = token.as_ref().borrow_mut();
                        item = _item.items.get_mut(j).unwrap();
                        checked = item.as_ref().borrow().checked.clone();

                        item_body = "".to_string();
                        if item.as_ref().borrow().task {
                            checkbox = self.renderer.checkbox(checked);

                            if loose {
                                if item.as_ref().borrow().tokens.len() > 0 &&
                                    item.as_ref().borrow().tokens[0].as_ref().borrow()._type == "paragraph"
                                {
                                    let text = format!("{} {}",
                                                       checkbox.clone(),
                                                       item.as_ref().borrow().tokens[0].as_ref().borrow().text.clone()
                                    );

                                    item.as_ref().borrow_mut().tokens[0].as_ref().borrow_mut().text = text.clone();

                                    if item.as_ref().borrow().tokens.len() > 0 &&
                                        item.as_ref().borrow().tokens[0].as_ref().borrow().tokens.len() > 0 &&
                                        item.as_ref().borrow().tokens[0].as_ref().borrow().tokens[0].as_ref().borrow().text == "text"
                                    {
                                        let text = format!("{} {}",
                                                           checkbox.clone(),
                                                           item.as_ref().borrow().tokens[0].as_ref().borrow().tokens[0].as_ref().borrow().text.clone()
                                        );

                                        item.as_ref().borrow_mut().tokens[0].as_ref().borrow_mut().tokens[0].as_ref().borrow_mut().text = text.clone();
                                    }
                                } else {
                                    item.as_ref().borrow_mut().tokens.insert(0,
                                                                             Rc::new(RefCell::new(
                                                                                 Token {
                                                                                     _type: "text",
                                                                                     raw: "".to_string(),
                                                                                     href: "".to_string(),
                                                                                     title: "".to_string(),
                                                                                     text: checkbox.clone(),
                                                                                     tokens: vec![],
                                                                                     tag: "".to_string(),
                                                                                     ordered,
                                                                                     start,
                                                                                     lang: "".to_string(),
                                                                                     loose,
                                                                                     items: vec![],
                                                                                     depth: 0,
                                                                                     escaped: false,
                                                                                     pre: false,
                                                                                     task,
                                                                                     checked,
                                                                                     in_link: false,
                                                                                     in_raw_block: false,
                                                                                     links: vec![],
                                                                                     align: vec![],
                                                                                     rows: vec![],
                                                                                     header: vec![],
                                                                                     code_block_style: "".to_string()
                                                                                 }
                                                                             )))
                                }
                            } else {
                                item_body = format!("{}{}",
                                                    item_body,
                                                    checkbox.clone()
                                );
                            }
                        }
                        item_body = format!("{}{}",
                                            item_body,
                                            self.parse(&mut item.as_ref().borrow_mut().tokens, loose)
                        );

                        body = format!("{}{}",
                                       body,
                                       self.renderer.list_item(item_body.as_str())
                        );
                    }

                    out = format!("{}{}",
                                  out,
                                  self.renderer.list(body.as_str(), ordered, start)
                    );
                    i += 1;
                    continue;
                }

                "html"          => {
                    out.push_str(self.renderer.html(token.as_ref().borrow().text.as_str()).as_str());
                    i += 1;
                    continue;
                }

                "paragraph"     => {
                    let text_ = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, Renderer::new(self.options));
                    let _text = self.renderer.paragraph(text_.as_str());
                    out.push_str(_text.as_str());
                    i += 1;
                    continue;
                }

                "text"          => {
                    body = if token.as_ref().borrow().tokens.len() > 0 {
                        self.parse_inline(&mut token.as_ref().borrow_mut().tokens, self.renderer)
                    } else {
                        String::from(token.as_ref().borrow().text.as_str())
                    };

                    while i + 1 < l &&
                        tokens.get(i + 1).unwrap().as_ref().borrow()._type == "text"
                    {
                        // Double Check Increment
                        i += 1;
                        token = tokens.get_mut( i).unwrap();

                        let mut _body = if token.as_ref().borrow().tokens.len() > 0 {
                            self.parse_inline(&mut token.as_ref().borrow_mut().tokens, self.renderer)
                        } else {
                            token.as_ref().borrow().text.to_string()
                        };

                        body.push_str("\n");
                        body.push_str(_body.as_str());
                    }

                    let _body = self.renderer.paragraph(body.as_str());
                    out.push_str( if top { _body.as_str() } else { body.as_str() });

                    i += 1;
                    continue;
                }

                _               => {
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token.as_ref().borrow()._type);
                    if self.options.silent {
                        println!("{}", err_msg);
                    } else {
                        panic!("{}", err_msg);
                    }
                }
            }
            i += 1;
        }
        out
    }

    fn parse_inline(&mut self, tokens:  &mut Vec<Rc<RefCell<Token>>>, mut renderer: Renderer) -> String {

        let mut token: &mut Rc<RefCell<Token>>;
        let mut ret: bool = false;
        let mut out = String::from("");

        let l = tokens.len();
        for i in 0..l {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                // Implement Extensions
            }

            token = tokens.get_mut(i).unwrap();

            match token.as_ref().borrow()._type {
                "escape"        => {
                    out.push_str(renderer.text(token.as_ref().borrow().text.as_str()).as_str());
                    continue;
                }

                "html"          => {
                    out.push_str(renderer.html(token.as_ref().borrow().text.as_str()).as_str());
                    continue;
                }

                "link"          => {
                    let _text = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.link(
                        token.as_ref().borrow().href.as_str(),
                        token.as_ref().borrow().title.as_str(),
                        _text.as_str()
                    ).as_str());
                    continue;
                }

                "image"         => {
                    out.push_str(renderer.image(
                        token.as_ref().borrow().href.as_str(),
                        token.as_ref().borrow().title.as_str(),
                        token.as_ref().borrow().text.as_str()
                    ).as_str());
                    continue;
                }

                "strong"        => {
                    let _text = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.strong(_text.as_str()).as_str());
                    continue;
                }

                "em"            => {
                    let _text = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.em(_text.as_str()).as_str());
                    continue;
                }

                "codespan"      => {
                    out.push_str(renderer.codespan(token.as_ref().borrow().text.as_str()).as_str());
                    continue;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    continue;
                }

                "del"           => {
                    let _text = self.parse_inline(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.del(_text.as_str()).as_str());
                    continue;
                }

                "text"          => {
                    out.push_str(renderer.text(token.as_ref().borrow().text.as_str()).as_str());
                    continue;
                }

                _               => {
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token.as_ref().borrow()._type);
                    if self.options.silent {
                        println!("{}", err_msg);
                    } else {
                        panic!("{}", err_msg);
                    }
                }
            }
        }
        out
    }

    fn  parse_inline_tr(&mut self, mut tokens:  &mut Vec<Rc<RefCell<Token>>>, mut renderer: TextRenderer) -> String {

        let mut token: &mut Rc<RefCell<Token>>;
        let mut ret: bool = false;
        let mut out = String::from("");

        let l = tokens.len();
        for i in 0..l {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                // Implement Extensions
            }

            token = tokens.get_mut(i).unwrap();

            match token.as_ref().borrow()._type {
                "escape"        => {
                    out.push_str(renderer.text(token.as_ref().borrow().text.as_str()).as_str());
                    break;
                }

                "html"          => {
                    out.push_str(renderer.html(token.as_ref().borrow().text.as_str()).as_str());
                    break;
                }

                "link"          => {
                    let _text = self.parse_inline_tr(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.link(
                        token.as_ref().borrow().href.as_str(),
                        token.as_ref().borrow().title.as_str(),
                        _text.as_str()
                    ).as_str());
                    break;
                }

                "image"         => {
                    out.push_str(renderer.image(
                        token.as_ref().borrow().href.as_str(),
                        token.as_ref().borrow().title.as_str(),
                        token.as_ref().borrow().text.as_str()
                    ).as_str());
                    break;
                }

                "strong"        => {
                    let _text = self.parse_inline_tr(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.strong(_text.as_str()).as_str());
                    break;
                }

                "em"            => {
                    let _text = self.parse_inline_tr(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.em(_text.as_str()).as_str());
                    break;
                }

                "codespan"      => {
                    out.push_str(renderer.codespan(token.as_ref().borrow().text.as_str()).as_str());
                    break;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    break;
                }

                "del"           => {
                    let _text = self.parse_inline_tr(&mut token.as_ref().borrow_mut().tokens, renderer);
                    out.push_str(renderer.del(_text.as_str()).as_str());
                    break;
                }

                "text"          => {
                    out.push_str(renderer.text(token.as_ref().borrow().text.as_str()).as_str());
                    break;
                }

                _               => {
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token.as_ref().borrow()._type);
                    if self.options.silent {
                        println!("{}", err_msg);
                    } else {
                        panic!("{}", err_msg);
                    }
                }
            }
        }
        out
    }
}

pub fn parse(mut tokens: &mut Vec<Rc<RefCell<Token>>>, options: Options) -> String {
    let mut parser = Parser::new(options);
    parser.parse(&mut tokens, true)
}

pub fn parse_inline(mut tokens: Vec<Rc<RefCell<Token>>>, options: Options) -> String {
    let mut parser = Parser::new(options);
    parser.parse_inline(&mut tokens, parser.renderer)
}
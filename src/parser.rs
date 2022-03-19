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
            let mut _type = "";

            {
                _type = token.as_ref().borrow()._type.clone();
            }

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

                    {
                        let mut heading_token_rc = token.as_ref().borrow_mut();


                        let text =  {
                            self.parse_inline(&mut heading_token_rc.tokens, Renderer::new(self.options))
                        };

                        let level = {
                            heading_token_rc.depth
                        };

                        let _raw = {
                            self.parse_inline_tr(&mut heading_token_rc.tokens, TextRenderer::new())
                        };

                        let raw = unescape(_raw.as_str());
                        let _out = self.renderer.heading(text.as_str(), level, raw.as_str(), &mut self.slugger);

                        out.push_str(_out.as_str());
                    }

                    i += 1;
                    continue;
                }

                "code"          => {

                    {
                        let mut code_token_rc = token.as_ref().borrow_mut();

                        {
                            out.push_str(self.renderer.code(
                                code_token_rc.text.as_str(),
                                code_token_rc.lang.as_str(),
                                code_token_rc.escaped
                            ).as_str());
                        }
                    }

                    i += 1;
                    continue;
                }

                "table"         => {
                    {
                        let mut table_token = token.as_ref().borrow_mut();
                        header = "".to_string();

                        // header
                        cell = "".to_string();
                        l2 = table_token.header.len();

                        for j in 0..l2 {
                            {
                                let header_tokens = &mut table_token.header[j].as_ref().borrow_mut().tokens;

                                let align = table_token.align[j].clone();

                                let flags = Flags {
                                    header: true,
                                    align
                                };

                                let content = self.parse_inline(header_tokens, self.renderer);
                                let cells = self.renderer.tablecell(content.as_str(), flags);

                                cell = format!("{}{}",
                                               cell,
                                               cells
                                );
                            }
                        }

                        header = format!("{}{}",
                                         header,
                                         self.renderer.tablerow(cell.as_str())
                        );

                        body = "".to_string();
                        l2 = table_token.rows.len();

                        for j in 0..l2 {
                            {
                                let mut aligns = table_token.align.clone();
                                row = table_token.rows.get_mut(j).unwrap();

                                cell = "".to_string();
                                l3 = row.len();

                                for k in 0..l3 {

                                    let curr_token = row.get_mut(k).unwrap();
                                    let row_tokens = &mut curr_token.as_ref().borrow_mut().tokens;

                                    let flags = Flags {
                                        header: false,
                                        align: aligns[k].clone()
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
                        }
                        out = format!("{}{}",
                                      out,
                                      self.renderer.table(header.as_str(), body.as_str())
                        );
                    }
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
                    let mut list_token = token.as_ref().borrow_mut();

                    ordered = list_token.ordered.clone();
                    start = list_token.start.clone();
                    loose = list_token.loose.clone();
                    l2 = list_token.items.len();

                    body = "".to_string();
                    for j in 0..l2 {
                        item = list_token.items.get_mut(j).unwrap();
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

            let mut _type = "";

            {
                _type = token.as_ref().borrow()._type.clone();
            }

            match _type {
                "escape"        => {
                    {
                        let escape_token = token.as_ref().borrow();
                        out.push_str(renderer.text(escape_token.text.as_str()).as_str());
                    }

                    continue;
                }

                "html"          => {
                    {
                        let html_token = token.as_ref().borrow();
                        out.push_str(renderer.html(html_token.text.as_str()).as_str());
                    }
                    continue;
                }

                "link"          => {

                    {
                        let mut link_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline(&mut link_token.tokens, renderer);

                        out.push_str(renderer.link(
                            link_token.href.as_str(),
                            link_token.title.as_str(),
                            _text.as_str()
                        ).as_str());
                    }

                    continue;
                }

                "image"         => {

                    {
                        let mut image_token = token.as_ref().borrow();

                        out.push_str(renderer.image(
                            image_token.href.as_str(),
                            image_token.title.as_str(),
                            image_token.text.as_str()
                        ).as_str());
                    }

                    continue;
                }

                "strong"        => {
                    {
                        let mut strong_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline(&mut strong_token.tokens, renderer);
                        out.push_str(renderer.strong(_text.as_str()).as_str());
                    }

                    continue;
                }

                "em"            => {
                    {
                        let mut em_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline(&mut em_token.tokens, renderer);
                        out.push_str(renderer.em(_text.as_str()).as_str());
                    }
                    continue;
                }

                "codespan"      => {
                    {
                        let codespan_token = token.as_ref().borrow();
                        out.push_str(renderer.codespan(codespan_token.text.as_str()).as_str());
                    }
                    continue;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    continue;
                }

                "del"           => {

                    {
                        let mut del_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline(&mut del_token.tokens, renderer);
                        out.push_str(renderer.del(_text.as_str()).as_str());
                    }

                    continue;
                }

                "text"          => {
                    {
                        let text_token = token.as_ref().borrow();
                        out.push_str(renderer.text(text_token.text.as_str()).as_str());
                    }

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

            let mut _type = "";

            {
                _type = token.as_ref().borrow()._type.clone();
            }

            match _type {
                "escape"        => {
                    {
                        let escape_token = token.as_ref().borrow();
                        out.push_str(renderer.text(escape_token.text.as_str()).as_str());
                    }

                    break;
                }

                "html"          => {
                    {
                        let html_token = token.as_ref().borrow();
                        out.push_str(renderer.html(html_token.text.as_str()).as_str());
                    }

                    break;
                }

                "link"          => {

                    {
                        let mut link_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline_tr(&mut link_token.tokens, renderer);
                        out.push_str(renderer.link(
                            link_token.href.as_str(),
                            link_token.title.as_str(),
                            _text.as_str()
                        ).as_str());
                    }

                    break;
                }

                "image"         => {
                    {
                        let image_token = token.as_ref().borrow();
                        out.push_str(renderer.image(
                            image_token.href.as_str(),
                            image_token.title.as_str(),
                            image_token.text.as_str()
                        ).as_str());
                    }
                    break;
                }

                "strong"        => {
                    {
                        let mut strong_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline_tr(&mut strong_token.tokens, renderer);
                        out.push_str(renderer.strong(_text.as_str()).as_str());
                    }
                    break;
                }

                "em"            => {
                    {
                        let mut em_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline_tr(&mut em_token.tokens, renderer);
                        out.push_str(renderer.em(_text.as_str()).as_str());
                    }
                    break;
                }

                "codespan"      => {
                    {
                        let codespan_token = token.as_ref().borrow();
                        out.push_str(renderer.codespan(codespan_token.text.as_str()).as_str());
                    }
                    break;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    break;
                }

                "del"           => {
                    {
                        let mut del_token = token.as_ref().borrow_mut();
                        let _text = self.parse_inline_tr(&mut del_token.tokens, renderer);
                        out.push_str(renderer.del(_text.as_str()).as_str());
                    }
                    break;
                }

                "text"          => {
                    {
                        let text_token = token.as_ref().borrow();
                        out.push_str(renderer.text(text_token.text.as_str()).as_str());
                    }
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
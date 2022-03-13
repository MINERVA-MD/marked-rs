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
    fn parse(&mut self, tokens: &mut Vec<Token>, top: bool) -> String;
    fn parse_inline(&mut self,  tokens:  &mut Vec<Token>, renderer: Renderer) -> String;
    fn parse_inline_tr(&mut self, tokens:  &mut Vec<Token>, renderer: TextRenderer) -> String;
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
    fn parse(&mut self, mut tokens: &mut Vec<Token>, top: bool) -> String {
        let mut token: &mut Token;
        let mut out = String::from("");

        let mut item: &mut Token;
        let mut cell: String = "".to_string();
        let mut header: String = "".to_string();
        let mut item_body: String = "".to_string();
        let mut row: &mut Vec<Token> = &mut vec![];

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


            match token._type {
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
                    let text = self.parse_inline(&mut token.tokens, Renderer::new(self.options));
                    let level = token.depth;
                    let _raw = self.parse_inline_tr(&mut token.tokens, TextRenderer::new());
                    let raw = unescape(_raw.as_str());
                    let _out = self.renderer.heading(text.as_str(), level, raw.as_str(), &mut self.slugger);

                    out.push_str(_out.as_str());
                    i += 1;
                    continue;
                }

                "code"          => {
                    out.push_str(self.renderer.code(
                        token.text.as_str(),
                        token.lang.as_str(),
                        token.escaped).as_str()
                    );
                    i += 1;
                    continue;
                }

                "table"         => {
                    header = "".to_string();

                    // header
                    cell = "".to_string();
                    l2 = token.header.len();

                    for j in 0..l2{
                        let curr_token = token.header.get_mut(j).unwrap();
                        let header_tokens = &mut curr_token.tokens;
                        let flags = Flags {
                            header: true,
                            align: token.align[j].clone()
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
                    l2 = token.rows.len();

                    for j in 0..l2 {
                        row = token.rows.get_mut(j).unwrap();

                        cell = "".to_string();
                        l3 = row.len();

                        for k in 0..l3 {
                            let curr_token = row.get_mut(k).unwrap();
                            let row_tokens = &mut curr_token.tokens;
                            let flags = Flags {
                                header: false,
                                align: token.align[k].clone()
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
                    body = self.parse(&mut token.tokens, true);
                    out.push_str(self.renderer.blockquote(body.as_str()).as_str());
                    i += 1;
                    continue;
                }

                "list"          => {
                    ordered = token.ordered.clone();
                    start = token.start.clone();
                    loose = token.loose.clone();
                    l2 = token.items.len();

                    body = "".to_string();
                    for j in 0..l2 {
                        item = token.items.get_mut(j).unwrap();
                        checked = item.checked.clone();

                        item_body = "".to_string();
                        if item.task {
                            checkbox = self.renderer.checkbox(checked);

                            if loose {
                                if item.tokens.len() > 0 &&
                                    item.tokens[0]._type == "paragraph"
                                {
                                    let text = format!("{} {}",
                                                       checkbox.clone(),
                                                       item.tokens[0].text.clone()
                                    );

                                    item.tokens[0].text = text.clone();

                                    if item.tokens.len() > 0 &&
                                        item.tokens[0].tokens.len() > 0 &&
                                        item.tokens[0].tokens[0].text == "text"
                                    {
                                        let text = format!("{} {}",
                                                           checkbox.clone(),
                                                           item.tokens[0].tokens[0].text.clone()
                                        );

                                        item.tokens[0].tokens[0].text = text.clone();
                                    }
                                } else {
                                    item.tokens.insert(0, Token {
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
                                    })
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
                                            self.parse(&mut item.tokens, loose)
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
                    out.push_str(self.renderer.html(token.text.as_str()).as_str());
                    i += 1;
                    continue;
                }

                "paragraph"     => {
                    let text_ = self.parse_inline(&mut token.tokens, Renderer::new(self.options));
                    let _text = self.renderer.paragraph(text_.as_str());
                    out.push_str(_text.as_str());
                    i += 1;
                    continue;
                }

                "text"          => {
                    body = if token.tokens.len() > 0 {
                        self.parse_inline(&mut token.tokens, self.renderer)
                    } else {
                        String::from(token.text.as_str())
                    };

                    while i + 1 < l &&
                        tokens.get(i + 1).unwrap()._type == "text"
                    {
                        // Double Check Increment
                        i += 1;
                        token = tokens.get_mut( i).unwrap();

                        let mut _body = if token.tokens.len() > 0 {
                            self.parse_inline(&mut token.tokens, self.renderer)
                        } else {
                            token.text.to_string()
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
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token._type);
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

    fn parse_inline(&mut self, tokens:  &mut Vec<Token>, mut renderer: Renderer) -> String {

        let mut token: &mut Token;
        let mut ret: bool = false;
        let mut out = String::from("");

        let l = tokens.len();
        for i in 0..l {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                // Implement Extensions
            }

            token = tokens.get_mut(i).unwrap();

            match token._type {
                "escape"        => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    continue;
                }

                "html"          => {
                    out.push_str(renderer.html(token.text.as_str()).as_str());
                    continue;
                }

                "link"          => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.link(
                        token.href.as_str(),
                        token.title.as_str(),
                        _text.as_str()
                    ).as_str());
                    continue;
                }

                "image"         => {
                    out.push_str(renderer.image(
                        token.href.as_str(),
                        token.title.as_str(),
                        token.text.as_str()
                    ).as_str());
                    continue;
                }

                "strong"        => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.strong(_text.as_str()).as_str());
                    continue;
                }

                "em"            => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.em(_text.as_str()).as_str());
                    continue;
                }

                "codespan"      => {
                    out.push_str(renderer.codespan(token.text.as_str()).as_str());
                    continue;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    continue;
                }

                "del"           => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    println!("Del: {}", _text);
                    out.push_str(renderer.del(_text.as_str()).as_str());
                    continue;
                }

                "text"          => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    continue;
                }

                _               => {
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token._type);
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

    fn  parse_inline_tr(&mut self, mut tokens:  &mut Vec<Token>, mut renderer: TextRenderer) -> String {

        let mut token: &mut Token;
        let mut ret: bool = false;
        let mut out = String::from("");

        let l = tokens.len();
        for i in 0..l {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                // Implement Extensions
            }

            token = tokens.get_mut(i).unwrap();

            match token._type {
                "escape"        => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    break;
                }

                "html"          => {
                    out.push_str(renderer.html(token.text.as_str()).as_str());
                    break;
                }

                "link"          => {
                    let _text = self.parse_inline_tr(&mut token.tokens, renderer);
                    out.push_str(renderer.link(
                        token.href.as_str(),
                        token.title.as_str(),
                        _text.as_str()
                    ).as_str());
                    break;
                }

                "image"         => {
                    out.push_str(renderer.image(
                        token.href.as_str(),
                        token.title.as_str(),
                        token.text.as_str()
                    ).as_str());
                    break;
                }

                "strong"        => {
                    let _text = self.parse_inline_tr(&mut token.tokens, renderer);
                    out.push_str(renderer.strong(_text.as_str()).as_str());
                    break;
                }

                "em"            => {
                    let _text = self.parse_inline_tr(&mut token.tokens, renderer);
                    out.push_str(renderer.em(_text.as_str()).as_str());
                    break;
                }

                "codespan"      => {
                    out.push_str(renderer.codespan(token.text.as_str()).as_str());
                    break;
                }

                "br"            => {
                    out.push_str(renderer.br().as_str());
                    break;
                }

                "del"           => {
                    let _text = self.parse_inline_tr(&mut token.tokens, renderer);
                    out.push_str(renderer.del(_text.as_str()).as_str());
                    break;
                }

                "text"          => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    break;
                }

                _               => {
                    let err_msg = format!(r#"Token with "{}" type was not found."#, token._type);
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

pub fn parse(mut tokens: &mut Vec<Token>, options: Options) -> String {
    let mut parser = Parser::new(options);
    parser.parse(&mut tokens, true)
}

pub fn parse_inline(mut tokens: Vec<Token>, options: Options) -> String {
    let mut parser = Parser::new(options);
    parser.parse_inline(&mut tokens, parser.renderer)
}
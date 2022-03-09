use crate::tokenizer::Token;
use crate::slugger::Slugger;
use crate::defaults::Options;
use crate::helpers::unescape;
use crate::text_renderer::TextRenderer;
use crate::renderer::{IRenderer, Renderer};



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
        let mut out = String::from("");
        let mut token: &mut Token;

        let mut item: &str = "";
        let mut cell: &str = "";
        let mut header: &str = "";
        let mut start: i32 = 0;
        let mut checked: bool = false;
        let mut loose: bool = false;
        let mut ordered: bool = false;
        let mut body: String= "".to_string();

        let l = tokens.len();
        let (mut l2, mut l3) = (0 as usize, 0 as usize);
        for mut i in 0..l {

            // Run any renderer extensions
            if self.options.extensions.is_some() {
                todo!("Implement Extensions")
            }

            token = tokens.get_mut(i).unwrap();


            match token._type {
                "space" => { continue; }

                "hr"    => {
                    out.push_str(self.renderer.hr().as_str());
                    continue;
                }

                "heading"    => {
                    let text = self.parse_inline(&mut token.tokens, Renderer::new(self.options));
                    let level = token.depth;
                    let _raw = self.parse_inline_tr(&mut token.tokens, TextRenderer::new());
                    let raw = unescape(_raw.as_str());
                    let _out = self.renderer.heading(text.as_str(), level, raw.as_str(), &mut self.slugger);

                    out.push_str(_out.as_str());
                    continue;
                }

                "code"      => {
                    out.push_str(self.renderer.code(
                        token.text.as_str(),
                        token.lang.as_str(),
                        token.escaped).as_str()
                    );
                    continue;
                }

                "table"     => {
                    header = "";

                    cell = "";
                    l2 = token.header.len();

                    for j in 0..l2{
                        todo!();
                    }

                    continue;
                }
                "blockquote" => {
                    body = self.parse(&mut token.tokens, true);
                    out.push_str(self.renderer.blockquote(body.as_str()).as_str());
                    continue;
                }

                "list"      => {
                    ordered = token.ordered;
                    start = token.start;
                    loose = token.loose;
                    l2 = token.items.len();

                    body = "".to_string();

                    for j in 0..l2 {
                        // item = token.items.get(j).unwrap().as_str();
                    }
                    continue;
                }

                "html"      => {
                    out.push_str(self.renderer.html(token.text.as_str()).as_str());
                    continue;
                }

                "paragraph"      => {
                    let text_ = self.parse_inline(&mut token.tokens, Renderer::new(self.options));
                    let _text = self.renderer.paragraph(text_.as_str());
                    out.push_str(_text.as_str());
                    continue;
                }

                "text"      => {
                    body = if token.tokens.len() > 0 {
                        self.parse_inline(&mut token.tokens, Renderer::new(self.options))
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
                            self.parse_inline(&mut token.tokens, Renderer::new(self.options))
                        } else {
                            token.text.to_string()
                        };

                        body.push_str("\n");
                        body.push_str(_body.as_str());
                    }

                    let _body = self.renderer.paragraph(body.as_str());
                    out.push_str( if top { _body.as_str() } else { body.as_str() });
                    continue;
                }
                _           => {
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

    fn parse_inline(&mut self, tokens:  &mut Vec<Token>, mut renderer: Renderer) -> String {

        let mut token: &mut Token;
        let mut ret: bool = false;
        let mut out = String::from("");

        let l = tokens.len();
        for i in 0..l {
            // Run any renderer extensions
            if self.options.extensions.is_some() {
                todo!("Implement Extensions")
            }


            token = tokens.get_mut(i).unwrap();

            match token._type {
                "escape"    => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    break;
                }

                "html"    => {
                    out.push_str(renderer.html(token.text.as_str()).as_str());
                    break;
                }

                "link"    => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.link(
                        token.href.as_str(),
                        token.title.as_str(),
                        _text.as_str()
                    ).as_str());
                    break;
                }

                "image"    => {
                    out.push_str(renderer.image(
                        token.href.as_str(),
                        token.title.as_str(),
                        token.text.as_str()
                    ).as_str());
                    break;
                }

                "strong"    => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.strong(_text.as_str()).as_str());
                    break;
                }

                "em"    => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.em(_text.as_str()).as_str());
                    break;
                }

                "codespan"    => {
                    out.push_str(renderer.codespan(token.text.as_str()).as_str());
                    break;
                }

                "br"    => {
                    out.push_str(renderer.br().as_str());
                    break;
                }

                "del"    => {
                    let _text = self.parse_inline(&mut token.tokens, renderer);
                    out.push_str(renderer.del(_text.as_str()).as_str());
                    break;
                }

                "text"    => {
                    out.push_str(renderer.text(token.text.as_str()).as_str());
                    break;
                }

                _           => {
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

    fn parse_inline_tr(&mut self, mut tokens:  &mut Vec<Token>, renderer: TextRenderer) -> String {
        todo!()
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
use std::rc::Rc;
use std::cell::RefCell;


use crate::slugger::Slugger;
use crate::renderer::Renderer;
use crate::tokenizer::{Token};
use crate::lexer::{ILexer, Lexer};
use crate::parser::{IParser, Parser};
use crate::text_renderer::TextRenderer;
use crate::defaults::{Callback, get_default_options, Options};

pub struct Marked {
    pub opt: Options,
    pub parser: Parser,
    pub renderer: Renderer,
    pub text_renderer: TextRenderer,
    pub lexer: Lexer,
    pub slugger: Slugger,
}

impl Marked {

    pub fn new(opt: Option<Options>) -> Self {
        let options = if opt.is_some() { opt.unwrap() }  else { get_default_options() };
        Self {
            opt: options,
            parser: Parser::new(options),
            renderer: Renderer::new(options),
            text_renderer: TextRenderer::new(),
            lexer: Lexer::new(options),
            slugger: Slugger::new()
        }
    }

    pub fn marked(&mut self, src: &str, opt: Option<Options>, callback: Option<Callback>) -> String {
        // Skipping pre-flight checks for now

        if opt.is_none() {
            self.set_options(get_default_options());
        } else {
            self.set_options(opt.unwrap())
        }

        if callback.is_some() {
            // TODO: implement this; change callback pattern to use FnMut or observer-like pattern
        }

        // TODO: Wrap this with error handling
        let mut lexer = Lexer::new(self.opt);
        let mut tokens = lexer.lex(src);

        if self.opt.walk_tokens.is_some() {
            self.walk_tokens(&mut tokens, self.opt.walk_tokens.unwrap())
        }

        let mut parser = Parser::new(self.opt);

        parser.parse(&mut tokens, true)
    }

    pub fn parse(&mut self, src: &str, opt: Option<Options>, callback: Option<Callback>) -> String {
        // Skipping pre-flight checks for now
        self.marked(src, opt, callback)
    }

    pub fn use_(&mut self) {
        // TODO: Skipping pre-flight checks for now
    }

    pub fn set_options(&mut self, opt: Options) {
        self.opt = opt;
    }

    pub fn get_defaults(&mut self) -> &mut Options {
       &mut self.opt
    }

    pub fn walk_tokens(&mut self, mut tokens:  &mut Vec<Rc<RefCell<Token>>>, callback: Callback) {
        // Skipping pre-flight checks for now
        for token in tokens.iter_mut() {
            callback(token);

            match token.as_ref().borrow()._type {

                "table"      => {
                    for mut cell in token.as_ref().borrow_mut().header.iter_mut() {
                        self.walk_tokens(&mut cell.as_ref().borrow_mut().tokens, callback);
                    }

                    for mut row in token.as_ref().borrow_mut().rows.iter_mut() {
                        for mut rcell in row.iter_mut() {
                            self.walk_tokens(&mut rcell.as_ref().borrow_mut().tokens, callback)
                        }
                    }

                    break;
                }

                "list"      => {
                    self.walk_tokens(&mut token.as_ref().borrow_mut().items, callback);
                    break;
                }

                _       => {
                    if self.opt.extensions.is_some() {
                        // todo!("Iterate through child tokens")
                    } else if token.as_ref().borrow().tokens.len() > 0 {
                        self.walk_tokens(&mut token.as_ref().borrow_mut().tokens, callback);
                    }
                }
            }
        }

    }

    pub fn parse_inline(&mut self, src: &str, opt: Option<Options>) -> String {
        // TODO: Skipping pre-flight checks for now

        // panic::set_hook(Box::new(|_info| {
        //     if opt.silent {
        //         ret =  "<p>An error occurred:</p><pre>Unable to parse</pre>".to_string()
        //     }
        // }));

        if opt.is_none() {
            self.set_options(get_default_options());
        } else {
            self.set_options(opt.unwrap())
        }

        let mut tokens = self.lexer.lex_inline(src, self.opt);
        if self.opt.walk_tokens.is_some() {
            self.walk_tokens(&mut tokens, self.opt.walk_tokens.unwrap());
        }

        self.parser.parse_inline(&mut tokens, self.renderer)
    }
}
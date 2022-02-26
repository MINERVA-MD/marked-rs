use crate::slugger::Slugger;
use crate::defaults::Options;
use crate::renderer::Renderer;
use crate::lexer::{ILexer, Lexer};
use crate::parser::{IParser, Parser};
use crate::text_renderer::TextRenderer;
use crate::tokenizer::{Token, Tokenizer};

type Callback = fn(tokens: &mut Token);

pub struct Marked {
    pub defaults: Options,
    pub parser: Parser,
    pub renderer: Renderer,
    pub text_renderer: TextRenderer,
    pub lexer: Lexer,
    pub tokenizer: Tokenizer,
    pub slugger: Slugger,
}

impl Marked {

    pub fn new() {

    }

    pub fn marked(src: &str, opt: Options, callback: Callback) {
        // Skipping pre-flight checks for now

    }

    pub fn parse() {
        // Skipping pre-flight checks for now
    }

    pub fn use_() {
        // Skipping pre-flight checks for now
    }

    pub fn walk_tokens(&mut self, mut tokens:  &mut Vec<Token>, callback: Callback) {
        // Skipping pre-flight checks for now
        for token in tokens.iter_mut() {
            callback(token);

            match token._type {

                "table"      => {
                    break;
                }

                "list"      => {
                    self.walk_tokens(&mut token.items, callback);
                    break;
                }

                _       => {
                    if self.defaults.extensions.is_some() {
                        todo!("Iterate through child tokens")
                    } else if token.tokens.len() > 0 {
                        self.walk_tokens(&mut token.tokens, callback);
                    }
                }
            }
        }

    }

    pub fn parse_inline(&mut self, src: &str, opt: Options) -> String {
        // Skipping pre-flight checks for now
        let mut ret: String = String::from("");

        // panic::set_hook(Box::new(|_info| {
        //     if opt.silent {
        //         ret =  "<p>An error occurred:</p><pre>Unable to parse</pre>".to_string()
        //     }
        // }));

        let mut tokens = self.lexer.lex_inline(src, opt);
        if opt.walk_tokens.is_some() {
            self.walk_tokens(&mut tokens, opt.walk_tokens.unwrap());
        }
        ret = self.parser.parse_inline(&mut tokens, self.renderer);
        ret
    }
}
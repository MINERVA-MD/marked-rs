use std::io::Write;
use rand::Rng;
use regex::Regex;
use crate::defaults::{get_default_options, Options};
use crate::rules::get_default_rules;
use crate::tokenizer::{ITokenizer, Token, Tokenizer};

pub struct State {
    pub in_link: bool,
    pub in_raw_block: bool,
    pub top: bool
}

pub struct Lexer {
    pub links: Vec<String>,
    pub tokens: Vec<Token>,
    pub token_links: Vec<&'static str>,
    pub options: Options,
    pub tokenizer: Tokenizer,
    pub inline_queue: Vec<InlineToken>,
    pub state: State
}

#[derive(Clone)]
pub struct InlineToken {
    pub src: String,
    pub tokens: Vec<Token>
}

pub trait ILexer {
    fn lex(&mut self, src: &str) -> &mut Vec<Token>;
    fn block_tokens(&mut self, src: &str, tokens: Vec<Token>) -> Vec<Token>;
    fn inline_tokens(&mut self, src: &str, tokens: Vec<Token>) -> Vec<Token>;
    fn lex_inline(&mut self, src: &str, options: Options) -> Vec<Token>;
    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool;
    fn inline(&mut self, src: String, tokens: Vec<Token>);
    fn check_extensions_inline(&mut self, extensions_block: Option<&'static str>) -> bool;
}

impl Lexer {
    pub fn new(options: Options) -> Self  {
        Self {
            links: vec![],
            tokens: vec![],
            token_links: vec![],
            options,
            tokenizer: Tokenizer::new(Some(options), get_default_rules()),
            inline_queue: Default::default(),
            state: State {
                in_link: false,
                in_raw_block: false,
                top: false
            }
        }
    }

    pub fn _lex(src: &str, options: Options) -> Lexer  {
        let mut lexer = Lexer::new(options);
        lexer.lex(src);

        lexer
    }
}

impl ILexer for Lexer {

    fn lex(&mut self, src: &str) -> &mut Vec<Token> {
        let mut new_src = regx(r#"\r\n|\r"#).replace_all(src, "\n").to_string();
        new_src = regx(r#"\t"#).replace_all(new_src.as_str(), "    ").to_string();

        let mut tokens: Vec<Token> = self.block_tokens(new_src.as_str(), vec![]);

        println!("Length: {:?}", tokens);

        let mut next: InlineToken;

        if self.inline_queue.len() > 0 {
            next = self.inline_queue.remove(0);
        } else {
            return &mut self.tokens;
        }

        while self.inline_queue.len() > 0 {
            self.inline_tokens(next.src.as_str(), vec![]);
            next = self.inline_queue.remove(0);
        }
        &mut self.tokens
    }

    fn block_tokens(&mut self, src: &str, mut tokens: Vec<Token>) -> Vec<Token> {

        let mut _src: String = String::from(src);
        if self.options.pedantic {
            _src = regx(r#"(?m)^ +$"#).replace_all(src, "").to_string();
        }

        let mut token: Option<Token>;
        let mut last_token: &mut Token;
        let mut cut_src: String;

        while _src.len() > 0 {
            println!("Current String: {} +++++++++++++++++++++++++++++++++++++", _src);
            if self.options.extensions.is_some()
            && self.check_extensions_block(self.options.extensions)
            {
                continue;
            }

            // newline
            token = self.tokenizer.space(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if idx == 1 && tokens.len() > 0 {
                    // if there's a single \n as a spacer, it's terminating the last line,
                    // so move it there so that we don't get unnecessary paragraph tags
                    let t_idx = tokens.len() - 1;
                    tokens[t_idx].append_to_raw("\n");

                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // code
            token = self.tokenizer.code(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;
                let q_idx = self.inline_queue.len() - 1;

                last_token = &mut tokens[t_idx];
                _src = String::from(&_src[idx..]);

                if last_token._type == "paragraph" ||
                    last_token._type == "text"
                {
                    last_token.append_to_raw("\n");
                    last_token.append_to_raw(_token.raw.as_str());

                    last_token.append_to_text("\n");
                    last_token.append_to_text(_token.text.as_str());

                    self.inline_queue[q_idx].src = last_token.text.to_string();
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // fences
            token = self.tokenizer.fences(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // heading
            token = self.tokenizer.heading(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // hr
            token = self.tokenizer.hr(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // blockquote
            token = self.tokenizer.blockquote(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // list
            token = self.tokenizer.list(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // html
            token = self.tokenizer.html(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // def
            token = self.tokenizer.def(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;
                let q_idx = self.inline_queue.len() - 1;

                last_token = &mut tokens[t_idx];
                _src = String::from(&_src[idx..]);

                if last_token._type == "paragraph" ||
                    last_token._type == "text"
                {
                    last_token.append_to_raw("\n");
                    last_token.append_to_raw(_token.raw.as_str());

                    last_token.append_to_text("\n");
                    last_token.append_to_text(_token.text.as_str());

                    self.inline_queue[q_idx].src = last_token.text.to_string();
                } else if _token.tag <= 0 || _token.tag > t_idx {
                    // tokens.push(_token);
                    todo!()
                }
                continue;
            }


            // table (gfm)
            token = self.tokenizer.table(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // lheading
            token = self.tokenizer.lheading(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // top-level paragraph
            // prevent paragraph consuming extensions by clipping 'src' to extension start
            cut_src = _src.clone();
            if self.options.extensions.is_some() {
                todo!("Implement logic for top-level paragraph");
            }


            // text
            token = self.tokenizer.text(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;
                let q_idx = self.inline_queue.len() - 1;

                last_token = &mut tokens[t_idx];
                _src = String::from(&_src[idx..]);

                if last_token._type == "text"
                {
                    last_token.append_to_raw("\n");
                    last_token.append_to_raw(_token.raw.as_str());

                    last_token.append_to_text("\n");
                    last_token.append_to_text(_token.text.as_str());

                    self.inline_queue[q_idx].src = last_token.text.to_string();
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            if _src.len() > 0 {
                let err_msg = format!("Infinite loop on byte:  {}", _src.chars().nth(0).unwrap() as u32);

                if self.options.silent {
                    println!("Warning! {}", err_msg);
                    break;
                } else {
                    panic!("{}", err_msg);
                }
            }
        }
        self.state.top = true;
        return tokens;
    }

    fn inline_tokens(&mut self, src: &str, mut tokens: Vec<Token>) -> Vec<Token> {
        // todo!();
        // Mask out reflinks
        if self.links.len() > 0 {
            todo!();
        }

        // todo!("Mask out other blocks");
        // todo!("Mask out escaped em & strong delimiters");

        let mut _src: String = String::from(src);
        // todo!("Check this initialization");
        let mut _cut_src: String = String::from("");
        let mut _masked_src: String = String::from(src);

        let mut prev_char: String = "".to_string();
        let mut _match: Vec<&str>;
        let mut token: Option<Token>;
        let mut last_token: &mut Token;
        let mut _keep_prev_char: bool = false;

        while _src.len() > 0 {

            if !_keep_prev_char {
                prev_char = "".to_string();
            }
            _keep_prev_char = false;

            if self.options.extensions.is_some()
                && self.check_extensions_inline(self.options.extensions)
            {
                continue;
            }


            // escape
            token = self.tokenizer.escape(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // tag
            token = self.tokenizer.tag(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;

                last_token = &mut tokens[t_idx];
                _src = String::from(&_src[idx..]);

                if _token._type == "text" ||
                    last_token._type == "text"
                {
                    last_token.append_to_raw(_token.raw.as_str());
                    last_token.append_to_raw(_token.text.as_str());
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // link
            token = self.tokenizer.link(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // reflink, nolink
            token = self.tokenizer.ref_link(src, &self.links);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;

                last_token = &mut tokens[t_idx];
                _src = String::from(&_src[idx..]);

                if _token._type == "text" ||
                    last_token._type == "text"
                {
                    last_token.append_to_raw(_token.raw.as_str());
                    last_token.append_to_raw(_token.text.as_str());
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // em & strong
            token = self.tokenizer.em_strong(src, _masked_src.as_str(), prev_char.to_string().as_str());
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // code
            token = self.tokenizer.code_span(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // br
            token = self.tokenizer.br(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // del (gfm)
            token = self.tokenizer.del(src);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // autolink
            token = self.tokenizer.autolink(src, mangle);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // url (gfm)
            token = self.tokenizer.url(src, mangle);
            if !self.state.in_link && token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // text
            // prevent inlineText consuming extensions by clipping 'src' to extension start
            _cut_src = _src.clone();
            if self.options.extensions.is_some() {
                // todo!("Implement logic to avoid clipping src");
            }

            // Inline Text
            token = self.tokenizer.inline_text(_cut_src.as_str(), smartypants);
            if token.is_some() {
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;

                _src = String::from(&_src[idx..]);

                let last_char = _token.raw.chars().last().unwrap();
                if last_char != '_' {
                    prev_char = last_char.to_string();
                }

                _keep_prev_char = true;
                last_token = &mut tokens[t_idx];


                if last_token._type == "text"
                {
                    last_token.append_to_raw(_token.raw.as_str());
                    last_token.append_to_raw(_token.text.as_str());
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            if _src.len() > 0 {
                let err_msg = format!("Infinite loop on byte:  {}", _src.chars().nth(0).unwrap() as u32);

                if self.options.silent {
                    println!("Warning! {}", err_msg);
                    break;
                } else {
                    panic!("{}", err_msg);
                }
            }
        }
        return tokens;
    }

    fn lex_inline(&mut self, src: &str, options: Options) -> Vec<Token> {
        let mut lexer = Lexer::new(options);
        return lexer.inline_tokens(src, vec![]);
    }

    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool {
        return true;
    }

    fn inline(&mut self, src: String, tokens: Vec<Token>) {
        self.inline_queue.push(
            InlineToken {
                src,
                tokens
            }
        )
    }

    fn check_extensions_inline(&mut self, extensions_inline: Option<&'static str>) -> bool {
        return true;
    }

}


pub fn regx(regex: &str) -> Regex {
    return Regex::new(regex).unwrap();
}

/**
 * smartypants text replacement
 */
pub fn smartypants(text: &str) -> String {
    let mut ret_text = text
        // em-dashes
        .replace("---", "\u{2014}")
        // en-dashes
        .replace("--", "\u{2013}");

    // opening singles
    ret_text = regx(r#"(^|[-\u2014/(\[{"\s])'"#).replace_all(ret_text.as_str(), "${1}\u{2018}").to_string();
    // closing singles & apostrophes
    ret_text = ret_text.replace("'", "\u{2019}");
    // opening doubles
    ret_text = regx(r#"(^|[-\u2014/(\[{\u2018\s])""#).replace_all(ret_text.as_str(), "${1}\u{201c}").to_string();
    // closing doubles
    ret_text = ret_text.replace(r#"""#, "\u{201d}");
    // ellipses
    ret_text = regx(r#"\.{3}"#).replace_all(ret_text.as_str(), "\u{2026}").to_string();

    ret_text
}

/**
 * mangle email addresses
 */
pub fn mangle(text: &str) -> String {
    // let mut chars = text.chars();
    let mut out = String::new();
    let n = text.chars().count();
    let mut rng = rand::thread_rng();

    for i in 0..n {
        let mut ch: String = String::new();
        let mut c = text.chars().nth(i).unwrap() as u32;

        if rng.gen::<f64>() > 0.50 {
            ch.push_str(&format!("x{:x}", c));
        } else {
            ch.push_str(&format!("{}", c))
        }

        out.push_str(&format!("&#{};", ch ));
    }

    out
}
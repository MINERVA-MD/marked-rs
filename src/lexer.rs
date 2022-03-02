use std::io::Write;
use rand::Rng;
use regex::Regex;
use crate::defaults::{get_default_options, Options};
use crate::rules::{get_default_rules, get_rules};
use crate::tokenizer::{ITokenizer, Token, Tokenizer};

pub struct State {
    pub in_link: bool,
    pub in_raw_block: bool,
    pub top: bool
}

pub struct Lexer<'a> {
    pub links: Vec<String>,
    pub tokens: Vec<Token>,
    pub token_links: Vec<&'static str>,
    pub options: Options,
    pub tokenizer: Tokenizer,
    pub inline_queue: Vec<InlineToken<'a>>,
    pub state: State
}

#[derive(Clone, Debug)]
pub struct InlineToken<'a> {
    pub src: String,
    pub tokens: &'a mut Vec<Token>
}

pub trait ILexer<'a> {
    fn lex(&mut self, src: &str) -> &mut Vec<Token>;
    fn block_tokens(&mut self, src: &str, tokens: &mut Vec<Token>, opt: &str);
    fn inline_tokens(&mut self, src: &str, tokens: &mut Vec<Token>) -> &mut Vec<Token>;
    fn lex_inline(&mut self, src: &str, options: Options) -> &mut Vec<Token>;
    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool;
    fn inline(&mut self, src: String, tokens: &mut Vec<Token>);
    fn check_extensions_inline(&mut self, extensions_block: Option<&'static str>) -> bool;
}


impl Lexer<'a> {
    pub fn new(options: Options) -> Self  {
        Self {
            links: vec![],
            tokens: vec![],
            token_links: vec![],
            options,
            tokenizer: Tokenizer::new(Some(options), get_rules(options)),
            inline_queue: Default::default(),
            state: State {
                in_link: false,
                in_raw_block: false,
                top: true
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

        self.block_tokens(new_src.as_str(), &mut vec![], "self");

        let mut next: InlineToken;

        loop {
            if self.inline_queue.len() == 0 {
                break;
            } else {
                next = self.inline_queue.remove(0);
                self.inline_tokens(next.src.as_str(), next.tokens);
            }
        }

        &mut self.tokens
    }

    fn block_tokens(&mut self, src: &str, mut tokens: &mut Vec<Token>, opt: &str) {

        let mut _src: String = String::from(src);
        if self.options.pedantic {
            _src = regx(r#"(?m)^ +$"#).replace_all(_src.as_str(), "").to_string();
        }

        let mut token: Option<Token>;
        let mut last_token: &mut Token;
        let mut cut_src: String;
        let mut last_paragraph_clipped = false;

        while _src.len() > 0 {
            if self.options.extensions.is_some()
            && self.check_extensions_block(self.options.extensions)
            {
                continue;
            }

            // newline
            token = self.tokenizer.space(_src.as_str());
            if token.is_some() {
                println!("Entered Newline/Space Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();

                _src = String::from(&_src[idx..]);

                if idx == 1 && tokens.len() > 0 {
                    // if there's a single \n as a spacer, it's terminating the last line,
                    // so move it there so that we don't get unnecessary paragraph tags
                    let t_idx = tokens.len() - 1;
                    tokens.get_mut(t_idx).unwrap().raw.push_str("\n");
                } else {
                    if opt == "self" {
                        self.tokens.push(_token)
                    } else {
                        tokens.push(_token);
                    }
                }
                continue;
            }

            // code
            token = self.tokenizer.code(_src.as_str());
            if token.is_some() {
                println!("Entered Code Block");
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
                    if opt == "self" {
                        self.tokens.push(_token)
                    } else {
                        tokens.push(_token);
                    }
                }
                continue;
            }


            // fences
            token = self.tokenizer.fences(_src.as_str());
            if token.is_some() {
                println!("Entered Fences Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // heading
            token = self.tokenizer.heading(_src.as_str());
            if token.is_some() {
                println!("Entered Heading Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // hr
            token = self.tokenizer.hr(_src.as_str());
            if token.is_some() {
                println!("Entered Hr Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // blockquote
            token = self.tokenizer.blockquote(_src.as_str());
            if token.is_some() {
                println!("Entered Blockquote Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // list
            token = self.tokenizer.list(_src.as_str());
            if token.is_some() {
                println!("Entered List Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // html
            token = self.tokenizer.html(_src.as_str());
            if token.is_some() {
                println!("Entered HTML Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // def
            token = self.tokenizer.def(_src.as_str());
            if token.is_some() {
                println!("Entered Def Block");
                let _token = token.unwrap();

                if tokens.len() > 0 {

                    let idx = _token.raw.len();
                    let t_idx = tokens.len() - 1;

                    last_token = &mut tokens[t_idx];
                    _src = String::from(&_src[idx..]);

                    if last_token._type == "paragraph" ||
                        last_token._type == "text"
                    {
                        last_token.append_to_raw("\n");
                        last_token.append_to_raw(_token.raw.as_str());

                        last_token.append_to_text("\n");
                        last_token.append_to_text(_token.text.as_str());

                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue[q_idx].src = last_token.text.to_string();
                    } else if true  {
                        // tokens.push(_token);
                        // TODO: check if link with token.tag exist
                    }

                } else if true {

                }
                continue;
            }


            // table (gfm)
            token = self.tokenizer.table(_src.as_str());
            if token.is_some() {
                println!("Entered Table (GFM) Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // lheading
            token = self.tokenizer.lheading(_src.as_str());
            if token.is_some() {
                println!("Entered LHeading Block");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if opt == "self" {
                    self.tokens.push(_token)
                } else {
                    tokens.push(_token);
                }
                continue;
            }

            // top-level paragraph
            // prevent paragraph consuming extensions by clipping 'src' to extension start
            cut_src = _src.clone();
            if self.options.extensions.is_some() {
                // todo!("Implement logic for top-level paragraph");
            }

            // paragraph
            token = self.tokenizer.paragraph(cut_src.as_str());
            if self.state.top &&
                token.is_some()
            {
                println!("Entered Paragraph Block");
                let mut _token = token.unwrap();
                self.inline(String::from(_token.text.as_str()), &mut _token.tokens);

                let idx = _token.raw.len();

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;
                    last_token = tokens.get_mut(t_idx).unwrap();

                    if last_paragraph_clipped &&
                        last_token._type == "paragraph" {

                        last_token.append_to_raw("\n");
                        last_token.append_to_raw(_token.raw.as_str());

                        last_token.append_to_text("\n");
                        last_token.append_to_text(_token.text.as_str());

                        self.inline_queue.remove(self.inline_queue.len() - 1);

                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue.get_mut(q_idx).unwrap().src = last_token.text.to_string();
                    } else {
                        if opt == "self" {
                            self.tokens.push(_token)
                        } else {
                            tokens.push(_token);
                        }
                    }
                } else {
                    if opt == "self" {
                        self.tokens.push(_token)
                    } else {
                        tokens.push(_token);
                    }
                }

                last_paragraph_clipped = cut_src.len() != _src.len();
                _src = String::from(&_src[idx..]);
                continue;
            }


            // text
            token = self.tokenizer.text(_src.as_str());
            if token.is_some() {
                println!("Entered Text Block");

                let mut _token = token.unwrap();
                self.inline(String::from(_token.text.as_str()), &mut _token.tokens);

                let idx = _token.raw.len();

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    last_token = tokens.get_mut(t_idx).unwrap();
                    _src = String::from(&_src[idx..]);

                    if last_token._type == "text"
                    {
                        last_token.append_to_raw("\n");
                        last_token.append_to_raw(_token.raw.as_str());

                        last_token.append_to_text("\n");
                        last_token.append_to_text(_token.text.as_str());

                        self.inline_queue.remove(self.inline_queue.len() - 1);
                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue[q_idx].src = last_token.text.to_string();
                    } else {
                        if opt == "self" {
                            self.tokens.push(_token)
                        } else {
                            tokens.push(_token);
                        }
                    }
                } else {
                    if opt == "self" {
                        self.tokens.push(_token)
                    } else {
                        tokens.push(_token);
                    }
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
        // return tokens;
    }

    fn inline_tokens(&mut self, src: &str, mut tokens: &mut Vec<Token>) -> &mut Vec<Token> {
        // todo!();
        // Mask out reflinks
        if self.links.len() > 0 {
            // todo!();
        }


        // println!("Top: \n{:?}", tokens);

        // todo!("Mask out other blocks");
        // todo!("Mask out escaped em & strong delimiters");

        let mut _src: String = String::from(src);
        // todo!("Check this initialization");
        let mut _cut_src: String = String::from("");
        let mut _masked_src: String = String::from(src);

        let mut prev_char: String = "".to_string();
        let mut _match: Vec<&str>;
        let mut token: Option<Token>;
        let mut last_token: Token;
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
            token = self.tokenizer.escape(_src.as_str());
            if token.is_some() {
                println!("Inside Escape");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // tag
            token = self.tokenizer.tag(_src.as_str());
            if token.is_some() {
                println!("Inside Tag");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;

                let _last_token = tokens.get(t_idx).unwrap();
                last_token = _last_token.clone();
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
            token = self.tokenizer.link(_src.as_str());
            if token.is_some() {
                println!("Inside Link");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // reflink, nolink
            token = self.tokenizer.ref_link(_src.as_str(), &self.links);
            if token.is_some() {
                println!("Inside Reflink/Nolink");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                let t_idx = tokens.len() - 1;

                let _last_token = tokens.get(t_idx).unwrap();
                last_token = _last_token.clone();
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
            token = self.tokenizer.em_strong(_src.as_str(), _masked_src.as_str(), prev_char.to_string().as_str());
            if token.is_some() {
                println!("Inside Em/Strong");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // code
            token = self.tokenizer.code_span(_src.as_str());
            if token.is_some() {
                println!("Inside Code Span");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // br
            token = self.tokenizer.br(_src.as_str());
            if token.is_some() {
                println!("Inside Br");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // del (gfm)
            token = self.tokenizer.del(_src.as_str());
            if token.is_some() {
                println!("Inside Del");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // autolink
            token = self.tokenizer.autolink(_src.as_str(), mangle);
            if token.is_some() {
                println!("Inside Autolink");
                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }


            // url (gfm)
            token = self.tokenizer.url(_src.as_str(), mangle);
            if !self.state.in_link && token.is_some() {
                println!("Inside Url");
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
                println!("Entered Inline Text");

                let _token = token.unwrap();
                let idx = _token.raw.len();
                _src = String::from(&_src[idx..]);

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;


                    let last_char = _token.raw.chars().last().unwrap();
                    if last_char != '_' {
                        prev_char = last_char.to_string();
                    }

                    _keep_prev_char = true;
                    let _last_token = tokens.get(t_idx).unwrap();
                    last_token = _last_token.clone();

                    if last_token._type == "text"
                    {
                        last_token.append_to_raw(_token.raw.as_str());
                        last_token.append_to_raw(_token.text.as_str());
                    } else {
                        tokens.push(_token);
                    }
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

    fn lex_inline(&mut self, src: &str, options: Options) -> &mut Vec<Token> {
        let mut lexer = Lexer::new(options);
        return &mut lexer.inline_tokens(src, &mut vec![]);
    }

    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool {
        return true;
    }

    fn inline(&mut self, src: String, tokens: &mut Vec<Token>) {
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
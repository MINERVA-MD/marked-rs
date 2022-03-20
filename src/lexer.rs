use rand::Rng;
use std::rc::Rc;
use regex::Regex;
use std::io::Write;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};


use crate::token::*;
use crate::helpers::repeat_string;
use crate::defaults::{get_default_options, Options};
use crate::rules::{get_default_rules, get_rules, MDInline};
use crate::token;
use crate::tokenizer::{ITokenizer, Link, Token, Tokenizer};

pub struct State {
    pub in_link: bool,
    pub in_raw_block: bool,
    pub top: bool
}

pub struct Lexer {
    pub state: State,
    pub links: Vec<Link>,
    pub options: Options,
    pub tokenizer: Tokenizer,
    pub tokens: Vec<Rc<RefCell<Token>>>,
    pub inline_queue: Vec<InlineToken>,
}


#[derive(Clone, Debug)]
pub struct InlineToken {
    pub src: String,
    pub token: Rc<RefCell<Token>>,
}

pub trait ILexer {
    fn lex_ac<'a>(&mut self, src: &str) -> Vec<token::Token>;
    fn lex<'a>(&mut self, src: &str) -> Vec<Rc<RefCell<Token>>>;
    fn inline(&mut self, src: &str, token: Rc<RefCell<Token>>);
    fn lex_inline(&mut self, src: &str, options: Options) -> Vec<Rc<RefCell<Token>>>;
    fn block_tokens<'a>(&mut self, src: &str, tokens: &'a mut Vec<Rc<RefCell<Token>>>) -> &'a mut Vec<Rc<RefCell<Token>>>;
    fn inline_tokens<'a>(&mut self, src: &str, tokens: &'a mut Vec<Rc<RefCell<Token>>>) -> &'a mut Vec<Rc<RefCell<Token>>>;
    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool;
    fn check_extensions_inline(&mut self, extensions_block: Option<&'static str>) -> bool;
}


impl Lexer {
    pub fn new(options: Options) -> Self  {
        Self {
            links: vec![],
            tokens: vec![],
            options,
            tokenizer: Tokenizer::new(Some(options)),
            inline_queue: vec![],
            state: State {
                in_link: false,
                in_raw_block: false,
                top: true
            }
        }
    }

    pub fn get_links(&mut self) -> Vec<Link> {
        self.links.clone()
    }

    pub fn _lex(src: &str, options: Options) -> Lexer  {
        let mut lexer = Lexer::new(options);
        lexer.lex(src);
        lexer
    }

    pub fn get_tokens(&mut self) -> Vec<Token> {
        let tokens: Vec<Token> = self.tokens
            .iter_mut()
            .map(|t_rc| {
                let token_rc = t_rc
                    .as_ref()
                    .borrow_mut();

                Token {
                    _type: token_rc._type.clone(),
                    raw: token_rc.raw.clone(),
                    href: token_rc.href.clone(),
                    title: token_rc.title.clone(),
                    text: token_rc.text.clone(),
                    tokens: token_rc.tokens.to_owned(),
                    tag: token_rc.tag.clone(),
                    ordered: token_rc.ordered.clone(),
                    start: token_rc.start.clone(),
                    lang: token_rc.lang.clone(),
                    loose: token_rc.loose.clone(),
                    items: token_rc.items.to_owned(),
                    depth: token_rc.depth.clone(),
                    escaped: token_rc.escaped.clone(),
                    pre: token_rc.pre.clone(),
                    task: token_rc.task.clone(),
                    checked: token_rc.checked.clone(),
                    in_link: token_rc.in_link.clone(),
                    in_raw_block: token_rc.in_raw_block.clone(),
                    links: token_rc.links.to_owned(),
                    align: token_rc.align.to_owned(),
                    rows: token_rc.rows.to_owned(),
                    header: token_rc.header.to_owned(),
                    code_block_style: token_rc.code_block_style.clone(),
                }

            })
            .collect();

        tokens
    }

    pub fn capture_tokens(&mut self) -> Vec<token::Token> {
        let mut tokens: Vec<token::Token> = vec![];

        for mut token in self.tokens.iter_mut() {
            Lexer::capture_tokens_helper_ac(token, &mut tokens)
        }

        tokens
    }

    pub fn capture_tokens_ac(tokens: &mut Vec<Rc<RefCell<Token>>> ) -> Vec<token::Token> {
        let mut tokens_ac: Vec<token::Token> = vec![];

        for mut token in tokens.iter_mut() {
            Lexer::capture_tokens_helper_ac(token, &mut tokens_ac)
        }

        tokens_ac
    }

    pub fn capture_tokens_helper_ac(token: &mut Rc<RefCell<Token>>, mut tokens: &mut Vec<token::Token>)  {

        let mut token_rc = token.as_ref().borrow_mut();

        let mut token_ac = token::Token {
            _type: token_rc._type.clone(),
            raw: token_rc.raw.clone(),
            href: token_rc.href.clone(),
            title: token_rc.title.clone(),
            text: token_rc.text.clone(),
            tokens: vec![],
            tag: token_rc.tag.clone(),
            ordered: token_rc.ordered.clone(),
            start: token_rc.start.clone(),
            lang: token_rc.lang.clone(),
            loose: token_rc.loose.clone(),
            items: vec![],
            depth: token_rc.depth.clone(),
            escaped: token_rc.escaped.clone(),
            pre: token_rc.pre.clone(),
            task: token_rc.task.clone(),
            checked: token_rc.checked.clone(),
            in_link: token_rc.in_link.clone(),
            in_raw_block: token_rc.in_raw_block.clone(),
            links: token_rc.links.clone(),
            align: token_rc.align.clone(),
            rows: vec![],
            header: vec![],
            code_block_style: token_rc.code_block_style.clone()
        };

        {
            // Tokens
            for mut token_t in token_rc.tokens.iter_mut() {
                Lexer::capture_tokens_helper_ac(&mut token_t, &mut token_ac.tokens)
            }
        }

        {
            // Items
            for mut token_i in token_rc.items.iter_mut() {
                Lexer::capture_tokens_helper_ac(&mut token_i, &mut token_ac.items)
            }
        }

        {
            // Rows
            for mut token_r in token_rc.rows.iter_mut() {
                let mut rows: Vec<token::Token> = vec![];
                for mut row in token_r.iter_mut(){
                    Lexer::capture_tokens_helper_ac(&mut row, &mut rows);
                }
                token_ac.rows.push(rows)
            }
        }

        {
            // Header
            for mut token_h in token_rc.header.iter_mut() {
                Lexer::capture_tokens_helper_ac(&mut token_h, &mut token_ac.header)
            }
        }

        tokens.push(token_ac);
    }

    pub fn capture_tokens_rc(tokens: &mut Vec<token::Token> ) -> Vec<Rc<RefCell<Token>>> {
        let mut tokens_rc: Vec<Rc<RefCell<Token>>> = vec![];

        for mut token in tokens.iter_mut() {
            Lexer::capture_tokens_helper_rc(token, &mut tokens_rc)
        }

        tokens_rc
    }

    pub fn capture_tokens_helper_rc(token: &mut token::Token, mut tokens: &mut Vec<Rc<RefCell<Token>>>)  {

        let mut token_ac = token;

        let mut token_rc = Token {
            _type: token_ac._type.clone(),
            raw: token_ac.raw.clone(),
            href: token_ac.href.clone(),
            title: token_ac.title.clone(),
            text: token_ac.text.clone(),
            tokens: vec![],
            tag: token_ac.tag.clone(),
            ordered: token_ac.ordered.clone(),
            start: token_ac.start.clone(),
            lang: token_ac.lang.clone(),
            loose: token_ac.loose.clone(),
            items: vec![],
            depth: token_ac.depth.clone(),
            escaped: token_ac.escaped.clone(),
            pre: token_ac.pre.clone(),
            task: token_ac.task.clone(),
            checked: token_ac.checked.clone(),
            in_link: token_ac.in_link.clone(),
            in_raw_block: token_ac.in_raw_block.clone(),
            links: token_ac.links.clone(),
            align: token_ac.align.clone(),
            rows: vec![],
            header: vec![],
            code_block_style: token_ac.code_block_style.clone()
        };

        {
            // Tokens
            for mut token_t in token_ac.tokens.iter_mut() {
                Lexer::capture_tokens_helper_rc(&mut token_t, &mut token_rc.tokens)
            }
        }

        {
            // Items
            for mut token_i in token_ac.items.iter_mut() {
                Lexer::capture_tokens_helper_rc(&mut token_i, &mut token_rc.items)
            }
        }

        {
            // Rows
            for mut token_r in token_ac.rows.iter_mut() {
                let mut rows: Vec<Rc<RefCell<Token>>> = vec![];
                for mut row in token_r.iter_mut(){
                    Lexer::capture_tokens_helper_rc(&mut row, &mut rows);
                }
                token_rc.rows.push(rows)
            }
        }

        {
            // Header
            for mut token_h in token_ac.header.iter_mut() {
                Lexer::capture_tokens_helper_rc(&mut token_h, &mut token_rc.header)
            }
        }

        tokens.push(Rc::new(RefCell::new(
            token_rc
        )));
    }
}

impl ILexer for Lexer {

    fn lex<'a>(&mut self, src: &str) -> Vec<Rc<RefCell<Token>>> {
        let mut new_src = regx(r#"\r\n|\r"#).replace_all(src, "\n").to_string();
        new_src = regx(r#"\t"#).replace_all(new_src.as_str(), "    ").to_string();

        let mut _tokens = vec![];
        let mut tokens: &mut Vec<Rc<RefCell<Token>>> = self.block_tokens(new_src.as_str(), &mut _tokens);


        while self.inline_queue.len() > 0 {
            let next = self.inline_queue.remove(0);
            let i_tokens = &mut next.token.as_ref().borrow_mut().tokens;
            let inline_tokens = self.inline_tokens(
                next.src.as_str(),
                i_tokens
            );
        }

        self.tokens.append(&mut tokens);
        self.tokens.clone()
    }

    fn lex_ac<'a>(&mut self, src: &str) -> Vec<token::Token> {
        let mut new_src = regx(r#"\r\n|\r"#).replace_all(src, "\n").to_string();
        new_src = regx(r#"\t"#).replace_all(new_src.as_str(), "    ").to_string();

        let mut _tokens = vec![];
        let mut tokens: &mut Vec<Rc<RefCell<Token>>> = self.block_tokens(new_src.as_str(), &mut _tokens);


        // // println!("{:#?}", tokens);

        while self.inline_queue.len() > 0 {
            let next = self.inline_queue.remove(0);
            let i_tokens = &mut next.token.as_ref().borrow_mut().tokens;
            let inline_tokens = self.inline_tokens(
                next.src.as_str(),
                i_tokens
            );
            // println!("Tokens for {} ==> {:#?}", next.src.as_str(), i_tokens);
        }

        // println!("{:#?}", tokens);

        self.tokens.append(&mut tokens);
        self.capture_tokens()
    }

    fn inline(&mut self, src: &str, mut token: Rc<RefCell<Token>>) {
        self.inline_queue.push(InlineToken {
            src: src.to_string(),
            token
        });
    }

    fn lex_inline(&mut self, src: &str, options: Options) -> Vec<Rc<RefCell<Token>>> {
        let mut lexer = Lexer::new(options);

        let mut inline_tokens = vec![];
        lexer.inline_tokens(src, &mut inline_tokens);

        inline_tokens
    }

    fn block_tokens<'a>(&mut self, src: &str, mut tokens: &'a mut Vec<Rc<RefCell<Token>>>) ->  &'a mut Vec<Rc<RefCell<Token>>>{

        let mut _src: String = String::from(src);
        if self.options.pedantic {
            _src = regx(r#"(?m)^ +$"#).replace_all(_src.as_str(), "").to_string();
        }

        let mut cut_src: String;
        let mut last_token: Token;
        let mut token: Option<Token>;

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
                // println!("Entered Newline/Space Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();

                _src = String::from(&_src[idx..]);

                if idx == 1 && tokens.len() > 0 {
                    // if there's a single \n as a spacer, it's terminating the last line,
                    // so move it there so that we don't get unnecessary paragraph tags

                    let t_idx = tokens.len() - 1;
                    tokens.get_mut(t_idx).unwrap()
                        .as_ref()
                        .borrow_mut()
                        .raw.push_str("\n");

                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // code
            token = self.tokenizer.code(_src.as_str());
            if token.is_some() {
                // println!("Entered Code Block");
                let mut _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();

                _src = String::from(&_src[idx..]);

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    let is_paragraph = tokens.get(t_idx).unwrap().as_ref().borrow()._type == "paragraph";
                    let is_text = tokens.get(t_idx).unwrap().as_ref().borrow()._type == "text";

                    if  is_paragraph || is_text
                    {
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw(_token.as_ref().borrow_mut().raw.as_str());

                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text(_token.as_ref().borrow_mut().text.as_str());

                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue[q_idx].src = tokens.get(t_idx).unwrap().as_ref().borrow().text.to_string();
                    } else {
                        tokens.push(_token);
                    }
                } else {
                    tokens.push(_token);
                }
                continue;
            }


            // fences
            token = self.tokenizer.fences(_src.as_str());
            if token.is_some() {
                // println!("Entered Fences Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();

                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // heading
            token = self.tokenizer.heading(_src.as_str());
            if token.is_some() {
                // println!("Entered Heading Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();

                _src = String::from(&_src[idx..]);

                // TODO: This is the incorrect call here - call should be made to self.inline instead
                self.inline(_token.as_ref().borrow().text.as_str(), Rc::clone(&_token));

                tokens.push(_token);
                continue;
            }

            // hr
            token = self.tokenizer.hr(_src.as_str());
            if token.is_some() {
                // println!("Entered Hr Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(_token);
                continue;
            }

            // blockquote
            token = self.tokenizer.blockquote(_src.as_str());
            if token.is_some() {

                let mut blockquote_token = token.unwrap();

                // Add block tokens
                let mut block_tokens: Vec<Rc<RefCell<Token>>> = vec![];
                self.block_tokens( blockquote_token.text.as_str(), &mut block_tokens);

                blockquote_token.tokens.append(&mut block_tokens);

                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    blockquote_token
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                // println!("Entered Blockquote Block");

                tokens.push(_token);
                continue;
            }

            // list
            token = self.tokenizer.list(_src.as_str());
            if token.is_some() {
                // println!("Entered List Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);


                let l = _token.as_ref().borrow().items.len();
                // Item child tokens handled here at end because we needed to have the final item to trim it first

                for i in 0..l {
                    self.state.top = false;

                    let mut block_tokens: Vec<Rc<RefCell<Token>>> = vec![];
                    self.block_tokens(_token.as_ref().borrow_mut().items[i].as_ref().borrow().text.as_str(),
                                      &mut block_tokens
                    );

                    _token.as_ref().borrow_mut().items[i].as_ref().borrow_mut().tokens.append(&mut block_tokens);

                    let spacers: Vec<Rc<RefCell<Token>>> = _token.as_ref().borrow_mut().items[i].as_ref().borrow().tokens.clone()
                        .into_iter()
                        .filter(|t| t.as_ref().borrow()._type == "space")
                        .collect();

                    let has_multiple_line_breaks = spacers.iter()
                        .all(|t| {
                            let chars: Vec<String> = t.as_ref().borrow().raw.split("")
                                .map(|x| x.to_string())
                                .collect();

                            let mut line_breaks: usize = 0;
                            for char in chars.iter() {
                                if char == "\n" {
                                    line_breaks += 1;
                                }
                                if line_breaks > 1 {
                                    return true
                                }
                            }
                            return false;
                        });

                    if !_token.as_ref().borrow().loose &&
                        spacers.len() > 0 &&
                        has_multiple_line_breaks
                    {
                        _token.as_ref().borrow_mut().loose = true;
                        _token.as_ref().borrow_mut().items[i].as_ref().borrow_mut().loose = true;
                    }
                }

                tokens.push(_token);
                continue;
            }


            // html
            token = self.tokenizer.html(_src.as_str());
            if token.is_some() {
                // println!("Entered HTML Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                if self.options.sanitize {
                    // TODO: This is the incorrect call here - call should be made to self.inline instead
                    self.inline(_token.as_ref().borrow().text.as_str(), Rc::clone(&_token));
                }

                tokens.push(_token);
                continue;
            }


            // def
            token = self.tokenizer.def(_src.as_str());
            if token.is_some() {
                // println!("Entered Def Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                let link_idx = self.links.iter().position(|l| l.tag ==  _token.as_ref().borrow().tag );
                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    let mut __last_token = tokens.get_mut(t_idx).unwrap();
                    let mut _last_token = __last_token
                        .as_ref()
                        .borrow_mut();

                    if _last_token._type == "paragraph" ||
                        _last_token._type == "text"
                    {
                        _last_token.append_to_raw("\n");
                        _last_token.append_to_raw(_token.as_ref().borrow_mut().raw.as_str());

                        _last_token.append_to_text("\n");
                        _last_token.append_to_text(_token.as_ref().borrow_mut().text.as_str());

                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue[q_idx].src = _last_token.text.to_string();
                    } else if link_idx.is_none()  {
                        self.links.push(Link {
                            href:  _token.as_ref().borrow().href.to_string(),
                            title:  _token.as_ref().borrow().title.to_string(),
                            tag:  _token.as_ref().borrow().tag.to_string()
                        });
                    }

                } else if link_idx.is_none() {
                    self.links.push(Link {
                        href:  _token.as_ref().borrow().href.to_string(),
                        title:  _token.as_ref().borrow().title.to_string(),
                        tag:  _token.as_ref().borrow().tag.to_string()
                    });
                }
                continue;
            }


            // table (gfm)
            let mut inline_tokens = &mut vec![];
            token = self.tokenizer.table(_src.as_str(), inline_tokens);
            if token.is_some() {
                // println!("Entered Table (GFM) Block");

                // Process inline tokens for headers and rows
                let mut table_token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                {
                    let token_rc = table_token.as_ref().borrow_mut();

                    let idx = token_rc.raw.len();
                    _src = String::from(&_src[idx..]);

                    let mut l = token_rc.header.len();
                    for j in 0..l {
                        let mut j_tokens = vec![];
                        self.inline_tokens(token_rc.header[j].as_ref().borrow().text.as_str(), &mut j_tokens);


                        {
                            token_rc.header[j].as_ref().borrow_mut().tokens = j_tokens;
                        }

                    }

                    l = token_rc.rows.len();
                    for j in 0..l {
                        for k in 0..token_rc.rows[j].len() {
                            let mut j_tokens = vec![];

                            {
                                self.inline_tokens(token_rc.rows[j][k].as_ref().borrow().text.as_str(), &mut j_tokens);
                            }

                            {
                                token_rc.rows[j][k].as_ref().borrow_mut().tokens = j_tokens;
                            }

                        }
                    }
                }
                tokens.push(table_token);
                continue;
            }

            // lheading
            token = self.tokenizer.lheading(_src.as_str());
            if token.is_some() {
                // println!("Entered LHeading Block");
                let mut _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                // TODO: This is the incorrect call here - call should be made to self.inline instead
                self.inline(_token.as_ref().borrow().text.as_str(), Rc::clone(&_token));

                tokens.push(_token);
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
                // println!("Entered Paragraph Block");
                let _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();

                // TODO: This is the incorrect call here - call should be made to self.inline instead
                self.inline(_token.as_ref().borrow().text.as_str(), Rc::clone(&_token));


                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    if last_paragraph_clipped &&
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow()._type == "paragraph"
                    {
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw(_token.as_ref().borrow_mut().raw.as_str());

                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text(_token.as_ref().borrow_mut().text.as_str());

                        // println!("Removing last token");
                        self.inline_queue.remove(self.inline_queue.len() - 1);

                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue.get_mut(q_idx).unwrap().src = tokens.get(t_idx).unwrap().as_ref().borrow().text.to_string();
                    } else {
                        tokens.push(_token);
                    }
                } else {
                    tokens.push(_token);
                }

                last_paragraph_clipped = cut_src.len() != _src.len();
                _src = String::from(&_src[idx..]);
                continue;
            }


            // text
            token = self.tokenizer.text(_src.as_str());
            if token.is_some() {
                // println!("Entered Text Block");

                let mut _token: Rc<RefCell<Token>> = Rc::new(RefCell::new(
                    token.unwrap()
                ));

                let idx = _token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                // TODO: This is the incorrect call here - call should be made to self.inline instead
                self.inline(_token.as_ref().borrow().text.as_str(), Rc::clone(&_token));


                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    if tokens.get_mut(t_idx).unwrap().as_ref().borrow()._type == "text"
                    {
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_raw(_token.as_ref().borrow_mut().raw.as_str());

                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text("\n");
                        tokens.get_mut(t_idx).unwrap().as_ref().borrow_mut().append_to_text(_token.as_ref().borrow_mut().text.as_str());

                        // println!("Removing last token from text block");
                        self.inline_queue.remove(self.inline_queue.len() - 1);
                        let q_idx = self.inline_queue.len() - 1;
                        self.inline_queue[q_idx].src = tokens.get_mut(t_idx).unwrap().as_ref().borrow().text.to_string();
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
                    // println!("Warning! {}", err_msg);
                    break;
                } else {
                    panic!("{}", err_msg);
                }
            }
        }
        self.state.top = true;

        return tokens;
    }

    fn inline_tokens<'a>(&mut self, src: &str, mut tokens: &'a mut Vec<Rc<RefCell<Token>>>) -> &'a mut Vec<Rc<RefCell<Token>>> {

        let mut _src: String = String::from(src);
        // todo!("Check this initialization");
        let mut _cut_src: String = String::from("");
        let mut _masked_src: String = String::from(src);

        let mut prev_char: String = "".to_string();
        let mut _match: Vec<&str>;
        let mut token: Option<Token>;
        let mut last_token: Token;
        let mut _keep_prev_char: bool = false;

        // Mask out reflinks
        if self.links.len() > 0 {
            // println!("Entered Inline Reflinks Masking");
            let mut end_idx = 0;
            loop {
                let match_caps = self.tokenizer.rules.inline.exec_fc(_masked_src.as_str(), MDInline::RefLinkSearch, None);
                if match_caps.is_some() {

                    let caps = match_caps.unwrap();
                    let end = caps.get(0).unwrap().end();

                    if end == end_idx {
                        break;
                    } else {
                        end_idx = end;
                    }

                    let match0 = caps.get(0).map_or("", |m| m.as_str());
                    let link_match = match0.rfind('[');

                    if link_match.is_some() {
                        let link_match_idx = link_match.unwrap() + 1;
                        if link_match_idx < match0.len() - 1 {
                            let match_substr = &match0[link_match_idx..match0.len() - 1];
                            let idx_of_link = self.links.iter().position(|l| l.tag.contains(match_substr) );
                            if idx_of_link.is_some() {
                                let start = caps.get(0).unwrap().start();
                                let end = caps.get(0).unwrap().end();

                                let slice_start = &_masked_src[0..start];
                                let slice_end = &_masked_src[end..];
                                let count = match0.len() - 2;
                                let repeated_str = repeat_string("a", count);


                                _masked_src = format!("{}[{}]{}",
                                                      slice_start.to_string(),
                                                      repeated_str.to_string(),
                                                      slice_end.to_string()
                                );
                            }
                        }

                    }
                } else {
                    break;
                }

            }
        }


        // Mask out other blocks
        loop {
            let match_caps = self.tokenizer.rules.inline.exec_fc(_masked_src.as_str(), MDInline::BlockSkip, None);
            if match_caps.is_some() {
                // println!("Entered Other Blocks Masking");
                let caps = match_caps.unwrap();
                let match0 = caps.get(0).map_or("", |m| m.as_str());

                let start = caps.get(0).unwrap().start();
                let end = caps.get(0).unwrap().end();

                let slice_start = &_masked_src[0..start];
                let slice_end = &_masked_src[end..];
                let count = match0.len() - 2;
                let repeated_str = repeat_string("a", count);


                _masked_src = format!("{}[{}]{}",
                                      slice_start.to_string(),
                                      repeated_str.to_string(),
                                      slice_end.to_string()
                );


            } else {
                break;
            }
        }


        // Mask out escaped em & strong delimiters
        loop {
            let match_caps = self.tokenizer.rules.inline.exec_fc(_masked_src.as_str(), MDInline::EscapedEmSt, None);
            if match_caps.is_some() {
                // println!("Entered Escaped Em/Strong Delim Masking");
                let caps = match_caps.unwrap();
                let match0 = caps.get(0).map_or("", |m| m.as_str());

                let start = caps.get(0).unwrap().start();
                let end = caps.get(0).unwrap().end();

                let slice_start = &_masked_src[0..start];
                let slice_end = &_masked_src[end..];

                _masked_src = format!("{}++{}",
                                      slice_start.to_string(),
                                      slice_end.to_string()
                );
            } else {
                break;
            }
        }


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
                // println!("Inside Escape");
                let escape_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = escape_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(escape_token);
                continue;
            }


            // tag
            let mut in_link = self.state.in_link.clone();
            let mut in_raw_block = self.state.in_raw_block.clone();
            token = self.tokenizer.tag(_src.as_str(), &mut in_link, &mut in_raw_block);
            if token.is_some() {
                // println!("Inside Tag");

                self.state.in_link = in_link.clone();
                self.state.in_raw_block = in_raw_block.clone();

                let tag_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = tag_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;

                    let mut _last_token = tokens.get_mut(t_idx).unwrap();

                    if tag_token.as_ref().borrow()._type == "text" &&
                        _last_token.as_ref().borrow()._type == "text"
                    {
                        _last_token.as_ref().borrow_mut().append_to_raw(tag_token.as_ref().borrow().raw.as_str());
                        _last_token.as_ref().borrow_mut().append_to_text(tag_token.as_ref().borrow().text.as_str());

                    } else {
                        tokens.push(tag_token);
                    }
                } else {
                    tokens.push(tag_token);
                }
                continue;
            }


            // link
            token = self.tokenizer.link(_src.as_str());
            if token.is_some() {
                // println!("Inside Link");
                let link_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = link_token.as_ref().borrow_mut().raw.len();
                _src = String::from(&_src[idx..]);

                // Add tokens here
                if link_token.as_ref().borrow()._type == "link" {
                    self.state.in_link = true;

                    let mut l_tokens = vec![];
                    self.inline_tokens(link_token.as_ref().borrow().text.as_str(), &mut l_tokens);
                    link_token.as_ref().borrow_mut().tokens.append(&mut l_tokens);

                    self.state.in_link = false;
                }

                tokens.push(link_token);
                continue;
            }


            // reflink, nolink
            token = self.tokenizer.ref_link(_src.as_str(), &self.links);
            if token.is_some() {

                let reflink_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = reflink_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                // Add tokens here
                if reflink_token.as_ref().borrow()._type == "link" {
                    self.state.in_link = true;

                    let mut rl_tokens = vec![];
                    self.inline_tokens(reflink_token.as_ref().borrow().text.as_str(), &mut rl_tokens);
                    reflink_token.as_ref().borrow_mut().tokens.append(&mut rl_tokens);

                    self.state.in_link = false;
                }

                // println!("Entered Inline Reflink/Nolink");

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;
                    let _last_token = tokens.get_mut(t_idx).unwrap();

                    if reflink_token.as_ref().borrow()._type == "text" &&
                        _last_token.as_ref().borrow()._type == "text"
                    {
                        _last_token.as_ref().borrow_mut().append_to_raw(reflink_token.as_ref().borrow().raw.as_str());
                        _last_token.as_ref().borrow_mut().append_to_text(reflink_token.as_ref().borrow().text.as_str());
                    } else {
                        tokens.push(reflink_token);
                    }
                } else {
                    tokens.push(reflink_token);
                }
                continue;
            }


            // em & strong
            token = self.tokenizer.em_strong(_src.as_str(), _masked_src.as_str(), prev_char.to_string().as_str());
            if token.is_some() {

                let em_strong_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = em_strong_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                let mut em_tokens = vec![];
                self.inline_tokens(em_strong_token.as_ref().borrow().text.as_str(), &mut em_tokens);
                em_strong_token.as_ref().borrow_mut().tokens.append(&mut em_tokens);

                // println!("Inside Em/Strong");


                tokens.push(em_strong_token);
                continue;
            }

            // code
            token = self.tokenizer.code_span(_src.as_str());
            if token.is_some() {
                // println!("Inside Code Span");
                let code_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = code_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(code_token);
                continue;
            }

            // br
            token = self.tokenizer.br(_src.as_str());
            if token.is_some() {
                // println!("Inside Br");
                let br_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = br_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(br_token);
                continue;
            }

            // del (gfm)
            token = self.tokenizer.del(_src.as_str());
            if token.is_some() {

                let del_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = del_token.as_ref().borrow().raw.len();

                let mut il_tokens = vec![];
                self.inline_tokens(del_token.as_ref().borrow().text.as_str(), &mut il_tokens);
                del_token.as_ref().borrow_mut().tokens.append(&mut il_tokens);

                // println!("Inside Del");
                _src = String::from(&_src[idx..]);

                tokens.push(del_token);
                continue;
            }


            // autolink
            token = self.tokenizer.autolink(_src.as_str(), mangle);
            if token.is_some() {
                // println!("Inside Autolink");
                let autolink_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = autolink_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(autolink_token);
                continue;
            }


            // url (gfm)
            token = self.tokenizer.url(_src.as_str(), mangle);
            if !self.state.in_link && token.is_some() {
                // println!("Inside Url");
                let url_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = url_token.as_ref().borrow().raw.len();
                _src = String::from(&_src[idx..]);

                tokens.push(url_token);
                continue;
            }


            // text
            // prevent inlineText consuming extensions by clipping 'src' to extension start
            _cut_src = _src.clone();
            if self.options.extensions.is_some() {
                // todo!("Implement logic to avoid clipping src");
            }

            // Inline Text
            let in_raw_block = self.state.in_raw_block.clone();
            token = self.tokenizer.inline_text(_cut_src.as_str(), in_raw_block, smartypants);
            if token.is_some() {
                // println!("Entered Inline Text");
                let inline_text_token = Rc::new(RefCell::new(token.unwrap()));
                let idx = inline_text_token.as_ref().borrow().raw.len();

                _src = String::from(&_src[idx..]);

                let last_char = inline_text_token.as_ref().borrow().raw.chars().last().unwrap();
                if last_char != '_' {
                    // Track prevChar before string of ____ started
                    prev_char = last_char.to_string();
                    // // println!("PrevChar: {} {}", prev_char, prev_char.len())
                }

                _keep_prev_char = true;

                if tokens.len() > 0 {
                    let t_idx = tokens.len() - 1;
                    let _last_token = tokens.get_mut(t_idx).unwrap();

                    if _last_token.as_ref().borrow()._type == "text"
                    {
                        // // println!("Changing last token");
                        _last_token.as_ref().borrow_mut().append_to_raw(inline_text_token.as_ref().borrow().raw.as_str());
                        _last_token.as_ref().borrow_mut().append_to_text(inline_text_token.as_ref().borrow().text.as_str());
                    } else {
                        tokens.push(inline_text_token);
                        // // println!("Token added via {} => {:#?}", _src_b, tokens);
                    }
                } else {
                    tokens.push(inline_text_token);
                    // // println!("Token added via {} => {:#?}", _src_b, tokens);
                }
                continue;
            }

            if _src.len() > 0 {
                let err_msg = format!("Infinite loop on byte:  {}", _src.chars().nth(0).unwrap() as u32);

                if self.options.silent {
                    // println!("Warning! {}", err_msg);
                    break;
                } else {
                    panic!("{}", err_msg);
                }
            }
        }
        return tokens;
    }

    fn check_extensions_block(&mut self, extensions_block: Option<&'static str>) -> bool {
        return true;
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
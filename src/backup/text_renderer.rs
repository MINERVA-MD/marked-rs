#![allow(warnings, unused)]
/**
 * TextRenderer
 * returns only the textual part of the token
 */
pub struct TextRenderer {}

impl TextRenderer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clone for TextRenderer {
    fn clone(&self) -> Self {
        *self
    }
}

impl Copy for TextRenderer {}

pub trait ITextRenderer {
    fn strong(&mut self, text: &str) -> String;
    fn em(&mut self, text: &str) -> String;
    fn codespan(&mut self, text: &str) -> String;
    fn del(&mut self, text: &str) -> String;
    fn html(&mut self, text: &str) -> String;
    fn text(&mut self, text: &str) -> String;
    fn link(&mut self, href: &str, title: &str, text: &str) -> String;
    fn image(&mut self, href: &str, title: &str, text: &str) -> String;
    fn br(&mut self) -> String;
}

impl ITextRenderer for TextRenderer {
    fn strong(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn em(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn codespan(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn del(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn html(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn text(&mut self, text: &str) -> String {
        String::from(text)
    }

    fn link(&mut self, _href: &str, _title: &str, text: &str) -> String {
        let mut ret = String::from("");
        ret.push_str(text);
        ret
    }

    fn image(&mut self, _href: &str, _title: &str, text: &str) -> String {
        let mut ret = String::from("");
        ret.push_str(text);
        ret
    }

    fn br(&mut self,) -> String {
        String::from("")
    }
}


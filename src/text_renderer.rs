
/**
 * TextRenderer
 * returns only the textual part of the token
 */
pub struct TextRenderer {}

trait ITextRenderer {
    fn strong(text: &str) -> String;
    fn em(text: &str) -> String;
    fn codespan(text: &str) -> String;
    fn del(text: &str) -> String;
    fn html(text: &str) -> String;
    fn text(text: &str) -> String;
    fn link(href: &str, title: &str, text: &str) -> String;
    fn image(href: &str, title: &str, text: &str) -> String;
    fn br() -> String;
}

impl ITextRenderer for  TextRenderer {
    fn strong(text: &str) -> String {
        String::from(text)
    }

    fn em(text: &str) -> String {
        String::from(text)
    }

    fn codespan(text: &str) -> String {
        String::from(text)
    }

    fn del(text: &str) -> String {
        String::from(text)
    }

    fn html(text: &str) -> String {
        String::from(text)
    }

    fn text(text: &str) -> String {
        String::from(text)
    }

    fn link(href: &str, title: &str, text: &str) -> String {
        let mut ret = String::from("");
        ret.push_str(text);
        ret
    }

    fn image(href: &str, title: &str, text: &str) -> String {
        let mut ret = String::from("");
        ret.push_str(text);
        ret
    }

    fn br() -> String {
        String::from("")
    }
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {}
    }
}
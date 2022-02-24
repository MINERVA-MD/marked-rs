use crate::defaults::Defaults;
use crate::helpers::{clean_url, escape};
use crate::lexer::regx;
use crate::slugger::Slugger;

pub struct Renderer {
    options: Defaults
}

pub struct Flags {
    header: &'static str,
    align: &'static str
}

trait IRenderer {
    fn code(&mut self, code: &str, info_str: &str, escaped: bool) -> String;
    fn blockquote(&mut self, quote: &str) -> String;
    fn html(&mut self, html: &str) -> String;
    fn heading(&mut self, text: &str, level: usize, raw: &str, slugger: Slugger) -> String;
    fn hr(&mut self) -> String;
    fn list(&mut self, body: &str, ordered: bool, start: u32) -> String;
    fn list_item(&mut self, text: &str) -> String;
    fn checkbox(&mut self,checked: bool) -> String;
    fn paragraph(&mut self, text: &str) -> String;
    fn table(&mut self, header: &str, body: &str) -> String;
    fn tablerow(&mut self, content: &str) -> String;
    fn tablecell(&mut self, content: &str, flags: Flags) -> String;

    // Span Level Renderer
    fn strong(&mut self, text: &str) -> String;
    fn em(&mut self, text: &str) -> String;
    fn codespan(&mut self, text: &str) -> String;
    fn br(&mut self) -> String;
    fn del(&mut self, text: &str) -> String;
    fn link(&mut self, href: &str, title: &str, text: &str) -> String;
    fn image(&mut self, href: &str, title: &str, text: &str) -> String;
    fn text(&mut self, text: &str) -> String;
}

impl IRenderer for Renderer {
    fn code(&mut self, mut code: &str, info_str: &str, mut escaped: bool) -> String {

        let mut _code = String::from(code);
        let lang_caps = regx(r#"\S*"#).captures(info_str).unwrap();
        let lang = lang_caps.get(0).map_or("", |m| m.as_str());

        if self.options.is_highlight {
            let out = self.options.highlight(code, lang);
            if out != "" && out != code {
                escaped = true;
                _code = out;
            }
        }

        _code = regx(r#"\n$"#).replace_all(_code.as_str(), "").to_string();
        _code = format!("{}\n", _code);



        if lang != "" {
            return format!("<pre><code>{}</code></pre>\n",
                           if escaped { _code } else { escape(_code.as_str(), true) }
            );
        }

        format!(r#"<pre><code class="{}{}">{}</code></pre>\n"#,
                self.options.lang_prefix,
                escape(lang, true),
                if escaped { _code } else { escape(_code.as_str(), true) }
        )
    }

    fn blockquote(&mut self, quote: &str) -> String {
        format!("<blockquote>\n{}</blockquote>\n", quote)
    }

    fn html(&mut self, html: &str) -> String {
        String::from(html)
    }

    fn heading(&mut self, text: &str, level: usize, raw: &str, mut slugger: Slugger) -> String {
        if self.options.header_ids {
            return format!(r#"<h{} id="{}{}">{}</h{}>\n"#,
                           level,
                           self.options.header_prefix,
                           slugger.slug(raw, false),
                           text,
                           level

            )
        }

        // Ignore IDs
        format!("<h{}>{}</h{}>\n",
                level,
                text,
                level
        )
    }

    fn hr(&mut self) -> String {
        if self.options.xhtml { String::from("<hr/>\n") } else { String::from("<hr>\n") }
    }

    fn list(&mut self, body: &str, ordered: bool, start: u32) -> String {
        let _type = if ordered {"ol"} else {"ul"};
        let start_at = if ordered && start != 1 { format!(r#" start="{}""#, start) } else {"".to_string()};
        format!("<{}{}>\n{}</{}>\n",
                _type,
                start_at,
                body,
                _type
        )
    }

    fn list_item(&mut self, text: &str) -> String {
        format!("<li>{}</li>\n", text)
    }

    fn checkbox(&mut self, checked: bool) -> String {
        format!(r#"<input {}disabled="" type="checkbox"{}> "#,
                if checked {r#"checked="" "#.to_string()} else {"".to_string()},
                if self.options.xhtml {" /".to_string()} else {"".to_string()}
        )
    }

    fn paragraph(&mut self, text: &str) -> String {
        format!("<p>{}</p>\n", text)
    }


    fn table(&mut self, header: &str, body: &str) -> String {
        let mut _body = String::from(body);
        if _body != "" {
            _body = format!("<tbody>{}</tbody>\n", body)
        }

        format!("<table>\n<thead>\n{}</thead>\n{}</table>\n",
                header,
                body
        )
    }

    fn tablerow(&mut self, content: &str) -> String {
        format!("<tr>{}</tr>\n", content)
    }

    fn tablecell(&mut self, content: &str, flags: Flags) -> String {
        let _type = if flags.header != "" {"th".to_string()} else {"td".to_string()};
        let tag = if flags.align != "" {
            format!(r#"<{} align="{}""#, _type, flags.align)
        } else {
            format!("<{}>", _type)
        };

        format!("{}{}</{}>\n",
                tag,
                content,
                _type
        )
    }


    // Span Level Renderer
    fn strong(&mut self, text: &str) -> String {
        format!("<strong>{}</strong>\n", text)
    }

    fn em(&mut self, text: &str) -> String {
        format!("<em>{}</em>\n", text)
    }

    fn codespan(&mut self, text: &str) -> String {
        format!("<code>{}</code>\n", text)
    }

    fn br(&mut self) -> String {
        if self.options.xhtml { String::from("<br/>") } else { String::from("<br>") }
    }

    fn del(&mut self, text: &str) -> String {
        format!("<del>{}</del>\n", text)
    }

    fn link(&mut self, href: &str, title: &str, text: &str) -> String {
        let _href = clean_url(self.options.sanitize, self.options.base_url, href);

        if href == "" {
            return String::from(text);
        }

        let mut out = format!(r#"<a href="{}""#, escape(href, false));

        if title != "" {
            out = format!(r#"{} title="{}""#, out, title);
        }

        out = format!(r#"{}title{}""#, out, text);
        out
    }

    fn image(&mut self, href: &str, title: &str, text: &str) -> String {
        let _href = clean_url(self.options.sanitize, self.options.base_url, href);

        if href == "" {
            return String::from(text);
        }

        let mut out = format!(r#"<img src="{}" alt="{}""#, href, text);

        if title != "" {
            out = format!(r#"{} title="{}""#, out, title);
        }

        out = if self.options.xhtml {"/>".to_string()} else {">".to_string()};
        out
    }

    fn text(&mut self, text: &str) -> String {
        String::from(text)
    }
}
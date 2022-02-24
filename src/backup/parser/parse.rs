use regex::Regex;

pub struct Parser {
    html: String,
}

impl Parser {
    pub fn parse_markdown(markdown: &str)-> String {
        return String::from(markdown);
    }

    pub fn parse(markdown: &str)-> String {
        let mut html = String::from(markdown);

        let h3_re = Regex::new(r"^### (.*$)").unwrap();
        let h2_re = Regex::new(r"^## (.*$)").unwrap();
        let h1_re = Regex::new(r"^# (.*$)").unwrap();
        let b_re = Regex::new(r"\*\*(.*)\*\*").unwrap();
        let i_re = Regex::new(r"\*(.*)\*").unwrap();

        html = h3_re.replace_all(&html, "<h3>$1</h3>").parse().unwrap();
        html = h2_re.replace_all(&html, "<h2>$1</h2>").parse().unwrap();
        html = h1_re.replace_all(&html, "<h1>$1</h1>").parse().unwrap();
        html = b_re.replace_all(&html, "<b>$1</b>").parse().unwrap();
        html = i_re.replace_all(&html, "<i>$1</i>").parse().unwrap();

        // println!("{}", re.is_match(&html));
        println!("{}", html);

        return html;
    }
}




use crate::lexer::regx;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Slugger {
    seen: HashMap<String, i32>
}

impl Slugger {
    pub fn new() -> Self {
        Self {
            seen: Default::default()
        }
    }

    pub fn serialize(value: &str) -> String {
        // TODO: Not sure about how these regexes are escaped; double check regex docs
        let html_re = regx("(?i)<[!\\\\/a-z].*?>");
        let chars_re = regx(r#"[\u2000-\u206F\u2E00-\u2E7F\\'!\\"\\#$%&()*+,./:;<=>?@\[\]^`\{|\}~]"#);
        let space_re = regx(r#"\s"#);

        let mut serialized_str= value.to_lowercase().trim().to_string();

        serialized_str = html_re.replace_all(serialized_str.as_str(), "").to_string();
        serialized_str = chars_re.replace_all(serialized_str.as_str(), "").to_string();
        serialized_str = space_re.replace_all(serialized_str.as_str(),"-").to_string();

        serialized_str
    }

    pub fn get_next_safe_slug(&mut self, original_slug: &str, is_dryrun: bool) -> String {
        let mut slug = String::from(original_slug);
        let mut occurrence_accumulator = 0;

        if self.seen.contains_key(slug.as_str()) {
            occurrence_accumulator = *self.seen.get(original_slug).unwrap();

            loop {
                occurrence_accumulator += 1;
                slug = format!("{}-{}", original_slug, occurrence_accumulator);

                if !self.seen.contains_key(slug.as_str()) {
                    break;
                }
            }
        }
        if !is_dryrun {
            self.seen.insert(original_slug.to_string(), occurrence_accumulator);
            self.seen.insert(slug.to_string(), 0);
        }
        slug
    }

    pub fn slug(&mut self, value: &str, dryrun: bool) -> String {
        let slug = Slugger::serialize(value);
        let next_slug = self.get_next_safe_slug(slug.as_str(), dryrun);

        String::from(next_slug)
    }
}
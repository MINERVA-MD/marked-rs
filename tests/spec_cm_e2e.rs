#![allow(warnings, unused)]
macro_rules! spec_test {
    ($path:expr, $from:expr, $to:expr) => {

        #[cfg(test)]
        mod tests {
            use std::fs;
            use std::env;
            use std::io::Write;
            use ntest::timeout;
            use seq_macro::seq;
            use std::path::Path;
            use serde_json::Result;
            use test_case::test_case;
            use marked_rs::helpers::Spec;
            use marked_rs::marked::Marked;
            use serde::{Serialize, Deserialize};
            use marked_rs::defaults::get_default_options;
            use pretty_assertions::{assert_eq, assert_ne};


            fn deserialize_specs(path: &str)-> String {
                let data: String = fs::read_to_string(path)
                    .expect("Unable to read file");
                return data;
            }

            fn get_specs() -> Vec<Spec> {
                let data: String = deserialize_specs($path);
                let specs: Vec<Spec> = serde_json::from_str(&data).unwrap();
                return specs;
            }

            fn html_entity_compare(str1: String, str2: String) {
                let mut str1_decoded = String::from("");
                let mut str2_decoded = String::from("");

                html_escape::decode_html_entities_to_string(str1, &mut str1_decoded);
                html_escape::decode_html_entities_to_string(str2, &mut str2_decoded);

                pretty_assertions::assert_eq!(str1_decoded, str2_decoded);
            }

            seq!(N in $from..$to {

                #(#[test_case(N + 1)])*
                #[timeout(8000)]
                fn verify_commonmark_specs(index: usize) {
                    let specs: Vec<Spec> = get_specs();
                    let spec: &Spec = &specs[index];

                    let md = &spec.markdown;
                    let mut marked = Marked::new(None);
                    let mut options = get_default_options();

                    options.gfm = false;
                    options.pedantic = false;
                    options.header_ids = false;

                    let expected_std_html = &spec.html;
                    let expected_marked_html = &spec.marked;
                    let spec_should_fail = &spec.should_fail;

                    let actual_html = marked.parse(md, Some(options), None);

                    if *expected_marked_html != actual_html {

                        if vec![603, 604].contains(&*&spec.example)
                        {
                            html_entity_compare(String::from(expected_marked_html), actual_html);
                            return;
                        }
                        println!("Failing Spec : {}", &spec.example)
                    }
                    pretty_assertions::assert_eq!(*expected_marked_html, actual_html);
                }
            });
        }
    };
}


// 652
spec_test!("tests/fixtures/marked-specs/commonmark/commonmark.0.30.json", 0, 651);



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
            use marked_rs::marked::Marked;
            use serde::{Serialize, Deserialize};
            use marked_rs::defaults::get_default_options;
            use pretty_assertions::{assert_eq, assert_ne};

            #[derive(Serialize, Deserialize, Debug)]
            struct Spec {
                markdown: String,
                html: String,
                example: i32,
                start_line: i32,
                end_line: i32,
                section: String,
                marked: String,
                should_fail: bool
            }

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

            seq!(N in $from..$to {

                #(#[test_case(N + 1)])*
                #[timeout(5000)]
                fn verify_specs(index: usize) {
                    let specs: Vec<Spec> = get_specs();
                    let spec: &Spec = &specs[index];

                    let md = &spec.markdown;
                    let mut marked = Marked::new(None);
                    let mut options = get_default_options();

                    options.pedantic = false;
                    options.header_ids = false;

                    let expected_std_html = &spec.html;
                    let expected_marked_html = &spec.marked;
                    let spec_should_fail = &spec.should_fail;

                    let actual_html = marked.parse(md, Some(options), None);

                    if !(*spec_should_fail) {
                        pretty_assertions::assert_eq!(*expected_marked_html, actual_html)
                    }
                }
            });
        }
    };
}


// 651

spec_test!("tests/fixtures/marked-specs/commonmark/commonmark.0.30.json", 0, 10);



macro_rules! spec_test {
    ($path:expr, $from:expr, $to:expr) => {

        #[cfg(test)]
        mod tests {
            use std::fs;
            use std::env;
            use seq_macro::seq;
            use std::path::Path;
            use serde_json::Result;
            use test_case::test_case;
            use serde::{Serialize, Deserialize};

            use marked_rs::parser::parse::Parser;

            #[derive(Serialize, Deserialize, Debug)]
            struct Spec {
                markdown: String,
                html: String,
                example: i64,
                start_line: i64,
                end_line: i64,
                section: String,
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
                #(#[test_case(N)])*
                fn verify_specs(index: usize) {
                    let specs: Vec<Spec> = get_specs();
                    let spec: &Spec = &specs[index];
                    let parse_actual = Parser::parse(&spec.markdown);
                    let parse_expected = &spec.html;
                    assert_eq!(*parse_expected, parse_actual);
                }
            });
        }
    };
}

// spec_test!("tests/fixtures/md/spec-v3.json", 0, 651);



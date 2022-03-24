#![allow(warnings, unused)]

use std::fs;
use std::path::PathBuf;
use test_case::test_case;
use std::collections::HashMap;
use marked_rs::defaults::Options;
use pretty_assertions::{assert_eq, assert_ne};
use marked_rs::helpers::{get_completion_table, Spec, SpecSectionSummary};
use marked_rs::marked::Marked;

pub fn run_specs(title: &str, dir: &str, show_completion_table: bool, options: Options) {
    let mut specs: Vec<Vec<Spec>> = vec![];

    let paths = fs::read_dir(dir).unwrap();
    let mut spec_summary_map: HashMap<String, SpecSectionSummary> = HashMap::new();

    for path in paths {
        let spec = get_specs(path.unwrap().path());
        specs.push(spec);
    }

    for spec_gp in specs.iter() {
        for spec in spec_gp.iter() {

            let mut marked = Marked::new(None);

            let md = &spec.markdown;
            let expected_html = &spec.marked;
            let spec_should_fail = &spec.should_fail;
            let section = String::from(&spec.section.clone());
            let actual_html = marked.parse(md, Some(options), None);

            let spec_passed = *expected_html == actual_html && !*spec_should_fail;

            if !spec_summary_map.contains_key(section.as_str())
            {
                spec_summary_map.insert(section.clone(), SpecSectionSummary {
                    section: section.clone(),
                    pass: 0,
                    total: 0
                });
            }

            let mut section_summary = spec_summary_map.get_mut(&section.clone()).unwrap();
            section_summary.total += 1;
            section_summary.pass += if spec_passed {1} else {0};
            println!("Finished Processing Spec#: {} ... {}",
                     *&spec.example,
                if spec_passed {"Ok"} else {"Fail"}
            );
        }
    }

    let mut specs_summary = vec![];
    for (section, summary) in spec_summary_map.into_iter() {
        specs_summary.push(summary.clone())
    }

    println!("{:#?}", specs_summary.len());

    if show_completion_table {
        let completion_table = get_completion_table(title, &mut specs_summary);
        println!("{}", completion_table);
        // fs::write("completion_table.txt", completion_table).expect("Unable to write file")

    }
}

fn deserialize_specs(path: PathBuf)-> String {
    let data: String = fs::read_to_string(path)
        .expect("Unable to read file");
    return data;
}

fn get_specs(path: PathBuf) -> Vec<Spec> {
    let data: String = deserialize_specs(path);
    let specs: Vec<Spec> = serde_json::from_str(&data).unwrap();
    return specs;
}


#[cfg(test)]
mod specs {
    use marked_rs::defaults::{get_default_options, get_options};
    use marked_rs::rules::test;
    use super::*;


    #[test]
    fn run_cm_specs() {
        let mut options = get_options(false, false, false);
        run_specs("CommonMark", "tests/fixtures/marked-specs/commonmark", true, options);
    }

}
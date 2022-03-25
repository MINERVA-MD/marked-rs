#![allow(warnings, unused)]

use std::fs;
use ntest::timeout;
use std::io::Write;
use std::path::PathBuf;
use test_case::test_case;
use std::fs::OpenOptions;
use marked_rs::marked::Marked;
use std::collections::HashMap;
use chrono::Utc;
use pretty_assertions::{assert_eq, assert_ne};
use marked_rs::defaults::{get_options, Options};
use marked_rs::helpers::{get_completion_table, MdSpec, Spec, SpecSectionSummary};


pub fn run_specs(title: &str, dir: &str, show_completion_table: bool, options: Options) {
    let mut specs: Vec<Vec<Spec>> = vec![];

    let paths = fs::read_dir(dir).unwrap();
    let mut spec_summary_map: HashMap<String, SpecSectionSummary> = HashMap::new();

    for path in paths {
        let spec = get_base_specs(path.unwrap().path());
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
        specs_summary.push(summary.clone());
    }


    if show_completion_table {
        let completion_table = get_completion_table(title, &mut specs_summary);

        let now = Utc::now();
        let res = now.format("%Y-%m-%d@%H-%M-%S");

        let table = format!("tests/specs/{}_{}.spec.txt", title.replace(" ", "-").as_str(), res);
        write_table(table.as_str(), completion_table.clone());
    }
}

pub fn run_md_specs<'a>(title: &str, dir: &str, show_completion_table: bool) {
    let mut specs: Vec<Vec<MdSpec>> = vec![];
    let mut base: String = "".to_string();

    let paths = fs::read_dir(dir).unwrap();
    let mut spec_summary_map: HashMap<String, SpecSectionSummary> = HashMap::new();

    for path in paths {
        let spec = get_md_specs(path.unwrap().path());
        specs.push(spec);
    }

    for spec_gp in specs.iter() {
        for spec in spec_gp.iter() {

            let mut marked = Marked::new(None);

            let md = &spec.markdown;
            let expected_html = &spec.marked;
            let spec_base_url = &spec.base_url;
            let spec_options = &spec.options;

            let mut options = get_options(
                spec_options[0],
                spec_options[1],
                spec_options[2],
                spec_options[3],
                spec_options[4],
                spec_options[5],
                spec_options[6],
                spec_options[7],
                spec_options[8],
                spec_options[9]
            );

           if *&spec.example == 79 {
               options.base_url = "/base/";
           } else if *&spec.example == 80 {
               options.base_url = "http://example.com/base/"
           }

            let spec_should_fail = &spec.should_fail;
            let section = String::from(&spec.section.clone());
            let actual_html = marked.parse(md, Some(options), None);

            let mut spec_passed = *expected_html == actual_html && !*spec_should_fail;

            if !spec_passed {
                html_entity_compare(String::from(expected_html), actual_html);
                spec_passed = true;
            }

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
        specs_summary.push(summary.clone());
    }


    if show_completion_table {
        let completion_table = get_completion_table(title, &mut specs_summary);

        let now = Utc::now();
        let res = now.format("%Y-%m-%d@%H-%M-%S");

        let table = format!("tests/specs/{}_{}.spec.txt", title.replace(" ", "-").as_str(), res);
        write_table(table.as_str(), completion_table.clone());
    }
}

fn html_entity_compare(str1: String, str2: String) -> bool {
    let mut str1_decoded = String::from("");
    let mut str2_decoded = String::from("");

    html_escape::decode_html_entities_to_string(str1, &mut str1_decoded);
    html_escape::decode_html_entities_to_string(str2, &mut str2_decoded);

    pretty_assertions::assert_eq!(str1_decoded, str2_decoded);
    str1_decoded == str2_decoded
}

fn write_table(path: &str, table: String) {
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", table) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn deserialize_specs(path: PathBuf)-> String {
    let data: String = fs::read_to_string(path)
        .expect("Unable to read file");
    return data;
}

fn get_base_specs(path: PathBuf) -> Vec<Spec> {
    let data: String = deserialize_specs(path);
    let specs: Vec<Spec> = serde_json::from_str(&data).unwrap();
    return specs;
}

fn get_md_specs(path: PathBuf) -> Vec<MdSpec> {
    let data: String = deserialize_specs(path);
    let specs: Vec<MdSpec> = serde_json::from_str(&data).unwrap();
    return specs;
}


#[cfg(test)]
mod specs {
    use marked_rs::defaults::{get_base_options, get_default_options, get_options};
    use marked_rs::rules::test;
    use super::*;


    #[test]
    fn run_base_specs() {
        let mut options = get_base_options(false, false, false, false);
        run_specs("CommonMark", "tests/fixtures/marked-specs/commonmark", true, options);

        options = get_base_options(true, false, false, false);
        run_specs("GFM", "tests/fixtures/marked-specs/gfm", true, options);
    }

    #[test]
    #[timeout(800000)]
    fn run_og_new_specs() {
        run_md_specs("Original", "tests/fixtures/marked-specs/original/json", true);
        run_md_specs("New", "tests/fixtures/marked-specs/new/json", true);
    }

    fn output_completion_table() {

    }

}
use std::fs;
use std::mem::size_of_val;
use marked_rs::marked::Marked;
use marked_rs::lexer::{ILexer, Lexer};
use pulldown_cmark::{Parser, Options, html};
use comrak::{markdown_to_html, ComrakOptions};
use marked_rs::defaults::{ get_default_options };
use criterion::{black_box, criterion_group, criterion_main, Criterion};


fn bench(c: &mut Criterion) {

    let mut marked = Marked::new(None);
    let mut lexer = Lexer::new(get_default_options());
    let md = fs::read_to_string("tests/fixtures/md/spec.md").expect("Unable to read file");
    let md_sm = fs::read_to_string("tests/fixtures/md/spec-2.md").expect("Unable to read file");


    let tokens = lexer.lex_ac(md_sm.as_str());
    let tokens_memory_usage = size_of_val(&*tokens) as f64 / 1000000_f64;

    println!("Tokens: {} | Size: {} MB", tokens.len(), tokens_memory_usage);


    // c.bench_function("Pulldown Cmark", |b| b.iter(|| html::push_html(black_box(&mut String::new()), black_box(Parser::new_ext(md.as_str(), Options::empty())))));
    // c.bench_function("Comrak", |b| b.iter(|| markdown_to_html(black_box(md.as_str()), black_box(&ComrakOptions::default()))));
    // c.bench_function("Marked", |b| b.iter(|| marked.parse(black_box(md_sm.as_str()), black_box(None), black_box(None))));
}


criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = bench
}

criterion_main!(benches);
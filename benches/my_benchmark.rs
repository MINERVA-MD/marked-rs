use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use marked_rs::defaults::{get_base_options, get_default_options};
use marked_rs::lexer::{ILexer, Lexer};
use marked_rs::marked::Marked;

fn bench(c: &mut Criterion) {

    let mut marked = Marked::new(None);
    let mut lexer = Lexer::new(get_default_options());
    let md = fs::read_to_string("tests/fixtures/md/spec-2.md").expect("Unable to read file");

    c.bench_function("lexer", |b| b.iter(|| lexer.lexify(black_box(md.as_str()))));
    c.bench_function("marked", |b| b.iter(|| marked.parse(black_box(md.as_str()), black_box(None), black_box(None))));
}


criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = bench
}

criterion_main!(benches);
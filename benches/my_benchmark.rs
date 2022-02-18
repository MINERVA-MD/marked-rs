use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let look_ups: [&str; 10] = [
        "&zwj;",
        "&smtes;",
        "&Iota;",
        "&NegativeMediumSpace",
        "&TRADE;",
        "&TildeFullEqual;",
        "&ThickSpace;",
        "&cups;",
        "&dbkarow;",
        "&gnapprox;",
    ];

    for i in &look_ups[0..1] {
        // c.bench_function("entity_lookup", |b| b.iter(|| Entity::entity_lookup(black_box(i))));
        // c.bench_function("entity_lookup3", |b| b.iter(|| Entity::entity_lookup3(black_box(i))));
    }
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
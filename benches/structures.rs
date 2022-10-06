use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nonspacing_bench::load_and_filter;
use nonspacing_bench::structure::btree_set::btree_filter;
use nonspacing_bench::structure::hashset::hashset_filter;
use nonspacing_bench::structure::naive::naive_filter;
use nonspacing_bench::structure::roaring_bitmap::roaring_filter;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Naive", |b| {
        b.iter(|| load_and_filter(naive_filter()))
    });

    c.bench_function("BTreeSet", |b| {
        b.iter(|| load_and_filter(btree_filter()))
    });

    c.bench_function("RoaringBitmap", |b| {
        b.iter(|| load_and_filter(roaring_filter()))
    });

    c.bench_function("HashSet", |b| {
        b.iter(|| load_and_filter(hashset_filter()))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
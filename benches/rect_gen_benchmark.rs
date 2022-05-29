use bktrk::util::{rect_of_ranges, rect_of_ranges_itertools};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function(
        "generate rectangle iterator via iterator combinators",
        |b| b.iter(|| rect_of_ranges(black_box(0..100), black_box(0..100))),
    );

    c.bench_function(
        "generating rectangle iterator via itertools cartesian product",
        |b| b.iter(|| rect_of_ranges_itertools(black_box(0..100), black_box(0..100))),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

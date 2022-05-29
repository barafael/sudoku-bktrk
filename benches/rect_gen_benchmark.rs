use bktrk::bktrk;
use bktrk::board::Board;
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

    c.bench_function("solve sudoku", |b| {
        b.iter(|| {
            let mut board = Board([
                [3, 0, 6, 5, 0, 8, 4, 0, 0],
                [5, 2, 0, 0, 0, 0, 0, 0, 0],
                [0, 8, 7, 0, 0, 0, 0, 3, 1],
                [0, 0, 3, 0, 1, 0, 0, 8, 0],
                [9, 0, 0, 8, 6, 3, 0, 0, 5],
                [0, 5, 0, 0, 9, 0, 6, 0, 0],
                [1, 3, 0, 0, 0, 0, 2, 5, 0],
                [0, 0, 0, 0, 0, 0, 0, 7, 4],
                [0, 0, 5, 2, 0, 6, 3, 0, 0],
            ]);

            bktrk(black_box(&mut board), 0, 0);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

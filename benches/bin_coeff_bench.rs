use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dynamic_binomial::bin_coeff::{bottom_up_bin_coeff, memoized_bin_coeff, naive_bin_coeff};

pub fn bench_bins(c: &mut Criterion) {
    let mut group = c.benchmark_group("binomial coefficent functions comparison");

    let mut bench_inputs = Vec::new();
    for n in 3..=67_u64 {
        //max size where result will fit in u64
        for k in 2..n {
            bench_inputs.push((n, k));
        }
    }
    for (test_no, input) in bench_inputs.iter().enumerate() {
        //benchmark each function with a copy of each instance of  num objects and selections
        group.bench_with_input(
            BenchmarkId::new("naive/D&C", test_no + 1),
            &input,
            |benches, input| benches.iter(|| black_box(naive_bin_coeff(input.0, input.1))),
        );
        group.bench_with_input(
            BenchmarkId::new("bottom up", test_no + 1),
            &input,
            |benches, input| benches.iter(|| black_box(bottom_up_bin_coeff(input.0, input.1))),
        );
        group.bench_with_input(
            BenchmarkId::new("memoized", test_no + 1),
            &input,
            |benches, input| benches.iter(|| black_box(memoized_bin_coeff(input.0, input.1))),
        );
    }

    group.finish();
}

pub fn bench_bins_no_black_box(c: &mut Criterion) {
    let mut group = c.benchmark_group("binomial coefficent functions comparison no black box");

    let mut bench_inputs = Vec::new();
    for n in 3..=67_u64 {
        //max size where result will fit in u64
        for k in 2..n {
            bench_inputs.push((n, k));
        }
    }
    for (test_no, input) in bench_inputs.iter().enumerate() {
        //benchmark each function with a copy of each instance of  num objects and selections
        group.bench_with_input(
            BenchmarkId::new("naive/D&C", test_no + 1),
            &input,
            |benches, input| benches.iter(|| naive_bin_coeff(input.0, input.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("bottom up", test_no + 1),
            &input,
            |benches, input| benches.iter(|| bottom_up_bin_coeff(input.0, input.1)),
        );
        group.bench_with_input(
            BenchmarkId::new("memoized", test_no + 1),
            &input,
            |benches, input| benches.iter(|| memoized_bin_coeff(input.0, input.1)),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_bins, bench_bins_no_black_box);
criterion_main!(benches);

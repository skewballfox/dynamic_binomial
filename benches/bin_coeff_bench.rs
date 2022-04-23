use criterion::{criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use dynamic_binomial::bin_coeff::{bottom_up_bin_coeff, memoized_bin_coeff, naive_bin_coeff};
use rand::{thread_rng, Rng};

pub fn bench_bins(c: &mut Criterion) {
    let mut group = c.benchmark_group("binomial coefficent functions comparison");
    let mut rng = thread_rng();
    for bound in (10..50).step_by(10) {
        let num_objects = rng.gen_range(3..bound);
        let num_selections = rng.gen_range(2..num_objects);
        group.bench_function(
            &format!("naive/D&C {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| naive_bin_coeff(num_objects, num_selections)),
        );
        group.bench_function(
            &format!("bottom up {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| bottom_up_bin_coeff(num_objects, num_selections)),
        );
        group.bench_function(
            &format!("memoized {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| memoized_bin_coeff(num_objects, num_selections)),
        );
    }
    group.finish();
}

criterion_group!(benches, bench_bins);
criterion_main!(benches);

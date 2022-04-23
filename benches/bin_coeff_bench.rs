use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dynamic_binomial::bin_coeff::{bottom_up_bin_coeff, memoized_bin_coeff, naive_bin_coeff};
use rand::{thread_rng, Rng};

pub fn bench_bins(c: &mut Criterion) {
    let mut group = c.benchmark_group("binomial coefficent functions comparison");
    let mut rng = thread_rng();
    //avoid allocating memory in loop
    let mut num_objects: u64 = 0;
    let mut num_selections: u64 = 0;

    for bound in (10..50).step_by(10) {
        num_objects = rng.gen_range(3..bound);
        num_selections = rng.gen_range(2..num_objects);

        //benchmark each function with a copy of each instance of  num objects and selections
        group.bench_function(
            &format!("naive/D&C {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| black_box(naive_bin_coeff(num_objects, num_selections))),
        );
        group.bench_function(
            &format!("bottom up {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| black_box(bottom_up_bin_coeff(num_objects, num_selections))),
        );
        group.bench_function(
            &format!("memoized {} choose {}", num_objects, num_selections),
            |benches| benches.iter(|| black_box(memoized_bin_coeff(num_objects, num_selections))),
        );
    }

    group.finish();
}

criterion_group!(benches, bench_bins);
criterion_main!(benches);

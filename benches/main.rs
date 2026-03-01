use criterion::{criterion_group, criterion_main, Criterion};
use ecgen::{
    combin::{comb, emk_comb_gen},
    gray_code::brgc_gen,
    perm::{ehr_gen, factorial, sjt_gen},
    set_bipart::{set_bipart_gen, stirling2nd2},
    set_partition::{set_partition_gen, stirling2nd},
};

fn bench_comb(c: &mut Criterion) {
    c.bench_function("comb", |b| {
        b.iter(|| comb(std::hint::black_box(100), std::hint::black_box(50)));
    });
}

fn bench_emk_comb_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("emk_comb_gen");
    group.bench_function("n=10,k=5", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in emk_comb_gen(std::hint::black_box(10), std::hint::black_box(5)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=15,k=3", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in emk_comb_gen(std::hint::black_box(15), std::hint::black_box(3)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

fn bench_factorial(c: &mut Criterion) {
    c.bench_function("factorial", |b| {
        b.iter(|| factorial(std::hint::black_box(20)));
    });
}

fn bench_sjt_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("sjt_gen");
    group.bench_function("n=5", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in sjt_gen(std::hint::black_box(5)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=7", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in sjt_gen(std::hint::black_box(7)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

fn bench_ehr_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("ehr_gen");
    group.bench_function("n=5", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in ehr_gen(std::hint::black_box(5)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=7", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in ehr_gen(std::hint::black_box(7)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

fn bench_brgc_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("brgc_gen");
    group.bench_function("n=8", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in brgc_gen(std::hint::black_box(8)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=12", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in brgc_gen(std::hint::black_box(12)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

fn bench_stirling2nd(c: &mut Criterion) {
    c.bench_function("stirling2nd", |b| {
        b.iter(|| stirling2nd(std::hint::black_box(30), std::hint::black_box(10)));
    });
}

fn bench_set_partition_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_partition_gen");
    group.bench_function("n=6,k=3", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in set_partition_gen(std::hint::black_box(6), std::hint::black_box(3)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=8,k=4", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in set_partition_gen(std::hint::black_box(8), std::hint::black_box(4)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

fn bench_stirling2nd2(c: &mut Criterion) {
    c.bench_function("stirling2nd2", |b| {
        b.iter(|| stirling2nd2(std::hint::black_box(20)));
    });
}

fn bench_set_bipart_gen(c: &mut Criterion) {
    let mut group = c.benchmark_group("set_bipart_gen");
    group.bench_function("n=10", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in set_bipart_gen(std::hint::black_box(10)) {
                count += 1;
            }
            count
        });
    });
    group.bench_function("n=15", |b| {
        b.iter(|| {
            let mut count = 0;
            for _ in set_bipart_gen(std::hint::black_box(15)) {
                count += 1;
            }
            count
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_comb,
    bench_emk_comb_gen,
    bench_factorial,
    bench_sjt_gen,
    bench_ehr_gen,
    bench_brgc_gen,
    bench_stirling2nd,
    bench_set_partition_gen,
    bench_stirling2nd2,
    bench_set_bipart_gen
);
criterion_main!(benches);

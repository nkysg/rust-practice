use criterion::{black_box, criterion_group, criterion_main, Criterion};
use smallvec::{SmallVec, smallvec};

fn bench_smallvec(c: &mut Criterion) {
    let mut group = c.benchmark_group("smallvec_vs_vec");
    group.bench_function("smallvec_push_4", |b| {
        b.iter(|| {
            let mut v : SmallVec<i32, 4> = smallvec![];
            for i in 0..4 {
                v.push(black_box(i));
            }
        })
    });
    group.bench_function("vec_push_4", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..4 {
                v.push(black_box(i));
            }
        })
    });
}

criterion_group!(benches, bench_smallvec);
criterion_main!(benches);
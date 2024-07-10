use benches;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stm::TVar;

fn bench_writes(c: &mut Criterion) {
    let mut group = c.benchmark_group("nocontension");
    group.sample_size(1024);

    let t = TVar::new(0);
    group.bench_function("bench_writes", |b| {
        b.iter(|| {
            benches::write(&t, black_box(1));
        })
    });
}

criterion_group!(nocontension, bench_writes);
criterion_main!(nocontension);

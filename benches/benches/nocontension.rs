use benches;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stm::TVar;

fn bench_writes(c: &mut Criterion) {
    let t = TVar::new(0);
    c.bench_function("bench_writes", |b| {
        b.iter(|| {
            benches::write(&t, black_box(1));
        })
    });
}

criterion_group!(nocontension, bench_writes);
criterion_main!(nocontension);

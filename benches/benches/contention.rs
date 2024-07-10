use criterion::{criterion_group, criterion_main, Criterion};
use stm::{atomically, TVar};

fn bench_concurrent_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("contention");
    group.sample_size(1024);

    let t = TVar::new(0);
    group.bench_function("bench_concurrent_counts", |b| {
        b.iter(|| {
            let t = t.clone();
            std::thread::spawn(move || {
                atomically(|tx| {
                    let old = tx.read(&t)?;
                    tx.write(&t, old + 1)
                });
            })
            .join()
        })
    });
}

criterion_group!(contention, bench_concurrent_count);
criterion_main!(contention);

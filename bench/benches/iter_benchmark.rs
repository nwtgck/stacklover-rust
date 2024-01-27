use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn iter_i64() -> impl Iterator<Item = i64> {
    black_box(1..)
        .map(|x| x * 3)
        .take_while(|x| *x < 999999999)
        .chain("HELLO".chars().map(|c| c as i64).flat_map(|i| [i, i - 65]))
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator sum");
    group.bench_function("bare", |b| b.iter(|| iter_i64().sum::<i64>()));
    group.bench_function("boxed", |b| b.iter(|| Box::new(iter_i64()).sum::<i64>()));
    group.bench_function("stacklover", |b| {
        stacklover::define_struct! {
            // struct name to be defined
            Iterator1,
            fn () -> impl Iterator<Item=i64> {
                iter_i64()
            }
        }
        b.iter(|| Iterator1::new().into_inner().sum::<i64>())
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

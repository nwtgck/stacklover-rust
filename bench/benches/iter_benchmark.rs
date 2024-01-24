use criterion::{criterion_group, criterion_main, Criterion};
use stacklover::stacklover;

fn iter_i32() -> impl Iterator<Item = i32> {
    (1..)
        .map(|x| x * 3)
        .take_while(|x| *x < 20)
        .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65]))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bare iter", |b| b.iter(|| iter_i32().sum::<i32>()));
    c.bench_function("boxed iter", |b| {
        b.iter(|| Box::new(iter_i32()).sum::<i32>())
    });
    c.bench_function("stacklover iter", |b| {
        stacklover! {
            // struct name to be defined
            Iterator1,
            fn () -> impl Iterator<Item=i32> {
                iter_i32()
            }
        }
        b.iter(|| Iterator1::new().into_inner().sum::<i32>())
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

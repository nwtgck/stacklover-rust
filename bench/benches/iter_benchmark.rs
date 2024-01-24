use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stacklover::stacklover;

fn iter_i32() -> impl Iterator<Item = i32> {
    black_box(
        (1..)
            .map(|x| x * 3)
            .take_while(|x| *x < 999999999)
            .chain("HELLO".chars().map(|c| c as i32).flat_map(|i| [i, i - 65])),
    )
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator sum");
    group.bench_function("bare", |b| b.iter(|| iter_i32().sum::<i32>()));
    group.bench_function("boxed", |b| b.iter(|| Box::new(iter_i32()).sum::<i32>()));
    group.bench_function("stacklover", |b| {
        stacklover! {
            // struct name to be defined
            Iterator1,
            fn () -> impl Iterator<Item=i32> {
                iter_i32()
            }
        }
        b.iter(|| Iterator1::new().into_inner().sum::<i32>())
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use p256k1::{context::Scratch, point::Point, scalar::Scalar};

use criterion::{criterion_group, criterion_main, Criterion};
use rand_core::OsRng;

#[allow(non_snake_case)]
pub fn bench_ecmult(c: &mut Criterion) {
    let mut rng = OsRng::default();
    let n = 2048usize;

    let scalars: Vec<Scalar> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
    let points: Vec<Point> = (0..n)
        .map(|_| Point::from(Scalar::random(&mut rng)))
        .collect();

    c.bench_function("point multimult", |b| {
        b.iter(|| Point::multimult(scalars.clone(), points.clone()))
    });

    c.bench_function("point ecmult", |b| {
        b.iter(|| {
            let mut p = Point::identity();
            for i in 0..n {
                p += scalars[i] * points[i];
            }
        })
    });
}

#[allow(non_snake_case)]
pub fn bench_scratch(c: &mut Criterion) {
    let mut rng = OsRng::default();
    //let n = 1024usize * 1024usize * 32usize;
    let n = 1024usize * 32usize;

    let scalars: Vec<Scalar> = (0..n).map(|_| Scalar::random(&mut rng)).collect();
    let points: Vec<Point> = (0..n)
        .map(|_| Point::from(Scalar::random(&mut rng)))
        .collect();
    let scratch = Scratch::new(n * 1024);

    c.bench_function("point multimult with default scratch size", |b| {
        b.iter(|| Point::multimult(scalars.clone(), points.clone()))
    });

    c.bench_function("point multimult with scratch size 32M", |b| {
        b.iter(|| Point::multimult_scratch_size(scalars.clone(), points.clone(), n * 1024))
    });

    c.bench_function("point multimult with scratch size 16M", |b| {
        b.iter(|| Point::multimult_scratch_size(scalars.clone(), points.clone(), n * 1024 / 2))
    });

    c.bench_function("point multimult with scratch size 8M", |b| {
        b.iter(|| Point::multimult_scratch_size(scalars.clone(), points.clone(), n * 1024 / 4))
    });

    c.bench_function("point multimult with passed 32M scratch", |b| {
        b.iter(|| Point::multimult_scratch(scalars.clone(), points.clone(), &scratch))
    });
}

criterion_group!(benches, bench_ecmult, bench_scratch);
criterion_main!(benches);

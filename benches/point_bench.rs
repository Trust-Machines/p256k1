use p256k1::{point::Point, scalar::Scalar};

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

criterion_group!(benches, bench_ecmult);
criterion_main!(benches);

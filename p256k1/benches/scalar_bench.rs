use num_traits::identities::One;
use p256k1::scalar::Scalar;
use rand_core::OsRng;

use criterion::{criterion_group, criterion_main, Criterion};

#[allow(non_snake_case)]
pub fn bench_pow(c: &mut Criterion) {
    let mut rng = OsRng::default();
    let n = 64usize;
    let k = 1024u32;

    let scalars: Vec<Scalar> = (0..n).map(|_| Scalar::random(&mut rng)).collect();

    c.bench_function("scalar iterative pow", |b| {
        b.iter(|| {
            for i in &scalars {
                (0..k).fold(Scalar::one(), |s, _| s * i);
            }
        })
    });

    c.bench_function("scalar square and multiply pow", |b| {
        b.iter(|| {
            for i in &scalars {
                let _ = i ^ k;
            }
        })
    });
}

criterion_group!(benches, bench_pow);
criterion_main!(benches);

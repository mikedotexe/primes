use affine_math::{Affine, Ctx};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_mul_ctx(c: &mut Criterion) {
    let mut ctx = Ctx::new();
    let mut a = Affine {
        a0: 1.0,
        terms: Vec::new(),
    };
    let mut b = Affine {
        a0: 2.0,
        terms: Vec::new(),
    };
    for _ in 0..64 {
        a.terms.push((ctx.fresh(), 0.01));
        b.terms.push((ctx.fresh(), 0.02));
    }
    c.bench_function("affine_math_mul_ctx", |ben| {
        ben.iter(|| {
            let mut local_ctx = ctx.clone();
            let mut acc = a.clone();
            for _ in 0..128 {
                acc = acc.mul_ctx(&b, &mut local_ctx);
            }
            black_box(acc)
        })
    });
}

fn bench_mul_hybrid(c: &mut Criterion) {
    let mut ctx = Ctx::new();
    let mut a = Affine {
        a0: 1.0,
        terms: Vec::new(),
    };
    let mut b = Affine {
        a0: 2.0,
        terms: Vec::new(),
    };
    for _ in 0..64 {
        a.terms.push((ctx.fresh(), 0.01));
        b.terms.push((ctx.fresh(), 0.02));
    }
    c.bench_function("affine_math_mul_hybrid_max8", |ben| {
        ben.iter(|| {
            let mut local_ctx = ctx.clone();
            let mut acc = a.clone();
            for _ in 0..128 {
                acc = acc.mul_hybrid(&b, &mut local_ctx, 8);
            }
            black_box(acc)
        })
    });
}

#[cfg(any(feature = "hybrid", feature = "rigorous"))]
fn bench_inari_ia(c: &mut Criterion) {
    use inari::Interval;
    let a = Interval::new(0.36, 1.64).unwrap(); // proxy for a's interval
    let b = Interval::new(0.72, 3.28).unwrap(); // proxy for b's interval
    c.bench_function("inari_mul_interval", |ben| {
        ben.iter(|| {
            let mut acc = a;
            for _ in 0..128 {
                acc = acc * b;
            }
            black_box(acc)
        })
    });
}

#[cfg(not(any(feature = "hybrid", feature = "rigorous")))]
fn bench_inari_ia(_c: &mut Criterion) {}

criterion_group!(benches, bench_mul_ctx, bench_mul_hybrid, bench_inari_ia);
criterion_main!(benches);

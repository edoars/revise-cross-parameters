use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use pprof::{
    criterion::{Output, PProfProfiler},
    flamegraph::Options,
};
use revise_cross_parameters::{binom, float::Float, prob_b, prob_b_new, prob_beta};

const PARS: [(i64, i64, i64); 3] = [(127, 163, 85), (127, 252, 212), (127, 960, 938)];

macro_rules! bench_floats_func {
    ($fn_name:ident, $func:ident: $(#[cfg($meta:meta)] $type:ty),+) => {
        fn $fn_name(c:&mut Criterion) {
            let mut group = c.benchmark_group(stringify!($fn_name));

            for par in PARS.iter() {
                $(
                    #[cfg($meta)]
                    group.bench_with_input(BenchmarkId::new(stringify!($type), format!("{:?}", par)), &par, |b, s| {
                        b.iter(|| $func::<$type>(black_box(*s)))
                    });
                )+
            }
            group.finish();
        }
    };
}

fn binom_func<T: Float>(par: &(i64, i64, i64)) -> T {
    let (_, t, _) = par;
    let k = t / 2;
    binom(*t, k)
}

fn prob_beta_func<T: Float>(par: &(i64, i64, i64)) -> T {
    let (p, t, _) = par;
    prob_beta(*t, 0, *p)
}

fn prob_b_func<T: Float>(par: &(i64, i64, i64)) -> T {
    let (p, t, w) = par;
    prob_b(*t, 0, *w, *p)
}

fn prob_b_new_func<T: Float>(par: &(i64, i64, i64)) -> (i64, T) {
    let (p, t, w) = par;
    prob_b_new(*t, 0, *w, *p)
}

macro_rules! bench_floats {
    ($(#[cfg($meta:meta)] $type:ty,)+) => {
        bench_floats_func!(bench_binom, binom_func: $(#[cfg($meta)] $type),+);
        bench_floats_func!(bench_prob_beta, prob_beta_func: $(#[cfg($meta)] $type),+);
        bench_floats_func!(bench_prob_b, prob_b_func: $(#[cfg($meta)] $type),+);
        bench_floats_func!(bench_prob_b_new, prob_b_new_func: $(#[cfg($meta)] $type),+);
    };
}

bench_floats!(
    #[cfg(feature = "inexact")]
    revise_cross_parameters::float::F64Num,
    #[cfg(feature = "rug")]
    revise_cross_parameters::float::RugNum,
    #[cfg(feature = "dashu")]
    revise_cross_parameters::float::DashuNum,
    #[cfg(feature = "nightly-float")]
    revise_cross_parameters::float::F128Num,
);

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(Some(Options::default()))));
    targets = bench_binom, bench_prob_beta, bench_prob_b, bench_prob_b_new
}

criterion_main!(benches);

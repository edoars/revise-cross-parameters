#![cfg_attr(feature = "nightly-float", feature(f128))]
use float::Float;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use std::cmp::{max, min};

pub mod float;

fn get_default_pb_style(quiet: bool) -> ProgressStyle {
    match quiet {
        false => ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-"),
        true => ProgressStyle::default_bar().template("").unwrap(),
    }
}

// Helper function to compute binomial coefficient using high precision
#[inline]
pub fn binom<T: Float>(n: i64, k: i64) -> T {
    if k < 0 || k > n {
        return T::zero();
    }

    let k = T::from(min(k, n - k));
    let mut r = T::one();
    let mut d = T::one();
    let mut n = T::from(n);
    let one = T::one();

    loop {
        if d > k {
            break;
        }

        r.mul_div_assign(&n, &d);
        n -= &one;
        d += &one;
    }

    r
}

#[inline]
pub fn prob_beta<T: Float>(t: i64, ts: i64, p: i64) -> T {
    let p_minus_one = T::from(p - 1);
    let inv_p_minus_one = T::one() / p_minus_one;
    let one_minus_inv_p_minus_one = T::one() - &inv_p_minus_one;

    (ts..=t)
        .map(|j| {
            binom::<T>(t, j)
                * inv_p_minus_one.pow(j as u32)
                * one_minus_inv_p_minus_one.pow((t - j) as u32)
        })
        .sum()
}

#[inline]
pub fn prob_b<T: Float>(t: i64, ts: i64, w: i64, p: i64) -> T {
    let p_minus_one = T::from(p - 1);
    let inv_p_minus_one = T::one() / p_minus_one;
    let one_minus_inv_p_minus_one = T::one() - &inv_p_minus_one;
    let binom_t_w_squared = binom::<T>(t, w).pow(2);

    let sum: T = (ts..=t)
        .map(|j| {
            binom::<T>(t, j)
                * inv_p_minus_one.pow(j as u32)
                * one_minus_inv_p_minus_one.pow((t - j) as u32)
                * (max(0, j - (t - w))..=min(j, w))
                    .map(|ws| binom::<T>(j, ws).pow(2) * binom::<T>(t - j, w - ws))
                    .sum::<T>()
                / &binom_t_w_squared
        })
        .sum();

    let result = sum / prob_beta::<T>(t, ts, p);
    if result.is_nan() {
        // TODO: warn of NaN
        T::zero()
    } else {
        result
    }
}

#[inline]
pub fn estimate_attack<T: Float>(t: i64, w: i64, p: i64, quiet: bool) -> (i64, f64) {
    let result = (0..=u16::try_from(t).unwrap())
        .into_par_iter()
        .progress_with_style(get_default_pb_style(quiet))
        .map(|ts| {
            let beta_prob = prob_beta::<T>(t, ts as i64, p);
            let b_prob = prob_b::<T>(t, ts as i64, w, p);

            let comp: T = T::one() / beta_prob + T::one() / b_prob;

            (comp, ts)
        })
        .min_by(|(comp_a, _), (comp_b, _)| comp_a.partial_cmp(comp_b).unwrap())
        .unwrap();

    let complog = result.0.log2();
    let ts = result.1;

    (ts as i64, complog)
}

#[inline]
pub fn prob_b_new<T: Float>(t: i64, ts: i64, w: i64, p: i64) -> (i64, T) {
    let p_minus_one = T::from(p - 1);
    let inv_p_minus_one = T::one() / p_minus_one;
    let one_minus_inv_p_minus_one = T::one() - &inv_p_minus_one;
    let beta_prob = prob_beta::<T>(t, ts, p);
    let binom_tw = binom::<T>(t, w);

    let result = (w..=t)
        .map(|aa| {
            let prob = (ts..=t)
                .map(|j| {
                    binom::<T>(t, j)
                        * inv_p_minus_one.pow(j as u32)
                        * one_minus_inv_p_minus_one.pow((t - j) as u32)
                        * (max(0, aa - j)..=min(t - j, w))
                            .map(|ws| {
                                binom::<T>(t - j, ws)
                                    * binom::<T>(j, aa - ws)
                                    * binom::<T>(j, w - ws)
                            })
                            .sum::<T>()
                })
                .sum::<T>()
                / binom::<T>(t, aa);
            (aa, prob)
        })
        .max_by(|(_, comp_a), (_, comp_b)| comp_a.partial_cmp(comp_b).unwrap())
        .unwrap();

    let prob = result.1 / (beta_prob * binom_tw);
    if prob.is_nan() {
        // TODO: warn of NaN
        (result.0, T::zero())
    } else {
        (result.0, prob)
    }
}

#[inline]
pub fn estimate_attack_new<T: Float>(t: i64, w: i64, p: i64, quiet: bool) -> (i64, i64, f64) {
    let result = (0..=u16::try_from(t).unwrap())
        .into_par_iter()
        .progress_with_style(get_default_pb_style(quiet))
        .map(|ts| {
            let beta_prob = prob_beta::<T>(t, ts as i64, p);
            let (aa, b_prob) = prob_b_new::<T>(t, ts as i64, w, p);

            let comp = T::one() / beta_prob + T::one() / b_prob;

            (comp, ts, aa)
        })
        .min_by(|(comp_a, _, _), (comp_b, _, _)| comp_a.partial_cmp(comp_b).unwrap())
        .unwrap();

    let complog = result.0.log2();
    let ts = result.1;
    let aa = result.2;

    (ts as i64, aa, complog)
}

#[cfg(test)]
mod tests {
    macro_rules! float_test {
        ($name:ident: $type:ty) => {
            mod $name {
                use crate::{float::Float, prob_b, prob_b_new, prob_beta};

                #[test]
                fn test_prob_beta() {
                    let (p, t, _) = (127, 163, 85);
                    let ts = 35;
                    let prob = prob_beta::<$type>(t, ts, p);

                    assert_eq!(-prob.log2().round(), 127.0);
                }

                #[test]
                fn test_prob_b() {
                    let (p, t, w) = (127, 163, 85);
                    let ts = 35;
                    let prob = prob_b::<$type>(t, ts, w, p);

                    assert_eq!(-prob.log2().round(), 127.0);
                }

                #[test]
                fn test_prob_b_new() {
                    let (p, t, w) = (127, 252, 212);
                    let ts = 38;
                    let (_, prob) = prob_b_new::<$type>(t, ts, w, p);

                    assert_eq!(-prob.log2().round(), 120.0);
                }
            }
        };
    }

    macro_rules! tests {
        ($(#[cfg($meta:meta)] $name:ident: $type:ty,)*) => {
            $(
                #[cfg($meta)]
                float_test! { $name: $type }
            )*
        };

        ($($name:ident: $type:ty,)*) => {
            $(
                float_test! { $name: $type }
            )*
        };
    }

    tests! {
        #[cfg(feature = "inexact")] f64: crate::float::F64Num,
        #[cfg(feature = "rug")] rug: crate::float::RugNum,
        #[cfg(feature = "dashu")] dashu: crate::float::DashuNum,
        #[cfg(feature = "nightly-float")] f128: crate::float::F128Num,
    }
}

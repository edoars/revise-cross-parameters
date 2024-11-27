use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use rug::ops::Pow;
use rug::Float;
use std::{cmp::max, cmp::min};

const PRECISION: u32 = 32; // Adjust precision as needed

fn get_default_pb_style(quiet: bool) -> ProgressStyle {
    let progress_style = match quiet {
        false => ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
            .unwrap()
            .progress_chars("#>-"),
        true => ProgressStyle::default_bar().template("").unwrap(),
    };

    progress_style
}

// Helper function to compute binomial coefficient using high precision
#[inline]
fn binom(n: i64, k: i64) -> Float {
    if k < 0 || k > n {
        return Float::with_val(PRECISION, 0);
    }

    let k = min(k, n - k);
    let mut result = Float::with_val(PRECISION, 1);

    for i in 0..k {
        result *= Float::with_val(PRECISION, n - i);
        result /= Float::with_val(PRECISION, i + 1);
    }

    result
}

#[inline]
fn prob_beta(t: i64, ts: i64, p: i64) -> Float {
    let mut sum = Float::with_val(PRECISION, 0);
    let p_minus_one = Float::with_val(PRECISION, p - 1);
    let one = Float::with_val(PRECISION, 1);
    let inv_p_minus_one = Float::with_val(PRECISION, &one / &p_minus_one);

    for j in ts..=t {
        let binom_tj = binom(t, j);
        let inv_p_power = inv_p_minus_one.clone().pow(j);
        let comp_power = (one.clone() - &inv_p_minus_one).pow(t - j);
        sum += binom_tj * inv_p_power * comp_power;
    }

    sum
}

#[inline]
fn prob_b(t: i64, ts: i64, w: i64, p: i64) -> Float {
    let mut outer_sum = Float::with_val(PRECISION, 0);
    let beta_prob = prob_beta(t, ts, p);
    let p_minus_one = Float::with_val(PRECISION, p - 1);
    let one = Float::with_val(PRECISION, 1);
    let inv_p_minus_one = Float::with_val(PRECISION, &one / &p_minus_one);
    let binom_t_w_squared = binom(t, w).pow(2);

    for j in ts..=t {
        let binom_tj = binom(t, j);
        let inv_p_power = inv_p_minus_one.clone().pow(j);
        let comp_power = (one.clone() - &inv_p_minus_one).pow(t - j);

        let mut inner_sum = Float::with_val(PRECISION, 0);
        let ws_start = max(0, j - (t - w));
        let ws_end = min(j, w);

        for ws in ws_start..=ws_end {
            let term = binom(j, ws).pow(2) * binom(t - j, w - ws);
            inner_sum += term;
        }

        inner_sum /= &binom_t_w_squared;
        outer_sum += binom_tj * inv_p_power * comp_power * inner_sum;
    }

    outer_sum / beta_prob
}

#[inline]
pub fn attack(t: i64, w: i64, p: i64, quiet: bool) -> (i64, Float) {
    let result = (0..=u16::try_from(t).unwrap())
        .into_par_iter()
        .progress_with_style(get_default_pb_style(quiet))
        .map(|ts| {
            let beta_prob = prob_beta(t, ts as i64, p);
            let b_prob = prob_b(t, ts as i64, w, p);

            let comp = Float::with_val(PRECISION, 1) / &beta_prob
                + Float::with_val(PRECISION, 1) / &b_prob;

            (comp, ts)
        })
        .min_by(|(comp_a, _), (comp_b, _)| comp_a.partial_cmp(comp_b).unwrap())
        .unwrap();

    let complog = result.0.log2();
    let ts = result.1;

    (ts as i64, complog)
}

#[inline]
fn prob_b_new(t: i64, ts: i64, w: i64, p: i64) -> (i64, Float) {
    let mut max_prob = Float::with_val(PRECISION, 0);
    let mut max_aa: i64 = 0;

    let beta_prob = prob_beta(t, ts, p);
    let binom_tw = binom(t, w);
    let p_minus_one = Float::with_val(PRECISION, p - 1);
    let one = Float::with_val(PRECISION, 1);
    let inv_p_minus_one = Float::with_val(PRECISION, &one / &p_minus_one);

    for aa in w..=t {
        let mut outer_sum = Float::with_val(PRECISION, 0);

        for j in ts..=t {
            let binom_tj = binom(t, j);
            let inv_p_power = inv_p_minus_one.clone().pow(j);
            let comp_power = (one.clone() - &inv_p_minus_one).pow(t - j);

            let mut inner_sum = Float::with_val(PRECISION, 0);
            let ws_start = max(0, aa - j);
            let ws_end = min(t - j, aa);

            for ws in ws_start..=ws_end {
                let term = binom(t - j, ws) * binom(j, aa - ws) * binom(j, w - ws);
                inner_sum += term;
            }

            outer_sum += binom_tj * inv_p_power * comp_power * inner_sum
        }

        outer_sum /= binom(t, aa);

        if outer_sum > max_prob {
            max_prob = outer_sum;
            max_aa = aa;
        }
    }

    (max_aa, max_prob / (beta_prob * binom_tw))
}

#[inline]
pub fn attack_new(t: i64, w: i64, p: i64, quiet: bool) -> (i64, i64, Float) {
    let result = (0..=u16::try_from(t).unwrap())
        .into_par_iter()
        .progress_with_style(get_default_pb_style(quiet))
        .map(|ts| {
            let beta_prob = prob_beta(t, ts as i64, p);
            let (aa, b_prob) = prob_b_new(t, ts as i64, w, p);

            let comp = Float::with_val(PRECISION, 1) / &beta_prob
                + Float::with_val(PRECISION, 1) / &b_prob;

            (comp, ts, aa)
        })
        .min_by(|(comp_a, _, _), (comp_b, _, _)| comp_a.partial_cmp(comp_b).unwrap())
        .unwrap();

    let complog = result.0.log2();
    let ts = result.1;
    let aa = result.2;

    (ts as i64, aa, complog)
}

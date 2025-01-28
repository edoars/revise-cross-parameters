use crate::float::{Float, FloatAssignOps, FloatOps, MulDivAssign};
use derive_more::{Add, Div, Mul, Sub};
use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

macro_rules! newfloat {
    ($(#[$attr:meta])* $name:ident: $inner:ident) => {
        #[derive(Debug, PartialEq, PartialOrd, Mul, Div, Add, Sub)]
        #[mul(forward)]
        #[div(forward)]
        $(#[$attr])*
        pub struct $name($inner);

        impl Sum for $name {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                iter.fold(Self::zero(), |a, b| a + b)
            }
        }

        impl<'a> AddAssign<&'a $name> for $name {
            fn add_assign(&mut self, rhs: &'a Self) {
                self.0 += &rhs.0
            }
        }

        impl<'a> SubAssign<&'a $name> for $name {
            fn sub_assign(&mut self, rhs: &'a Self) {
                self.0 -= &rhs.0
            }
        }

        impl<'a> MulAssign<&'a $name> for $name {
            fn mul_assign(&mut self, rhs: &'a Self) {
                self.0 *= &rhs.0
            }
        }

        impl<'a> DivAssign<&'a $name> for $name {
            fn div_assign(&mut self, rhs: &'a Self) {
                self.0 /= &rhs.0
            }
        }

        impl<'a> Add<&'a $name> for $name {
            type Output = Self;

            fn add(self, rhs: &'a Self) -> Self {
                $name(self.0 + &rhs.0)
            }
        }

        impl<'a> Sub<&'a $name> for $name {
            type Output = Self;

            fn sub(self, rhs: &'a Self) -> Self {
                $name(self.0 - &rhs.0)
            }
        }

        impl<'a> Mul<&'a $name> for $name {
            type Output = Self;

            fn mul(self, rhs: &'a Self) -> Self {
                $name(self.0 * &rhs.0)
            }
        }

        impl<'a> Div<&'a $name> for $name {
            type Output = Self;

            fn div(self, rhs: &'a Self) -> Self {
                $name(self.0 / &rhs.0)
            }
        }

        impl FloatOps for $name {}
        impl<'a> FloatOps<&'a Self, Self> for $name {}
        impl<'a> FloatAssignOps<&'a Self> for $name {}
    };
}

#[cfg(feature = "inexact")]
pub(super) mod inexact {
    use super::*;

    newfloat! {
        /// Newtype wrapper around primitive [`f64`] for [`Float`] implementation.
        F64Num: f64
    }

    impl From<i64> for F64Num {
        fn from(value: i64) -> Self {
            F64Num(value as f64)
        }
    }

    impl Zero for F64Num {
        fn zero() -> Self {
            F64Num(0.0)
        }

        fn is_zero(&self) -> bool {
            self.0 == 0.0
        }

        fn set_zero(&mut self) {
            self.0 = 0.0;
        }
    }

    impl One for F64Num {
        fn one() -> Self {
            F64Num(1.0)
        }

        fn is_one(&self) -> bool
        where
            Self: PartialEq,
        {
            self.0 == 1.0
        }

        fn set_one(&mut self) {
            self.0 = 1.0;
        }
    }

    impl<'a> MulDivAssign<&'a Self, &'a Self> for F64Num {
        fn mul_div_assign(&mut self, mul: &'a Self, div: &'a Self) {
            self.0 *= mul.0 / div.0
        }
    }

    impl Float for F64Num {
        fn pow(&self, exp: u32) -> Self {
            F64Num(self.0.powi(exp as i32))
        }

        fn log2(&self) -> f64 {
            self.0.log2()
        }

        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
    }
}

#[cfg(feature = "rug")]
pub(super) mod rug {
    use super::*;
    use ::rug::Float as RugFloat;

    newfloat! {
        /// Newtype wrapper around [`::rug::Float`] for [`Float`] implementation.
        ///
        /// Use a fixed precision of 64 significant bits for float representation.
        RugNum: RugFloat
    }

    impl RugNum {
        const PRECISION: u32 = 64;
    }

    impl From<i64> for RugNum {
        fn from(value: i64) -> Self {
            RugNum(RugFloat::with_val(Self::PRECISION, value))
        }
    }

    impl Zero for RugNum {
        fn zero() -> Self {
            RugNum(RugFloat::with_val(Self::PRECISION, 0))
        }

        fn is_zero(&self) -> bool {
            self.0 == RugFloat::with_val(Self::PRECISION, 0)
        }

        fn set_zero(&mut self) {
            self.0 = RugFloat::with_val(Self::PRECISION, 0)
        }
    }

    impl One for RugNum {
        fn one() -> Self {
            RugNum(RugFloat::with_val(Self::PRECISION, 1))
        }

        fn is_one(&self) -> bool
        where
            Self: PartialEq,
        {
            self.0 == RugFloat::with_val(Self::PRECISION, 1)
        }

        fn set_one(&mut self) {
            self.0 = RugFloat::with_val(Self::PRECISION, 1)
        }
    }

    impl<'a> MulDivAssign<&'a Self, &'a Self> for RugNum {
        fn mul_div_assign(&mut self, mul: &'a Self, div: &'a Self) {
            self.0 *= &mul.0;
            self.0 /= &div.0
        }
    }

    impl Float for RugNum {
        fn pow(&self, exp: u32) -> Self {
            let result = RugFloat::with_val(Self::PRECISION, ::rug::ops::Pow::pow(&self.0, exp));
            RugNum(result)
        }

        fn log2(&self) -> f64 {
            self.0.clone().log2().to_f64()
        }

        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
    }
}

#[cfg(feature = "dashu")]
pub(super) mod dashu {
    use super::*;
    type FBig = dashu_float::FBig;

    newfloat! {
        /// Newtype wrapper around [`dashu_float::FBig`] for [`Float`] implementation.
        ///
        /// Use a fixed precision of 32 significant bits for float representation.
        DashuNum: FBig
    }

    impl DashuNum {
        const PRECISION: usize = 32;
    }

    impl From<i64> for DashuNum {
        fn from(value: i64) -> Self {
            DashuNum(FBig::from(value).with_precision(Self::PRECISION).value())
        }
    }

    impl Zero for DashuNum {
        fn zero() -> Self {
            DashuNum(FBig::ZERO)
        }

        fn is_zero(&self) -> bool {
            self.0 == FBig::ZERO
        }

        fn set_zero(&mut self) {
            self.0 = FBig::ZERO
        }
    }

    impl One for DashuNum {
        fn one() -> Self {
            DashuNum(FBig::ONE)
        }

        fn is_one(&self) -> bool
        where
            Self: PartialEq,
        {
            self.0 == FBig::ONE
        }

        fn set_one(&mut self) {
            self.0 = FBig::ONE
        }
    }

    impl<'a> MulDivAssign<&'a Self, &'a Self> for DashuNum {
        fn mul_div_assign(&mut self, mul: &'a Self, div: &'a Self) {
            self.0 *= &mul.0 / &div.0;
        }
    }

    impl Float for DashuNum {
        fn pow(&self, exp: u32) -> Self {
            DashuNum(self.0.powi(exp.into()))
        }

        fn log2(&self) -> f64 {
            let log2 = self.0.ln() / FBig::from(2).with_precision(Self::PRECISION).value().ln();
            log2.to_f64().value()
        }

        fn is_nan(&self) -> bool {
            false
        }
    }
}

#[cfg(feature = "nightly-float")]
pub(super) mod f128 {
    use super::*;

    newfloat! {
        /// Newtype wrapper around unstable [`prim@f128`] float type for [`Float`] implementation.
        F128Num: f128
    }

    impl From<i64> for F128Num {
        fn from(value: i64) -> Self {
            F128Num(f128::from(value as f64))
        }
    }

    impl Zero for F128Num {
        fn zero() -> Self {
            F128Num(0.0_f128)
        }

        fn is_zero(&self) -> bool {
            self.0 == 0.0_f128
        }

        fn set_zero(&mut self) {
            self.0 = 0.0_f128;
        }
    }

    impl One for F128Num {
        fn one() -> Self {
            F128Num(1.0_f128)
        }

        fn is_one(&self) -> bool
        where
            Self: PartialEq,
        {
            self.0 == 1.0_f128
        }

        fn set_one(&mut self) {
            self.0 = 1.0_f128;
        }
    }

    impl<'a> MulDivAssign<&'a Self, &'a Self> for F128Num {
        fn mul_div_assign(&mut self, mul: &'a Self, div: &'a Self) {
            self.0 *= mul.0 / div.0
        }
    }

    impl Float for F128Num {
        fn pow(&self, exp: u32) -> Self {
            F128Num(self.0.powi(exp as i32))
        }

        fn log2(&self) -> f64 {
            self.0.log2() as f64
        }

        fn is_nan(&self) -> bool {
            self.0.is_nan()
        }
    }
}

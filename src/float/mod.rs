use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

mod floats;

/// The multiply and divide assignment operator.
pub trait MulDivAssign<Mul = Self, Div = Self> {
    /// Performs multiplication of `self` by `mul` and division by `div`, assigning the result to `self`.
    fn mul_div_assign(&mut self, mul: Mul, div: Div);
}

/// Generic trait for types implementing basic numeric operations.
pub trait FloatOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

/// Generic trait for types implementing numeric assignment operators (like `+=`).
pub trait FloatAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}

/// Generic trait for high-precision float.
pub trait Float
where
    Self: PartialOrd
        + Zero
        + One
        + FloatOps<Self, Self>
        + for<'a> FloatOps<&'a Self, Self>
        + for<'a> FloatAssignOps<&'a Self>
        + for<'a> MulDivAssign<&'a Self, &'a Self>
        + Sum
        + From<i64>
        + Send,
{
    /// Returns `self` to the power `exp`.
    fn pow(&self, exp: u32) -> Self;

    /// Computes the logarithm to base 2, rounding to the nearest.
    fn log2(&self) -> f64;

    /// Returns [`true`] if `self` is not a number.
    fn is_nan(&self) -> bool;
}

#[cfg(feature = "dashu")]
pub use floats::dashu::DashuNum;
#[cfg(feature = "nightly-float")]
pub use floats::f128::F128Num;
#[cfg(feature = "inexact")]
pub use floats::inexact::F64Num;
#[cfg(feature = "rug")]
pub use floats::rug::RugNum;

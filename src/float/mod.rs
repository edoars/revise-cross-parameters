use num_traits::{One, Zero};
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

mod floats;

pub trait MulDivAssign<Mul = Self, Div = Self> {
    fn mul_div_assign(&mut self, mul: Mul, div: Div);
}

pub trait FloatOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
{
}

pub trait FloaAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs>
{
}

pub trait Float
where
    Self: PartialOrd
        + Zero
        + One
        + FloatOps<Self, Self>
        + for<'a> FloatOps<&'a Self, Self>
        + for<'a> FloaAssignOps<&'a Self>
        + for<'a> MulDivAssign<&'a Self, &'a Self>
        + Sum
        + From<i64>
        + Send,
{
    fn pow(&self, exp: u32) -> Self;
    fn log2(&self) -> f64;
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

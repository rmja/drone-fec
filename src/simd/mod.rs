#[cfg(feature = "cortex-m4")]
mod m4;

#[cfg(not(feature = "cortex-m4"))]
mod fallback;

#[cfg(feature = "cortex-m4")]
pub use self::m4::*;

#[cfg(not(feature = "cortex-m4"))]
pub use self::fallback::*;

pub trait SaturateBits {
    fn saturate_bits(self, bits: usize) -> Self;
}

pub trait SaturatingExt {
    fn saturating_add_i8(self, rhs: Self) -> Self;
    fn saturating_sub_i8(self, rhs: Self) -> Self;
}

pub trait HalfExt {
    fn half_add_i8(self, rhs: Self) -> Self;
    fn half_sub_i8(self, rhs: Self) -> Self;
}

pub trait CmpExt {
    fn max_i8(self, rhs: Self) -> Self;
    fn min_i8(self, rhs: Self) -> Self;
}

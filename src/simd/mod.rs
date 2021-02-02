#[cfg(feature = "cortex-m4")]
mod m4;

#[cfg(not(feature = "cortex-m4"))]
mod fallback;

#[cfg(feature = "cortex-m4")]
pub use self::m4::*;

#[cfg(not(feature = "cortex-m4"))]
pub use self::fallback::*;

pub trait SaturateBits<T> {
    // Saturate to given number of bits.
    fn saturate_bits<const BITS: usize>(self) -> T;
}

pub trait SaturateInto<T> {
    fn saturate_into(self) -> T;
}

impl SaturateInto<i8> for i32 {
    #[inline(always)]
    fn saturate_into(self) -> i8 {
        let sat: i32 = self.saturate_bits::<8>();
        sat as i8
    }
}

impl SaturateInto<u8> for i32 {
    #[inline(always)]
    fn saturate_into(self) -> u8 {
        let sat: u32 = self.saturate_bits::<8>();
        sat as u8
    }
}

pub trait SaturatingExt {
    /// Quad 8-bit saturating add.
    fn saturating_add_i8(self, rhs: Self) -> Self;
    /// Quad 8-bit saturating sub.
    fn saturating_sub_i8(self, rhs: Self) -> Self;
}

pub trait HalfExt {
    /// Quad 8-bit signed addition with halved results.
    fn half_add_i8(self, rhs: Self) -> Self;
    /// Quad 8-bit signed subtraction with halved results
    fn half_sub_i8(self, rhs: Self) -> Self;
}

pub trait CmpExt {
    /// Quad 8-bit max.
    fn max_i8(self, rhs: Self) -> Self;
    /// Quad 8-bit min.
    fn min_i8(self, rhs: Self) -> Self;
}


#[cfg(any(test, target_tests))]
pub mod tests {
    use crate::dword::DWord;

    use super::*;

    #[test]
    fn saturate_bits_i32() {
        saturate_bits_i32_impl();
    }

    #[test]
    #[should_panic]
    fn saturate_bits_i32_0bits() {
        let _: i32 = 0i32.saturate_bits::<0>();
    }

    pub fn saturate_bits_i32_impl() {
        assert_eq!(-128, i32::MIN.saturate_bits::<8>());
        assert_eq!(-128, (-129_i32).saturate_bits::<8>());
        assert_eq!(-128, (-128_i32).saturate_bits::<8>());
        assert_eq!(-127, (-127_i32).saturate_bits::<8>());
        assert_eq!(0, 0_i32.saturate_bits::<8>());
        assert_eq!(126, 126_i32.saturate_bits::<8>());
        assert_eq!(127, 127_i32.saturate_bits::<8>());
        assert_eq!(127, 128_i32.saturate_bits::<8>());
        assert_eq!(127, i32::MAX.saturate_bits::<8>());

        assert_eq!(i32::MIN, i32::MIN.saturate_bits::<32>());
        assert_eq!(0x3FFFFFFF, i32::MAX.saturate_bits::<31>());
        assert_eq!(i32::MAX, i32::MAX.saturate_bits::<32>());
    }

    #[test]
    fn saturate_bits_u32() {
        saturate_bits_u32_impl();
    }

    #[test]
    #[should_panic]
    fn saturate_bits_u32_32bits() {
        let _: u32 = 0i32.saturate_bits::<32>();
    }

    pub fn saturate_bits_u32_impl() {
        assert_eq!(0_u32, 0.saturate_bits::<0>());
        assert_eq!(0_u32, 1.saturate_bits::<0>());

        assert_eq!(0_u32, 0.saturate_bits::<8>());
        assert_eq!(1_u32, 1.saturate_bits::<8>());
        assert_eq!(254_u32, 254.saturate_bits::<8>());
        assert_eq!(255_u32, 255.saturate_bits::<8>());
        assert_eq!(255_u32, 256.saturate_bits::<8>());
        assert_eq!(255_u32, i32::MAX.saturate_bits::<8>());

        assert_eq!(0x3FFFFFFF_u32, i32::MAX.saturate_bits::<30>());
        assert_eq!(0x7FFFFFFF_u32, i32::MAX.saturate_bits::<31>());
    }

    #[test]
    fn saturating_add() {
        saturating_add_impl();
    }

    pub fn saturating_add_impl() {
        let lhs = DWord::new_i8h([50, 120, 120, -120]).u32();
        let rhs = DWord::new_i8h([50, 20, -20, -20]).u32();
        assert_eq!(
            DWord::new_i8h([100, 127, 100, -128]).u32(),
            lhs.saturating_add_i8(rhs)
        )
    }

    #[test]
    fn saturating_sub() {
        saturating_sub_impl();
    }

    pub fn saturating_sub_impl() {
        let lhs = DWord::new_i8h([10, -10, -10, 0]).u32();
        let rhs = DWord::new_i8h([7, -7, 120, -128]).u32();
        assert_eq!(
            DWord::new_i8h([3, -3, -128, 127]).u32(),
            lhs.saturating_sub_i8(rhs)
        )
    }

    #[test]
    fn half_add() {
        half_add_impl();
    }

    pub fn half_add_impl() {
        let lhs = DWord::new_i8h([100, -1, 0, -100]).u32();
        let rhs = DWord::new_i8h([28, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([64, -65, -64, -65]).u32(),
            lhs.half_add_i8(rhs)
        )
    }

    #[test]
    fn half_sub() {
        half_sub_impl();
    }

    pub fn half_sub_impl() {
        let lhs = DWord::new_i8h([100, -1, 0, -100]).u32();
        let rhs = DWord::new_i8h([28, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([36, 63, 64, -36]).u32(),
            lhs.half_sub_i8(rhs)
        )
    }

    #[test]
    fn max() {
        max_impl();
    }

    pub fn max_impl() {
        let lhs = DWord::new_i8h([100, 1, 0, -100]).u32();
        let rhs = DWord::new_i8h([27, -128, -128, -29]).u32();
        assert_eq!(DWord::new_i8h([100, 1, 0, -29]).u32(), lhs.max_i8(rhs))
    }

    #[test]
    fn min() {
        min_impl()
    }

    pub fn min_impl() {
        let lhs = DWord::new_i8h([100, 1, 0, -100]).u32();
        let rhs = DWord::new_i8h([27, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([27, -128, -128, -100]).u32(),
            lhs.min_i8(rhs)
        )
    }
}

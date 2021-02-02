use super::*;
use crate::dword::DWord;

#[inline(always)]
const fn half_add(lhs: i8, rhs: i8) -> i8 {
    ((lhs as i16 + rhs as i16) / 2) as i8
}

#[inline(always)]
const fn half_sub(lhs: i8, rhs: i8) -> i8 {
    ((lhs as i16 - rhs as i16) / 2) as i8
}

impl SaturateBits for i32 {
    #[inline(always)]
    fn saturate_bits(self, bits: usize) -> Self {
        debug_assert!(bits <= 32);
        if bits > 0 {
            // bits   max_value
            //   0  > 0x00000000
            //   1  > 0x00000000
            //   2  > 0x00000001
            //  16  > 0x00007FFF
            //  31  > 0x3FFFFFFF
            //  32  > 0xCFFFFFFF
            let max_value = ((1u32 << (bits - 1)) - 1) as i32;
            let min_value = (-max_value) - 1;
            self.min(max_value).max(min_value)
        } else {
            0
        }
    }
}

impl SaturateBits for u32 {
    #[inline(always)]
    fn saturate_bits(self, bits: usize) -> Self {
        debug_assert!(bits <= 32);
        // bits   max_value
        //   0  > 0x00000000
        //   1  > 0x00000001
        //   2  > 0x00000003
        //  16  > 0x0000FFFF
        //  31  > 0xCFFFFFFF
        //  32  > 0xFFFFFFFF
        let max_value = 1u32.checked_shl(bits as u32).unwrap_or(0).wrapping_sub(1);
        self.min(max_value)
    }
}

macro_rules! impl_simd {
    ($type:ident, $dword_new:ident) => {
        impl SaturatingExt for $type {
            #[inline(always)]
            fn saturating_add_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    lhs[0].saturating_add(rhs[0]),
                    lhs[1].saturating_add(rhs[1]),
                    lhs[2].saturating_add(rhs[2]),
                    lhs[3].saturating_add(rhs[3]),
                ])
                .$type()
            }

            #[inline(always)]
            fn saturating_sub_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    lhs[0].saturating_sub(rhs[0]),
                    lhs[1].saturating_sub(rhs[1]),
                    lhs[2].saturating_sub(rhs[2]),
                    lhs[3].saturating_sub(rhs[3]),
                ])
                .$type()
            }
        }

        impl HalfExt for $type {
            #[inline(always)]
            fn half_add_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    half_add(lhs[0], rhs[0]),
                    half_add(lhs[1], rhs[1]),
                    half_add(lhs[2], rhs[2]),
                    half_add(lhs[3], rhs[3]),
                ])
                .$type()
            }

            #[inline(always)]
            fn half_sub_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    half_sub(lhs[0], rhs[0]),
                    half_sub(lhs[1], rhs[1]),
                    half_sub(lhs[2], rhs[2]),
                    half_sub(lhs[3], rhs[3]),
                ])
                .$type()
            }
        }

        impl CmpExt for $type {
            #[inline(always)]
            fn max_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    lhs[0].max(rhs[0]),
                    lhs[1].max(rhs[1]),
                    lhs[2].max(rhs[2]),
                    lhs[3].max(rhs[3]),
                ])
                .$type()
            }

            #[inline(always)]
            fn min_i8(self: $type, rhs: Self) -> Self {
                let lhs = DWord::$dword_new(self).i8h();
                let rhs = DWord::$dword_new(rhs).i8h();
                DWord::new_i8h([
                    lhs[0].min(rhs[0]),
                    lhs[1].min(rhs[1]),
                    lhs[2].min(rhs[2]),
                    lhs[3].min(rhs[3]),
                ])
                .$type()
            }
        }
    };
}

impl_simd!(i32, new_i32);
impl_simd!(u32, new_u32);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturate_bits_i32() {
        assert_eq!(0, (-1_i32).saturate_bits(0));
        assert_eq!(0, 0_i32.saturate_bits(0));
        assert_eq!(0, 1_i32.saturate_bits(0));

        assert_eq!(-128, (-129_i32).saturate_bits(8));
        assert_eq!(-128, (-128_i32).saturate_bits(8));
        assert_eq!(-127, (-127_i32).saturate_bits(8));
        assert_eq!(0, 0_i32.saturate_bits(8));
        assert_eq!(126, 126_i32.saturate_bits(8));
        assert_eq!(127, 127_i32.saturate_bits(8));
        assert_eq!(127, 128_i32.saturate_bits(8));

        assert_eq!(i32::MIN, i32::MIN.saturate_bits(32));
        assert_eq!(i32::MAX, i32::MAX.saturate_bits(32));
    }

    #[test]
    fn saturate_bits_u32() {
        assert_eq!(0, 0_u32.saturate_bits(0));
        assert_eq!(0, 1_u32.saturate_bits(0));

        assert_eq!(0, 0_u32.saturate_bits(8));
        assert_eq!(1, 1_u32.saturate_bits(8));
        assert_eq!(254, 254_u32.saturate_bits(8));
        assert_eq!(255, 255_u32.saturate_bits(8));
        assert_eq!(255, 256_u32.saturate_bits(8));

        assert_eq!(u32::MAX, u32::MAX.saturate_bits(32));
    }

    #[test]
    fn saturating_add() {
        let lhs = DWord::new_i8h([50, 120, 120, -120]).u32();
        let rhs = DWord::new_i8h([50, 20, -20, -20]).u32();
        assert_eq!(
            DWord::new_i8h([100, 127, 100, -128]).u32(),
            lhs.saturating_add_i8(rhs)
        )
    }

    #[test]
    fn saturating_sub() {
        let lhs = DWord::new_i8h([10, -10, -10, 0]).u32();
        let rhs = DWord::new_i8h([7, -7, 120, -128]).u32();
        assert_eq!(
            DWord::new_i8h([3, -3, -128, 127]).u32(),
            lhs.saturating_sub_i8(rhs)
        )
    }

    #[test]
    fn half_add() {
        let lhs = DWord::new_i8h([100, -1, 0, -100]).u32();
        let rhs = DWord::new_i8h([28, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([64, -64, -64, -64]).u32(),
            lhs.half_add_i8(rhs)
        )
    }

    #[test]
    fn half_sub() {
        let lhs = DWord::new_i8h([100, -1, 0, -100]).u32();
        let rhs = DWord::new_i8h([28, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([36, 63, 64, -35]).u32(),
            lhs.half_sub_i8(rhs)
        )
    }

    #[test]
    fn max() {
        let lhs = DWord::new_i8h([100, 1, 0, -100]).u32();
        let rhs = DWord::new_i8h([27, -128, -128, -29]).u32();
        assert_eq!(DWord::new_i8h([100, 1, 0, -29]).u32(), lhs.max_i8(rhs))
    }

    #[test]
    fn min() {
        let lhs = DWord::new_i8h([100, 1, 0, -100]).u32();
        let rhs = DWord::new_i8h([27, -128, -128, -29]).u32();
        assert_eq!(
            DWord::new_i8h([27, -128, -128, -100]).u32(),
            lhs.min_i8(rhs)
        )
    }
}

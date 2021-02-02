use super::*;
use crate::dword::DWord;

#[inline(always)]
const fn half_add(lhs: i8, rhs: i8) -> i8 {
    let sum = lhs as i16 + rhs as i16;
    if sum >= 0 {
        (sum / 2) as i8
    } else {
        ((sum - 1) / 2) as i8
    }
}

#[inline(always)]
const fn half_sub(lhs: i8, rhs: i8) -> i8 {
    let sum = lhs as i16 - rhs as i16;
    if sum >= 0 {
        (sum / 2) as i8
    } else {
        ((sum - 1) / 2) as i8
    }
}

impl SaturateBits<i32> for i32 {
    #[inline(always)]
    fn saturate_bits<const BITS: usize>(self) -> i32 {
        debug_assert!(BITS >= 1 && BITS <= 32);
        // bits   max_value
        //   1  > 0x00000000
        //   2  > 0x00000001
        //  16  > 0x00007FFF
        //  31  > 0x3FFFFFFF
        //  32  > 0xCFFFFFFF
        let max_value = ((1u32 << (BITS - 1)) - 1) as i32;
        let min_value = (-max_value) - 1;
        self.min(max_value).max(min_value)
    }
}

impl SaturateBits<u32> for i32 {
    #[inline(always)]
    fn saturate_bits<const BITS: usize>(self) -> u32 {
        debug_assert!(BITS <= 31);
        // bits   max_value
        //   0  > 0x00000000
        //   1  > 0x00000001
        //   2  > 0x00000003
        //  16  > 0x0000FFFF
        //  31  > 0xCFFFFFFF
        let max_value = 1i32.checked_shl(BITS as u32).unwrap_or(0).wrapping_sub(1);
        self.min(max_value) as u32
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

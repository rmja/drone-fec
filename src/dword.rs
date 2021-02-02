use crate::simd::*;
use core::ops::{BitAnd, BitOr, Shl, Shr};

/// A 32bit DWORD.
#[derive(Clone, Copy)]
pub union DWord {
    i32: i32,
    u32: u32,
    i8: [i8; 4],
    u8: [u8; 4],
}

impl DWord {
    #[inline]
    pub const fn new_i32(i32: i32) -> Self {
        Self { i32 }
    }

    #[inline]
    pub const fn new_u32(u32: u32) -> Self {
        Self { u32 }
    }

    #[inline]
    pub const fn new_i8h(i8: [i8; 4]) -> Self {
        Self { i8 }
    }

    #[inline]
    pub const fn i32(self) -> i32 {
        unsafe { self.i32 }
    }

    #[inline]
    pub const fn u32(self) -> u32 {
        unsafe { self.u32 }
    }

    /// Get the bytes in host order
    #[inline]
    pub const fn i8h(self) -> [i8; 4] {
        unsafe { self.i8 }
    }

    /// Get the bytes such that [0] is the MSB and [3] is the LSB.
    #[inline]
    pub const fn i8be(self) -> [i8; 4] {
        unsafe {
            Self {
                u32: self.u32.to_be(),
            }
            .i8
        }
    }

    /// Get the bytes such that [0] is the LSB and [3] is the MSB.
    #[inline]
    pub const fn i8le(self) -> [i8; 4] {
        unsafe {
            Self {
                u32: self.u32.to_le(),
            }
            .i8
        }
    }

    /// Get the bytes in host order
    #[inline]
    pub const fn u8h(self) -> [u8; 4] {
        unsafe { self.u8 }
    }

    /// Get the bytes such that [0] is the MSB and [3] is the LSB.
    #[inline]
    pub const fn u8be(self) -> [u8; 4] {
        unsafe {
            Self {
                u32: self.u32.to_be(),
            }
            .u8
        }
    }

    /// Get the bytes such that [0] is the LSB and [3] is the MSB.
    #[inline]
    pub const fn u8le(self) -> [u8; 4] {
        unsafe {
            Self {
                u32: self.u32.to_le(),
            }
            .u8
        }
    }

    #[inline]
    pub const fn rotate_right(self, n: u32) -> Self {
        Self {
            u32: self.u32().rotate_right(n),
        }
    }

    #[inline]
    pub const fn rotate_left(self, n: u32) -> Self {
        Self {
            u32: self.u32().rotate_left(n),
        }
    }
}

impl BitAnd for DWord {
    type Output = DWord;

    #[inline(always)]
    fn bitand(self, rhs: Self) -> Self::Output {
        DWord::new_u32(self.u32() & rhs.u32())
    }
}

impl BitAnd<u32> for DWord {
    type Output = DWord;

    #[inline(always)]
    fn bitand(self, rhs: u32) -> Self::Output {
        DWord::new_u32(self.u32() & rhs)
    }
}

impl BitOr for DWord {
    type Output = DWord;

    #[inline(always)]
    fn bitor(self, rhs: Self) -> Self::Output {
        DWord::new_u32(self.u32() | rhs.u32())
    }
}

impl BitOr<u32> for DWord {
    type Output = DWord;

    #[inline(always)]
    fn bitor(self, rhs: u32) -> Self::Output {
        DWord::new_u32(self.u32() | rhs)
    }
}

impl Shl<usize> for DWord {
    type Output = DWord;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        DWord::new_u32(self.u32() << rhs)
    }
}

impl Shr<usize> for DWord {
    type Output = DWord;

    #[inline(always)]
    fn shr(self, rhs: usize) -> Self::Output {
        DWord::new_u32(self.u32() >> rhs)
    }
}

impl SaturateBits for DWord {
    #[inline(always)]
    fn saturate_bits(self, bits: usize) -> Self {
        DWord::new_u32(self.u32().saturate_bits(bits))
    }
}

impl SaturatingExt for DWord {
    #[inline(always)]
    fn saturating_add_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().saturating_add_i8(rhs.u32()))
    }

    #[inline(always)]
    fn saturating_sub_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().saturating_sub_i8(rhs.u32()))
    }
}

impl HalfExt for DWord {
    #[inline(always)]
    fn half_add_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().half_add_i8(rhs.u32()))
    }

    #[inline(always)]
    fn half_sub_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().half_sub_i8(rhs.u32()))
    }
}

impl CmpExt for DWord {
    #[inline(always)]
    fn max_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().max_i8(rhs.u32()))
    }

    #[inline(always)]
    fn min_i8(self, rhs: Self) -> Self {
        DWord::new_u32(self.u32().min_i8(rhs.u32()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        assert_eq!(4, core::mem::size_of::<DWord>());
    }

    #[test]
    fn repr() {
        let dword = DWord::new_i8h([50, 120, 120, -120]);
        let u32 = dword.u32();
        assert!(u32 == 0x32787888 || u32 == 0x88787832);
    }

    #[test]
    fn u8be() {
        let dword = DWord::new_u32(0xdeadbeef);
        let u8 = dword.u8be();
        assert_eq!(0xde, u8[0]);
        assert_eq!(0xad, u8[1]);
        assert_eq!(0xbe, u8[2]);
        assert_eq!(0xef, u8[3]);
    }

    #[test]
    fn u8le() {
        let dword = DWord::new_u32(0xdeadbeef);
        let u8 = dword.u8le();
        assert_eq!(0xef, u8[0]);
        assert_eq!(0xbe, u8[1]);
        assert_eq!(0xad, u8[2]);
        assert_eq!(0xde, u8[3]);
    }
}

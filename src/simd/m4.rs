//! Intrinsics for the M4 core.
//! See instruction summary table here, including the number of clock cycles per instruction:
//! https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-instructions
//! https://developer.arm.com/documentation/100166/0001/Programmers-Model/Instruction-set-summary/Table-of-processor-DSP-instructions
use super::*;

impl SaturateBits<i32> for i32 {
    #[inline(always)]
    fn saturate_bits<const BITS: usize>(self) -> i32 {
        debug_assert!(BITS >= 1 && BITS <= 32);
        unsafe {
            let r: i32;
            asm!(
                "ssat {r}, {sat}, {src}",
                r = out(reg) r,
                sat = const BITS,
                src = in(reg) self as u32,
                options(nomem, nostack)
            );
            r
        }
    }
}

impl SaturateBits<u32> for i32 {
    #[inline(always)]
    fn saturate_bits<const BITS: usize>(self) -> u32 {
        debug_assert!(BITS <= 31);
        unsafe {
            let r: u32;
            asm!(
                "usat {r}, {sat}, {src}",
                r = out(reg) r,
                sat = const BITS,
                src = in(reg) self,
                options(nomem, nostack)
            );
            r
        }
    }
}

macro_rules! impl_simd {
    ($type:ident) => {
        impl SaturatingExt for $type {
            #[inline(always)]
            fn saturating_add_i8(self: $type, rhs: $type) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "qadd8 {r}, {lhs}, {rhs}",
                        r = out(reg) r,
                        lhs = in(reg) self as usize,
                        rhs = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }

            #[inline(always)]
            fn saturating_sub_i8(self: $type, rhs: $type) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "qsub8 {r}, {lhs}, {rhs}",
                        r = out(reg) r,
                        lhs = in(reg) self as usize,
                        rhs = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }
        }

        impl HalfExt for $type {
            #[inline(always)]
            fn half_add_i8(self: $type, rhs: $type) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "shadd8 {r}, {lhs}, {rhs}",
                        r = out(reg) r,
                        lhs = in(reg) self as usize,
                        rhs = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }

            #[inline(always)]
            fn half_sub_i8(self: $type, rhs: $type) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "shsub8 {r}, {lhs}, {rhs}",
                        r = out(reg) r,
                        lhs = in(reg) self as usize,
                        rhs = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }
        }

        impl CmpExt for $type {
            #[inline(always)]
            fn max_i8(self: $type, rhs: Self) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "ssub8 {r}, {b}, {a}",  // bytewise b - a, set GE[3:0] per byte to 1 if the result is >= 0.
                        "sel {r}, {b}, {a}",    // select the bytes from b where GE is >= 0, from a otherwise.
                        r = out(reg) r,
                        a = in(reg) self as usize,
                        b = in(reg) rhs as usize,
                        options(nomem, nostack));
                    r as $type
                }
            }

            #[inline(always)]
            fn min_i8(self: $type, rhs: Self) -> Self {
                unsafe {
                    let r: usize;
                    asm!(
                        "ssub8 {r}, {a}, {b}",  // bytewise a - b, set GE[3:0] per byte to 1 if the result is >= 0.
                        "sel {r}, {b}, {a}",    // select the bytes from b where GE is >= 0, from a otherwise.
                        r = out(reg) r,
                        a = in(reg) self as usize,
                        b = in(reg) rhs as usize,
                        options(nomem, nostack));
                    r as $type
                }
            }
        }
    };
}

impl_simd!(i32);
impl_simd!(u32);

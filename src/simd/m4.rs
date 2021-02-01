use super::*;

impl SaturateBits for i32 {
    #[inline(always)]
    fn saturate_bits(self, bits: u32) -> Self {
        let r: i32;
        asm!(
            "ssat {Rd}, {sat}, {Rm}",
            Rd = out(reg) r,
            sat = in(reg) bits,
            Rm = in(reg) self,
            options(nomem, nostack)
        )
    }
}

impl SaturateBits for u32 {
    #[inline(always)]
    fn saturate_bits(self, bits: u32) -> Self {
        let r: i32;
        asm!(
            "usat {Rd}, {sat}, {Rm}",
            Rd = out(reg) r,
            sat = in(reg) bits,
            Rm = in(reg) self,
            options(nomem, nostack)
        )
    }
}

macro_rules! impl_simd {
    ($type:ident) => {
        impl SaturatingAddExt for $type {
            #[inline(always)]
            fn saturating_add_i8(self: $type, rhs: $type) -> $type {
                unsafe {
                    let r: usize;
                    asm!(
                        "qadd8 {r}, {Rn}, {Rm}",
                        r = out(reg) r,
                        Rn = in(reg) self as usize,
                        Rm = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }
        }

        impl SaturatingSubExt for $type {
            #[inline(always)]
            fn saturating_sub_i8(self: $type, rhs: $type) -> $type {
                unsafe {
                    let r: usize;
                    asm!(
                        "qsub8 {r}, {Rn}, {Rm}",
                        r = out(reg) r,
                        Rn = in(reg) self as usize,
                        Rm = in(reg) rhs as usize,
                        options(nomem, nostack, preserves_flags));
                    r as $type
                }
            }
        }
    };
}

impl_simd!(i32);
impl_simd!(u32);

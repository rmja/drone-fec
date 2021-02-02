//! UMTS BCJR Decoder
use crate::{BcjrDecoder, Llr, dword::DWord, simd::*};
use alloc::{collections::VecDeque, vec::Vec};

pub struct UmtsTrellis;

struct StateBytes {
    /// The values for states 7-4.
    s74: DWord,
    /// The values for states 3-0.
    s30: DWord,
}

impl BcjrDecoder for UmtsTrellis {
    fn decode<
        Lu: Iterator<Item = Llr>,
        Lv: Iterator<Item = Llr>,
        La: Iterator<Item = Llr>,
    >(&self, systematic: Lu, parity: Lv, apriori: La, terminated: bool) -> Vec<Llr> {
        let capacity = systematic.size_hint().1
            .or(parity.size_hint().1)
            .or(apriori.size_hint().1)
            .unwrap_or_else(|| systematic.size_hint().0);
        let mut g_vector = Vec::with_capacity(capacity);

        for ((lu, lv), la) in systematic.zip(parity).zip(apriori) {
            // Inner product of possible transmitted symbols and their received value.
            // G from state emitting u=0/v=0: 0*La + 0*LU - 0*LV
            // G from state emitting u=0/v=1: 0*La + 0*LU + 1*LV
            // G from state emitting u=1/v=0: 1*La + 1*LU - 0*LV
            // G from state emitting u=1/v=1: 1*La + 1*LU + 1*LV

            let g0p1 = lv.0 as i32;
            let g1p0 = la.0 as i32 + lu.0 as i32;
            let g1p1 = g0p1 + g1p0;

            let bytes = [
                0,
                (g0p1.saturate_bits(8) as u8),
                (g1p0.saturate_bits(8) as u8),
                (g1p1.saturate_bits(8) as u8),
            ];

            g_vector.push(DWord::new_u32(u32::from_le_bytes(bytes)));
        }

        assert!(g_vector.len() >= 6, "The input is not long enough to open and close the trellis.");

        let mut a_vector = Vec::with_capacity(g_vector.len());
        let mut l_app = VecDeque::with_capacity(g_vector.len());

        let (forward, tail) = g_vector.split_at(g_vector.len() - 3);
        let mut forward = forward.iter().map(|x| *x);

        // Only s0 is valid.
        let mut a74 = DWord::new_u32(0x80808080);
        let mut a30 = DWord::new_u32(0x80808000);

        // Only s4 and s0 are valid.
        let g = forward.next().unwrap();
        let a74us = compute_a74(a74, a30, g) & 0x000000FF;
        let a30us = compute_a30(a74, a30, g) & 0x000000FF;
        let coefficients = scale_coeff2((a74us << 8) | a30us);
        a74 = a74us.saturating_sub_i8(coefficients & 0x000000FF) | 0x80808000;
        a30 = a30us.saturating_sub_i8(coefficients & 0x000000FF) | 0x80808000;
        a_vector.push(StateBytes { s74: a74, s30: a30 });

        // Only s6, s4, s2 and s0 are valid.
        let g = forward.next().unwrap();
        let a74us = compute_a74(a74, a30, g) & 0x00FF00FF;
        let a30us = compute_a30(a74, a30, g) & 0x00FF00FF;
        let coefficients = scale_coeff4((a74us << 8) | a30us);
        a74 = a74us.saturating_sub_i8(coefficients & 0x00FF00FF) | 0x80008000;
        a30 = a30us.saturating_sub_i8(coefficients & 0x00FF00FF) | 0x80008000;
        a_vector.push(StateBytes { s74: a74, s30: a30 });

        for g in forward {
            // All states are valid.
            let a74us = compute_a74(a74, a30, g);
            let a30us = compute_a30(a74, a30, g);
            let coefficients = scale_coeff8(a74us, a30us);
            a74 = a74us.saturating_sub_i8(coefficients);
            a30 = a30us.saturating_sub_i8(coefficients);
            a_vector.push(StateBytes { s74: a74, s30: a30 });
        }

        // Only s3, s2, s1 and s0 are valid.
        let g = tail[0];
        let a30us = compute_a30( a74, a30, g);
        let coefficients = scale_coeff4(a30us);
        a74 = DWord::new_u32(0x80808080);
        a30 = a30us.saturating_sub_i8(coefficients);
        a_vector.push(StateBytes { s74: a74, s30: a30 });

        // Only s1 and s0 are valid.
        let g = tail[1];
        let a30us = compute_a30(a74, a30, g) & 0x0000FFFF;
        let coefficients = scale_coeff2(a30us);
        a74 = DWord::new_u32(0x80808080);
        a30 = a30us.saturating_sub_i8(coefficients & 0x0000FFFF) | 0x80800000;
        a_vector.push(StateBytes { s74: a74, s30: a30 });

        // We do not use the last value of g in the forward path.
        // Proceed with backward path.

        let forward = a_vector.iter().zip(&g_vector[1..]).map(|x| (x.0, *x.1));
        let mut head = forward.clone().take(2).rev();
        let mut backward = forward.skip(2).rev();

        let (mut b74, mut b30) = if terminated {
            // Only s0 is valid.
            let b74 = DWord::new_u32(0x80808080);
            let mut b30 = DWord::new_u32(0x80808000);

            {
                let (a, g) = backward.next().unwrap();

                // Emit llr.
                let max1 = compute_max1(a.s74, a.s30, g, b74, b30, 0x00000000, 0x0000FFFF);
                let max0 = compute_max0(a.s74, a.s30, g, b74, b30, 0x00000000, 0x0000FFFF);
                l_app.push_front(max1.saturating_sub(max0));

                // Only s1 and s0 are valid.
                let b30us = compute_b30(b74, b30, g) & 0x0000FFFF;
                let coefficients = scale_coeff2(b30us);
                // B74 remains -inf.
                b30 = b30us.saturating_sub_i8(coefficients & 0x0000FFFF) | 0x80800000;
            }

            {
                let (a, g) = backward.next().unwrap();

                // Emit llr.
                let max1 = compute_max1(a.s74, a.s30, g, b74, b30, 0x00000000, 0xFFFFFFFF);
                let max0 = compute_max0(a.s74, a.s30, g, b74, b30, 0x00000000, 0xFFFFFFFF);
                l_app.push_front(max1.saturating_sub(max0));

                // Only s3, s2, s1 and s0 are valid.
                let b30us = compute_b30(b74, b30, g);
                let coefficients = scale_coeff4(b30us);
                // B74 remains -inf.
                b30 = b30us.saturating_sub_i8(coefficients);
            }

            (b74, b30)
        } else {
            let b74 = DWord::new_u32(0x00000000);
            let b30 = DWord::new_u32(0x00000000);

            (b74, b30)
        };

        for (a, g) in backward {
            // Emit llr.
            let max1 = compute_max1(a.s74, a.s30, g, b74, b30, 0xFFFFFFFF, 0xFFFFFFFF);
            let max0 = compute_max0(a.s74, a.s30, g, b74, b30, 0xFFFFFFFF, 0xFFFFFFFF);
            l_app.push_front(max1.saturating_sub(max0));

            // All states are valid.
            let b74us = compute_b74(b74, b30, g);
            let b30us = compute_b30(b74, b30, g);
            let coefficients = scale_coeff8(b74us, b30us);
            b74 = b74us.saturating_sub_i8(coefficients);
            b30 = b30us.saturating_sub_i8(coefficients);
        }

        {
            let (a, g) = head.next().unwrap();

            // Emit llr.
            let max1 = compute_max1(a.s74, a.s30, g, b74, b30, 0x00FF00FF, 0x00FF00FF);
            let max0 = compute_max0(a.s74, a.s30, g, b74, b30, 0x00FF00FF, 0x00FF00FF);
            l_app.push_front(max1.saturating_sub(max0));

            // Only s6, s4, s2 and s0 are valid.
            let b74us = compute_b74(b74, b30, g) & 0x00FF00FF;
            let b30us = compute_b30(b74, b30, g) & 0x00FF00FF;
            let coefficients = scale_coeff4((b74us << 8) | b30us);
            b74 = b74us.saturating_sub_i8(coefficients & 0x00FF00FF) | 0x80008000;
            b30 = b30us.saturating_sub_i8(coefficients & 0x00FF00FF) | 0x80008000;
        }

        {
            let (a, g) = head.next().unwrap();

            // Emit llr.
            let max1 = compute_max1(a.s74, a.s30, g, b74, b30, 0x000000FF, 0x000000FF);
            let max0 = compute_max0(a.s74, a.s30, g, b74, b30, 0x000000FF, 0x000000FF);
            l_app.push_front(max1.saturating_sub(max0));

            /* Only s4 and s0 are valid. */
            let b74us = compute_b74(b74, b30, g) & 0x000000FF;
            let b30us = compute_b30(b74, b30, g) & 0x000000FF;
            let coefficients = scale_coeff2((b74us << 8) | b30us);
            b74 = b74us.saturating_sub_i8(coefficients & 0x000000FF) | 0x80808000;
            b30 = b30us.saturating_sub_i8(coefficients & 0x000000FF) | 0x80808000;
        }

        {
            let a74 = DWord::new_u32(0x80808080);
            let a30 = DWord::new_u32(0x80808000);
            let g = g_vector[0];

            // Emit llr.
            let max1 = compute_max1(a74, a30, g, b74, b30, 0x00000000, 0x000000FF);
            let max0 = compute_max0(a74, a30, g, b74, b30, 0x00000000, 0x000000FF);
            l_app.push_front(max1.saturating_sub(max0));
        }

        debug_assert!(head.next().is_none());

        l_app.make_contiguous().to_vec()
    }
}

#[inline]
fn compute_a74(a74_prev: DWord, a30_prev: DWord, g: DWord) -> DWord {
    // Case when u=0 is transmitted.
    let a74 =                           // pr     cr u/v
        ((a74_prev & 0x00FFFF00) <<  8) |   // s6 --> s7 0/0
                                            // s5 --> s6 0/1
        ((a30_prev & 0x00FFFF00) >>  8);    // s2 --> s5 0/1
                                            // s1 --> s4 0/0

    let g74 =                           // pr     cr u/v
        ((g & 0x000000FF ) << 24) |         // s6 <-> s7 0/0
        ((g & 0x0000FF00 ) <<  8) |         // s5 <-> s6 0/1
        ((g & 0x0000FFFF )      );          // s2 <-> s5 0/1
                                            // s1 <-> s4 0/0

    let zero74 = a74.saturating_add_i8(g74);

    // Case when u=1 is transmitted.
    let a74 =                           // pr     cr u/v
        ((a74_prev & 0xFF000000)      ) |   // s7 --> s7 1/1
        ((a74_prev & 0x000000FF) << 16) |   // s4 --> s6 1/0
        ((a30_prev & 0xFF000000) >> 16) |   // s3 --> s5 1/0
        ((a30_prev & 0x000000FF)      );    // s0 --> s4 1/1

    let g74 =                           // pr     cr u/v
        ((g & 0xFFFF0000)      ) |          // s7 <-> s7 1/1
                                            // s4 <-> s6 1/0
        ((g & 0x00FF0000) >>  8) |          // s3 <-> s5 1/0
        ((g & 0xFF000000) >> 24);           // s0 <-> s4 1/1

    let one74 = a74.saturating_add_i8(g74);

    zero74.max_i8(one74)
}

#[inline]
fn compute_a30(a74_prev: DWord, a30_prev: DWord, g: DWord) -> DWord {
    // Case when u=0 is transmitted.
    let a30 =                           // pr     cr u/v
        ((a74_prev & 0xFF000000)      ) |   // s7 --> s3 0/0
        ((a74_prev & 0x000000FF) << 16) |   // s4 --> s2 0/1
        ((a30_prev & 0xFF000000) >> 16) |   // s3 --> s1 0/1
        ((a30_prev & 0x000000FF)      );    // s0 --> s0 0/0

    let g30 =                           // pr     cr u/v
        ((g & 0x000000FF) << 24) |          // s7 <-> s3 0/0
        ((g & 0x0000FF00) <<  8) |          // s4 <-> s2 0/1
        ((g & 0x0000FFFF)      );           // s3 <-> s1 0/1
                                            // s0 <-> s0 0/0

    let zero30 = a30.saturating_add_i8(g30);

    // Case when u=1 is transmitted.
    let a30 =                           // pr     cr u/v
        ((a74_prev & 0x00FFFF00) << 8) |    // s6 --> s3 1/1
                                            // s5 --> s2 1/0
        ((a30_prev & 0x00FFFF00) >> 8);     // s2 --> s1 1/0
                                            // s1 --> s0 1/1

    let g30 =                           // pr     cr u/v
        ((g & 0xFFFF0000)      ) |          // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
        ((g & 0x00FF0000) >>  8) |          // s2 <-> s1 1/0
        ((g & 0xFF000000) >> 24);           // s1 <-> s0 1/1

    let one30 = a30.saturating_add_i8(g30);

    zero30.max_i8(one30)
}

#[inline]
fn compute_b74(b74_next: DWord, b30_next: DWord, g: DWord) -> DWord {
    // Case when u=0 is transmitted.
    let b74 =                           // cr     nx u/v
        ((b30_next & 0xFF000000)      ) |   // s7 <-- s3 0/0
        ((b74_next & 0xFFFF0000) >>  8) |   // s6 <-- s7 0/0
                                            // s5 <-- s6 0/1
        ((b30_next & 0x00FF0000) >> 16);    // s4 <-- s2 0/1

    let g74 =                           // cr     nx u/v
        ((g & 0x000000FF) << 24) |          // s7 <-> s3 0/0
        ((g & 0x000000FF) << 16) |          // s6 <-> s7 0/0
        ((g & 0x0000FF00)      ) |          // s5 <-> s6 0/1
        ((g & 0x0000FF00) >>  8);           // s4 <-> s2 0/1

    let zero74 = b74.saturating_add_i8(g74);

    // Case when u=1 is transmitted.
    let b74 =                           // cr     nx u/v
        ((b74_next & 0xFF000000)      ) |   // s7 <-- s7 1/1
        ((b30_next & 0xFFFF0000) >>  8) |   // s6 <-- s3 1/1
                                            // s5 <-- s2 1/0
        ((b74_next & 0x00FF0000) >> 16);    // s4 <-- s6 1/0

    let g74 =                           // cr     nx u/v
        ((g & 0xFF000000)      ) |          // s7 <-> s7 1/1
        ((g & 0xFFFF0000) >>  8) |          // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
        ((g & 0x00FF0000) >> 16);           // s4 <-> s6 1/0

    let one74 = b74.saturating_add_i8(g74);

    zero74.max_i8(one74)
}

#[inline]
fn compute_b30(b74_next: DWord, b30_next: DWord, g: DWord) -> DWord {
    // Case when u=0 is transmitted.
    let b30 =                           // cr     nx u/v
        ((b30_next & 0x0000FF00) << 16) |   // s3 <-- s1 0/1
        ((b74_next & 0x0000FFFF) <<  8) |   // s2 <-- s5 0/1
                                            // s1 <-- s4 0/0
        ((b30_next & 0x000000FF)      );    // s0 <-- s0 0/0

    let g30 =                           // cr     nx u/v
        ((g & 0x0000FF00) << 16) |          // s3 <-> s1 0/1
        ((g & 0x0000FFFF) <<  8) |          // s2 <-> s5 0/1
                                            // s1 <-> s4 0/0
        ((g & 0x000000FF)      );           // s0 <-> s0 0/0

    let zero30 = b30.saturating_add_i8(g30);

    // Case when u=1 is transmitted.
    let b30 =                           // cr     nx u/v
        ((b74_next & 0x0000FF00) << 16) |   // s3 <-- s5 1/0
        ((b30_next & 0x0000FFFF) <<  8) |   // s2 <-- s1 1/0
                                            // s1 <-- s0 1/1
        ((b74_next & 0x000000FF)      );    // s0 <-- s4 1/1

    let g30 =                           // cr     nx u/v
        ((g & 0x00FF0000 ) <<  8) |         // s3 <-> s5 1/0
        ((g & 0x00FF0000 )      ) |         // s2 <-> s1 1/0
        ((g & 0xFF000000 ) >> 16) |         // s1 <-> s0 1/1
        ((g & 0xFF000000 ) >> 24);          // s0 <-> s4 1/1

    let one30 = b30.saturating_add_i8(g30);

    zero30.max_i8(one30)
}

#[inline]
fn compute_max0(a74: DWord, a30: DWord, g: DWord, b74: DWord, b30: DWord, a74_valid: u32, a30_valid: u32) -> Llr {
    // States 7-4.
    let g74 =
        ((g & 0x000000FF) << 24) |          // s7 <-> s3 0/0
        ((g & 0x000000FF) << 16) |          // s6 <-> s7 0/0
        ((g & 0x0000FF00)      ) |          // s5 <-> s6 0/1
        ((g & 0x0000FF00) >>  8);           // s4 <-> s2 0/1

    // Align B for u=0 according to A.
    let b_for74 =
        ((b30 & 0xFF000000)      ) |        // s7 <-- s3 0/0
        ((b74 & 0xFFFF0000) >>  8) |        // s6 <-- s7 0/0
                                            // s5 <-- s6 0/1
        ((b30 & 0x00FF0000) >> 16);         // s4 <-- s2 0/1

    let sum74 = (a74.saturating_add_i8(g74.saturating_add_i8(b_for74)) & a74_valid) | (0x80808080 & !a74_valid);

    // States 3-0.
    let g30 =
        ((g & 0x0000FF00) << 16) |          // s3 <-> s1 0/1
        ((g & 0x0000FFFF) <<  8) |          // s2 <-> s5 0/1
                                            // s1 <-> s4 0/0
        ((g & 0x000000FF)      );           // s0 <-> s0 0/0

    // Align B for u=0 according to A.
    let b_for30 =
        ((b30 & 0x0000FF00) << 16) |        // s3 <-- s1 0/1
        ((b74 & 0x0000FFFF) <<  8) |        // s2 <-- s5 0/1
                                            // s1 <-- s4 0/0
        ((b30 & 0x000000FF)      );         // s0 <-- s0 0/0

    let sum30 = (a30.saturating_add_i8(g30.saturating_add_i8(b_for30)) & a30_valid) | (0x80808080 & !a30_valid);

    let mut max = sum74.max_i8(sum30);
    max = max.max_i8(max >> 16);
    max = max.max_i8(max >> 8);
    Llr((max.u32() & 0xFF) as i8)
}

#[inline]
fn compute_max1(a74: DWord, a30: DWord, g: DWord, b74: DWord, b30: DWord, a74_valid: u32, a30_valid: u32) -> Llr {
    // States 7-4.
    let g74 =
        ((g & 0xFF000000)      ) |          // s7 <-> s7 1/1
        ((g & 0xFFFF0000) >>  8) |          // s6 <-> s3 1/1
                                            // s5 <-> s2 1/0
        ((g & 0x00FF0000) >> 16);           // s4 <-> s6 1/0

    // Align B for u=1 according to A.
    let b_for74 =
        ((b74 & 0xFF000000)      ) |        // s7 <-- s7 1/1
        ((b30 & 0xFFFF0000) >>  8) |        // s6 <-- s3 1/1
                                            // s5 <-- s2 1/0
        ((b74 & 0x00FF0000) >> 16);         // s4 <-- s6 1/0

    let sum74 = (a74.saturating_add_i8(g74.saturating_add_i8(b_for74)) & a74_valid) | (0x80808080 & !a74_valid);

    // States 3-0.
    let g30 =
        ((g & 0x00FF0000) <<  8) |          // s3 <-> s5 1/0
        ((g & 0x00FF0000)      ) |          // s2 <-> s1 1/0
        ((g & 0xFF000000) >> 16) |          // s1 <-> s0 1/1
        ((g & 0xFF000000) >> 24);           // s0 <-> s4 1/1

    // Align B for u=1 according to A.
    let b_for30 =
        ((b74 & 0x0000FF00) << 16) |        // s3 <-- s5 1/0
        ((b30 & 0x0000FFFF) <<  8) |        // s2 <-- s1 1/0
                                            // s1 <-- s0 1/1
        ((b74 & 0x000000FF));               // s0 <-- s4 1/1

    let sum30 = (a30.saturating_add_i8(g30.saturating_add_i8(b_for30)) & a30_valid) | ( 0x80808080 & ( !a30_valid));

    let mut max = sum74.max_i8(sum30);
    max = max.max_i8(max >> 16);
    max = max.max_i8(max >> 8);
    Llr((max.u32() & 0xFF) as i8)
}

/// Get the scale coefficient so that the values accross two states sum to 0, as log(1) = 0
#[inline]
fn scale_coeff2(values10: DWord) -> DWord
{
    let u32 = ( values10 << 16 ) | values10;
    u32.rotate_right(8).half_add_i8(u32)
}

/// Get the scale coefficient so that the values accross two states sum to 0, as log(1) = 0
#[inline]
fn scale_coeff4(values30: DWord) -> DWord
{
    //                            v0+v3                    v3+v2                    v2+v1                    v1+v0
    // halves                     -----                    -----                    -----                    -----
    //                              2                        2                        2                        2
    //
    //                      v2+v1+v0+v3              v1+v0+v3+v2              v0+v3+v2+v1              v3+v2+v1+v0
    // quarters             -----------              -----------              -----------              -----------
    //                           4                        4                        4                        4

    #[cfg(test)]
    {
        let v30 = values30.i8h();
        let sum = v30[0] as i16 + v30[1] as i16 + v30[2] as i16 + v30[3] as i16;
        DWord::new_i8h([
            (sum / 4) as i8,
            (sum / 4) as i8,
            (sum / 4) as i8,
            (sum / 4) as i8,
        ])
    }
    #[cfg(not(test))]
    {
        let sum = values30.rotate_right(8).half_add_i8(values30);
        sum.rotate_right(16).half_add_i8(sum)
    }
}

/// Get the scale coefficient so that the values accross two states sum to 0, as log(1) = 0
#[inline]
fn scale_coeff8(values74: DWord, values30: DWord) -> DWord {
    //                            v7+v3                    v6+v2                    v5+v1                    v4+v0
    // halves                     -----                    -----                    -----                    -----
    //                              2                        2                        2                        2
    //
    //                      v4+v0+v7+v3              v7+v3+v6+v2              v6+v2+v5+v1              v5+v1+v4+v0
    // quarters             -----------              -----------              -----------              -----------
    //                           4                        4                        4                        4
    //
    //          v6+v2+v5+v1+v4+v0+v7+v3  v5+v1+v4+v0+v7+v3+v6+v2  v4+v0+v7+v3+v6+v2+v5+v1  v7+v3+v6+v2+v5+v1+v4+v0
    // eights   -----------------------  -----------------------  -----------------------  -----------------------
    //                     8                        8                        8                        8

    #[cfg(test)]
    {
        let v30 = values30.i8h();
        let v74 = values74.i8h();
        let sum =
            v30[0] as i16 + v30[1] as i16 + v30[2] as i16 + v30[3] as i16 +
            v74[0] as i16 + v74[1] as i16 + v74[2] as i16 + v74[3] as i16;

        DWord::new_i8h([
            (sum / 8) as i8,
            (sum / 8) as i8,
            (sum / 8) as i8,
            (sum / 8) as i8,
        ])
    }
    #[cfg(not(test))]
    {
        let sum = values74.half_add_i8(values30);
        let sum = sum.rotate_right(8).half_add_i8(sum);
        sum.rotate_right(16).half_add_i8(sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{bcjr::*, llr_vec};

    static UMTS: UmtsTrellis = UmtsTrellis;

    #[test]
    fn decode_byte() {
        let systematic = llr_vec![
            4, 4, -4, 4, 4, -4, -4, 4,
            -4, -4, -4];
        let parity = llr_vec![
            4, -4, -4, 4, 4, -4, 4, 4,
            -4, -4, -4];
        let apriori = vec![Llr::ZERO; 8 + 3];

        assert_eq!(llr_vec![
            24, 24, -24, 24, 24, -24, -24, 24, -24, -24, -24],
            UMTS.decode(systematic.into_iter(), parity.into_iter(), apriori.into_iter(), true));
    }

    #[test]
    fn decode_excel_example_decoder1() {
        let systematic = llr_vec![
            -4, -4, -4, 4, -4, -4, 4, 4,
            -4, -4, -4, -4, -4, -4, 4, -4,
            4, -4, 4,
        ];
        let parity = llr_vec![
            -4, -4, -4, 4, 4, 4, -4, -4,
            -4, 4, 4, 4, -4, -4, -4, 4,
            4, 4, 4,
        ];
        let apriori = vec![Llr::ZERO; 16 + 3];

        assert_eq!(llr_vec![
            -24, -24, -24, 24, -24, -24, 24, 24,
            -24, -24, -24, -24, -24, -24, 24, -24,
            24, -24, 24],
            UMTS.decode(systematic.into_iter(), parity.into_iter(), apriori.into_iter(), true));
    }
}
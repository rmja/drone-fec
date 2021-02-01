use crate::{BcjrDecoder, Llr};
use alloc::vec::Vec;
use streaming_iterator::StreamingIterator;

/// A Turbo decoder.
pub struct TurboDecoder<B: BcjrDecoder> {
    /// The BCJR algoritm decoder.
    bcjr: B,
    /// The a-posteriori log-likelihood ratios (produced by the first decoder).
    l_app_deinterleaved: Vec<Llr>,
    la_second: Vec<Llr>,
}

pub struct TurboDecodeIterator<'a, B: BcjrDecoder, I: IntoIterator<Item = usize> + Clone> {
    decoder: &'a mut TurboDecoder<B>,
    systematic: &'a [Llr],
    first_decoder_systematic_termination: Option<&'a [Llr]>,
    first_decoder_parity: &'a [Llr],
    second_decoder_systematic_termination: Option<&'a [Llr]>,
    second_decoder_parity: &'a [Llr],
    interleaver: I,
}

impl<B: BcjrDecoder> TurboDecoder<B> {
    pub fn new(bcjr: B) -> Self {
        Self {
            bcjr,
            l_app_deinterleaved: vec![],
            la_second: vec![],
        }
    }

    pub fn decode<'a, I: IntoIterator<Item = usize> + Clone>(
        &'a mut self,
        systematic: &'a [Llr],
        first_decoder_systematic_termination: Option<&'a [Llr]>,
        first_decoder_parity: &'a [Llr],
        second_decoder_systematic_termination: Option<&'a [Llr]>,
        second_decoder_parity: &'a [Llr],
        interleaver: I,
    ) -> TurboDecodeIterator<'a, B, I> {
        assert_eq!(systematic.len() + first_decoder_systematic_termination.map_or(0, |x| x.len()), first_decoder_parity.len());
        assert_eq!(systematic.len() + second_decoder_systematic_termination.map_or(0, |x| x.len()), second_decoder_parity.len());
        TurboDecodeIterator {
            decoder: self,
            systematic,
            first_decoder_systematic_termination,
            first_decoder_parity,
            second_decoder_systematic_termination,
            second_decoder_parity,
            interleaver,
        }
    }
}

impl<'a, B: BcjrDecoder, I: IntoIterator<Item = usize> + Clone> StreamingIterator for TurboDecodeIterator<'a, B, I> {
    type Item = Vec<Llr>;

    fn advance(&mut self) {
        let first_term_len = self.first_decoder_systematic_termination.map_or(0, |x| x.len());
        let second_term_len = self.second_decoder_systematic_termination.map_or(0, |x| x.len());

        // Prepare the input symbols for the first decoder. It consists of:
        // * The systematic llr's.
        // * The parity llr's.
        // * The a-priori llr's - these are equiprobable in the first iteration, and the extrinsic information from the
        //   second decoder in the following iterations.

        // Find the a-priori llr's La for the first decoder.
        let l_app_deinterleaved = &self.decoder.l_app_deinterleaved;
        let la_first = if l_app_deinterleaved.is_empty() {
            // This is the first iteration - all llr's are equiprobable.

            vec![Llr::ZERO; self.systematic.len() + first_term_len]
        }
        else {
            let mut la_first = Vec::with_capacity(self.systematic.len() + first_term_len);
            let la_second = &self.decoder.la_second;

            la_first.extend(self.interleaver.clone().into_iter().enumerate().map(|(index, int_index)| {
                // Compute the extrinsic information from the a-posteriori LLR (Lapp) from second decoder,
                // to be used now as the a-priori LLR for the first decoder.
                // This is eqn. 28. in the turbo.pdf reference.

                let l_app = l_app_deinterleaved[int_index];
                let l_a = la_second[index];
                let l_u = self.systematic[int_index];
                let l_e = l_app.saturating_sub(l_a).saturating_sub(l_u);

                l_e
            }));

            // The extrinsic information is not valid for the termination.
            for _ in 0..first_term_len {
                la_first.push(Llr::ZERO);
            }

            la_first
        };

        debug_assert_eq!(la_first.len(), self.systematic.len() + first_term_len);

        let systematic_termination = self.first_decoder_systematic_termination.iter().map(|x| *x).flatten();

        // Run the BCJR algorithm and compute the a-posteriori llr's Lapp for the first decoder.
        let first_l_app = self.decoder.bcjr.decode(
            self.systematic.iter()
                .chain(systematic_termination).copied(),
            self.first_decoder_parity.iter().copied(),
            la_first.iter().copied(),
            self.first_decoder_systematic_termination.is_some());

        let mut la_second = Vec::with_capacity(self.systematic.len() + second_term_len);
        la_second.extend(self.interleaver.clone().into_iter().enumerate().map(|(de_index, int_index)| {
            // Compute the extrinsic information from the a-posteriori LLR (Lapp) from the first decoder,
            // to be used as the a priori LLR for the second decoder.
            // This is eqn. 28 in the turbo.pdf reference.
            let Lapp = first_l_app[int_index];
            let La = la_first[int_index];
            let lu = self.systematic[int_index];
            let Le = Lapp.saturating_sub(La).saturating_sub(lu);

            Le
        }));

        // The extrinsic information is not valid for the termination.
        for _ in 0..second_term_len {
            la_second.push(Llr::ZERO);
        }

        // Compute Lapp.
        let systematic_termination = self.second_decoder_systematic_termination.iter().map(|x| *x).flatten();

        // Compute the a-posteriori llr's Lapp for the second decoder.
        let second_l_app = self.decoder.bcjr.decode(
            self.interleaver.clone().into_iter().map(|int_index| self.systematic[int_index])
                .chain(systematic_termination.copied()),
            self.second_decoder_parity.iter().copied(),
            la_second.iter().copied(),
            self.second_decoder_systematic_termination.is_some());

        // De-interleave Lapp for decision making.
        let mut l_app = vec![Llr::ZERO; second_l_app.len()];
        for (de_index, int_index) in self.interleaver.clone().into_iter().enumerate() {
            l_app[de_index] = second_l_app[int_index];
        }

        self.decoder.la_second = la_second;
        self.decoder.l_app_deinterleaved = l_app;
    }

    fn get(&self) -> Option<&Self::Item> {
        Some(&self.decoder.l_app_deinterleaved)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{interleavers::qpp::Qpp, llr_vec, umts::UmtsBcjrDecoder};

    use super::*;

    #[test]
    fn decode_excel_example() {
        let systematic = llr_vec![
            -4, -4, -4,  4, -4, -4,  4,  4,
            -4, -4, -4, -4, -4, -4,  4, -4,
        ];
        let first_decoder_systematic_termination = llr_vec![
             4, -4,  4,
        ];
        let first_decoder_parity = llr_vec![
            -4, -4, -4,  4,  4,  4, -4, -4,
            -4,  4,  4,  4, -4, -4, -4,  4,
             4,  4,  4,
        ];
        let second_decoder_systematic_termination = llr_vec![
            -4, -4, -4,
        ];
        let second_decoder_parity = llr_vec![
            -4, -4, -4,  4,  4,  4, -4,  4,
             4, -4, -4,  4, -4,  4, -4,  4,
            -4, -4, -4,
        ];

        let mut turbo = TurboDecoder::new(UmtsBcjrDecoder);
        let interleaver = Qpp::new(16, 1, 4);

        let mut iterator = turbo.decode(
            &systematic,
            Some(&first_decoder_systematic_termination),
            &first_decoder_parity,
            Some(&second_decoder_systematic_termination),
            &second_decoder_parity,
            interleaver);

        while let Some(l_app) = iterator.next() {
            break;
        }
    }
}
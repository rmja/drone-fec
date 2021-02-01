use crate::{Llr, symbol::Symbol};
use alloc::vec::Vec;

pub struct TurboDecoder {
    first_decoder_is_terminated: bool,
    second_decoder_is_terminated: bool,
}

pub struct TurboIterator<'a, I: IntoIterator<Item = usize> + Clone> {
    decoder: &'a TurboDecoder,
    source: &'a [Symbol],
    interleaver: I,
    l_app_deinterleaved: Vec<Llr>,
    l_app_interleaved: Vec<Llr>,
    la_second: Vec<Llr>,
}

impl TurboDecoder {
    pub fn new(first_decoder_is_terminated: bool, second_decoder_is_terminated: bool) -> Self {
        Self {
            first_decoder_is_terminated,
            second_decoder_is_terminated,
        }
    }

    pub fn decode<'a, I: IntoIterator<Item = usize> + Clone>(&'a self, source: &'a [Symbol], interleaver: I) -> TurboIterator<'a, I> {
        TurboIterator {
            decoder: self,
            source,
            interleaver,
            l_app_deinterleaved: vec![],
            l_app_interleaved: vec![],
            la_second: vec![],
        }
    }
}


impl<I: IntoIterator<Item = usize> + Clone> Iterator for TurboIterator<'_, I> {
    type Item = Vec<Llr>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<I: IntoIterator<Item = usize> + Clone> TurboIterator<'_, I> {
    fn run_first_decoder(&mut self) {
        for (de_index, int_index) in self.interleaver.clone().into_iter().enumerate() {
            let lu = self.source[int_index].lu;

            // Compute the extrinsic information from the a-posteriori LLR (Lapp) from second decoder,
            // to be used now as the a priori LLR for the first decoder.
            // This is eqn. 28. in the turbo.pdf reference.

            let Lapp = self.l_app_deinterleaved[int_index];
            let La = self.la_second[de_index];
            let Le = Lapp.saturating_sub(La).saturating_sub(lu);
            
            self.l_app_interleaved[int_index] = Le;
        }

        // uint16_t interleavedIndex;

        // for ( interleavedIndex = 0; interleavedIndex < self->__blockLength; interleavedIndex++ )
        // {
        //     uint16_t deinterleavedIndex = interleaver_deinterleave( self->__interleaver, interleavedIndex );
        //     llr_t LU = self->__getBlockInput( self, interleavedIndex );

        //     /* Compute the extrinsic information from the a-posteriori LLR (Lapp) from second decoder,
        //      * to be used now as the a priori LLR for the first decoder.
        //      * Eqn. 28 in http://paginas.fe.up.pt/~sam/textos/From%20BCJR%20to%20turbo.pdf */
        //     int32_t Le = self->__LappDeinterleavedBuffer[interleavedIndex];                /*  Lapp */
        //     Le -= self->__LaSecondBuffer[deinterleavedIndex];                     /* -  La (Le2) */
        //     Le -= LU;                                                           /* -   U (intrinsic) */
        //     self->__LappInterleavedLaFirstBuffer[interleavedIndex] = saturate_llr( Le );   /* =  Le (extrinsic) */
        // }
    }

    fn run_second_decoder() {

    }
}
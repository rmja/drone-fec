use crate::Llr;
use alloc::vec::Vec;

/// MAP decoder using the BCJR algorithm.
pub trait BcjrDecoder {
    /// Decode a block
    /// * `L_u` is the `systematic` part,
    /// * `L_v` is the `parity` part, and
    /// * `L_a` is the `a-priori` part.
    /// All parts must have the same number of elements.
    fn decode<
        Lu: Iterator<Item = Llr>,
        Lv: Iterator<Item = Llr>,
        La: Iterator<Item = Llr>,
    >(&self, systematic: Lu, parity: Lv, apriori: La, terminated: bool) -> Vec<Llr>;
}

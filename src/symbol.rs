use crate::Llr;

pub struct Symbol {
    /// The systematic part.
    pub lu: Llr,
    /// The parity part.
    pub lv: Llr,
    /// The apriori part.
    pub la: Llr,
}

pub trait NewSymbol<T> {
    fn new(lu: T, lv: T, la: T) -> Self;
}

impl NewSymbol<Llr> for Symbol {
    fn new(lu: Llr, lv: Llr, la: Llr) -> Self {
        Self { lu, lv, la }
    }
}

impl NewSymbol<i8> for Symbol {
    fn new(lu: i8, lv: i8, la: i8) -> Self {
        Self { lu: lu.into(), lv: lv.into(), la: la.into() }
    }
}

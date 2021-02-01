/// Log-Likelihood Ratio.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Llr(pub i8);

impl Llr {
    pub const ZERO: Llr = Llr(0);

    pub fn saturating_sub(self: Llr, rhs: Llr) -> Llr {
        Llr(self.0.saturating_sub(rhs.0))
    }

    pub fn hard(self) -> bool {
        self.0 > 0
    }
}

impl Into<Llr> for i8 {
    fn into(self) -> Llr {
        Llr(self)
    }
}
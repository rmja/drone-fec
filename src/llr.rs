/// Log-Likelihood Ratio.
/// A value >0 means a likely 1, and <0 a likely 0.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Llr(pub i8);

#[macro_export]
macro_rules! llr_vec {
    ($($llr:expr),+) => {
        vec![$(crate::Llr($llr)),+]
    };
    ($($llr:expr),+,) => {
        llr_vec!($($llr),+)
    };
}

impl Llr {
    /// The equiprobable value.
    pub const ZERO: Llr = Llr(0);

    pub fn saturating_sub(self: Llr, rhs: Llr) -> Llr {
        Llr(self.0.saturating_sub(rhs.0))
    }

    /// Make a hard decode decision.
    pub fn hard(self) -> bool {
        self.0 > 0
    }
}

impl Into<Llr> for i8 {
    fn into(self) -> Llr {
        Llr(self)
    }
}
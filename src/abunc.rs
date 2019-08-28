use crate::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbUnc {
    val: f64,
    unc: f64,
}

impl AbUnc {
    /// Creates a new absolute uncertainty. See [ab().](unc/fn.ab.html)
    pub fn new(val: f64, unc: f64) -> AbUnc {
        AbUnc { val, unc }
    }
}

impl Uncertainty for AbUnc {
    fn to_ab(self) -> AbUnc {
        self
    }

    fn to_rel(self) -> RelUnc {
        RelUnc::new(self.val, self.unc / self.val)
    }

    fn val(&self) -> f64 {
        self.val
    }

    fn unc(&self) -> f64 {
        self.unc
    }
}

impl fmt::Display for AbUnc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.val, self.unc)
    }
}

#[cfg(test)]
mod tests {
    use crate::AbUnc;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<AbUnc>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<AbUnc>();
    }
}

use crate::*;
use std::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Hash)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AbUnc {
    pub(crate) val: f64,
    pub(crate) unc: f64,
}

impl Uncertainty for AbUnc {
    fn to_ab(self) -> AbUnc {
        self
    }

    fn to_rel(self) -> RelUnc {
        RelUnc {
            val: self.val,
            unc: self.unc / self.val,
        }
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
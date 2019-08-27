use crate::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
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

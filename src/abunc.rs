use crate::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct AbUncVal {
    pub(crate) val: f64,
    pub(crate) unc: f64,
}

impl UncertainValue for AbUncVal {
    fn as_ab(self) -> AbUncVal {
        self
    }

    fn as_rel(self) -> RelUncVal {
        RelUncVal {
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

impl fmt::Display for AbUncVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.val, self.unc)
    }
}

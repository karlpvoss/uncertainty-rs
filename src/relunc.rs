use crate::*;
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
pub struct RelUnc {
    pub(crate) val: f64,
    pub(crate) unc: f64,
}

impl UncertainValue for RelUnc {
    fn as_ab(self) -> AbUnc {
        Unc::ab(self.val, self.unc * self.val)
    }

    fn as_rel(self) -> RelUnc {
        self
    }

    fn val(&self) -> f64 {
        self.val
    }

    fn unc(&self) -> f64 {
        self.unc
    }
}

impl fmt::Display for RelUnc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}%", self.val, self.unc * 100.0)
    }
}

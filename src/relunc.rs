use crate::*;
use std::fmt;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelUnc {
    pub(crate) val: f64,
    pub(crate) unc: f64,
}

impl RelUnc {
    /// Raise a relative uncertainty to an integer power.
    /// Casts the i32 to an f64 in order to calculate the change in uncertainty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uncertainty_rs::*;
    /// # use approx::assert_abs_diff_eq;
    /// let two = Unc::rel(2.0, 0.01);
    /// let eight = two.powi(3);
    /// assert_abs_diff_eq!(eight.val(), 8.0);
    /// assert_abs_diff_eq!(eight.unc(), 0.03);
    /// ```
    pub fn powi(self, power: i32) -> RelUnc {
        RelUnc {
            val: self.val.powi(power),
            unc: self.unc * f64::from(power),
        }
    }

    /// Raise a relative uncertainty to an floating power.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uncertainty_rs::*;
    /// # use approx::assert_abs_diff_eq;
    /// let two = Unc::rel(2.0, 0.01);
    /// let eight = two.powf(3.0);
    /// assert_abs_diff_eq!(eight.val(), 8.0);
    /// assert_abs_diff_eq!(eight.unc(), 0.03);
    /// ```
    pub fn powf(self, power: f64) -> RelUnc {
        RelUnc {
            val: self.val.powf(power),
            unc: self.unc * power,
        }
    }
}

impl Uncertainty for RelUnc {
    fn to_ab(self) -> AbUnc {
        Unc::ab(self.val, self.unc * self.val)
    }

    fn to_rel(self) -> RelUnc {
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


#[cfg(test)]
mod tests {
    use crate::RelUnc;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<RelUnc>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<RelUnc>();
    }
}
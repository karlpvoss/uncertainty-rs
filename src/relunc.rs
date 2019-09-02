use crate::*;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Copy, Clone)]
#[cfg_attr(test, derive(PartialEq))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RelUnc {
    val: f64,
    unc: f64,
}

impl RelUnc {
    /// Create a relative uncertainty. The first parameter is the base value, and the second
    /// parameter is the relative uncertainty in the value, expressed as a decimal fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use uncertainty::*;
    /// let u: RelUnc = RelUnc::new(10.0, 0.1);
    /// assert_eq!(u.val(), 10.0);
    /// assert_eq!(u.unc(), 0.1);
    /// ```
    pub fn new(val: f64, unc: f64) -> RelUnc {
        RelUnc { val, unc }
    }

    /// Raise a relative uncertainty to an integer power.
    ///
    /// # Potential problems
    ///
    /// Casts the i32 to an f64 in order to calculate the change in uncertainty.
    ///
    /// # Examples
    ///
    /// ```
    /// # use uncertainty::*;
    /// # use approx::assert_abs_diff_eq;
    /// let two = RelUnc::new(2.0, 0.01);
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
    /// # use uncertainty::*;
    /// # use approx::assert_abs_diff_eq;
    /// let two = RelUnc::new(2.0, 0.01);
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
        AbUnc::new(self.val, self.unc * self.val)
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

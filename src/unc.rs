use crate::*;
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

/// A fieldless struct only used to instantiate AbUnc and RelUnc values.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Unc;

impl Unc {
    /// Create an absolute uncertainty. The first parameter is the base value, and the second
    /// parameter is the absolute uncertainty in that value.
    ///
    /// # Examples
    ///
    /// ```
    /// use uncertainty_rs::*;
    ///
    /// let u: AbUnc = Unc::ab(10.0, 1.0);
    /// assert_eq!(u.val(), 10.0);
    /// assert_eq!(u.unc(), 1.0);
    /// ```
    pub fn ab(val: f64, unc: f64) -> AbUnc {
        AbUnc { val, unc }
    }

    /// Create a relative uncertainty. The first parameter is the base value, and the second
    /// parameter is the relative uncertainty in the value, expressed as a decimal fraction.
    ///
    /// # Examples
    ///
    /// ```
    /// use uncertainty_rs::*;
    ///
    /// let u: RelUnc = Unc::rel(10.0, 0.1);
    /// assert_eq!(u.val(), 10.0);
    /// assert_eq!(u.unc(), 0.1);
    /// ```
    pub fn rel(val: f64, unc: f64) -> RelUnc {
        RelUnc { val, unc }
    }
}

/// Used to define behaviour for values which have uncertain values.
pub trait Uncertainty: Sized + Copy {
    /// Convert any uncertainty value into one which has an absolute uncertain value.
    fn to_ab(self) -> AbUnc;
    /// Convert any uncertainty value into one which has a relative uncertain value.
    fn to_rel(self) -> RelUnc;
    /// Get the base value of the uncertainty.
    fn val(&self) -> f64;
    /// Get the uncertainty value; this will depend on whether the base type is a relative or
    /// absolute uncertainty.
    fn unc(&self) -> f64;

    /// Return the minimum possible value of the uncertainty. This is functionally the base value -
    /// the uncertainty value.
    fn min(&self) -> f64 {
        let x = self.to_ab();
        x.val() - x.unc()
    }

    /// Return the maximum possible value of the uncertainty. This is functionally the base value +
    /// the uncertainty value.
    fn max(&self) -> f64 {
        let x = self.to_ab();
        x.val() + x.unc()
    }

    /// Given two uncertainty values, determine whether they overlap.
    /// This is done by comparing the minimums and maximums to see whether they intersect.
    /// Intersecting uncertainties are said to be equal or overlap.
    fn overlap<T: Uncertainty>(&self, other: &T) -> bool {
        if self.val() > other.val() {
            self.min() <= other.max()
        } else {
            self.max() >= other.min()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Unc;

    #[test]
    fn test_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Unc>();
    }

    #[test]
    fn test_sync() {
        fn assert_sync<T: Sync>() {}
        assert_sync::<Unc>();
    }
}
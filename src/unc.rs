use crate::*;

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

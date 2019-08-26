use crate::*;

pub struct Unc;

impl Unc {
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

pub trait Uncertainty: Sized + Copy {
    fn as_ab(self) -> AbUnc;
    fn as_rel(self) -> RelUnc;
    fn val(&self) -> f64;
    fn unc(&self) -> f64;

    fn min(&self) -> f64 {
        let x = self.as_ab();
        x.val() - x.unc()
    }

    fn max(&self) -> f64 {
        let x = self.as_ab();
        x.val() + x.unc()
    }

    fn overlap<T: Uncertainty>(&self, other: &T) -> bool {
        if self.val() > other.val() {
            self.min() <= other.max()
        } else {
            self.max() >= other.min()
        }
    }
}

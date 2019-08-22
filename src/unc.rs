use crate::*;

/// UncVal, or an Uncertain Value is a way of representing a numerical value of which the true
/// value is not known. The the uncertainty can be expressed using absolute uncertainty,
/// [ab()](struct.UncVal.html#method.ab), or relative uncertainty,
/// [rel()](struct.UncVal.html#method.rel).
///
/// These can look like so:
///
/// Absolute: 14.6 ± 0.2 mm
///
/// Relative: 14.7 ± 0.01369%
///
/// It should be apparent upon inspection that these two measurements are equal to each other, even
/// though they slightly differ in value. This is because there is some overlap between the two
/// measurements when the uncertainty value is considered. This can be determined as follows:
///
/// ```
/// let one = UncVal::ab(14.6, 0.2);
/// let two = UncVal::rel(14.7, 0.01369);
/// assert!(one.overlap(two));
/// ```
#[derive(Debug, Copy, Clone)]
pub struct UncVal;

impl UncVal {
    /// ```
    /// use uncertainty_rs::*;
    ///
    /// let u: AbUncVal = UncVal::ab(10.0, 1.0);
    /// assert_eq!(u.val(), 10.0);
    /// assert_eq!(u.unc(), 1.0);
    /// ```
    pub fn ab(val: f64, unc: f64) -> AbUncVal {
        AbUncVal { val, unc }
    }

    /// ```
    /// use uncertainty_rs::*;
    ///
    /// let u: RelUncVal = UncVal::rel(10.0, 0.1);
    /// assert_eq!(u.val(), 10.0);
    /// assert_eq!(u.unc(), 0.1);
    /// ```
    pub fn rel(val: f64, unc: f64) -> RelUncVal {
        RelUncVal { val, unc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_add_and_sub() {
        let one = UncVal::ab(10.0, 0.5);
        let two = 4.5;
        let three = UncVal::rel(20.0, 0.1); // 10% uncertainty is 2.0
        let eq = one + two - three.as_ab();

        assert_abs_diff_eq!(eq.val, -5.5);
        assert_abs_diff_eq!(eq.unc, 2.5);
    }

    #[test]
    fn test_div_and_mul() {
        let one = UncVal::rel(20.0, 0.05);
        let two = 2.0;
        let three = UncVal::ab(3.5, 0.35); // 0.35 ab uncertainty is 10%
        let eq = one / two * three.as_rel();

        assert_abs_diff_eq!(eq.val(), 35.0);
        assert_abs_diff_eq!(eq.unc(), 0.15);
    }

    #[test]
    fn test_all_ops() {
        let two = UncVal::ab(2.0, 0.02);
        let three = UncVal::ab(3.0, 0.03);
        let four = RelUncVal::from(4.0);
        let five = UncVal::rel(5.0, 0.1);
        let eq = ((two + three).as_rel() / five * four).as_ab() - two;

        assert_abs_diff_eq!(eq.val(), 2.0);
        assert_abs_diff_eq!(eq.unc(), 0.46);
    }

    #[test]
    fn test_trait_ab() {
        let x = UncVal::ab(10.0, 0.5);

        assert_eq!(x.clone().as_rel(), UncVal::rel(10.0, 0.05));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.5);
    }

    #[test]
    fn test_trait_rel() {
        let x = UncVal::rel(10.0, 0.05);

        assert_eq!(x.clone().as_ab(), UncVal::ab(10.0, 0.5));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.05);
    }

    #[test]
    fn test_trait_min() {
        assert_abs_diff_eq!(9.5, UncVal::ab(10.0, 0.5).min());
    }

    #[test]
    fn test_trait_max() {
        assert_abs_diff_eq!(101.0, UncVal::rel(100.0, 0.01).max());
    }

    #[test]
    fn test_traitoverlap() {
        let one = UncVal::ab(10.0, 0.6);
        let two = UncVal::ab(11.0, 0.5);
        assert!(one.overlap(&two));
    }
}

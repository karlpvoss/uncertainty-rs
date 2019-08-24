use crate::*;

/// UncVal, or an Uncertain Value is a way of representing a numerical value of which the true
/// value is not known. The the uncertainty can be expressed using absolute uncertainty,
/// [ab()](struct.Unc.html#method.ab), or relative uncertainty,
/// [rel()](struct.Unc.html#method.rel).
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
/// use uncertainty_rs::*;
///
/// let one = Unc::ab(14.6, 0.2);
/// let two = Unc::rel(14.7, 0.01369);
/// assert!(one.overlap(&two));
/// ```
#[derive(Debug, Copy, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_add_and_sub() {
        let one = Unc::ab(10.0, 0.5);
        let two = 4.5;
        let three = Unc::rel(20.0, 0.1); // 10% uncertainty is 2.0
        let eq = one + two - three.as_ab();

        assert_abs_diff_eq!(eq.val, -5.5);
        assert_abs_diff_eq!(eq.unc, 2.5);
    }

    #[test]
    fn test_div_and_mul() {
        let one = Unc::rel(20.0, 0.05);
        let two = 2.0;
        let three = Unc::ab(3.5, 0.35); // 0.35 ab uncertainty is 10%
        let eq = one / two * three.as_rel();

        assert_abs_diff_eq!(eq.val(), 35.0);
        assert_abs_diff_eq!(eq.unc(), 0.15);
    }

    #[test]
    fn test_all_ops() {
        let two = Unc::ab(2.0, 0.02);
        let three = Unc::ab(3.0, 0.03);
        let four = RelUnc::from(4.0);
        let five = Unc::rel(5.0, 0.1);
        let eq = ((two + three).as_rel() / five * four).as_ab() - two;

        assert_abs_diff_eq!(eq.val(), 2.0);
        assert_abs_diff_eq!(eq.unc(), 0.46);
    }

    #[test]
    fn test_trait_ab() {
        let x = Unc::ab(10.0, 0.5);

        assert_eq!(x.clone().as_rel(), Unc::rel(10.0, 0.05));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.5);
    }

    #[test]
    fn test_trait_rel() {
        let x = Unc::rel(10.0, 0.05);

        assert_eq!(x.clone().as_ab(), Unc::ab(10.0, 0.5));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.05);
    }

    #[test]
    fn test_trait_min() {
        assert_abs_diff_eq!(9.5, Unc::ab(10.0, 0.5).min());
    }

    #[test]
    fn test_trait_max() {
        assert_abs_diff_eq!(101.0, Unc::rel(100.0, 0.01).max());
    }

    #[test]
    fn test_traitoverlap() {
        let one = Unc::ab(10.0, 0.6);
        let two = Unc::ab(11.0, 0.5);
        assert!(one.overlap(&two));
    }
}

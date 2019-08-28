use std::ops::Div;

use crate::*;

impl Div<RelUnc> for RelUnc {
    type Output = RelUnc;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, other: RelUnc) -> Self::Output {
        unc::rel(self.val() / other.val(), self.unc() + other.unc())
    }
}

impl Div<f64> for RelUnc {
    type Output = RelUnc;

    fn div(self, other: f64) -> Self::Output {
        self / RelUnc::from(other)
    }
}

impl Div<RelUnc> for f64 {
    type Output = RelUnc;

    fn div(self, other: RelUnc) -> Self::Output {
        RelUnc::from(self) / other
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = unc::rel(10.0, 0.1);
        let two = unc::rel(2.0, 0.1);
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 5.0);
        assert_abs_diff_eq!(quot.unc(), 0.2);
    }

    #[test]
    fn test_left_unc() {
        let one = unc::rel(1.0, 0.1);
        let two = 2.0;
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 0.5);
        assert_abs_diff_eq!(quot.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = unc::rel(1.0, 0.1);
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 2.0);
        assert_abs_diff_eq!(quot.unc(), 0.1);
    }
}

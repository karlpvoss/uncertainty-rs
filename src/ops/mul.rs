use std::ops::Mul;

use crate::*;

impl Mul<RelUnc> for RelUnc {
    type Output = RelUnc;

    fn mul(self, other: RelUnc) -> Self::Output {
        Unc::rel(self.val() * other.val(), self.unc() + other.unc())
    }
}

impl Mul<f64> for RelUnc {
    type Output = RelUnc;

    fn mul(self, other: f64) -> Self::Output {
        self * RelUnc::from(other)
    }
}

impl Mul<RelUnc> for f64 {
    type Output = RelUnc;

    fn mul(self, other: RelUnc) -> Self::Output {
        other * RelUnc::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = Unc::rel(1.0, 0.1);
        let two = Unc::rel(2.0, 0.1);
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.2);
    }

    #[test]
    fn test_left_unc() {
        let one = Unc::rel(1.0, 0.1);
        let two = 2.0;
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = Unc::rel(1.0, 0.1);
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.1);
    }
}

use std::ops::Mul;

use crate::{RelUncVal, UncVal, UncertainValue};

impl Mul<RelUncVal> for RelUncVal {
    type Output = RelUncVal;

    fn mul(self, other: RelUncVal) -> Self::Output {
        UncVal::rel(self.val() * other.val(), self.unc() + other.unc())
    }
}

impl Mul<f64> for RelUncVal {
    type Output = RelUncVal;

    fn mul(self, other: f64) -> Self::Output {
        self * RelUncVal::from(other)
    }
}

impl Mul<RelUncVal> for f64 {
    type Output = RelUncVal;

    fn mul(self, other: RelUncVal) -> Self::Output {
        other * RelUncVal::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{UncVal, UncertainValue};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = UncVal::rel(1.0, 0.1);
        let two = UncVal::rel(2.0, 0.1);
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.2);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::rel(1.0, 0.1);
        let two = 2.0;
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::rel(1.0, 0.1);
        let prod = one * two;

        assert_abs_diff_eq!(prod.val(), 2.0);
        assert_abs_diff_eq!(prod.unc(), 0.1);
    }
}

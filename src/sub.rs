use std::ops::Sub;

use crate::{AbUncVal, UncVal, UncertainValue};

impl Sub<AbUncVal> for AbUncVal {
    type Output = AbUncVal;

    fn sub(self, other: Self) -> Self::Output {
        UncVal::ab(self.val() - other.val(), self.unc() + other.unc())
    }
}

impl Sub<f64> for AbUncVal {
    type Output = AbUncVal;

    fn sub(self, other: f64) -> Self::Output {
        self - AbUncVal::from(other)
    }
}

impl Sub<AbUncVal> for f64 {
    type Output = AbUncVal;

    fn sub(self, other: AbUncVal) -> Self::Output {
        other - AbUncVal::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{UncVal, UncertainValue};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = UncVal::ab(2.0, 0.2);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.3);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = 2.0;
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::ab(1.0, 0.1);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.1);
    }
}

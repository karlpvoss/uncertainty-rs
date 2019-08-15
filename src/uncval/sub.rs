use std::ops::Sub;

use crate::UncVal;

impl Sub<UncVal> for UncVal {
    type Output = UncVal;

    fn sub(self, other: Self) -> Self::Output {
        let one = self.as_ab();
        let two = other.as_ab();

        UncVal::ab(one.val - two.val, one.unc + two.unc)
    }
}

impl Sub<f64> for UncVal {
    type Output = UncVal;

    fn sub(self, other: f64) -> Self::Output {
        self - UncVal::from(other)
    }
}

impl Sub<UncVal> for f64 {
    type Output = UncVal;

    fn sub(self, other: UncVal) -> Self::Output {
        other - UncVal::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{UncVal, UncertaintyType};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = UncVal::ab(2.0, 0.2);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val, -1.0);
        assert_abs_diff_eq!(diff.unc, 0.3);
        assert_eq!(diff.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = 2.0;
        let diff = one - two;

        assert_abs_diff_eq!(diff.val, -1.0);
        assert_abs_diff_eq!(diff.unc, 0.1);
        assert_eq!(diff.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::ab(1.0, 0.1);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val, -1.0);
        assert_abs_diff_eq!(diff.unc, 0.1);
        assert_eq!(diff.ty, UncertaintyType::Absolute);
    }
}

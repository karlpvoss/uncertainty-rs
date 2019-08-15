use std::ops::Add;

use crate::UncVal;

impl Add<UncVal> for UncVal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let one = self.as_ab();
        let two = other.as_ab();

        UncVal::ab(one.val + two.val, one.unc + two.unc)
    }
}

impl Add<f64> for UncVal {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        self + UncVal::from(other)
    }
}

impl Add<UncVal> for f64 {
    type Output = UncVal;
    fn add(self, other: UncVal) -> Self::Output {
        other + UncVal::from(self)
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
        let sum = one + two;

        assert_abs_diff_eq!(sum.val, 3.0);
        assert_abs_diff_eq!(sum.unc, 0.3);
        assert_eq!(sum.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = 2.0;
        let sum = one + two;

        assert_abs_diff_eq!(sum.val, 3.0);
        assert_abs_diff_eq!(sum.unc, 0.1);
        assert_eq!(sum.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::ab(1.0, 0.1);
        let sum = one + two;

        assert_abs_diff_eq!(sum.val, 3.0);
        assert_abs_diff_eq!(sum.unc, 0.1);
        assert_eq!(sum.ty, UncertaintyType::Absolute);
    }
}

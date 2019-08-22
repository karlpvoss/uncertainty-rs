use std::ops::Add;

use crate::{AbUncVal, UncVal, UncertainValue};

impl Add<AbUncVal> for AbUncVal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        UncVal::ab(self.val() + other.val(), self.unc() + other.unc())
    }
}

impl Add<f64> for AbUncVal {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        self + AbUncVal::from(other)
    }
}

impl Add<AbUncVal> for f64 {
    type Output = AbUncVal;

    fn add(self, other: AbUncVal) -> Self::Output {
        other + AbUncVal::from(self)
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
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.3);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::ab(1.0, 0.1);
        let two = 2.0;
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::ab(1.0, 0.1);
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.1);
    }
}

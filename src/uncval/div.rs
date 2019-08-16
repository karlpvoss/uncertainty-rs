use std::ops::Div;

use crate::{RelUncVal, UncVal, UncertainValue};

impl Div<RelUncVal> for RelUncVal {
    type Output = RelUncVal;

    fn div(self, other: RelUncVal) -> Self::Output {
        UncVal::rel(self.val() / other.val(), self.unc() + other.unc())
    }
}

impl Div<f64> for RelUncVal {
    type Output = RelUncVal;

    fn div(self, other: f64) -> Self::Output {
        self / RelUncVal::from(other)
    }
}

impl Div<RelUncVal> for f64 {
    type Output = RelUncVal;

    fn div(self, other: RelUncVal) -> Self::Output {
        RelUncVal::from(self) / other
    }
}

#[cfg(test)]
mod tests {
    use crate::{UncVal, UncertainValue};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = UncVal::rel(10.0, 0.1);
        let two = UncVal::rel(2.0, 0.1);
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 5.0);
        assert_abs_diff_eq!(quot.unc(), 0.2);
    }

    #[test]
    fn test_left_unc() {
        let one = UncVal::rel(1.0, 0.1);
        let two = 2.0;
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 0.5);
        assert_abs_diff_eq!(quot.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = UncVal::rel(1.0, 0.1);
        let quot = one / two;

        assert_abs_diff_eq!(quot.val(), 2.0);
        assert_abs_diff_eq!(quot.unc(), 0.1);
    }
}

use std::ops::Sub;

use crate::{AbUnc, Unc, UncertainValue};

impl Sub<AbUnc> for AbUnc {
    type Output = AbUnc;

    fn sub(self, other: Self) -> Self::Output {
        Unc::ab(self.val() - other.val(), self.unc() + other.unc())
    }
}

impl Sub<f64> for AbUnc {
    type Output = AbUnc;

    fn sub(self, other: f64) -> Self::Output {
        self - AbUnc::from(other)
    }
}

impl Sub<AbUnc> for f64 {
    type Output = AbUnc;

    fn sub(self, other: AbUnc) -> Self::Output {
        other - AbUnc::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Unc, UncertainValue};
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = Unc::ab(1.0, 0.1);
        let two = Unc::ab(2.0, 0.2);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.3);
    }

    #[test]
    fn test_left_unc() {
        let one = Unc::ab(1.0, 0.1);
        let two = 2.0;
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = Unc::ab(1.0, 0.1);
        let diff = one - two;

        assert_abs_diff_eq!(diff.val(), -1.0);
        assert_abs_diff_eq!(diff.unc(), 0.1);
    }
}

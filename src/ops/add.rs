use std::ops::Add;

use crate::*;

impl Add<AbUnc> for AbUnc {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        AbUnc::new(self.val() + other.val(), self.unc() + other.unc())
    }
}

impl Add<f64> for AbUnc {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        self + AbUnc::from(other)
    }
}

impl Add<AbUnc> for f64 {
    type Output = AbUnc;

    fn add(self, other: AbUnc) -> Self::Output {
        other + AbUnc::from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_both_unc() {
        let one = AbUnc::new(1.0, 0.1);
        let two = AbUnc::new(2.0, 0.2);
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.3);
    }

    #[test]
    fn test_left_unc() {
        let one = AbUnc::new(1.0, 0.1);
        let two = 2.0;
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.1);
    }

    #[test]
    fn test_right_unc() {
        let one = 2.0;
        let two = AbUnc::new(1.0, 0.1);
        let sum = one + two;

        assert_abs_diff_eq!(sum.val(), 3.0);
        assert_abs_diff_eq!(sum.unc(), 0.1);
    }
}

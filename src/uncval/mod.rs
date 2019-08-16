pub mod add;
pub mod convert;
pub mod div;
pub mod mul;
pub mod sub;

use std::fmt;

#[derive(Debug, Copy, Clone)]
pub struct UncVal;

pub trait UncertainValue: Sized + Copy {
    fn as_ab(self) -> AbUncVal;
    fn as_rel(self) -> RelUncVal;
    fn val(&self) -> f64;
    fn unc(&self) -> f64;

    fn min(&self) -> f64 {
        let x = self.as_ab();
        x.val() - x.unc()
    }

    fn max(&self) -> f64 {
        let x = self.as_ab();
        x.val() + x.unc()
    }

    fn overlap<T: UncertainValue>(&self, other: &T) -> bool {
        if self.val() > other.val() {
            self.min() <= other.max()
        } else {
            self.max() >= other.min()
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct AbUncVal {
    val: f64,
    unc: f64,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RelUncVal {
    val: f64,
    unc: f64,
}

impl UncVal {
    pub fn ab(val: f64, unc: f64) -> AbUncVal {
        AbUncVal { val, unc }
    }

    pub fn rel(val: f64, unc: f64) -> RelUncVal {
        RelUncVal { val, unc }
    }
}

impl UncertainValue for AbUncVal {
    fn as_ab(self) -> AbUncVal {
        self
    }

    fn as_rel(self) -> RelUncVal {
        RelUncVal {
            val: self.val,
            unc: self.unc / self.val,
        }
    }

    fn val(&self) -> f64 {
        self.val
    }

    fn unc(&self) -> f64 {
        self.unc
    }
}

impl UncertainValue for RelUncVal {
    fn as_ab(self) -> AbUncVal {
        UncVal::ab(self.val, self.unc * self.val)
    }

    fn as_rel(self) -> RelUncVal {
        self
    }

    fn val(&self) -> f64 {
        self.val
    }

    fn unc(&self) -> f64 {
        self.unc
    }
}

impl fmt::Display for AbUncVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}", self.val, self.unc)
    }
}

impl fmt::Display for RelUncVal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ± {}%", self.val, self.unc * 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_ab_constructor() {
        let u: AbUncVal = UncVal::ab(10.0, 1.0);
        assert_eq!(u.val, 10.0);
        assert_eq!(u.unc, 1.0);
    }

    #[test]
    fn test_rel_constructor() {
        let u: RelUncVal = UncVal::rel(10.0, 0.1);
        assert_eq!(u.val, 10.0);
        assert_eq!(u.unc, 0.1);
    }

    #[test]
    fn test_add_and_sub() {
        let one = UncVal::ab(10.0, 0.5);
        let two = 4.5;
        let three = UncVal::rel(20.0, 0.1); // 10% uncertainty is 2.0
        let eq = one + two - three.as_ab();

        assert_abs_diff_eq!(eq.val, -5.5);
        assert_abs_diff_eq!(eq.unc, 2.5);
    }

    #[test]
    fn test_div_and_mul() {
        let one = UncVal::rel(20.0, 0.05);
        let two = 2.0;
        let three = UncVal::ab(3.5, 0.35); // 0.35 ab uncertainty is 10%
        let eq = one / two * three.as_rel();

        assert_abs_diff_eq!(eq.val(), 35.0);
        assert_abs_diff_eq!(eq.unc(), 0.15);
    }

    #[test]
    fn test_all_ops() {
        let two = UncVal::ab(2.0, 0.02);
        let three = UncVal::ab(3.0, 0.03);
        let four = RelUncVal::from(4.0);
        let five = UncVal::rel(5.0, 0.1);
        let eq = ((two + three).as_rel() / five * four).as_ab() - two;

        assert_abs_diff_eq!(eq.val(), 2.0);
        assert_abs_diff_eq!(eq.unc(), 0.46);
    }

    #[test]
    fn test_trait_ab() {
        let x = UncVal::ab(10.0, 0.5);

        assert_eq!(x.clone().as_rel(), UncVal::rel(10.0, 0.05));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.5);
    }

    #[test]
    fn test_trait_rel() {
        let x = UncVal::rel(10.0, 0.05);

        assert_eq!(x.clone().as_ab(), UncVal::ab(10.0, 0.5));
        assert_eq!(x.val(), 10.0);
        assert_eq!(x.unc(), 0.05);
    }

    #[test]
    fn test_trait_min() {
        assert_abs_diff_eq!(9.5, UncVal::ab(10.0, 0.5).min());
    }

    #[test]
    fn test_trait_max() {
        assert_abs_diff_eq!(101.0, UncVal::rel(100.0, 0.01).max());
    }

    #[test]
    fn test_traitoverlap() {
        let one = UncVal::ab(10.0, 0.6);
        let two = UncVal::ab(11.0, 0.5);
        assert!(one.overlap(&two));
    }
}

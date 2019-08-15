pub mod add;
pub mod sub;

use crate::UncertaintyType;

#[derive(Debug)]
pub struct UncVal {
    pub val: f64,
    pub unc: f64,
    pub ty: UncertaintyType,
}

impl UncVal {
    pub fn ab(val: f64, unc: f64) -> Self {
        UncVal {
            val,
            unc,
            ty: UncertaintyType::Absolute,
        }
    }

    pub fn rel(val: f64, unc: f64) -> Self {
        UncVal {
            val,
            unc,
            ty: UncertaintyType::Relative,
        }
    }

    pub fn as_ab(self) -> Self {
        match self.ty {
            UncertaintyType::Absolute => self,
            UncertaintyType::Relative => UncVal {
                val: self.val,
                unc: self.val * self.unc,
                ty: UncertaintyType::Absolute,
            },
        }
    }

    pub fn as_rel(self) -> Self {
        match self.ty {
            UncertaintyType::Relative => self,
            UncertaintyType::Absolute => UncVal {
                val: self.val,
                unc: self.unc / self.val,
                ty: UncertaintyType::Relative,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_ab_constructor() {
        let u: UncVal = UncVal::ab(10.0, 1.0);
        assert_eq!(u.val, 10.0);
        assert_eq!(u.unc, 1.0);
        assert_eq!(u.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_rel_constructor() {
        let u: UncVal = UncVal::rel(10.0, 0.1);
        assert_eq!(u.val, 10.0);
        assert_eq!(u.unc, 0.1);
        assert_eq!(u.ty, UncertaintyType::Relative);
    }

    #[test]
    fn test_add_and_sub() {
        let one = UncVal::ab(10.0, 0.5);
        let two = 4.5;
        let three = UncVal::rel(20.0, 0.1); // 10% uncertainty is 2.0
        let eq = one + two - three;

        assert_abs_diff_eq!(eq.val, -5.5);
        assert_abs_diff_eq!(eq.unc, 2.5);
        assert_eq!(eq.ty, UncertaintyType::Absolute);
    }
}

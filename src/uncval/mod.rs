pub mod add;
pub mod convert;
pub mod sub;

#[derive(Debug, Copy, Clone)]
pub struct UncVal;

pub trait UncertainValue {
    fn as_ab(self) -> AbUncVal;
    fn as_rel(self) -> RelUncVal;
}

#[derive(Debug, Copy, Clone)]
pub struct AbUncVal {
    pub val: f64,
    pub unc: f64,
}

#[derive(Debug, Copy, Clone)]
pub struct RelUncVal {
    pub val: f64,
    pub unc: f64,
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
}

impl UncertainValue for RelUncVal {
    fn as_ab(self) -> AbUncVal {
        UncVal::ab(self.val, self.unc * self.val)
    }

    fn as_rel(self) -> RelUncVal {
        self
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
}

pub mod add;
pub mod convert;
pub mod div;
pub mod mul;
pub mod sub;

#[derive(Debug, Copy, Clone)]
pub struct UncVal;

pub trait UncertainValue {
    fn as_ab(self) -> AbUncVal;
    fn as_rel(self) -> RelUncVal;
    fn val(&self) -> f64;
    fn unc(&self) -> f64;
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
}

use std::convert::From;

use crate::{AbUncVal, RelUncVal, UncVal, UncertainValue};

impl From<f64> for AbUncVal {
    fn from(float: f64) -> AbUncVal {
        UncVal::ab(float, 0.0)
    }
}

impl From<f64> for RelUncVal {
    fn from(float: f64) -> RelUncVal {
        UncVal::rel(float, 0.0)
    }
}

impl From<RelUncVal> for AbUncVal {
    fn from(x: RelUncVal) -> AbUncVal {
        x.as_ab()
    }
}

impl From<AbUncVal> for RelUncVal {
    fn from(x: AbUncVal) -> RelUncVal {
        x.as_rel()
    }
}

#[cfg(test)]
mod tests {
    use crate::{AbUncVal, RelUncVal, UncVal, UncertainValue};
    use approx::assert_abs_diff_eq;

    #[test]
    fn ab_from_f64() {
        let x: AbUncVal = 1.0.into();
        assert_abs_diff_eq!(x.val, 1.0);
        assert_abs_diff_eq!(x.unc, 0.0);
    }

    #[test]
    fn rel_from_f64() {
        let x: RelUncVal = 1.0.into();
        assert_abs_diff_eq!(x.val(), 1.0);
        assert_abs_diff_eq!(x.unc(), 0.0);
    }

    #[test]
    fn rel_from_ab() {
        let x = UncVal::ab(10.0, 1.0).as_rel();

        assert_abs_diff_eq!(x.val(), 10.0);
        assert_abs_diff_eq!(x.unc(), 0.1);
    }

    #[test]
    fn ab_from_rel() {
        let x = UncVal::rel(10.0, 0.1).as_ab();

        assert_abs_diff_eq!(x.val(), 10.0);
        assert_abs_diff_eq!(x.unc(), 1.0);
    }
}

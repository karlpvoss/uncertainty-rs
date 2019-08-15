use std::convert::From;

use crate::{AbUncVal, RelUncVal, UncVal};

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

#[cfg(test)]
mod tests {
    use crate::AbUncVal;
    use approx::assert_abs_diff_eq;

    #[test]
    fn from_f64() {
        let x: AbUncVal = 1.0.into();
        assert_abs_diff_eq!(x.val, 1.0);
        assert_abs_diff_eq!(x.unc, 0.0);
    }
}

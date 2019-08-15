use std::convert::From;

use crate::UncVal;

impl From<f64> for UncVal {
    fn from(float: f64) -> UncVal {
        UncVal::ab(float, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{UncVal, UncertaintyType};
    use approx::assert_abs_diff_eq;

    #[test]
    fn from_f64() {
        let x: UncVal = 1.0.into();
        assert_abs_diff_eq!(x.val, 1.0);
        assert_abs_diff_eq!(x.unc, 0.0);
        assert_eq!(x.ty, UncertaintyType::Absolute);
    }
}

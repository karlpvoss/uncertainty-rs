use std::convert::From;

use crate::*;

impl From<f64> for AbUnc {
    fn from(float: f64) -> AbUnc {
        Unc::ab(float, 0.0)
    }
}

impl From<f64> for RelUnc {
    fn from(float: f64) -> RelUnc {
        Unc::rel(float, 0.0)
    }
}

impl From<RelUnc> for AbUnc {
    fn from(x: RelUnc) -> AbUnc {
        x.to_ab()
    }
}

impl From<AbUnc> for RelUnc {
    fn from(x: AbUnc) -> RelUnc {
        x.to_rel()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn ab_from_f64() {
        let x: AbUnc = 1.0.into();
        assert_abs_diff_eq!(x.val(), 1.0);
        assert_abs_diff_eq!(x.unc(), 0.0);
    }

    #[test]
    fn rel_from_f64() {
        let x: RelUnc = 1.0.into();
        assert_abs_diff_eq!(x.val(), 1.0);
        assert_abs_diff_eq!(x.unc(), 0.0);
    }

    #[test]
    fn rel_from_ab() {
        let x = Unc::ab(10.0, 1.0).to_rel();

        assert_abs_diff_eq!(x.val(), 10.0);
        assert_abs_diff_eq!(x.unc(), 0.1);
    }

    #[test]
    fn ab_from_rel() {
        let x = Unc::rel(10.0, 0.1).to_ab();

        assert_abs_diff_eq!(x.val(), 10.0);
        assert_abs_diff_eq!(x.unc(), 1.0);
    }
}

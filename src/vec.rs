use crate::*;

pub struct UncVec;

impl UncVec {
    fn ab<T: IntoIterator<Item = f64>>(vals: T, unc: f64) -> Vec<AbUnc> {
        vals.into_iter().map(|val| Unc::ab(val, unc)).collect()
    }

    fn rel<T: IntoIterator<Item = f64>>(vals: T, unc: f64) -> Vec<RelUnc> {
        vals.into_iter().map(|val| Unc::rel(val, unc)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_ab() {
        let v = vec![1.0, 2.0, 3.0];
        let uv: Vec<AbUnc> = UncVec::ab(v, 0.02);

        assert_eq!(uv[0].val(), 1.0);
        assert_eq!(uv[1].val(), 2.0);
        assert_eq!(uv[2].val(), 3.0);
        for each in uv {
            assert_abs_diff_eq!(each.unc(), 0.02);
        }
    }

    #[test]
    fn test_rel() {
        let v = vec![1.0, 2.0, 3.0];
        let uv: Vec<RelUnc> = UncVec::rel(v, 0.02);

        assert_eq!(uv[0].val(), 1.0);
        assert_eq!(uv[1].val(), 2.0);
        assert_eq!(uv[2].val(), 3.0);
        for each in uv {
            assert_abs_diff_eq!(each.unc(), 0.02);
        }
    }
}

use crate::UncVal;
use crate::UncertaintyType;

pub struct UncVec {
    pub vals: Vec<f64>,
    pub unc: f64,
    pub ty: UncertaintyType,
}

impl UncVec {
    pub fn ab(vals: Vec<f64>, unc: f64) -> Self {
        UncVec {
            vals,
            unc,
            ty: UncertaintyType::Absolute,
        }
    }

    pub fn rel(vals: Vec<f64>, unc: f64) -> Self {
        UncVec {
            vals,
            unc,
            ty: UncertaintyType::Relative,
        }
    }

    pub fn as_ab(self) -> Vec<UncVal> {
        let unc = self.unc;
        let vals = self.vals.into_iter();
        match self.ty {
            UncertaintyType::Absolute => vals.map(|val| UncVal::ab(val, unc)).collect(),
            UncertaintyType::Relative => vals.map(|val| UncVal::rel(val, unc).as_ab()).collect(),
        }
    }

    pub fn as_rel(self) -> Vec<UncVal> {
        let unc = self.unc;
        let vals = self.vals.into_iter();
        match self.ty {
            UncertaintyType::Relative => vals.map(|val| UncVal::rel(val, unc)).collect(),
            UncertaintyType::Absolute => vals.map(|val| UncVal::ab(val, unc).as_rel()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ab_constructor() {
        let v = vec![1.0, 2.0, 3.0];
        let uv = UncVec::ab(v.clone(), 0.1);
        assert_eq!(uv.vals, v);
        assert_eq!(uv.unc, 0.1);
        assert_eq!(uv.ty, UncertaintyType::Absolute);
    }

    #[test]
    fn test_rel_constructor() {
        let v = vec![1.0f64, 2.0, 3.0];
        let uv = UncVec::rel(v.clone(), 0.1);
        assert_eq!(uv.vals, v);
        assert_eq!(uv.unc, 0.1);
        assert_eq!(uv.ty, UncertaintyType::Relative);
    }
}

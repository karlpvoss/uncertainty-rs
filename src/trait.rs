use crate::{AbUncVal, RelUncVal};

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

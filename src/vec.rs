use crate::*;

pub struct UncVec;

impl UncVec {
    /// This is a method which allows the conversion of a list of values into a list of absolute
    /// uncertainties, each with the same uncertainty value.
    ///
    /// # Examples
    ///
    /// ```
    /// use uncertainty::Uncertainty;
    /// use uncertainty::vec::UncVec;
    ///
    /// let vals = vec![1.0, 2.0, 3.0];
    /// let uncertainties = UncVec::ab(vals, 0.1);
    ///
    /// assert_eq!(uncertainties[0].val(), 1.0);
    /// assert_eq!(uncertainties[1].val(), 2.0);
    /// assert_eq!(uncertainties[2].val(), 3.0);
    ///
    /// for each in uncertainties {
    ///     assert_eq!(each.unc(), 0.1);
    /// }
    /// ```
    pub fn ab<T: IntoIterator<Item = f64>>(vals: T, unc: f64) -> Vec<AbUnc> {
        vals.into_iter().map(|val| Unc::ab(val, unc)).collect()
    }

    /// This is a method which allows the conversion of a list of values into a list of relative
    /// uncertainties, each with the same uncertainty value.
    ///
    /// # Examples
    ///
    /// ```
    /// use uncertainty::Uncertainty;
    /// use uncertainty::vec::UncVec;
    ///
    /// let vals = vec![1.0, 2.0, 3.0];
    /// let uncertainties = UncVec::rel(vals, 0.1);
    ///
    /// assert_eq!(uncertainties[0].val(), 1.0);
    /// assert_eq!(uncertainties[1].val(), 2.0);
    /// assert_eq!(uncertainties[2].val(), 3.0);
    ///
    /// for each in uncertainties {
    ///     assert_eq!(each.unc(), 0.1);
    /// }
    /// ```
    pub fn rel<T: IntoIterator<Item = f64>>(vals: T, unc: f64) -> Vec<RelUnc> {
        vals.into_iter().map(|val| Unc::rel(val, unc)).collect()
    }
}

#[cfg(test)]
mod tests {}

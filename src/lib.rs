//! Unc, or an Uncertainty is a way of representing a numerical value of which the true
//! value is not known. The uncertainty can be expressed using absolute uncertainty,
//! [ab()](unc/struct.Unc.html#method.ab), or relative uncertainty,
//! [rel()](unc/struct.Unc.html#method.rel).
//!
//! These can look like so:
//!
//! Absolute: 14.6 ± 0.2 mm
//!
//! Relative: 14.7 ± 0.01369%
//!
//! It should be apparent upon inspection that these two measurements are equal to each other, even
//! though they slightly differ in value. This is because there is some overlap between the two
//! measurements when the uncertainty value is considered. This can be determined as follows:
//!
//! ```
//! use uncertainty_rs::*;
//!
//! let one = Unc::ab(14.6, 0.2);
//! let two = Unc::rel(14.7, 0.01369);
//! assert!(one.overlap(&two));
//! ```
//!
//! There are also more arithmetic operations supported; absolute uncertainties can be added and
//! subtracted, and relative uncertainties can be multiplied and divided. Relative uncertainties
//! can also be raised to [powers](relunc/struct.RelUnc.html#method.powi). To convert between
//! the two, you can make use of the [Uncertainty](unc/trait.Uncertainty.html) trait.
//!
//! ```
//! use uncertainty_rs::*;
//! use approx::assert_abs_diff_eq;
//!
//! let one = Unc::ab(14.6, 0.2);
//! let two = Unc::rel(14.7, 0.01369);
//! let three = Unc::rel(2.0, 0.05);
//!
//! let eq: AbUnc = one + two.to_ab();
//! let eq: RelUnc = eq.to_rel() * three;
//!
//! assert_abs_diff_eq!(eq.val(), 58.6, epsilon = 0.0000001);
//! assert_abs_diff_eq!(eq.unc(), 0.0636943, epsilon = 0.0000001);
//! ```
//!
//! Conversion between the types can also be done via the From trait:
//!
//! ```
//! # use uncertainty_rs::*;
//! # use approx::assert_abs_diff_eq;
//! let x: AbUnc = Unc::rel(2.0, 0.1).into();
//! assert_abs_diff_eq!(x.unc(), 0.2);
//!
//! let y = AbUnc::from(2.0);
//! assert_abs_diff_eq!(y.val(), 2.0);
//! assert_abs_diff_eq!(y.unc(), 0.0);
//! ```
//!
//! Using the From trait to create an Uncertainty value will result in a value with zero
//! uncertainty in it's value. This is the way scalar values should be treated within the system
//! for arithmetic purposes.
//!
//! It's possible to write a function which takes either an absolute or relative uncertainty using
//! the Uncertainty trait:
//!
//! ```
//! # use uncertainty_rs::*;
//! fn print<T: Uncertainty>(x: T) {
//!     let x = x.to_ab();
//!     println!("The value is: {}, and the uncertainty is: {}!", x.val(), x.unc());
//! }
//!
//! print(AbUnc::from(10.0));
//! print(RelUnc::from(20.0));
//! ```
//!
//! However, if you only want to accept specifically Relative or Absolute uncertain values, you can
//! do this by using the AbUnc and RelUnc types:
//!
//! ```
//! # use uncertainty_rs::*;
//! fn print_ab(x: AbUnc) {
//!     println!("The value of this absolute uncertainty is: {}", x.val());
//! }
//!
//! print_ab(Unc::ab(14.7, 0.02));
//! ```

pub mod abunc;
pub mod convert;
pub mod ops;
pub mod relunc;
pub mod unc;
pub mod vec;

pub use abunc::AbUnc;
pub use relunc::RelUnc;
pub use unc::Unc;
pub use unc::Uncertainty;

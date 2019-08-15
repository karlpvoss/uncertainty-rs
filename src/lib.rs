pub mod uncval;
pub mod uncvec;

pub use uncval::UncVal;
pub use uncvec::UncVec;

#[derive(Debug)]
#[cfg_attr(test, derive(PartialEq))]
pub enum UncertaintyType {
    Absolute,
    Relative,
}

#![feature(decl_macro)]
#![feature(const_fn_floating_point_arithmetic)]

#[cfg(feature = "nist")]
pub use self::isotope::Isotope;
pub use self::{
    element::{Element, ELEMENTS},
    error::{Error, Result},
    uncertain::{uncertain, Uncertain},
};

#[cfg(feature = "nist")]
pub mod isotopes;

mod element;
mod error;
#[cfg(feature = "nist")]
mod isotope;
mod periodic_table;
mod standard_atomic_weight;
mod uncertain;

#![feature(decl_macro)]

#[cfg(feature = "nist")]
pub use self::isotope::Isotope;
pub use self::{
    element::{Element, ELEMENTS},
    error::{Error, Result},
    uncertain::{uncertain, Uncertain},
};

#[cfg(feature = "nist")]
pub mod isotopes;

pub mod prelude {
    #[cfg(feature = "nist")]
    pub use crate::isotope::Isotope;
    #[cfg(feature = "nist")]
    pub use crate::isotopes;
    pub use crate::{
        element::{Element, ELEMENTS},
        error::{Error, Result},
        uncertain::{uncertain, Uncertain},
    };
}

mod element;
mod error;
#[cfg(feature = "nist")]
mod isotope;
mod periodic_table;
mod standard_atomic_weight;
mod uncertain;

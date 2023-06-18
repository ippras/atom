#![feature(const_fn_floating_point_arithmetic)]

pub use self::{
    atom::{Atom, ABRIDGED, ATOMS, UNABRIDGED},
    error::{Error, Result},
};

pub const PERIODS: usize = 7;

pub const GROUPS: usize = 18;

mod atom;
mod error;

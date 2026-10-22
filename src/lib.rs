//! Periodic table data, physical constants and molecular-model reference
//! parameters, with allocation-free table lookups.
//!
//! The README is included below, and every example in it is compiled and run
//! as a doctest — a reference crate whose documentation has drifted from its
//! data is worse than one with no documentation at all.
#![doc = include_str!("../README.md")]

mod charge_params;
pub mod crippen;
pub mod mmff;
pub mod polar_surface;
pub mod uff;
pub use charge_params::*;

mod consts;
pub use consts::*;

//! Periodic table data and physical constants as plain `const` arrays indexed
//! by atomic number.
//!
//! The README is included below, and every example in it is compiled and run
//! as a doctest — a reference crate whose documentation has drifted from its
//! data is worse than one with no documentation at all.
#![doc = include_str!("../README.md")]

mod consts;
pub use consts::*;

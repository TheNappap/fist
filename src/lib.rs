//! fist crate

#![warn(missing_docs)]
#![allow(incomplete_features)]
#![feature(unsize)]

mod fist;
mod dyn_fist;
mod fist_impl;
#[cfg(test)]
mod tests;

pub use fist::Fist;
pub use dyn_fist::DynFist;
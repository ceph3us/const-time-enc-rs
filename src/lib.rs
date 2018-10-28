#[cfg(test)]
#[macro_use]
extern crate quickcheck;

pub mod base64;
mod error;

pub use error::*;

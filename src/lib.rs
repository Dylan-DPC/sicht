#![deny(rust_2018_idioms)]
// #![deny(missing_docs)]
#![allow(clippy::pedantic)]

pub mod birelational_map;
pub mod diplopie;
mod iter;
pub mod map;

#[cfg(feature = "serde")]
pub mod serde;

pub use crate::diplopie::Diplopie;
pub use crate::map::SichtMap;

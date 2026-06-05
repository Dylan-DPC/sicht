#![deny(rust_2018_idioms)]
// #![deny(missing_docs)]
#![allow(clippy::pedantic)]

pub mod birelational_map;
pub mod diplopia;
mod iter;
pub mod map;

#[cfg(feature = "serde")]
pub mod serde;

pub use crate::diplopia::Diplopia;
pub use crate::map::SichtMap;

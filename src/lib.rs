#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(allocator_api)]
pub mod diplopie;
pub mod iter;
pub mod map;
pub mod serde;

pub use crate::diplopie::Diplopie;
pub use crate::map::SichtMap;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

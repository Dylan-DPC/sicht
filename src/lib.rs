#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(allocator_api)]
mod iter;
pub mod map;
mod node;
mod fix;

pub use crate::map::{SichtMap, SichtSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#![deny(rust_2018_idioms)]
#![deny(clippy::pedantic, clippy::dbg_macro)]
#![feature(allocator_api)]
use crate::node::Root;
use std::alloc::{Allocator, Global};
use std::borrow::Borrow;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

mod node;

pub struct SichtSet<K, O, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: SichtMap<K, O, (), A>,
}

pub struct SichtMap<K, O, V, A = Global>
where
    A: Allocator + Clone,
{
    root: Option<Root<K, O, V>>,
    length: usize,
    alloc: ManuallyDrop<A>,
    _marker: PhantomData<Box<(K, O, V), A>>,
}

impl<K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

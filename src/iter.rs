use crate::{selector::Oder, SichtMap};
use std::alloc::Allocator;
use std::collections::btree_map::Iter;
use kuh::Derow;

impl<'a, K, O, V, A> IntoIterator for &'a SichtMap<'a, K, O, V, A>
where
    K: Ord + Derow<'a, Target: Ord + PartialEq<K>> + Clone + 'a,
    O: Ord + Derow<'a, Target: Ord + PartialEq<O>> + Clone + 'a,
    V: 'a,
    A: Allocator + Clone,
{
    type Item = (&'a Oder<'a, K, O>, &'a V);
    type IntoIter = Iter<'a, Oder<'a, K, O>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

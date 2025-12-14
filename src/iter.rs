use crate::{selector::Oder, SichtMap};
use std::alloc::Allocator;
use std::collections::btree_map::Iter;
use kuh::Derow;

impl<'a, K, O, V, A> IntoIterator for &'a SichtMap<K, O, V, A>
where
    K: Ord + Derow<'a, Target: Ord + PartialEq<K>> + Clone + 'a,
    O: Ord + Derow<'a, Target: Ord + PartialEq<O>> + Clone + 'a,
    V: 'a,
    A: Allocator + Clone,
{
    type Item = (&'a Oder<K, O>, &'a V);
    type IntoIter = Iter<'a, Oder<K, O>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

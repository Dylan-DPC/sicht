use crate::SichtMap;
use kuh::Derow;
use std::alloc::Allocator;
use std::collections::btree_map::Iter;

impl<'a, K, O, V, A> IntoIterator for &'a SichtMap<K, O, V, A>
where
    K: Ord + Derow<'a, Target: Ord + PartialEq<K>> + Clone + 'a,
    O: Ord + Derow<'a, Target: Ord + PartialEq<O>> + Clone + 'a,
    V: 'a,
    A: Allocator + Clone,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

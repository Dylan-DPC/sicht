use crate::{selector::Oder, SichtMap};
use std::alloc::Allocator;
use std::collections::btree_map::Iter;
use kuh::{KuhnvertOwned, KuhnvertBorrowed};

impl<'a, K, O, V, A> IntoIterator for &'a SichtMap<'a, K, O, V, A>
where
    K: Ord + KuhnvertOwned<To = K> + KuhnvertBorrowed<To = K>,
    O: Ord + KuhnvertOwned<To = O> + KuhnvertBorrowed<To = O>,
    A: Allocator + Clone,
{
    type Item = (&'a Oder<'a, K, O>, &'a V);
    type IntoIter = Iter<'a, Oder<'a,K, O>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

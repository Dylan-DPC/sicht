use crate::{Diplopia, SichtMap};
use std::collections::btree_map::{BTreeMap, Iter};

impl<'a, K, O, V> IntoIterator for &'a SichtMap<K, O, V>
where
    K: Ord + Clone + 'a,
    O: Ord + Clone + 'a,
    V: Ord + 'a,
{
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<K, O, V> FromIterator<(K, O, V)> for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Ord,
{
    fn from_iter<I: IntoIterator<Item = (K, O, V)>>(iter: I) -> Self {
        iter.into_iter()
            .map(|(key, cokey, value)| ((key.clone(), value), (key, cokey)))
            .collect()
    }
}

impl<K, O, V> FromIterator<((K, V), (K, O))> for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Ord,
{
    fn from_iter<I: IntoIterator<Item = ((K, V), (K, O))>>(iter: I) -> Self {
        let (map, lookup): (BTreeMap<K, V>, Diplopia<K, O>) = iter.into_iter().unzip();
        Self::with_fields(map, lookup)
    }
}

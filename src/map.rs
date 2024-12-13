use crate::selector::Oder;
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};

pub struct SichtMap<K, O, V, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: BTreeMap<Oder<K, O>, V, A>,
}

pub struct SichtSet<K, O, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: SichtMap<K, O, (), A>,
}

impl<K, O, V> SichtMap<K, O, V>
where
    K: Ord,
    O: Ord,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    pub fn get(&self, key: K) -> Option<&V>
    where
        K: PartialEq + Eq + PartialOrd + Ord,
    {
        self.map.get(&Oder::new_left(key))
    }

    pub fn get_with_outer_key(&self, key: &Oder<K, O>) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_with_outer_key_mut(&mut self, key: &Oder<K, O>) -> Option<&mut V> {
        self.map.get_mut(key)
    }

    pub fn insert_with_both_keys(&mut self, key: K, cokey: O, value: V) {
        self.map.insert(Oder::new(key, cokey), value);
    }

    pub fn insert_with_cokey(&mut self, cokey: O, value: V) {
        self.map.insert(Oder::new_right(cokey), value);
    }

    pub fn iter(&self) -> Iter<'_, Oder<K, O>, V> {
        self.map.iter()
    }
}

impl<K, O, V, A> Debug for SichtMap<K, O, V, A>
where
    K: Ord + Debug,
    O: Ord + Debug,
    V: Debug,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K: Ord, O, V, A> Clone for SichtMap<K, O, V, A>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Clone,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<K, O, V> Default for SichtMap<K, O, V>
where
    K: Ord + Default,
    O: Ord + Default,
    V: Default,
{
    fn default() -> Self {
        SichtMap::new()
    }
}

impl<K, O, V> FromIterator<(Oder<K, O>, V)> for SichtMap<K, O, V>
where
    K: Ord,
    O: Ord,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Oder<K, O>, V)>,
    {
        let map = BTreeMap::from_iter(iter);
        SichtMap { map: map }
    }
}

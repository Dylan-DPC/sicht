use crate::selector::Oder;
use std::alloc::{Allocator, Global};
use std::collections::BTreeMap;
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

impl<K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord + Debug,
    A: Allocator + Clone,
{
    #[must_use]
    pub fn new() -> Self {
        todo!()
    }

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
}

impl<K, O, V, A> Debug for SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<K: Ord, O, V, A> Clone for SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<K, O, V, A: Allocator + Clone> Default for SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn default() -> Self {
        todo!()
    }
}

impl<K: Ord, O, V, A: Allocator + Clone> FromIterator<(K, O, V)> for SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, O, V)>,
    {
        todo!()
    }
}

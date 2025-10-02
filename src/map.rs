use crate::selector::Oder;
use kuh::{Derow, Kuh};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};

pub struct SichtMap<'a, K, O, V, A = Global>
where
    K: Ord + Derow<'a> + Clone,
    O: Ord + Derow<'a> + Clone,
    A: Allocator + Clone,
{
    pub(crate) map: BTreeMap<Oder<'a, K, O>, V, A>,
}

impl<'a, K, O, V> SichtMap<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone,
    O: Ord + Derow<'a> + Clone,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<'a, K, O, V, A> SichtMap<'a, K, O, V, A>
where
    K: Ord + Derow<'a, Target: Ord + PartialEq<K>> + Clone,
    O: Ord + Derow<'a, Target: Ord + PartialEq<O>> + Clone,

    A: Allocator + Clone,
{
    pub fn get_with_base_key(&self, key: &K) -> Option<&V> {
        self.map
            .iter()
            .find(|(Oder { left: l, right: _ }, _)| matches!(l, Some(k) if k.derow() == key))
            .map(|(_, v)| v)
    }

    pub fn get_with_outer_key(&self, key: &O) -> Option<&V> {
        self.map.iter().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if k.derow() == key)
        }).map(|(_, v)| v)
    }

    pub fn get_with_outer_key_mut(&mut self, key: &'a O) -> Option<&mut V> {
        self.map
            .iter_mut()
            .find(
                |(Oder { left: _, right: r }, _)| matches!(r, Some(k) if *k == Kuh::Borrowed(key.derow())),
            )
            .map(|(_, v)| v)
    }

    pub fn get_with_both_keys(&mut self, key: &Oder<'a, K, O>) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_with_both_keys_mut(&mut self, key: &Oder<'a, K, O>) -> Option<&mut V> {
        self.map.get_mut(key)
    }
    pub fn insert_with_both_keys(&mut self, key: K, cokey: O, value: V) {
        self.map.insert(Oder::new(key, cokey), value);
    }

    pub fn insert_with_cokey(&mut self, cokey: O, value: V) {
        self.map.insert(Oder::new_right(cokey), value);
    }

    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, Oder<'a, K, O>, V> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<'a, K, O, V, A> Debug for SichtMap<'a, K, O, V, A>
where
    K: Ord + Derow<'a, Target: Debug + Ord + PartialEq<K>> + Debug + Clone,
    O: Ord + Derow<'a, Target: Debug + Ord + PartialEq<O>> + Debug + Clone,
    V: Debug,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K, O, V, A> Clone for SichtMap<'a, K, O, V, A>
where
    K: Ord + Derow<'a, Target: Clone>  + Clone,
    O: Ord + Derow<'a, Target: Clone>  + Clone,
    V: Clone,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<'a, K, O, V> Default for SichtMap<'a, K, O, V>
where
    K: Ord + Derow<'a> + Clone + Default,
    O: Ord + Derow<'a> + Clone + Default,
{
    fn default() -> Self {
        SichtMap::new()
    }
}

impl<'a, K, O, V> Serialize for SichtMap<'a, K, O, V>
where
    K: Serialize + Ord + Derow<'a, Target = K> + Clone,
    O: Serialize + Ord + Derow<'a, Target = O> + Clone,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_map(self.iter())
    }
}

impl<'de, K, O, V> Deserialize<'de> for SichtMap<'de, K, O, V>
where
    K: Deserialize<'de> + Ord + Derow<'de, Target: Ord> + Clone + 'de,
    O: Deserialize<'de> + Ord + Derow<'de, Target: Ord> + Clone + 'de,
    V: Deserialize<'de>,
    &'de <K as Derow<'de>>::Target: Derow<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: BTreeMap<Oder<'de, K, O>, V> = BTreeMap::deserialize(deserializer)?;
        Ok(Self { map })
    }
}

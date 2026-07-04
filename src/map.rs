use crate::Diplopie;
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};

pub struct SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    pub(crate) map: BTreeMap<K, V>,
    lookup: Diplopie<K, O>,
}

impl<K, O, V> SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    #[must_use]
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
            lookup: Diplopie::default(),
        }
    }

    #[must_use]
    pub fn with_fields(map: BTreeMap<K, V>, lookup: Diplopie<K, O>) -> Self {
        Self { map, lookup }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.get_with_base_key(key)
    }

    pub fn get_with_base_key(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_with_outer_key(&self, key: &O) -> Option<&V> {
        let base_key = self.lookup.get_urbild(key)?;
        self.get_with_base_key(base_key)
    }

    pub fn insert(&mut self, key: K, cokey: O, value: V) {
        self.insert_with_both_keys(key, cokey, value);
    }

    pub fn insert_with_both_keys(&mut self, key: K, cokey: O, value: V) {
        self.lookup.insert(key.clone(), cokey);
        self.map.insert(key, value);
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    #[must_use]
    pub fn lookup(&self) -> &Diplopie<K, O> {
        &self.lookup
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K, O, V> Debug for SichtMap<K, O, V>
where
    K: Ord + Debug + Clone,
    O: Ord + Debug + Clone,
    V: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, O, V> Clone for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Clone,
{
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            lookup: self.lookup.clone(),
        }
    }
}
impl<K, O, V> Default for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn default() -> Self {
        SichtMap::new()
    }
}
pub trait RetrieveCokey {
    type Key: Ord + Clone;
    type Cokey: Ord + Clone;
    fn retrieve_cokey(&self, key: &Self::Key) -> Option<&Self::Cokey>;
}
impl<K, O, V> RetrieveCokey for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: RetrieveCokey<Key = K, Cokey = O>,
{
    type Key = K;
    type Cokey = O;
    fn retrieve_cokey(&self, key: &K) -> Option<&<V as RetrieveCokey>::Cokey> {
        self.get(key)?.retrieve_cokey(key)
    }
}

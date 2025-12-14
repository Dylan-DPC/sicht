use crate::selector::Oder;
use serde::{Deserialize, Deserializer};
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use kuh::Derow;

pub struct SichtMap<K, O, V, A = Global>
where
    K: Ord + Clone,
    O: Ord + Clone,
    A: Allocator + Clone,
{
    pub(crate) map: BTreeMap<Oder<K, O>, V, A>,
}

impl<'a, K, O, V> SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }
}

impl<'a, K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord + Clone,
    O: Ord + Clone,

    A: Allocator + Clone,
{
    pub fn get_with_base_key<L>(&self, key: &L) -> Option<&V> 
        where
            K: Derow<'a, Target = L>, 
            L: PartialEq<L> + PartialEq<K> + ?Sized,
    {
        self.map
            .iter()
            .find(|(Oder { left: l, right: _ }, _)| matches!(l, Some(k) if key == k))
            .map(|(_, v)| v)
    }

    pub fn get_with_outer_key<P>(&self, key: &P) -> Option<&V> 
        where
            O: Derow<'a, Target = P>,
            P: PartialEq<P> + PartialEq<O> + ?Sized,
    {
        self.map.iter().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if key == k)
        }).map(|(_, v)| v)
    }

    pub fn get_with_outer_key_mut<P: AsRef<O>>(&mut self, key: &P) -> Option<&mut V> 
    
        where
            O: Derow<'a, Target = P>,
            P: PartialEq<P> + PartialEq<O>,
    {
        self.map
            .iter_mut()
            .find(
                |(Oder { left: _, right: r }, _)| matches!(r, Some(k) if k == key.as_ref())
            )
            .map(|(_, v)| v)
    }

    pub fn get_with_both_keys(&mut self, key: &K, other: &O) -> Option<&V> {
        let oder = Oder::new(key.to_owned(), other.to_owned());
        let result = self.map.get(&oder);
        result

    }

    pub fn get_with_both_keys_mut(&mut self, key: &Oder<K, O>) -> Option<&mut V> {
        self.map.get_mut(key)
    }
    
    pub fn insert_with_both_keys (&mut self, key: K, cokey: O, value: V) 
    {
        self.map.insert(Oder::new(key, cokey), value);
    }

    pub fn insert_with_cokey(&mut self, cokey: O, value: V) 
        where
    {
        self.map.insert(Oder::new_right(cokey), value);
    }

    pub fn contains_both_keys<L, P>(&self, key: &K, cokey: &O) -> bool 
        where
            K: Borrow<L>,
            O: Borrow<P>,
    {
        let oder = Oder::new(key.to_owned(), cokey.to_owned());
        self.map.contains_key(&oder)

    }
    


    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, Oder<K, O>, V> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<'a, K, O, V, A> Debug for SichtMap<K, O, V, A>
where
    K: Ord + Debug + Clone,
    O: Ord + Debug + Clone,
    V: Debug,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K, O, V, A> Clone for SichtMap<K, O, V, A>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Clone,
    Oder<K, O>: Clone,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        Self { map: self.map.clone() }
    }
}
impl<'a, K, O, V> Default for SichtMap<K, O, V>
where
    K: Ord + Clone + Default,
    O: Ord + Clone + Default,
{
    fn default() -> Self {
        SichtMap::new()
    }
}

impl<'de, K, O, V> Deserialize<'de> for SichtMap<K, O, V>
where
    K: Deserialize<'de> + Ord + Clone + 'de,
    O: Deserialize<'de> + Ord + Clone + 'de,
    V: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let map: BTreeMap<Oder<K, O>, V> = BTreeMap::deserialize(deserializer)?;
        Ok(Self { map })
    }
}

impl<'a, K, O, V> FromIterator<(Oder<K, O>, V)> for SichtMap<K, O, V> 
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self 
        where
            T: IntoIterator<Item = (Oder<K, O>, V)> {
                Self {
                    map: BTreeMap::from_iter(iter)
                }
    }
    }

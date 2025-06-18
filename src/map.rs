use crate::selector::Oder;
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};
use std::borrow::Cow;
use kuh::{Kuh, KuhnvertOwned, KuhnvertBorrowed};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SichtMap<'a, K, O, V, A = Global>
where
    K: Ord + KuhnvertOwned<To = K> + KuhnvertBorrowed<To = K>,
    O: Ord + KuhnvertOwned<To = O> + KuhnvertBorrowed<To = O>, 
    A: Allocator + Clone,
{
    map: BTreeMap<Oder<'a, K, O>, V, A>,
}

pub struct SichtSet<'a, K, O, A = Global>
where
    K: Ord + KuhnvertOwned<To = K>+ KuhnvertBorrowed<To = K>,
    O: Ord + KuhnvertOwned<To = O>+ KuhnvertBorrowed<To = O>,
    A: Allocator + Clone,
{
    map: SichtMap<'a, K, O, (), A>,
}

impl<'a, K, O, V> SichtMap<'a, K, O, V>
where
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K> + KuhnvertBorrowed<To = K>+ 'a,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O> + KuhnvertBorrowed<To = O>+ 'a,
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
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K>,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O>,
    A: Allocator + Clone,
{

     pub fn get_with_base_key(&'a self, key: Kuh<'a, K>) -> Option<&'a V> 
        where 
            K: 'a,
     {
        self.map.iter().find(|(Oder { left: l, right: _}, _)| {
            matches!(l, Some(k) if *k == key)
        }).map(|(_, v)| v)
    }



    pub fn get_with_outer_key(&self, key: &O) -> Option<&V> {
        self.map.iter().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if key == k.as_ref() )
        }).map(|(_, v)| v)
    }

    pub fn get_with_outer_key_mut(&mut self, key: &O) -> Option<&mut V> {
        self.map.iter_mut().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if key == k.as_ref())
        }).map(|(_, v)| v)
    }       

    pub fn get_with_both_keys(&self, key: &Oder<'a, K, O>) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_with_both_keys_mut(&mut self, key: &Oder<'a, K, O>) -> Option<&mut V> {
        self.map.get_mut(key)
    }
    pub fn insert_with_both_keys(&mut self, key: &'a K, cokey: &'a O, value: V) {
        self.map.insert(Oder::new(Kuh::Borrowed(key), Kuh::Borrowed(cokey)), value);
    }


    pub fn iter(&self) -> Iter<'_, Oder<'a, K, O>, V> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }


    pub fn contains_both_keys<Q, R>(&self, key: Kuh<'_, K>, other: Kuh<'_, O>) -> bool 
        where
            K: AsRef<Q> + KuhnvertOwned<To: KuhnvertBorrowed<To = K>> + KuhnvertBorrowed<To = K>,
            O: AsRef<Q> + KuhnvertOwned<To: KuhnvertBorrowed<To = O>> + KuhnvertBorrowed<To = O>,

    {
        self.get_with_both_keys(&Oder::new(key, other)).is_some()
    }

}

impl<K, O, V, A> Debug for SichtMap<'_, K, O, V, A>
where
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K> + Debug,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O> + Debug,
    V: Debug,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<'a, K: Ord, O, V, A> Clone for SichtMap<'a, K, O, V, A>
where
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K>,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O>,
    A: Allocator + Clone,
    BTreeMap<Oder<'a, K, O>, V, A>: Clone,
{
    fn clone(&self) -> Self {
        SichtMap { map: self.map.clone() }
    }
}
impl<K, O, V> Default for SichtMap<'_, K, O, V>
where
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K> + Default,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O> + Default,
{
    fn default() -> Self {
        SichtMap::new()
    }
}

impl<'a, K, O, V> FromIterator<(Oder<'a, K, O>, V)> for SichtMap<'a, K, O, V>
where
    K: Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K>,
    O: Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Oder<'a, K, O>, V)>,
    {
        let map = BTreeMap::from_iter(iter);
        SichtMap { map: map }
    }
}
impl<K, O, V> Serialize for SichtMap<'_, K, O, V> 
where
    K: Serialize + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K> + Ord,
    O: Serialize + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O> + Ord,
    V: Serialize
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where
            S: Serializer
      {
          serializer.collect_map(self)

      }
  }

impl<'de, 'a, K, O, V> Deserialize<'de> for SichtMap<'a, K, O, V> 
where
    K: Deserialize<'de> + Ord + KuhnvertOwned<To=K> + KuhnvertBorrowed<To=K>,
    O: Deserialize<'de> + Ord + KuhnvertOwned<To=O> + KuhnvertBorrowed<To=O>,
    &'a K: Deserialize<'de>,
    &'a O: Deserialize<'de>,
    V: Deserialize<'de>,
    'de: 'a,
    'a: 'de,
  {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where
            D: Deserializer<'de>
      {
          let map: BTreeMap<Oder<'a, K,O>, V> = BTreeMap::deserialize(deserializer)?;
          Ok(Self { 
              map
          })
      }
  }


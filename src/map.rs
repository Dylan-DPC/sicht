use crate::selector::Oder;
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};
use serde::{Serialize, Deserialize, Serializer, Deserializer};

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

     pub fn get_with_base_key(&self, key: &K) -> Option<&V> {
        self.map.iter().find(|(Oder { left: l, right: _}, _)| {
            matches!(l, Some(k) if k == key)
        }).map(|(_, v)| v)
    }



    pub fn get_with_outer_key(&self, key: &O) -> Option<&V> {
        self.map.iter().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if k == key)
        }).map(|(_, v)| v)
    }

    pub fn get_with_outer_key_mut(&mut self, key: &O) -> Option<&mut V> {
        self.map.iter_mut().find(|(Oder { left: _, right: r}, _)| {
            matches!(r, Some(k) if k == key)
        }).map(|(_, v)| v)
    }       

    pub fn get_with_both_keys(&mut self, key: &Oder<K, O>) -> Option<&V> {
        self.map.get(key)
    }

    pub fn get_with_both_keys_mut(&mut self, key: &Oder<K, O>) -> Option<&mut V> {
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

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
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
impl<K, O, V> Serialize for SichtMap<K, O, V> 
where
    K: Serialize + Ord,
    O: Serialize + Ord,
    V: Serialize
  {
      fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
        where
            S: Serializer
      {
          serializer.collect_map(self)

      }
  }

impl<'de, K, O, V> Deserialize<'de> for SichtMap<K, O, V> 
where
    K: Deserialize<'de> + Ord,
    O: Deserialize<'de> + Ord,
    V: Deserialize<'de>
  {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where
            D: Deserializer<'de>
      {
          let map: BTreeMap<Oder<K,O>, V> = BTreeMap::deserialize(deserializer)?;
          Ok(Self { 
              map
          })
      }
  }


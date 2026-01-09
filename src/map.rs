use crate::Diplopie;
use serde::{de::Visitor, Deserialize, Deserializer};
use std::alloc::{Allocator, Global};
use std::collections::{btree_map::Iter, BTreeMap};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct SichtMap<K, O, V, A = Global>
where
    K: Ord + Clone,
    O: Ord + Clone,
    A: Allocator + Clone,
{
    pub(crate) map: BTreeMap<K, V, A>,
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
}

impl<K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord + Clone,
    O: Ord + Clone,

    A: Allocator + Clone,
{
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

    pub fn insert_with_both_keys(&mut self, key: K, cokey: O, value: V) {
        self.lookup.insert(key.clone(), cokey);
        self.map.insert(key, value);
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.map.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<K, O, V, A> Debug for SichtMap<K, O, V, A>
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

impl<K, O, V, A> Clone for SichtMap<K, O, V, A>
where
    K: Ord + Clone,
    O: Ord + Clone,
    V: Clone,
    A: Allocator + Clone,
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
        struct MapVisitor<K, O, V> {
            map: PhantomData<(K, O, V)>,
            _lookup: (),
        }

        impl<K, O, V> MapVisitor<K, O, V> {
            fn new() -> Self {
                Self {
                    map: PhantomData,
                    _lookup: (),
                }
            }
        }

        impl<K, O, V> Visitor<'_> for MapVisitor<K, O, V>
        where
            K: Clone + Ord,
            O: Clone + Ord,
        {
            type Value = SichtMap<K, O, V>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_struct("Carriage", &["map", "lookup"], MapVisitor::new())
    }
}

impl<K, O, V> FromIterator<(K, O, V)> for SichtMap<K, O, V>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, O, V)>,
    {
        let (map, lookup) = iter.into_iter().fold(
            (BTreeMap::<K, V>::default(), Diplopie::default()),
            |(mut map, mut lookup), (key, cokey, item)| {
                lookup.insert(key.clone(), cokey);
                map.insert(key, item);

                (map, lookup)
            },
        );

        Self::with_fields(map, lookup)
    }
}

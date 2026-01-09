use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diplopie<K, O>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    bild: BTreeMap<K, O>,
    urbild: BTreeMap<O, K>,
}

impl<K, O> Diplopie<K, O>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    #[must_use]
    pub fn init(map: BTreeMap<K, O>) -> Self {
        let urbild = map.iter().map(|(k, o)| (o.clone(), k.clone())).collect();
        Self { bild: map, urbild }
    }

    pub fn get<Q>(&self, key: &K) -> Option<&O>
    where
        K: Borrow<Q>,
    {
        self.get_bild(key)
    }

    pub fn get_bild(&self, bild: &K) -> Option<&O> {
        self.bild.get(bild)
    }

    pub fn get_urbild(&self, urbild: &O) -> Option<&K> {
        self.urbild.get(urbild)
    }

    pub fn insert(&mut self, bild: K, urbild: O) {
        self.bild.insert(bild.clone(), urbild.clone());
        self.urbild.insert(urbild, bild);
    }

    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, K, O> {
        self.bild.iter()
    }
}

impl<K, O> Debug for Diplopie<K, O>
where
    K: Ord + Clone + Debug,
    O: Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, O> Default for Diplopie<K, O>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn default() -> Self {
        Self {
            bild: BTreeMap::default(),
            urbild: BTreeMap::default(),
        }
    }
}

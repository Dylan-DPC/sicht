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

    pub fn generate_from_iter(iter: impl Iterator<Item = (K, O)>) -> Self {
        let (bild, urbild) = iter.fold(
            (BTreeMap::default(), BTreeMap::default()),
            |(mut bild, mut urbild), (item, coitem)| {
                bild.insert(item.clone(), coitem.clone());
                urbild.insert(coitem, item);
                (bild, urbild)
            },
        );

        Self { bild, urbild }
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

impl<K, O> FromIterator<(K, O)> for Diplopie<K, O>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, O)>>(iter: I) -> Self {
        Self::generate_from_iter(iter.into_iter())
    }
}

impl<K, O> Extend<(K, O)> for Diplopie<K, O>
where
    K: Ord + Clone,
    O: Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, O)>,
    {
        iter.into_iter().for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

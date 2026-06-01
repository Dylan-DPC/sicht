use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};

/// Named after <https://en.wikipedia.org/wiki/Diplopia>
///
/// Also see <https://www.warbyparker.com/learn/od-vs-os>
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diplopia<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    od: BTreeMap<K, V>,
    os: BTreeMap<V, K>,
}

impl<K, V> Diplopia<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    #[must_use]
    pub fn init(map: BTreeMap<K, V>) -> Self {
        let os = map.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
        Self { od: map, os }
    }

    #[inline]
    pub fn get<Q>(&self, key: &K) -> Option<&V>
    where
        K: Borrow<Q>,
    {
        self.get_od(key)
    }

    pub fn get_od(&self, key: &K) -> Option<&V> {
        self.od.get(key)
    }

    pub fn get_os(&self, value: &V) -> Option<&K> {
        self.os.get(value)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.od.insert(key.clone(), value.clone());
        self.os.insert(value, key);
    }

    #[allow(clippy::iter_without_into_iter)]
    pub fn iter(&self) -> Iter<'_, K, V> {
        self.od.iter()
    }

    pub fn generate_from_iter(iter: impl Iterator<Item = (K, V)>) -> Self {
        let (od, os) = iter.fold(
            (BTreeMap::default(), BTreeMap::default()),
            |(mut normal, mut reverse), (item, coitem)| {
                normal.insert(item.clone(), coitem.clone());
                reverse.insert(coitem, item);
                (normal, reverse)
            },
        );

        Self { od, os }
    }
}

impl<K, V> Debug for Diplopia<K, V>
where
    K: Ord + Clone + Debug,
    V: Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> Default for Diplopia<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    fn default() -> Self {
        Self {
            od: BTreeMap::default(),
            os: BTreeMap::default(),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for Diplopia<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Self::generate_from_iter(iter.into_iter())
    }
}

impl<K, V> Extend<(K, V)> for Diplopia<K, V>
where
    K: Ord + Clone,
    V: Ord + Clone,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        iter.into_iter().for_each(move |(k, v)| {
            self.insert(k, v);
        });
    }
}

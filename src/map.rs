use crate::iter::DedupSortedIter;
use crate::node::Root;
use std::alloc::{Allocator, Global};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

pub struct SichtSet<K, O, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: SichtMap<K, O, (), A>,
}

pub struct SichtMap<K, O, V, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    root: Option<Root<K, O, V>>,
    length: usize,
    alloc: ManuallyDrop<A>,
    _marker: PhantomData<Box<(K, O, V), A>>,
}

impl<K, O, V, A> SichtMap<K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    fn bulk_build_from_sorted_iter<I>(iter: I, alloc: A) -> Self
    where
        K: Ord,
        I: IntoIterator<Item = (K, O, V)>,
    {
        let mut root = Root::new(alloc.clone());
        let mut length = 0;
        root.bulk_push(
            DedupSortedIter::new(iter.into_iter()),
            &mut length,
            alloc.clone(),
        );

        SichtMap { root: Some(root), length, alloc: ManuallyDrop::new(alloc), _marker: PhantomData}
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
        let mut inputs: Vec<_> = iter.into_iter().collect();
        if inputs.is_empty() {
            Self::new()
        } else {
            inputs.sort_by(|a, b| a.0.cmp(&b.0));
            Self::bulk_build_from_sorted_iter(inputs, Global)
        }
    }
}

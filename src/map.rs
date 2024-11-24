use std::alloc::{Allocator, Global};
use std::fmt::{Debug, Formatter};
use std::collections::BTreeMap;
use crate::selector::Oder;

pub struct SichtMap<'a, K, O, V, A = Global> 
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: BTreeMap<Oder<'a, K, O>, V, A>
}

pub struct SichtSet<'a, K, O, A = Global>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    map: SichtMap<'a, K, O, (), A>,
}


impl<'a, K, O, V, A> SichtMap<'a, K, O, V, A>
where
    K: Ord,
    O: Ord + Debug,
    A: Allocator + Clone,
{
    pub fn new() -> Self {
        todo!()
    }

    pub fn get(&'a self, key: &'a K) -> Option<&'a V>
    where
        K: PartialEq + Eq + PartialOrd + Ord,
    {
        self.map.get(&Oder::new_left(key)) 
    }

            
    
}

impl<'a, K, O, V, A> Debug for SichtMap<'a, K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a, K: Ord, O, V, A> Clone for SichtMap<'a, K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn clone(&self) -> Self {
        todo!()
    }
}
impl<'a, K, O, V, A: Allocator + Clone> Default for SichtMap<'a, K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn default() -> Self {
        todo!()
    }
}

impl<'a, K: Ord, O, V, A: Allocator + Clone> FromIterator<(K, O, V)> for SichtMap<'a, K, O, V, A>
where
    K: Ord,
    O: Ord,
    A: Allocator + Clone,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (K, O, V)>,
    {
       todo!() 
    }
}

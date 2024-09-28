use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub const CAPACITY: usize = 2 * 6 - 1;

pub(crate) struct NodeRef<B, K, O, V, T> {
    height: usize,
    node: NonNull<LeafNode<K, O, V>>,
    _marker: PhantomData<(B, T)>,
}

pub type Root<K, O, V> = NodeRef<Owned, K, O, V, LeafOrInternal>;

struct LeafNode<K, O, V> {
    parent: Option<NonNull<InternalNode<K, O, V>>>,
    parent_idx: MaybeUninit<u16>,
    len: u16,
    keys: [MaybeUninit<(K, O)>; CAPACITY],
    vals: [MaybeUninit<V>; CAPACITY],
}

struct InternalNode<K, O, V> {
    data: LeafNode<K, O, V>,
    edges: [MaybeUninit<NonNull<LeafNode<K, O, V>>>; 12],
}

pub enum Owned {}
pub enum LeafOrInternal {}

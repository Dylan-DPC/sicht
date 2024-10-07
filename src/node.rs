use std::alloc::Allocator;
use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ptr::NonNull;

pub const CAPACITY: usize = 2 * 6 - 1;

#[derive(Clone)]
pub(crate) struct NodeRef<B, K, O, V, T> {
    height: usize,
    node: NonNull<LeafNode<K, O, V>>,
    _marker: PhantomData<(B, T)>,
}

impl<B, K, O, V, T> NodeRef<B, K, O, V, T> {
    pub fn new<A: Allocator + Clone>(alloc: A) -> Self {
        Self::new_leaf(alloc).forget_type()
    }

    pub fn len(&self) -> usize {
        unsafe { usize::from((*Self::as_leaf_ptr(self)).len) }
    }

    pub fn as_leaf_ptr(this: &Self) -> *mut LeafNode<K, O, V> {
        this.node.as_ptr()
    }

    pub fn new_leaf<A: Allocator + Clone>(alloc: A) -> Self {
        Self::from_new_leaf(LeafNode::new(alloc))
    }

    pub fn from_new_leaf<A: Allocator + Clone>(leaf: Box<LeafNode<K, O, V>, A>) -> Self {
        NodeRef {
            height: 0,
            node: NonNull::from(Box::leak(leaf)),
            _marker: PhantomData,
        }
    }

    pub fn new_internal<A: Allocator + Clone>(child: Root<K, O, V>, alloc: A) -> Self {
        let mut new_node = unsafe { InternalNode::new(alloc) };
        new_node.edges[0].write(child.node);
        unsafe { NodeRef::from_new_internal(new_node, NonZero::new(child.height + 1).unwrap()) }
    }

    unsafe fn from_new_internal<A: Allocator + Clone>(
        internal: Box<InternalNode<K, O, V>, A>,
        height: NonZero<usize>,
    ) -> Self {
        let node = NonNull::from(Box::leak(internal)).cast();
        let mut this = NodeRef {
            height: height.into(),
            node,
            _marker: PhantomData,
        };
        this.borrow_mut().correct_all_childrens_parent_links();
        this
    }

    fn correct_all_childrens_parent_links(&mut self) {
        let len = self.len();
        unsafe { self.correct_childrens_parent_links(0..=len) }
    }

    unsafe fn correct_childrens_parent_links<I: Iterator<Item = usize>>(&mut self, range: I) {
        range.for_each(|i| {
            assert!(i <= self.len());
            unsafe { Handle::new_edge(self.reborrow_mut(), i) }.correct_parent_link();
        });
    }

    pub fn forget_type(self) -> NodeRef<B, K, O, V, T> {
        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData,
        }
    }

    pub fn force(self) -> ForceResult<NodeRef<B, K, O, V, Leaf>, NodeRef<B, K, O, V, Internal>> {
        if self.height == 0 {
            ForceResult::Leaf(NodeRef {
                height: self.height,
                node: self.node,
                _marker: PhantomData,
            })
        } else {
            ForceResult::Internal(NodeRef {
                height: self.height,
                node: self.node,
                _marker: PhantomData,
            })
        }
    }

    pub fn ascend(self) -> Result<Handle<NodeRef<B, K, O, V, Internal>, Edge>, Self> {
        let leaf_ptr = Self::as_leaf_ptr(&self);
        unsafe { (*leaf_ptr).parent }
            .as_ref()
            .map(|parent| Handle {
                node: NodeRef::from_internal(*parent, self.height + 1),
                idx: unsafe { usize::from((*leaf_ptr).parent_idx.assume_init()) },
                _marker: PhantomData,
            })
            .ok_or(self)
    }
}

impl<B, K, O, V, T> NodeRef<B, K, O, V, T> {
    pub fn last_leaf_edge(self) -> Handle<NodeRef<B, K, O, V, Leaf>, Edge> {
        let node = self;
        loop {
            match node.force() {
                ForceResult::Leaf(leaf) => return leaf.last_edge(),
                ForceResult::Internal(internal) => {
                    node = internal.last_edge().descend();
                }
            }
        }
    }

    pub fn last_edge(self) -> Handle<Self, Edge> {
        let len = self.len();
        unsafe { Handle::new_edge(self, len) }
    }
}

impl<B, K, O, V> NodeRef<B, K, O, V, Internal> {
    fn as_internal_ptr(this: &Self) -> *mut InternalNode<K, O, V> {
        this.node.as_ptr() as *mut InternalNode<K, O, V>
    }
}

impl<K, O, V> Root<K, O, V> {
    pub fn bulk_push<I, A>(&mut self, iter: I, length: &mut usize, alloc: A) -> Self
    where
        I: Iterator<Item = (K, O, V)>,
        A: Allocator + Clone,
    {
        let mut cur_node = self.borrow_mut().last_leaf_edge().node;
        iter.for_each(|(key, cokey, value)| {
            if cur_node.len() < CAPACITY {
                cur_node.push(key, value);
            } else {
                let mut open_node;
                let mut test_node = cur_node.forget_type();
                loop {
                    match test_node.ascend() {
                        Ok(parent) => {
                            let parent = parent.into_node();
                            if parent.len() < CAPACITY {
                                open_node = parent;
                                break;
                            } else {
                                test_node = parent.forget_type();
                            }
                        }
                        Err(_) => {
                            open_node = self.push_internal_level(alloc.clone());
                            break;
                        }
                    }
                }
            }
        });
    }

    pub fn push_internal_level<A: Allocator + Clone>(
        &mut self,
        alloc: A,
    ) -> NodeRef<Mut<'_>, K, O, V, Internal> {
        take_mut(self, |old_r| {
            NodeRef::new_internal(old_r, alloc).forget_type()
        });

        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData,
        }
    }
}

pub type Root<K, O, V> = NodeRef<Owned, K, O, V, LeafOrInternal>;

fn take_mut<T>(v: &mut T, change: impl FnOnce(T) -> T) {
    replace(v, |value| (change(value), ()))
}

fn replace<T, R>(v: &mut T, change: impl FnOnce(T) -> (T, R)) -> R {
    struct PanicGuard;
    impl Drop for PanicGuard {
        fn drop(&mut self) {
            std::process::abort()
        }
    }

    let guard = PanicGuard;
    let value = unsafe { std::ptr::read(v) };
    let (new_value, ret) = change(value);
    unsafe {
        std::ptr::write(v, new_value);
    }
    std::mem::forget(guard);
    ret
}

struct LeafNode<K, O, V> {
    parent: Option<NonNull<InternalNode<K, O, V>>>,
    parent_idx: MaybeUninit<u16>,
    len: u16,
    keys: [MaybeUninit<(K, O)>; CAPACITY],
    vals: [MaybeUninit<V>; CAPACITY],
}

impl<K, O, V> LeafNode<K, O, V> {
    pub fn new<A: Allocator + Clone>(alloc: A) -> Box<Self, A> {
        unsafe {
            let mut leaf = Box::new_uninit_in(alloc);
            LeafNode::init(leaf.as_mut_ptr());
            leaf.assume_init()
        }
    }

    pub unsafe fn init(this: *mut Self) {
        unsafe {
            (&raw mut (*this).parent).write(None);
            (&raw mut (*this).len).write(0);
        }
    }
}

struct InternalNode<K, O, V> {
    data: LeafNode<K, O, V>,
    edges: [MaybeUninit<NonNull<LeafNode<K, O, V>>>; 12],
}

impl<K, O, V> InternalNode<K, O, V> {
    pub fn new<A: Allocator + Clone>(alloc: A) -> Box<Self, A> {
        unsafe {
            let mut node = Box::<Self, _>::new_uninit_in(alloc);
            LeafNode::init(&raw mut (*node.as_mut_ptr()).data);
            node.assume_init()
        }
    }
}

struct Handle<N, T> {
    node: N,
    idx: usize,
    _marker: PhantomData<T>,
}

impl<N, T> Handle<N, T> {
    pub fn new(node: N, idx: usize) -> Self {
        Self {
            node,
            idx,
            _marker: PhantomData,
        }
    }
}

impl<B, K, O, V, T, H> Handle<NodeRef<B, K, O, V, T>, H> {
    pub unsafe fn new_edge(node: NodeRef<B, K, O, V, T>, idx: usize) -> Self {
        assert!(idx <= node.len());
        Handle {
            node,
            idx,
            _marker: PhantomData,
        }
    }

    pub fn descend(self) -> NodeRef<B, K, O, V, LeafOrInternal> {
        let parent_ptr = NodeRef::as_internal_ptr(&self.node);
        let node = unsafe {
            (*parent_ptr)
                .edges
                .get_unchecked(self.idx)
                .assume_init_read()
        };
        NodeRef {
            node,
            height: self.node.height - 1,
            _marker: PhantomData,
        }
    }
}

impl<T, B, K, O, V> Handle<NodeRef<B, K, O, V, Leaf>, T> {
    pub fn force(
        self,
    ) -> ForceResult<Handle<NodeRef<B, K, O, V, Leaf>, T>, Handle<NodeRef<B, K, O, V, Internal>, T>>
    {
        match self.node.force() {
            ForceResult::Leaf(node) => ForceResult::Leaf(Handle::new(node, self.idx)),
            ForceResult::Internal(node) => ForceResult::Internal(Handle::new(node, self.idx)),
        }
    }
}

#[derive(Clone)]
pub enum Owned {}
#[derive(Clone)]
pub enum LeafOrInternal {}

pub enum Internal {}
pub enum Leaf {}
pub enum Edge {}

pub enum ForceResult<L, I> {
    Leaf(L),
    Internal(I),
}

pub struct Mut<'a>(PhantomData<&'a mut ()>);

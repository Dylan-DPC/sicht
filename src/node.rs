use std::alloc::Allocator;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::num::NonZero;
use std::ptr::NonNull;
use std::slice::SliceIndex;

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

    pub fn forget_type(self) -> NodeRef<B, K, O, V, T> {
        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData,
        }
    }

    pub fn forget_me_type(self) -> NodeRef<B, K, O, V, Internal> {
        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData
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
        let mut node = self;
        loop {
            match node.force() {
                ForceResult::Leaf(leaf) => return leaf.last_edge(),
                ForceResult::Internal(internal) => {
                    node = internal.last_edge().descend();
                }
            }
        }
    }
}
impl<B, K, O, V, T> NodeRef<B, K, O, V, T> {
    pub fn last_edge(self) -> Handle<Self, Edge> {
        let len = self.len();
        unsafe { Handle::new_edge(self, len) }
    }
}

impl<'a, K: 'a, O: 'a, V: 'a, T> NodeRef<Mut<'a>, K, O, V, T> {
    unsafe fn reborrow_mut(&mut self) -> Self {
        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData,
        }
    }
}

impl<'a, K: 'a, O: 'a, V: 'a> NodeRef<Mut<'a>, K, O, V, Internal> {
    fn correct_all_childrens_parent_links(&mut self) {
        let len = self.len();
        unsafe { self.correct_childrens_parent_links(0..=len) };
    }

    unsafe fn correct_childrens_parent_links<I: Iterator<Item = usize>>(&mut self, range: I) {
        for i in range {
            assert!(i <= self.len());
            unsafe { Handle::new_edge(self.reborrow_mut(), i) }.correct_parent_link();
        }
    }
}

impl<'a, K: 'a, O: 'a, V: 'a> NodeRef<Mut<'a>, K, O, V, Leaf> {

    pub fn push(&mut self, key: K, cokey: O, val: V) -> *mut V {
        unsafe { self.push_with_handle(key, cokey, val).into_val_mut() }
    }

    pub unsafe fn push_with_handle<'b>(
        &mut self,
        key: K,
        cokey: O,
        val: V,
    ) -> Handle<NodeRef<Mut<'b>, K, O, V, Leaf>, KOV> {
        let len = self.len_mut();
        let idx = usize::from(*len);
        assert!(idx < CAPACITY);
        *len += 1;
        unsafe {
            self.key_area_mut(idx).write((key, cokey));
            self.val_area_mut(idx).write(val);
            Handle::new_kov(
                NodeRef {
                    height: self.height,
                    node: self.node,
                    _marker: PhantomData,
                },
                idx,
            )
        }
    }
}

impl<'a, K, O, V, T> NodeRef<Mut<'a>, K, O, V, T> {
    pub fn len_mut(&mut self) -> &mut u16 {
        &mut self.as_leaf_mut().len
    }

    pub fn key_area_mut<I, Op: ?Sized>(&mut self, index: I) -> &mut Op
    where
        I: SliceIndex<[MaybeUninit<(K, O)>], Output = Op>,
    {
        unsafe {
            self.as_leaf_mut()
                .keys
                .as_mut_slice()
                .get_unchecked_mut(index)
        }
    }

    pub fn val_area_mut<I, Va: ?Sized>(&mut self, index: I) -> &mut Va
    where
        I: SliceIndex<[MaybeUninit<V>], Output = Va>,
    {
        unsafe {
            self.as_leaf_mut()
                .vals
                .as_mut_slice()
                .get_unchecked_mut(index)
        }
    }

    fn as_leaf_mut(&mut self) -> &mut LeafNode<K, O, V> {
        let ptr = Self::as_leaf_ptr(self);
        unsafe { &mut *ptr }
    }

    fn into_leaf_mut(mut self) -> &'a mut LeafNode<K, O, V> {
        let ptr = Self::as_leaf_ptr(&mut self);
        unsafe { &mut *ptr }
    }
}

impl<'a, K: 'a, O: 'a, V: 'a> NodeRef<Mut<'a>, K, O, V, LeafOrInternal> {
    fn set_parent_link(&mut self, parent: NonNull<InternalNode<K, O, V>>, parent_idx: usize) {
        let leaf = Self::as_leaf_ptr(self);
        unsafe {
            (*leaf).parent = Some(parent);
            (*leaf).parent_idx.write(parent_idx as u16);
        }
    }
}

impl<B, K, O, V> NodeRef<B, K, O, V, Internal> {
    fn as_internal_ptr(this: &Self) -> *mut InternalNode<K, O, V> {
        this.node.as_ptr() as *mut InternalNode<K, O, V>
    }

    fn from_internal(node: NonNull<InternalNode<K, O, V>>, height: usize) -> Self {
        assert!(height > 0);
        NodeRef {
            height,
            node: node.cast(),
            _marker: PhantomData,
        }
    }
}

impl<'a, K: 'a, O:'a, V:'a> NodeRef<Mut<'a>, K, O, V, Internal> {
    fn push(&mut self, key: K, cokey: O, val: V, edge: Root<K, O, V>) {
        assert!(edge.height == self.height - 1);
        let len = self.len_mut();
        let idx = usize::from(*len);
        assert!(idx < CAPACITY);
        *len += 1;
        unsafe {
            self.key_area_mut(idx).write((key, cokey));
            self.val_area_mut(idx).write(val);
            self.edge_area_mut(idx + 1).write(edge.node);
            Handle::new_edge(self.reborrow_mut(), idx + 1).correct_parent_link();
        }

    }

    fn edge_area_mut<I, Op: ?Sized>(&mut self, index: I) -> &mut Op 
        where
            I: SliceIndex<[MaybeUninit<BoxedNode<K, O, V>>], Output=Op>
    {
        unsafe {
            self.as_internal_mut().edges.as_mut_slice().get_unchecked_mut(index)
        }
    }

    fn as_internal_mut(&mut self) -> &mut InternalNode<K, O, V> {
        let ptr = Self::as_internal_ptr(self);
        unsafe { &mut *ptr }
    }
}

type BoxedNode<K, O, V> = NonNull<LeafNode<K, O, V>>;

impl<K, O, V> Root<K, O, V> {
    pub fn bulk_push<I, A>(&mut self, iter: I, length: &mut usize, alloc: A)  
    where
        I: Iterator<Item = (K, O, V)>,
        A: Allocator + Clone,
    {
        let mut cur_node: NodeRef<Mut<'_>, _, _, _, Leaf> = self.borrow_mut().last_leaf_edge().into_node();
        for (key, cokey, value) in iter {
            if cur_node.len() < CAPACITY {
                cur_node.push(key, cokey, value);
            } else {
                let mut open_node: NodeRef<_, _, _, _, Internal>;
                let mut test_node = cur_node.forget_me_type();
                loop {
                    match test_node.ascend() {
                        Ok(parent) => {
                            let parent: NodeRef<Mut<'_>, _, _, _, Internal> = parent.into_node();
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
                let tree_height = open_node.height - 1;
                let mut right_tree = Root::new(alloc.clone());
                (0..tree_height).for_each(|_| {
                    right_tree.push_internal_level(alloc.clone());
                });
                open_node.push(key, cokey, value, right_tree);
                cur_node = open_node.forget_type().last_leaf_edge().into_node();
            }
            *length += 1;
        }
        self.fix_right_border_of_plentiful();
    }

    

}

impl<K, O, V> NodeRef<Owned, K, O, V, Internal> {
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

    fn new_internal<A: Allocator + Clone>(child: Root<K, O, V>, alloc: A) -> Self {
        let mut new_node = unsafe { InternalNode::new(alloc) };
        new_node.edges[0].write(child.node);
        unsafe { NodeRef::from_new_internal(new_node, NonZero::new(child.height + 1).unwrap()) }
    }
}

impl<K, O, V> NodeRef<Owned, K, O, V, LeafOrInternal> {

    pub fn ppush_internal_level<A: Allocator + Clone>(
        &mut self,
        alloc: A,
    ) -> NodeRef<Mut<'_>, K, O, V, Internal> {
        struct PanicGuard;
        impl Drop for PanicGuard {
            fn drop(&mut self) {
                std::process::abort()
            }
        }

        let guard = PanicGuard;
        let value = unsafe { std::ptr::read(self) };
            let new_value  = NodeRef::new_internal(value, alloc).forget_type();
            unsafe {
                std::ptr::write(self, new_value);
            }
            std::mem::forget(guard);


    }

    pub fn push_internal_level<A: Allocator + Clone>(
        &mut self,
        alloc: A,
    ) -> NodeRef<Mut<'_>, K, O, V, Internal> {
        take_mut(self, |old_r: NodeRef<Owned,  _, _, _, Internal>|{
            NodeRef::new_internal(old_r, alloc).forget_type()
        });

        NodeRef {
            height: self.height,
            node: self.node,
            _marker: PhantomData,
        }
    }
}

impl<K, O, V, T> NodeRef<Owned, K, O, V, T> {
    pub fn borrow_mut(&mut self) -> NodeRef<Mut<'_>, K, O, V, T> {
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

    pub fn into_node(self) -> N {
        self.node
    }
}

impl<B, K, O, V, T> Handle<NodeRef<B, K, O, V, T>, Edge> {
    pub unsafe fn new_edge(
        node: NodeRef<B, K, O, V, T>,
        idx: usize,
    ) -> Handle<NodeRef<B, K, O, V, T>, Edge> {
        assert!(idx <= node.len());
        Handle {
            node,
            idx,
            _marker: PhantomData,
        }
    }
}

impl<'a, K: 'a, O: 'a, V: 'a, T> Handle<NodeRef<Mut<'a>, K, O, V, T>, KOV> {
    pub fn into_val_mut(self) -> &'a mut V {
        assert!(self.idx < self.node.len());
        let leaf = self.node.into_leaf_mut();
        unsafe { leaf.vals.get_unchecked_mut(self.idx).assume_init_mut() }
    }
}

impl<'a, K, O, V> Handle<NodeRef<Mut<'a>, K, O, V, Internal>, Edge> {
    fn correct_parent_link(self) {
        let ptr = unsafe { NonNull::new_unchecked(NodeRef::as_internal_ptr(&self.node)) };
        let idx = self.idx;
        let mut child = self.descend();
        child.set_parent_link(ptr, idx);
    }
}

impl<T, B, K, O, V> Handle<NodeRef<B, K, O, V, LeafOrInternal>, T> {
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

impl<B, K, O, V> Handle<NodeRef<B, K, O, V, Internal>, Edge> {
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

impl<'a, B, K, O, V: 'a, T> Handle<NodeRef<B, K, O, V, T>, KOV> {
    pub unsafe fn new_kov(node: NodeRef<B, K, O, V, T>, idx: usize) -> Self {
        assert!(idx < node.len());

        Handle {
            node,
            idx,
            _marker: PhantomData,
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
pub enum KOV {}

pub enum ForceResult<L, I> {
    Leaf(L),
    Internal(I),
}

pub struct Mut<'a>(PhantomData<&'a mut ()>);



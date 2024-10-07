use std::iter::Peekable;
pub struct DedupSortedIter<K, O, V, I>
where
    I: Iterator<Item = (K, O, V)>,
{
    iter: Peekable<I>,
}

impl<K, O, V, I> DedupSortedIter<K, O, V, I>
where
    I: Iterator<Item = (K, O, V)>,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter: iter.peekable(),
        }
    }
}

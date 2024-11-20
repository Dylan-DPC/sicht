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

impl<K, O, V, I> Iterator for DedupSortedIter<K, O, V, I> 
where
    K: Eq,
    I: Iterator<Item = (K, O, V)>
{
    type Item = (K, O, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(next) = self.iter.next() else {
                return None
            };

            let Some(peeked) = self.iter.peek() else {
                return Some(next) 
            };

            if next.0 != peeked.0 {
                return Some(next)
            }
        }
    }
}


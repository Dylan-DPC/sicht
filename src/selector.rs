use serde::de::Visitor;
use serde::{Deserialize, Deserializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct Oder<E, D>
where
    E: Ord + Clone,
    D: Ord + Clone,
{
    pub left: Option<E>,
    pub right: Option<D>,
}

impl<E, D> Oder<E, D>
where
    E: Ord + Clone,
    D: Ord + Clone,
{
    
    pub fn new(left: E, right: D) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn new_left(left: E) -> Self {
        Self {
            left: Some(left),
            right: None,
        }
    }

    pub fn new_right(right: D) -> Self {
        Self {
            left: None,
            right: Some(right),
        }
    }

    pub fn is_filled(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }
}

impl<E,D> Clone for Oder<E, D>
where
    E: Ord + Clone,
    D: Ord + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<E,D> Debug for Oder<E, D>
where
    E: Ord + Clone + Debug,
    D: Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Oder")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<E,D> Default for Oder<E, D>
where
    E: Ord + Clone,
    D: Ord + Clone,
{
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

impl<E,D> PartialEq for Oder<E, D>
where
    E: Ord + Clone + PartialEq,
    D: Ord + Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.left.eq(&other.left)
    }
}
impl<E,D> Eq for Oder<E, D>
where
    E: Ord + Clone + Eq,
    D: Ord + Clone + Eq,
{
}

impl<E,D> PartialOrd<Self> for Oder<E, D>
where
    E: Ord + Clone + PartialOrd,
    D: Ord + Clone + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.left.partial_cmp(&other.left)
    }
    
}

impl<E,D> Ord for Oder<E, D>
where
    E: Ord + Clone + Ord,
    D: Ord + Clone + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.left.cmp(&other.left)
    }
}


impl<'de, E, D> Deserialize<'de> for Oder<E, D>
where
    E: Ord + Clone + 'de,
    D: Ord + Clone + 'de,
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        struct Visitoder<E, D> {
            _e: PhantomData<E>,
            _d: PhantomData<D>,
        }

        impl<E,D> Visitoder<E, D> {
            fn new() -> Self {
                Self {
                    _e: PhantomData,
                    _d: PhantomData,
                }
            }
        }

        impl<'de, E, D> Visitor<'de> for Visitoder<E, D>
        where
            E: Ord + Clone + 'de,
            D: Ord + Clone + 'de,
        {
            type Value = Oder<E, D>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_struct("Oder", &["left", "right"], Visitoder::new())
    }
}

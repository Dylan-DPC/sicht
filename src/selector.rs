use kuh::{Derow, Kuh};
use serde::de::Visitor;
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

pub struct Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a>,
    D: Ord + Clone + Derow<'a>,
{
    pub left: Option<Kuh<'a, E>>,
    pub right: Option<Kuh<'a, D>>,
}

impl<'a, E, D> Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a>,
    D: Ord + Clone + Derow<'a>,
{
    
    pub fn new(left: E, right: D) -> Self {
        Self::new_with_kuh(Kuh::Owned(left), Kuh::Owned(right))
    }

    pub fn new_with_kuh(left: Kuh<'a, E>, right: Kuh<'a, D>) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
        }
    }

    pub fn new_left(left: E) -> Self {
        Self {
            left: Some(Kuh::Owned(left)),
            right: None,
        }
    }

    pub fn new_right(right: D) -> Self {
        Self {
            left: None,
            right: Some(Kuh::Owned(right)),
        }
    }
}

impl<'a, E,D> Clone for Oder<'a, E, D>
where
    E: Ord +  Derow<'a, Target: Clone> + Clone,
    D: Ord + Derow<'a, Target: Clone> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            left: self.left.clone(),
            right: self.right.clone(),
        }
    }
}

impl<'a, E,D> Debug for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a, Target: Debug + Ord> + Debug,
    D: Ord + Clone + Derow<'a, Target: Debug + Ord> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Oder")
            .field("left", &self.left)
            .field("right", &self.right)
            .finish()
    }
}

impl<'a, E,D> Default for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a>,
    D: Ord + Clone + Derow<'a>,
{
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

impl<'a, E,D> PartialEq for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a, Target: PartialEq> + PartialEq,
    D: Ord + Clone + Derow<'a, Target: PartialEq> + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.left.eq(&other.left)
    }
}
impl<'a, E,D> Eq for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a, Target: Eq> + Eq,
    D: Ord + Clone + Derow<'a, Target: Eq> + Eq,
{
}

impl<'a, E,D> PartialOrd<Self> for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a, Target: PartialOrd + Eq> + PartialOrd,
    D: Ord + Clone + Derow<'a, Target: PartialOrd + Eq> + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.left.partial_cmp(&other.left)
    }
    
}

impl<'a, E,D> Ord for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a, Target: Ord> + Ord,
    D: Ord + Clone + Derow<'a, Target: Ord> + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.left.cmp(&other.left)
    }
}

impl<'a, E,D> Serialize for Oder<'a, E, D>
where
    E: Ord + Clone + Derow<'a> +Clone + Serialize,
    D: Ord + Clone + Derow<'a> +Clone + Serialize,
    <E as Derow<'a>>::Target: Serialize,
    <D as Derow<'a>>::Target: Derow<'a> +Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Oder", 2)?;
        state.serialize_field("left", &self.left)?;
        state.serialize_field("right", &self.right)?;
        state.end()
    }
}

impl<'de, E, D> Deserialize<'de> for Oder<'de, E, D>
where
    E: Ord + Clone + Derow<'de> +Deserialize<'de> + 'de,
    D: Ord + Clone + Derow<'de> +Deserialize<'de> + 'de,
{
    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error>
    where
        De: Deserializer<'de>,
    {
        struct Visitoder<E, D> {
            _e: PhantomData<E>,
            _d: PhantomData<D>,
        }

        impl<'a, E,D> Visitoder<E, D> {
            fn new() -> Self {
                Self {
                    _e: PhantomData,
                    _d: PhantomData,
                }
            }
        }

        impl<'de, E, D> Visitor<'de> for Visitoder<E, D>
        where
            E: Ord + Clone + Derow<'de> +Deserialize<'de> + 'de,
            D: Ord + Clone + Derow<'de> +Deserialize<'de> + 'de,
        {
            type Value = Oder<'de, E, D>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_struct("Oder", &["left", "right"], Visitoder::new())
    }
}

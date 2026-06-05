use crate::SichtMap;
use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt::{Display, Formatter};
use std::iter::Fuse;
use std::marker::PhantomData;

impl<'de, K, O, V> Deserialize<'de> for SichtMap<K, O, V>
where
    K: Deserialize<'de> + Ord + Clone + 'de,
    O: Deserialize<'de> + Ord + Clone + 'de,
    V: Deserialize<'de> + Ord,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MapVisitor<K, O, V> {
            map: PhantomData<(K, O, V)>,
            _lookup: (),
        }

        impl<K, O, V> MapVisitor<K, O, V> {
            fn new() -> Self {
                Self {
                    map: PhantomData,
                    _lookup: (),
                }
            }
        }

        impl<K, O, V> Visitor<'_> for MapVisitor<K, O, V>
        where
            K: Clone + Ord,
            O: Clone + Ord,
            V: Ord,
        {
            type Value = SichtMap<K, O, V>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }
        }

        deserializer.deserialize_map(MapVisitor::new())
    }
}

type First<T> = <T as Pair>::First;
type Second<T> = <T as Pair>::Second;

pub struct SichtMapDeserializer<'de, I, E>
where
    I: Iterator,
    I::Item: Pair,
{
    iter: Fuse<I>,
    value: Option<<I::Item as Pair>::Second>,
    count: usize,
    lifetime: PhantomData<&'de ()>,
    error: PhantomData<E>,
}

impl<I, E> SichtMapDeserializer<'_, I, E>
where
    I: Iterator,
    I::Item: Pair,
{
    #[allow(clippy::type_complexity)]
    pub fn next_pair(&mut self) -> Option<(First<I::Item>, Second<I::Item>)> {
        match self.iter.next() {
            Some(kv) => {
                self.count += 1;
                Some(Pair::split(kv))
            }
            None => None,
        }
    }
}

impl<'de, I, E> MapAccess<'de> for SichtMapDeserializer<'de, I, E>
where
    I: Iterator,
    I::Item: Pair,
    First<I::Item>: IntoDeserializer<'de, E>,
    Second<I::Item>: IntoDeserializer<'de, E>,
    E: serde::de::Error,
{
    type Error = E;

    fn next_key_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        match self.next_pair() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = self.value.take();
        let value = value.expect("next value called before next key");
        seed.deserialize(value.into_deserializer())
    }
}

pub trait Pair {
    type First;
    type Second;
    fn split(self) -> (Self::First, Self::Second);
}

#[derive(Debug)]
pub struct Error {}

impl Display for Error {
    fn fmt(&self, _f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(_msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

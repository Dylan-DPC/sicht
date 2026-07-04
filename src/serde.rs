use crate::SichtMap;
use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, Visitor};
use serde::ser::{SerializeMap, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::iter::Fuse;
use std::marker::PhantomData;

impl<K, O, V> Serialize for SichtMap<K, O, V>
where
    K: Serialize + Ord + Clone,
    O: Serialize + Ord + Clone,
    V: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

impl<'de, K, O, V> Deserialize<'de> for SichtMap<K, O, V>
where
    K: Deserialize<'de> + Ord + Clone + 'de,
    O: Deserialize<'de> + Ord + Clone + 'de,
    V: Deserialize<'de>,
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

        impl<'d, K, O, V> Visitor<'d> for MapVisitor<K, O, V>
        where
            K: Clone + Ord + Deserialize<'d>,
            O: Clone + Ord + Deserialize<'d>,
            V: Deserialize<'d>,
        {
            type Value = SichtMap<K, O, V>;

            fn expecting(&self, formatter: &mut Formatter<'_>) -> core::fmt::Result {
                write!(formatter, "Oder left or right are malformed")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'d>,
            {
                let mut sicht = SichtMap::default();
                while let Some(((key, cokey), value)) = map.next_entry()? {
                    sicht.insert_with_both_keys(key, cokey, value);
                }

                Ok(sicht)
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
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        todo!()
    }
}

impl std::error::Error for Error {}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

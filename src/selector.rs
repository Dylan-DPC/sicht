use serde::{Serialize, Deserialize};
use serde::de::Deserializer;
use serde::ser::{Serializer, SerializeStruct};
use kuh::{Kuh, KuhnvertOwned, KuhnvertBorrowed};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Oder<'a, E, D>
where
    E: PartialOrd + Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: Ord,
    <D as KuhnvertOwned>:: To: Ord,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,

{
    pub left: Option<Kuh<'a, E, <E as KuhnvertOwned>::To>>,
    pub right: Option<Kuh<'a, D, <D as KuhnvertOwned>::To>>,
}

impl<'a, E, D> Oder<'a, E, D>
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: Ord,
    <D as KuhnvertOwned>:: To: Ord,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,

{
    pub fn new(left: Kuh<'a, E, <E as KuhnvertOwned>::To>, right: Kuh<'a, D, <D as KuhnvertOwned>::To>) -> Self {
        Self {
            left: Some(left),
            right: Some(right),
        }
    }

}




impl<E, D> Debug for Oder<'_, E, D> 
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: Ord,
    <D as KuhnvertOwned>:: To: Ord,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,

{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Oder").finish()
    }
}
impl<E, D> Default for Oder<'_, E, D> 
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: Ord,
    <D as KuhnvertOwned>:: To: Ord,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,

{
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}

impl<'a, E, D> Serialize for Oder<'a, E, D>
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>>+ Serialize + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + Serialize + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: KuhnvertBorrowed<To=E> + Ord + Serialize,
    <D as KuhnvertOwned>:: To: KuhnvertBorrowed<To=D> + Ord + Serialize,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,
{
    fn serialize<S> (&self, serializer: S) -> Result<S::Ok, S::Error> 
    where
        S: Serializer
    {
        let mut state = serializer.serialize_struct("Oder", 2)?;
        state.serialize_field("left", &self.left)?;
        state.serialize_field("right", &self.right)?;
        state.end()
    }
}


impl<'de, E, D> Deserialize<'de> for Oder<'de, E, D> 
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>>+ Deserialize<'de> + 'de + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + Deserialize<'de> + 'de + KuhnvertBorrowed<To=D>,
    <E as KuhnvertOwned>:: To: Ord,
    <D as KuhnvertOwned>:: To: Ord,
    <E as KuhnvertBorrowed>:: To : Ord,
    <D as KuhnvertBorrowed>:: To : Ord,

{

    fn deserialize<De>(deserializer: De) -> Result<Self, De::Error> 
    where
        De: Deserializer<'de>,
    {
        let (left, right) = <(E, D)>::deserialize(deserializer)?;
        Ok(Self {
            left: Some(Kuh::Owned(left)),
            right: Some(Kuh::Owned(right)),
        })
    }
}


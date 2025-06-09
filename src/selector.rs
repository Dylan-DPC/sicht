use serde::{Serialize, Deserialize};
use serde::de::Deserializer;
use kuh::{Kuh, KuhnvertOwned, KuhnvertBorrowed};
use std::fmt::{Debug, Formatter};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Oder<'a, E, D>
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
{
    pub left: Option<Kuh<'a, E, <E as KuhnvertOwned>::To>>,
    pub right: Option<Kuh<'a, D, <D as KuhnvertOwned>::To>>,
}

impl<'a, E, D> Oder<'a, E, D>
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
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
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
    }
}
impl<E, D> Default for Oder<'_, E, D> 
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>> + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + KuhnvertBorrowed<To=D>,
{
    fn default() -> Self {
        Self {
            left: None,
            right: None,
        }
    }
}


impl<'de, E, D> Deserialize<'de> for Oder<'de, E, D> 
where
    E: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=E>>+ Deserialize<'de> + 'de + KuhnvertBorrowed<To=E>,
    D: Ord + KuhnvertOwned<To: KuhnvertBorrowed<To=D>> + Deserialize<'de> + 'de + KuhnvertBorrowed<To=D>,

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


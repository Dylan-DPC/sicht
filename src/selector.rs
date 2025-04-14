use serde::{Serialize, Deserialize};
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Oder<E, D>
where
    E: Ord,
    D: Ord,
{
    pub left: Option<E>,
    pub right: Option<D>,
}

impl<E, D> Oder<E, D>
where
    E: Ord,
    D: Ord,
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
            left : None,
            right: Some(right),
        }
    }
}
/*
impl<'de, E, D> Deserialize<'de> for Oder<E, D> {
where
    E: Deserialize<'de> + Ord,
    D: Deserialize<'de> + Ord,
  {
      fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
        where
            D: Deserializer<'de>
      {
          Ok(Self { 
            left: E::deserialize(deserializer),
            right: D::deserialize(deserializer),
          })
      }
  }
}
*/

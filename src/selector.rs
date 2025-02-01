#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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
            left: None,
            right: Some(right),
        }
    }
}



use super::super::constraints::{BoundaryError, MinMax};

pub enum ModelErrors<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Boundary(BoundaryError<T>),
}

impl<T> From<BoundaryError<T>> for ModelErrors<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: BoundaryError<T>) -> Self {
        Self::Boundary(value)
    }
}

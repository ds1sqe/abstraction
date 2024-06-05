use super::{min_max::MinMax, Limit};

#[derive(Debug)]
pub enum BoundaryError<T>
where
    T: MinMax + PartialOrd + Copy,
{
    FixedPoint(T),
    InvalidLimits { top: Limit<T>, bottom: Limit<T> },
    CannotCmp { top: Limit<T>, bottom: Limit<T> },
}

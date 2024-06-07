mod boundary;
mod compare;
mod fixed;
mod min_max;
mod relation;

use std::ops::{Add, Mul};

use self::boundary::BoundaryCheckError;
pub use self::boundary::{Boundary, BoundaryCheckResult, BoundaryError, Limit};
pub use self::fixed::Fixed;
use self::fixed::{FixedCheckError, FixedCheckResult};
pub use self::min_max::MinMax;
pub use self::relation::Linear;
use self::relation::{LinearCheckError, LinearCheckResult};

pub use compare::Compare;

#[derive(Debug)]
pub enum SingleConstrain<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Boundary(Boundary<T>),
    Fixed(Fixed<T>),
}

#[derive(Debug)]
pub enum DoubleConstrain<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    LinearRelation(Linear<M, O>),
}

pub enum SingleConstrainCheckError<T>
where
    T: MinMax + PartialOrd + Copy,
{
    BoundaryErr(BoundaryCheckError<T>),
    FixedErr(FixedCheckError<T>),
}

pub enum SingleConstrainCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Ok,
    Err(SingleConstrainCheckError<T>),
}

impl<T> From<SingleConstrainCheckError<T>> for SingleConstrainCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: SingleConstrainCheckError<T>) -> Self {
        Self::Err(value)
    }
}

impl<T> From<BoundaryCheckResult<T>> for SingleConstrainCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: BoundaryCheckResult<T>) -> Self {
        match value {
            BoundaryCheckResult::Ok => Self::Ok,
            BoundaryCheckResult::Err(e) => Self::Err(SingleConstrainCheckError::BoundaryErr(e)),
        }
    }
}

impl<T> From<FixedCheckResult<T>> for SingleConstrainCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: FixedCheckResult<T>) -> Self {
        match value {
            FixedCheckResult::Ok => Self::Ok,
            FixedCheckResult::Err(e) => Self::Err(SingleConstrainCheckError::FixedErr(e)),
        }
    }
}

impl<T> SingleConstrain<T>
where
    T: MinMax + PartialOrd + Copy,
{
    pub fn check(&self, value: &T) -> SingleConstrainCheckResult<T> {
        match self {
            SingleConstrain::Boundary(b) => b.is_in(*value).into(),
            SingleConstrain::Fixed(f) => f.is_in(value).into(),
        }
    }
}

pub enum DoubleConstrainCheckError<T, M, O>
where
    T: MinMax + Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    M: Mul + Copy,
    O: Add + Copy,
{
    Linear(LinearCheckError<T, M, O>),
}

impl<T, M, O> From<LinearCheckError<T, M, O>> for DoubleConstrainCheckError<T, M, O>
where
    T: MinMax + Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    M: Mul + Copy,
    O: Add + Copy,
{
    fn from(value: LinearCheckError<T, M, O>) -> Self {
        Self::Linear(value)
    }
}

pub enum DoubleConstrainCheckResult<T, M, O>
where
    T: MinMax + Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    M: Mul + Copy,
    O: Add + Copy,
{
    Ok,
    Err(DoubleConstrainCheckError<T, M, O>),
}

impl<T, M, O> From<LinearCheckResult<T, M, O>> for DoubleConstrainCheckResult<T, M, O>
where
    T: MinMax + Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    M: Mul + Copy,
    O: Add + Copy,
{
    fn from(value: LinearCheckResult<T, M, O>) -> Self {
        match value {
            LinearCheckResult::Ok => Self::Ok,
            LinearCheckResult::Err(e) => Self::Err(e.into()),
        }
    }
}

impl<M, O> DoubleConstrain<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    pub fn check<T>(&self, left: &T, right: &T) -> DoubleConstrainCheckResult<T, M, O>
    where
        T: MinMax + Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    {
        match self {
            DoubleConstrain::LinearRelation(l) => l.is_in::<T>(left, right).into(),
        }
    }
}

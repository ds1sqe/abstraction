use std::ops::{Add, Mul};

use super::Compare;

/// linear equation relation
/// ( left * mul + offset ) `cmp` right
#[derive(Debug, Clone, Copy)]
pub struct Linear<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    /// id of left one
    left: usize,
    /// id of right one
    right: usize,
    /// comparison
    cmp: Compare,
    /// multiplier
    mul: Option<M>,
    /// offset
    off: Option<O>,
}

pub enum LinearCheckError<T, M, O>
where
    T: Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    M: Mul + Copy,
    O: Add + Copy,
{
    NotIn {
        formula: Linear<M, O>,
        /// value of left
        left: T,
        /// value of right
        right: T,
    },
    CannotCompare {
        formula: Linear<M, O>,
        /// value of left
        left: T,
        /// value of right
        right: T,
    },
}

pub enum LinearCheckResult<T, M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
    T: Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
{
    Ok,
    Err(LinearCheckError<T, M, O>),
}

impl<T, M, O> From<LinearCheckError<T, M, O>> for LinearCheckResult<T, M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
    T: Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
{
    fn from(value: LinearCheckError<T, M, O>) -> Self {
        Self::Err(value)
    }
}

impl<M, O> Linear<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    pub fn new(left: usize, right: usize, mul: Option<M>, off: Option<O>, cmp: Compare) -> Self {
        Self {
            left,
            right,
            cmp,
            mul,
            off,
        }
    }

    pub fn is_in<T>(&self, left: &T, right: &T) -> LinearCheckResult<T, M, O>
    where
        T: Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
    {
        let mut left_one = *left;
        if let Some(mul) = &self.mul {
            left_one = left_one * (*mul);
        }
        if let Some(add) = &self.off {
            left_one = left_one + (*add);
        }

        if let Some(cmp_result) = left_one.partial_cmp(right) {
            if self.cmp.is_in(cmp_result) {
                // the point is out of range
                LinearCheckError::NotIn {
                    formula: *self,
                    left: *left,
                    right: *right,
                }
                .into()
            } else {
                LinearCheckResult::Ok
            }
        } else {
            LinearCheckError::CannotCompare {
                formula: *self,
                left: *left,
                right: *right,
            }
            .into()
        }
    }
}

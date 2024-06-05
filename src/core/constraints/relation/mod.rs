use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};

/// linear equation relation
/// ( left * mul + offset ) `cmp` right
#[derive(Clone, Copy)]
struct Linear<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    /// id of left one
    left: usize,
    /// id of right one
    right: usize,
    /// comparison
    cmp: Ordering,
    /// allow equal?
    eq: bool,
    /// multiplier
    mul: Option<M>,
    /// offset
    off: Option<O>,
}

pub enum LinearCheckResult<M, O, T>
where
    M: Mul + Copy,
    O: Add + Copy,
    T: Copy + Mul<M, Output = T> + Add<O, Output = T> + PartialOrd,
{
    Ok,
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

impl<M, O> Linear<M, O>
where
    M: Mul + Copy,
    O: Add + Copy,
{
    pub fn is_in<T>(&self, left: &T, right: &T) -> LinearCheckResult<M, O, T>
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
            if cmp_result != self.cmp && !(cmp_result.is_eq() && self.eq) {
                // the point is out of range
                LinearCheckResult::NotIn {
                    formula: *self,
                    left: *left,
                    right: *right,
                }
            } else {
                LinearCheckResult::Ok
            }
        } else {
            LinearCheckResult::CannotCompare {
                formula: *self,
                left: *left,
                right: *right,
            }
        }
    }
}

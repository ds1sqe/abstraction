use std::{
    cmp::Ordering,
    ops::{Add, Mul},
};

/// linear equation relation
/// ( left * mul + offset ) `cmp` right
struct Linear<M, O>
where
    M: Mul,
    O: Add,
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

impl<M: Mul, O: Add> Linear<M, O> {}

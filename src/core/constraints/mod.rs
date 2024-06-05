mod boundary;
mod fixed;
mod relation;

use self::boundary::{Boundary, MinMax};

pub enum Constrain<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Boundary(Boundary<T>),
}

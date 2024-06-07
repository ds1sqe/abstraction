pub trait MinMax {
    /// provide minimum value
    fn min() -> Self;
    /// provide maximum value
    fn max() -> Self;
}

impl MinMax for i64 {
    fn min() -> Self {
        i64::MIN
    }

    fn max() -> Self {
        i64::MAX
    }
}

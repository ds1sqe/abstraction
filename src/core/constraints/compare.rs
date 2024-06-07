use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub enum Compare {
    /// Less then ( < )
    LT,
    /// Less or equal ( <= )
    LTE,
    /// Greater then ( > )
    GT,
    /// Greater or equal ( >= )
    GTE,
    /// Equal ( = )
    EQ,
}

impl Compare {
    pub fn is_in(&self, cmp: Ordering) -> bool {
        match cmp {
            Ordering::Less => match self {
                Compare::LT | Compare::LTE => true,
                Compare::GT | Compare::GTE | Compare::EQ => false,
            },
            Ordering::Equal => match self {
                Compare::LTE | Compare::GTE | Compare::EQ => true,
                Compare::LT | Compare::GT => false,
            },
            Ordering::Greater => match self {
                Compare::GT | Compare::GTE => true,
                Compare::LT | Compare::LTE | Compare::EQ => false,
            },
        }
    }
}

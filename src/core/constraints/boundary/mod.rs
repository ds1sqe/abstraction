mod errors;

use super::{min_max::MinMax, Compare};
pub use errors::BoundaryError;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Limit<T>
where
    T: MinMax + PartialOrd + Copy,
{
    /// The limit's point
    pub point: T,
    /// allow equal
    pub equal: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct Top<T>
where
    T: MinMax + PartialOrd + Copy,
{
    pub limit: Limit<T>,
}

impl<T> Default for Top<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn default() -> Self {
        Self {
            limit: Limit {
                point: T::max(),
                equal: true,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bottom<T>
where
    T: MinMax + PartialOrd + Copy,
{
    pub limit: Limit<T>,
}

impl<T> Default for Bottom<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn default() -> Self {
        Self {
            limit: Limit {
                point: T::min(),
                equal: true,
            },
        }
    }
}

impl<T> From<Limit<T>> for Top<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: Limit<T>) -> Self {
        Self { limit: value }
    }
}

impl<T> From<Limit<T>> for Bottom<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: Limit<T>) -> Self {
        Self { limit: value }
    }
}

pub enum BoundaryCheckError<T>
where
    T: MinMax + PartialOrd + Copy,
{
    TooLow { value: T, bottom: Bottom<T> },
    TooHigh { value: T, top: Top<T> },
    CannotCmp,
}

pub enum BoundaryCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Ok,
    Err(BoundaryCheckError<T>),
}

impl<T> From<BoundaryCheckError<T>> for BoundaryCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn from(value: BoundaryCheckError<T>) -> Self {
        Self::Err(value)
    }
}

#[derive(Debug)]
pub struct Boundary<T>
where
    T: MinMax + PartialOrd + Copy,
{
    id: usize,
    pub top: Option<Top<T>>,
    pub bot: Option<Bottom<T>>,
}

impl<T> Boundary<T>
where
    T: MinMax + PartialOrd + Copy,
{
    pub fn new(id: usize) -> Self {
        Self {
            id,
            top: None,
            bot: None,
        }
    }

    fn check_new_limits(bot: &Limit<T>, top: &Limit<T>) -> Result<(), BoundaryError<T>> {
        match bot.point.partial_cmp(&top.point) {
            Some(ord) => match ord {
                Ordering::Less => Ok(()),
                Ordering::Equal => {
                    if bot.equal || top.equal {
                        Err(BoundaryError::FixedPoint(bot.point))
                    } else {
                        Err(BoundaryError::InvalidLimits {
                            top: *top,
                            bottom: *bot,
                        })
                    }
                }
                Ordering::Greater => Err(BoundaryError::InvalidLimits {
                    top: *top,
                    bottom: *bot,
                }),
            },
            None => Err(BoundaryError::CannotCmp {
                top: *top,
                bottom: *bot,
            }),
        }
    }

    pub fn create(
        id: usize,
        top: Option<Limit<T>>,
        bot: Option<Limit<T>>,
    ) -> Result<Self, BoundaryError<T>> {
        if bot.is_some() && top.is_some() {
            Self::check_new_limits(bot.as_ref().unwrap(), top.as_ref().unwrap())?;
        }

        Ok(Self {
            id,
            top: top.map(|t| t.into()),
            bot: bot.map(|b| b.into()),
        })
    }

    pub fn update(&mut self, top: Limit<T>, bot: Limit<T>) -> Result<(), BoundaryError<T>> {
        Self::check_new_limits(&bot, &top)?;
        self.top = Some(top.into());
        self.bot = Some(bot.into());
        Ok(())
    }

    pub fn set_top(&mut self, top: Limit<T>) -> Result<(), BoundaryError<T>> {
        if self.bot.is_some() {
            Self::check_new_limits(&self.bot.as_ref().unwrap().limit, &top)?;
        }
        self.top = Some(top.into());

        Ok(())
    }
    pub fn set_bot(&mut self, bot: Limit<T>) -> Result<(), BoundaryError<T>> {
        if self.top.is_some() {
            Self::check_new_limits(&bot, &self.top.as_ref().unwrap().limit)?;
        }
        self.bot = Some(bot.into());

        Ok(())
    }

    pub fn is_in(&self, value: T) -> BoundaryCheckResult<T> {
        if let Some(bot) = &self.bot {
            let cmp = match bot.limit.equal {
                true => Compare::GTE,
                false => Compare::GT,
            };

            match value.partial_cmp(&bot.limit.point) {
                Some(cmp_rst) => {
                    if !cmp.is_in(cmp_rst) {
                        // if bot.limit.equal,
                        //   value < bot.limit.point
                        // else (not bot.limit.equal)
                        //   value <= bot.limit.point

                        return BoundaryCheckError::TooLow {
                            value,
                            bottom: *bot,
                        }
                        .into();
                    }
                }
                None => return BoundaryCheckError::CannotCmp.into(),
            };
        }
        if let Some(top) = &self.top {
            let cmp = match top.limit.equal {
                true => Compare::LTE,
                false => Compare::LT,
            };

            match value.partial_cmp(&top.limit.point) {
                Some(cmp_rst) => {
                    if !cmp.is_in(cmp_rst) {
                        // if top.limit.equal,
                        //   top.limit.point < value
                        // else (not top.limit.equal)
                        //   top.limit.point <= value

                        return BoundaryCheckError::TooHigh { value, top: *top }.into();
                    }
                }
                None => return BoundaryCheckError::CannotCmp.into(),
            };
        }

        BoundaryCheckResult::Ok
    }
}

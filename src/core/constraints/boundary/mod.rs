mod errors;
mod min_max;

use errors::BoundaryError;
pub use min_max::MinMax;

use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
pub struct Limit<T>
where
    T: MinMax + PartialOrd + Copy,
{
    /// The limit's point
    point: T,
    /// allow equal
    equal: bool,
}

#[derive(Debug, Clone, Copy)]
struct Top<T>
where
    T: MinMax + PartialOrd + Copy,
{
    limit: Limit<T>,
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
struct Bottom<T>
where
    T: MinMax + PartialOrd + Copy,
{
    limit: Limit<T>,
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

enum BoundaryCheckResult<T>
where
    T: MinMax + PartialOrd + Copy,
{
    Ok,
    TooLow { value: T, bottom: Bottom<T> },
    TooHigh { value: T, top: Top<T> },
}

#[derive(Debug)]
pub struct Boundary<T>
where
    T: MinMax + PartialOrd + Copy,
{
    id: usize,
    top: Option<Top<T>>,
    bot: Option<Bottom<T>>,
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
                Ordering::Greater => Ok(()),
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
                Ordering::Less => Err(BoundaryError::InvalidLimits {
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
        bot: Option<Limit<T>>,
        top: Option<Limit<T>>,
    ) -> Result<Self, BoundaryError<T>> {
        if bot.is_some() && top.is_some() {
            Self::check_new_limits(bot.as_ref().unwrap(), top.as_ref().unwrap())?;
        }

        Ok(Self {
            id,
            top: top.map(|t| t.into()),
            bot: bot.map(|t| t.into()),
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
            if bot.limit.equal {
                if value < bot.limit.point {
                    return BoundaryCheckResult::TooLow {
                        value,
                        bottom: *bot,
                    };
                }
            } else if value <= bot.limit.point {
                return BoundaryCheckResult::TooLow {
                    value,
                    bottom: *bot,
                };
            }
        }
        if let Some(top) = &self.top {
            if top.limit.equal {
                if top.limit.point < value {
                    return BoundaryCheckResult::TooHigh { value, top: *top };
                }
            } else if top.limit.point <= value {
                return BoundaryCheckResult::TooHigh { value, top: *top };
            }
        }

        BoundaryCheckResult::Ok
    }
}

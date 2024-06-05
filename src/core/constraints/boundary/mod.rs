mod errors;
mod min_max;

use errors::BoundaryError;
use min_max::MinMax;

use std::cmp::Ordering;

#[derive(Debug, Copy)]
pub struct Limit<T>
where
    T: MinMax + PartialOrd + Copy,
{
    /// The limit's point
    point: T,
    /// allow equal
    equal: bool,
}

impl<T> Clone for Limit<T>
where
    T: MinMax + PartialOrd + Copy,
{
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }

    fn clone(&self) -> Self {
        Self {
            point: self.point,
            equal: self.equal,
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
struct Boundary<T>
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

    fn check(bot: &Limit<T>, top: &Limit<T>) -> Result<(), BoundaryError<T>> {
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
            Self::check(bot.as_ref().unwrap(), top.as_ref().unwrap())?;
        }

        Ok(Self {
            id,
            top: top.map(|t| t.into()),
            bot: bot.map(|t| t.into()),
        })
    }

    pub fn update(&mut self, top: Limit<T>, bot: Limit<T>) -> Result<(), BoundaryError<T>> {
        Self::check(&bot, &top)?;
        self.top = Some(top.into());
        self.bot = Some(bot.into());
        Ok(())
    }

    pub fn set_top(&mut self, top: Limit<T>) -> Result<(), BoundaryError<T>> {
        if self.bot.is_some() {
            Self::check(&self.bot.as_ref().unwrap().limit, &top)?;
        }
        self.top = Some(top.into());

        Ok(())
    }
    pub fn set_bot(&mut self, bot: Limit<T>) -> Result<(), BoundaryError<T>> {
        if self.top.is_some() {
            Self::check(&bot, &self.top.as_ref().unwrap().limit)?;
        }
        self.bot = Some(bot.into());

        Ok(())
    }
}

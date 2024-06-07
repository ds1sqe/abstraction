mod errors;

use std::{collections::HashMap, i64};

use self::errors::ModelErrors;

use super::constraints::{
    Compare, DoubleConstrainCheckError, DoubleConstrainCheckResult, SingleConstrainCheckError,
    SingleConstrainCheckResult,
};

pub use super::constraints::{Boundary, DoubleConstrain, Fixed, Limit, Linear, SingleConstrain};

/// T : type of variable := i64
/// M : type of multiplier := i64
/// O : type of offset(adder) := i64
#[derive(Debug)]
pub struct Model {
    pub single: HashMap<usize, Vec<SingleConstrain<i64>>>,
    pub double: HashMap<(usize, usize), Vec<DoubleConstrain<i64, i64>>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            single: HashMap::new(),
            double: HashMap::new(),
        }
    }

    pub fn add_boundary(
        &mut self,
        id: usize,
        top: Option<Limit<i64>>,
        bot: Option<Limit<i64>>,
    ) -> Result<(), ModelErrors<i64>> {
        if let Some(vec) = self.single.get_mut(&id) {
            vec.push(SingleConstrain::Boundary(Boundary::create(id, top, bot)?))
        } else {
            self.single.insert(id, Vec::new());

            return self.add_boundary(id, top, bot);
        }

        Ok(())
    }

    pub fn add_fixed(&mut self, id: usize, value: i64) {
        if let Some(vec) = self.single.get_mut(&id) {
            vec.push(SingleConstrain::Fixed(Fixed::new(id, value)));
        } else {
            self.single.insert(id, Vec::new());

            self.add_fixed(id, value);
        }
    }

    pub fn add_linear(
        &mut self,
        left_id: usize,
        right_id: usize,
        mul: Option<i64>,
        off: Option<i64>,
        cmp: Compare,
    ) {
        debug_assert!(left_id < right_id);

        if let Some(vec) = self.double.get_mut(&(left_id, right_id)) {
            vec.push(DoubleConstrain::LinearRelation(Linear::new(
                left_id, right_id, mul, off, cmp,
            )));
        } else {
            self.double.insert((left_id, right_id), Vec::new());

            self.add_linear(left_id, right_id, mul, off, cmp);
        }
    }

    pub fn check_single(
        &self,
        id: usize,
        value: i64,
    ) -> Option<Vec<SingleConstrainCheckError<i64>>> {
        let mut errors: Vec<SingleConstrainCheckError<i64>> = Vec::new();
        if let Some(vec) = self.single.get(&id) {
            for c in vec.iter() {
                match c.check(&value) {
                    SingleConstrainCheckResult::Ok => continue,
                    SingleConstrainCheckResult::Err(e) => errors.push(e),
                }
            }
        }
        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
    pub fn check_double(
        &self,
        left_id: usize,
        left_value: i64,
        right_id: usize,
        right_value: i64,
    ) -> Option<Vec<DoubleConstrainCheckError<i64, i64, i64>>> {
        let mut errors = Vec::new();

        if let Some(vec) = self.double.get(&(left_id, right_id)) {
            for c in vec.iter() {
                match c.check(&left_value, &right_value) {
                    DoubleConstrainCheckResult::Ok => continue,
                    DoubleConstrainCheckResult::Err(e) => errors.push(e),
                }
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(errors)
        }
    }
}

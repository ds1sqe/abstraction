/// represent id have to equal to value

#[derive(Debug, Clone, Copy)]
pub struct Fixed<V: PartialEq + Copy> {
    pub id: usize,
    pub value: V,
}

pub enum FixedCheckError<V: PartialEq + Copy> {
    NotEqual { fixed: Fixed<V>, value: V },
}
pub enum FixedCheckResult<V: PartialEq + Copy> {
    Ok,
    Err(FixedCheckError<V>),
}

impl<V: PartialEq + Copy> Fixed<V> {
    pub fn new(id: usize, val: V) -> Self {
        Self { id, value: val }
    }

    pub fn is_in(&self, val: &V) -> FixedCheckResult<V> {
        match self.value.eq(val) {
            true => FixedCheckResult::Ok,
            false => FixedCheckResult::Err(FixedCheckError::NotEqual {
                fixed: *self,
                value: *val,
            }),
        }
    }
}

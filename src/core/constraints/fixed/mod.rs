/// represent id have to equal to value

#[derive(Clone, Copy)]
struct Fixed<V: PartialEq + Copy> {
    id: usize,
    value: V,
}

pub enum FixedCheckResult<V: PartialEq + Copy> {
    Ok,
    NotEqual { fixed: Fixed<V>, value: V },
}

impl<V: PartialEq + Copy> Fixed<V> {
    pub fn is_in(&self, val: &V) -> FixedCheckResult<V> {
        match self.value.eq(val) {
            true => FixedCheckResult::Ok,
            false => FixedCheckResult::NotEqual {
                fixed: *self,
                value: *val,
            },
        }
    }
}

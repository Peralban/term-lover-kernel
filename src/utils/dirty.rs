
pub struct Dirty<T, U = bool> {
    value: T,
    changed: U,
}

impl<T, U> Dirty<T, U> {
    pub fn new(value: T, changed: U) -> Self {
        Self {
            value,
            changed,
        }
    }

    pub fn changed(&mut self) -> &mut U {
        &mut self.changed
    }

    pub fn value(&mut self) -> &mut T {
        &mut self.value
    }
}

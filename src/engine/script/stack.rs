use crate::{CaosError, ErrorType, Result};

pub(crate) struct Stack(Vec<usize>);

impl Stack {
    pub fn new() -> Self {
        Self(vec![0])
    }

    pub fn current(&self) -> Result<usize> {
        self.0.last().map(|i| *i).ok_or_else(Self::new_blow_err)
    }

    pub fn push(&mut self, index: usize) {
        self.0.push(index)
    }

    pub fn pop(&mut self) -> Result<usize> {
        self.0.pop().ok_or_else(Self::new_blow_err)
    }

    pub fn increment_by(&mut self, steps: usize) -> Result<()> {
        self.0
            .last_mut()
            .ok_or_else(Self::new_blow_err)
            .and_then(|i: &mut usize| match i.checked_add(steps) {
                Some(j) => {
                    *i = j;
                    Ok(())
                }
                None => Err(Self::new_blow_err()),
            })
    }

    pub fn next(&mut self) -> Result<()> {
        self.increment_by(1)
    }

    fn new_blow_err() -> CaosError {
        CaosError::new(ErrorType::BlownStack)
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

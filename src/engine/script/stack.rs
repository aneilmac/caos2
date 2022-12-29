use crate::{CaosError, ErrorType, Result};

pub(super) struct Stack(Vec<usize>);

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

    pub fn increment(&mut self) -> Result<()> {
        self.0
            .last_mut()
            .ok_or_else(Self::new_blow_err)
            .and_then(|i: &mut usize| match i.checked_add(1) {
                Some(j) => {
                    *i = j;
                    Ok(())
                }
                None => Err(Self::new_blow_err()),
            })
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

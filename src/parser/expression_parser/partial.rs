use crate::ast::Anything;
use crate::{CaosError, Rule};
use pest::iterators::Pair;

pub(crate) struct Partial<'i, T> {
    pub origin: Pair<'i, Rule>,
    pub arg_parts: Vec<Anything>,
    pub target_args: usize,
    pub complete_method: Box<dyn Fn(Pair<'i, Rule>, Vec<Anything>) -> Result<T, CaosError>>,
}

impl<'i, T> Partial<'i, T> {
    /// Returns true if the partial has enough arguments to complete/unwrap.
    pub fn is_ready(&self) -> bool {
        self.arg_parts.len() == self.target_args
    }

    /// Completes/unwraps the partial.
    pub fn complete(self) -> Result<T, CaosError> {
        (self.complete_method)(self.origin, self.arg_parts)
    }
}

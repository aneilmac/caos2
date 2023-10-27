use super::{ExpressionThunk, Partial};

use crate::{ast::Anything, CaosError};
use std::vec::Vec;

/// The [ExpressionStack] is used to take multiple [ExpressionThunk] objects and attempts to
/// complete/unwrap them.
///
/// [ExpressionThunk] objects are pushed onto the ExpressionStack, and [Anything]
/// objects are returned. It may take several [ExpressionThunk] pushes before an [Anything]
/// is produced.
pub struct ExpressionStack<'i> {
    pub root: Vec<Partial<'i, Anything>>,
}

impl<'i> ExpressionStack<'i> {
    /// Creates a new empty stack
    pub fn new() -> Self {
        ExpressionStack { root: Vec::new() }
    }

    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    /// Pushes an [Anything] into the argument list of the leaf node.
    /// If there is no leaf node (that is there are no partials to try and complete left),
    /// then returns the [Anything] argument back out.
    fn try_push_partial(&mut self, a: Anything) -> Option<Anything> {
        let leaf = self.root.last_mut();
        match leaf {
            Some(t) => {
                t.arg_parts.push(a);
                None
            }
            None => Some(a),
        }
    }

    /// Takes an [ExpressionThunk] and attempts to complete it, and then
    /// tries to complete any other expressions in the chain. If the chain has not completed
    /// then returns the result of that chain, otherwise returns nothing.
    pub fn push(&mut self, thunk: ExpressionThunk<'i>) -> Result<Option<Anything>, CaosError> {
        match thunk {
            // If pushed expression is ready then add to chain or immediately return it.
            ExpressionThunk::CompletedExpression(_, a) => {
                let res = self.try_push_partial(a);
                if res.is_some() {
                    return Ok(res);
                }
            }
            // If pushed expression is partial we may complete it or push it into the chain
            // as a partial thunk.
            ExpressionThunk::PartialExpression(p) => {
                if p.is_ready() {
                    let a = p.complete()?;
                    let res = self.try_push_partial(a);
                    if res.is_some() {
                        return Ok(res);
                    }
                } else {
                    self.root.push(p);
                    return Ok(None);
                }
            }
        }

        // Iteratively attempt to complete the chain from bottom to top.
        // Loop is exited once the full chain completes or further input is required.
        loop {
            let is_ready = self.root.last().map(|p| p.is_ready()).unwrap_or(false);
            if is_ready {
                let arg = self.root.pop().unwrap().complete()?;
                let res = self.try_push_partial(arg);
                if res.is_some() {
                    return Ok(res);
                }
            } else {
                return Ok(None);
            }
        }
    }
}

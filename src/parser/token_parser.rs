use crate::{
    ast::{Agent, Anything, Float, Integer, SString, Variable},
    parser::base::{
        parse_bytestring_literal, parse_float_literal, parse_int_literal, parse_string_literal,
    },
    CaosError, Rule,
};
use pest::iterators::{Pair, Pairs};
use std::vec::Vec;

pub(crate) trait ExpressionParser {
    fn parse_thunk<'i>(pair: Pair<'i, Rule>) -> Option<ExpressionThunk<'i>>;
}

pub(crate) struct Partial<'i> {
    pub origin: Pair<'i, Rule>,
    pub arg_parts: Vec<Anything>,
    pub target_args: usize,
    pub complete_method: Box<dyn Fn(Pair<'i, Rule>, Vec<Anything>) -> Result<Anything, CaosError>>,
}

impl<'i> Partial<'i> {
    fn is_ready(&self) -> bool {
        self.arg_parts.len() == self.target_args
    }

    fn complete(self) -> Result<Anything, CaosError> {
        (self.complete_method)(self.origin, self.arg_parts)
    }
}

pub(crate) enum ExpressionThunk<'i> {
    Completed(Anything),
    Partial(Partial<'i>),
}

impl<'i> ExpressionThunk<'i> {
    fn is_ready(&self) -> bool {
        match self {
            Self::Completed(_) => true,
            Self::Partial(p) => p.is_ready(),
        }
    }

    fn complete(self) -> Result<Anything, CaosError> {
        match self {
            Self::Completed(a) => Ok(a),
            Self::Partial(p) => p.complete(),
        }
    }
}

impl From<Anything> for ExpressionThunk<'_> {
    fn from(value: Anything) -> Self {
        ExpressionThunk::Completed(value)
    }
}

struct ExpressionStack<'i> {
    root: Vec<Partial<'i>>,
}

impl<'i> ExpressionStack<'i> {
    fn new() -> Self {
        ExpressionStack { root: Vec::new() }
    }

    fn is_empty(&self) -> bool {
        self.root.is_empty()
    }

    fn last<'a: 'i>(&'a self) -> Option<&'a Partial> {
        self.root.last()
    }

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

    fn push(&mut self, thunk: ExpressionThunk<'i>) -> Result<Option<Anything>, CaosError> {
        match thunk {
            ExpressionThunk::Completed(a) => {
                let res = self.try_push_partial(a);
                if res.is_some() {
                    return Ok(res);
                }
            }
            ExpressionThunk::Partial(p) => {
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

pub fn parse_expressions<'i>(mut pairs: Pairs<'i, Rule>) -> Result<Vec<Anything>, CaosError> {
    let mut parts = Vec::<Anything>::new();
    let mut stack: ExpressionStack<'i> = ExpressionStack::new();

    while let Some(pair) = pairs.next() {
        let rule = pair.as_rule();
        let thunk: ExpressionThunk = match rule {
            Rule::literal_string => Some(parse_string_literal(pair.clone()).map(Anything::from)?.into()),
            Rule::literal_byte_string => {
                Some(parse_bytestring_literal(pair.clone()).map(Anything::from)?.into())
            }
            Rule::literal_int => Some(parse_int_literal(pair.clone()).map(Anything::from)?.into()),
            Rule::literal_float => Some(parse_float_literal(pair.clone()).map(Anything::from)?.into()),
            _ => None,
        }
        .or_else(|| Agent::parse_thunk(pair.clone()))
        .or_else(|| Float::parse_thunk(pair.clone()))
        .or_else(|| Integer::parse_thunk(pair.clone()))
        .or_else(|| SString::parse_thunk(pair.clone()))
        .or_else(|| Variable::parse_thunk(pair.clone()))
        .ok_or_else(|| CaosError::new_parse_error(pair))?;

        let res = stack.push(thunk)?;
        if let Some(a) = res {
            parts.push(a);
        }
    }

    match stack.root.last() {
        None => Ok(parts),
        Some(p) => Err(CaosError::new_arg_count_error(
            p.target_args,
            p.arg_parts.len(),
            p.origin.line_col(),
        )),
    }
}

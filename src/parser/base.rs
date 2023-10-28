#[cfg(test)]
mod tests;

use crate::{
    ast::{ByteString, Label},
    CaosError, Rule,
};
use pest::iterators::Pair;

pub fn parse_label(pair: Pair<Rule>) -> Result<Label, CaosError> {
    match pair.as_rule() {
        Rule::label => Ok(pair.as_str().to_owned().into()),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_bytestring_literal(pair: Pair<Rule>) -> Result<ByteString, CaosError> {
    use std::cmp::{max, min};

    let v: Result<Vec<u8>, CaosError> = match pair.as_rule() {
        Rule::literal_byte_string => pair
            .into_inner()
            .map(parse_int_literal)
            .map(|i| i.map(|i| min(max(i, u8::MIN.into()), u8::MAX.into()) as u8))
            .collect(),
        _ => Err(CaosError::new_parse_error(pair)),
    };
    v.map(|v| v.into())
}

pub fn parse_int_literal(pair: Pair<Rule>) -> Result<i32, CaosError> {
    match pair.as_rule() {
        Rule::literal_int => pair
            .clone()
            .into_inner()
            .next()
            .map(parse_int_inner)
            .unwrap_or(Err(CaosError::new_parse_error(pair))),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

fn parse_int_inner(pair: Pair<Rule>) -> Result<i32, CaosError> {
    match pair.as_rule() {
        Rule::int_decimal => pair
            .as_str()
            .parse::<i32>()
            .map_err(|_| CaosError::new_parse_error(pair)),
        Rule::int_char_escape => Ok('\'' as i32),
        Rule::int_char_ok => pair
            .as_str()
            .chars()
            .next()
            .map(|i| Ok(i as i32))
            .unwrap_or(Err(CaosError::new_parse_error(pair))),
        Rule::int_binary => {
            i32::from_str_radix(pair.as_str(), 2).map_err(|_| CaosError::new_parse_error(pair))
        }
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

pub fn parse_float_literal(pair: Pair<Rule>) -> Result<f32, CaosError> {
    if pair.as_rule() != Rule::literal_float {
        return Err(CaosError::new_parse_error(pair));
    }
    return pair
        .as_str()
        .parse::<f32>()
        .map_err(|_| CaosError::new_parse_error(pair));
}

pub fn parse_string_literal(pair: Pair<Rule>) -> Result<String, CaosError> {
    match pair.as_rule() {
        Rule::literal_string => pair.into_inner().map(parse_string_raw).collect(),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

fn parse_string_raw(pair: Pair<Rule>) -> Result<&str, CaosError> {
    match pair.as_rule() {
        Rule::string_raw => Ok(pair.as_str()),
        Rule::escape_newline => Ok("\n"),
        Rule::escape_quote => Ok("\""),
        Rule::escape_backslash => Ok("\\"),
        _ => Err(CaosError::new_parse_error(pair)),
    }
}

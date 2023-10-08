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

#[cfg(test)]
mod tests {
    use crate::parser::CaosParser;
    use pest::Parser;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_label_basic() {
        for p in CaosParser::parse(Rule::label, "foo").expect("Parsed") {
            assert_eq!(
                parse_label(p).expect("Parsed label"),
                String::from("foo").into()
            );
        }
        for p in CaosParser::parse(Rule::label, "foo2").expect("Parsed") {
            assert_eq!(
                parse_label(p).expect("Parsed label"),
                String::from("foo2").into()
            );
        }
        for p in CaosParser::parse(Rule::label, "foo2_foo2").expect("Parsed") {
            assert_eq!(
                parse_label(p).expect("Parsed label"),
                String::from("foo2_foo2").into()
            );
        }
    }

    #[test]
    fn test_empty_label_failure() {
        CaosParser::parse(Rule::label, "").expect_err("Empty string now allowed");
    }

    #[test]
    fn test_num_label_failure() {
        CaosParser::parse(Rule::label, "2foo").expect_err("Can not start with a number");
    }

    #[test]
    fn test_empty_string_failure() {
        CaosParser::parse(Rule::literal_string, "").expect_err("Can not be empty");
    }

    #[test]
    fn test_noquotes_string_failure() {
        CaosParser::parse(Rule::literal_string, "hello").expect_err("Can not lack quotes");
    }

    #[test]
    fn test_quotes_string() {
        for p in CaosParser::parse(Rule::literal_string, r#""hello""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hello")
            );
        }
    }

    #[test]
    fn test_newline_escape_string() {
        for p in CaosParser::parse(Rule::literal_string, r#""hel\nlo""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hel\nlo")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""\nhello""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("\nhello")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""hello\n""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hello\n")
            );
        }
    }

    #[test]
    fn test_quote_escape_string() {
        for p in CaosParser::parse(Rule::literal_string, r#""hel\"lo""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hel\"lo")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""\"hello""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("\"hello")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""HELLO\"""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("HELLO\"")
            );
        }
    }

    #[test]
    fn test_backslash_escape_string() {
        for p in CaosParser::parse(Rule::literal_string, r#""hel\\lo""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hel\\lo")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""\\hello""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("\\hello")
            );
        }
        for p in CaosParser::parse(Rule::literal_string, r#""hello\\""#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hello\\")
            );
        }
    }

    #[test]
    fn test_atomic_string() {
        for p in CaosParser::parse(Rule::literal_string, r#""hell*como"*como"#).expect("Parsed") {
            assert_eq!(
                parse_string_literal(p).expect("Parsed string"),
                String::from("hell*como")
            );
        }
    }

    #[test]
    fn test_decimal_int() {
        for p in CaosParser::parse(Rule::literal_int, r#"0"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 0);
        }
        for p in CaosParser::parse(Rule::literal_int, r#"111119"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 111119);
        }
        for p in CaosParser::parse(Rule::literal_int, r#"-0"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 0);
        }

        for p in CaosParser::parse(Rule::literal_int, r#"-2353"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), -2353);
        }
    }

    #[test]
    fn test_char_int() {
        for p in CaosParser::parse(Rule::literal_int, r#"'\''"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 0x27);
        }
        for p in CaosParser::parse(Rule::literal_int, r#"'A'"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 0x41);
        }
        for p in CaosParser::parse(Rule::literal_int, r#"' '"#).expect("Parsed") {
            assert_eq!(parse_int_literal(p).expect("Parsed string"), 0x20);
        }
    }

    #[test]
    fn test_bytestring_empty() {
        for p in CaosParser::parse(Rule::literal_byte_string, r#"[]"#).expect("Parsed") {
            assert_eq!(
                parse_bytestring_literal(p).expect("Parsed string"),
                ByteString::default()
            );
        }
    }

    #[test]
    fn test_bytestring_single() {
        for p in CaosParser::parse(Rule::literal_byte_string, r#"[5]"#).expect("Parsed") {
            assert_eq!(
                parse_bytestring_literal(p).expect("Parsed string"),
                vec![5].into()
            );
        }
        for p in CaosParser::parse(Rule::literal_byte_string, r#"[ -5 ]"#).expect("Parsed") {
            assert_eq!(
                parse_bytestring_literal(p).expect("Parsed string"),
                vec![0].into()
            );
        }
    }

    #[test]
    fn test_bytestring_multiple() {
        for p in CaosParser::parse(Rule::literal_byte_string, r#"[0 1 2 3 4]"#).expect("Parsed") {
            assert_eq!(
                parse_bytestring_literal(p).expect("Parsed string"),
                vec![0, 1, 2, 3, 4].into()
            );
        }
        for p in
            CaosParser::parse(Rule::literal_byte_string, r#"[-0 -1 -2 -3 -4]"#).expect("Parsed")
        {
            assert_eq!(
                parse_bytestring_literal(p).expect("Parsed string"),
                vec![0, 0, 0, 0, 0].into()
            );
        }
    }

    #[test]
    fn test_float_full() {
        for p in CaosParser::parse(Rule::literal_float, r#"1.2"#).expect("Parsed") {
            assert_eq!(parse_float_literal(p).expect("Parsed string"), 1.2f32);
        }
        for p in CaosParser::parse(Rule::literal_float, r#"300.692"#).expect("Parsed") {
            assert_eq!(parse_float_literal(p).expect("Parsed string"), 300.692f32);
        }
        for p in CaosParser::parse(Rule::literal_float, r#"-300.692"#).expect("Parsed") {
            assert_eq!(parse_float_literal(p).expect("Parsed string"), -300.692f32);
        }
    }

    #[test]
    fn test_float_partial() {
        for p in CaosParser::parse(Rule::literal_float, r#".2"#).expect("Parsed") {
            assert_eq!(parse_float_literal(p).expect("Parsed string"), 0.2f32);
        }
        for p in CaosParser::parse(Rule::literal_float, r#".692"#).expect("Parsed") {
            assert_eq!(parse_float_literal(p).expect("Parsed string"), 0.692f32);
        }
    }

    #[test]
    fn test_float_fail() {
        CaosParser::parse(Rule::literal_float, r#"2"#).expect_err("Int can't be treated as float");
    }
}

mod evaluators;

use super::{Anything, Decimal, IntArg, Variable};
use crate::parser::{CaosParsable, CaosParseResult};
use caos_macros::{CaosParsable, CommandList, EvaluateCommand};
use evaluators::*;
use nom::bytes::complete::tag;
use nom::{
    branch::alt,
    bytes::complete::escaped_transform,
    character::complete::none_of,
    combinator::{map, value},
    sequence::delimited,
};

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum SString {
    #[syntax(with_parser = "parse_raw")]
    Raw(String),
    #[syntax(with_parser = "parse_variable")]
    Variable(Box<Variable>),
    #[syntax]
    Catx { category_id: Box<IntArg> },
    #[syntax]
    Hand,
    #[syntax]
    Wild {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        tag_stub: Box<SString>,
        offset: Box<IntArg>,
    },
    #[syntax]
    Bkgd { metaroom_id: Box<IntArg> },
    #[syntax]
    Ptxt,
    #[syntax]
    Face,
    #[syntax(name = "dbg#")]
    Dbg { variable: Box<IntArg> },
    #[syntax]
    Dbga { variable: Box<IntArg> },
    #[syntax]
    Fvwm { name: Box<SString> },
    #[syntax]
    Innl,
    #[syntax]
    Gtos { slot: Box<IntArg> },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: Box<SString>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist mon1")]
    HistMon1 {
        moniker: Box<SString>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist mon2")]
    HistMon2 {
        moniker: Box<SString>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist name")]
    HistName { moniker: Box<SString> },
    #[syntax(name = "hist next")]
    HistNext { moniker: Box<SString> },
    #[syntax(name = "hist prev")]
    HistPrev { moniker: Box<SString> },
    #[syntax(name = "hist utxt")]
    HistUtxt {
        moniker: Box<SString>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist wnam")]
    HistWnam {
        moniker: Box<SString>,
        event_no: Box<IntArg>,
    },
    #[syntax]
    Bkds { metaroom_id: Box<IntArg> },
    #[syntax]
    Emid,
    #[syntax]
    Erid { metaroom_id: Box<IntArg> },
    #[syntax]
    Mloc { metaroom_id: Box<IntArg> },
    #[syntax]
    Rate {
        room_type: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    #[syntax]
    Rloc { room_id: Box<IntArg> },
    // Resource
    #[syntax(name = "pray agts")]
    PrayAgts {
        resource_name: Box<SString>,
        string_tag: Box<SString>,
        default_value: Box<SString>,
    },
    #[syntax(name = "pray next")]
    PrayNext {
        resource_type: Box<SString>,
        last_known: Box<SString>,
    },
    #[syntax(name = "pray prev")]
    PrayPrev {
        resource_type: Box<SString>,
        last_known: Box<SString>,
    },
    #[syntax]
    Caos {
        inline: Box<IntArg>,
        state_trans: Box<IntArg>,
        p1: Box<Anything>,
        p2: Box<Anything>,
        commands: Box<SString>,
        throws: Box<IntArg>,
        catches: Box<IntArg>,
        report: Box<Variable>,
    },
    #[syntax]
    Rmsc { x: Box<IntArg>, y: Box<IntArg> },
    #[syntax]
    Vois,
    // Date
    #[syntax]
    Rtif {
        real_time: Box<IntArg>,
        format: Box<SString>,
    },
    #[syntax]
    Gamn { previous: Box<SString> },
    #[syntax]
    Gnam,
    #[syntax]
    Read {
        catalogue_tag: Box<SString>,
        offset: Box<IntArg>,
    },
    #[syntax]
    Subs {
        value: Box<SString>,
        start: Box<IntArg>,
        count: Box<IntArg>,
    },
    #[syntax]
    Vtos { value: Box<Decimal> },
    // World
    #[syntax]
    Pswd { world_index: Box<IntArg> },
    #[syntax]
    Wnam,
    #[syntax]
    Wrld { world_index: Box<IntArg> },
    #[syntax]
    Wuid,
}

impl From<String> for SString {
    fn from(s: String) -> Self {
        SString::Raw(s)
    }
}

fn parse_variable(input: &str) -> CaosParseResult<&str, SString> {
    map(Variable::parse_caos, |v| SString::Variable(Box::new(v)))(input)
}

fn parse_escaped(input: &str) -> CaosParseResult<&str, String> {
    escaped_transform(
        none_of("\\\""),
        '\\',
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
        )),
    )(input)
}

fn parse_raw(input: &str) -> CaosParseResult<&str, SString> {
    map(
        alt((
            delimited(tag("\""), parse_escaped, tag("\"")),
            map(tag(r#""""#), |_| String::new()),
        )),
        |s| SString::Raw(s),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escaped_empty() {
        let (_, res) = parse_escaped("").expect("Valid string");
        assert_eq!(res, String::new());
    }

    #[test]
    fn test_escaped_simple() {
        let (_, res) = parse_escaped("Hello world!").expect("Valid string");
        assert_eq!(res, "Hello world!");
    }

    #[test]
    fn test_escaped_newline() {
        let (_, res) = parse_escaped("Hello\\nworld!").expect("Valid string");
        assert_eq!(res, "Hello\nworld!");
    }

    #[test]
    fn test_escaped_quote() {
        let (_, res) = parse_escaped(r#"Hello\"\\world!"#).expect("Valid string");
        assert_eq!(res, r#"Hello"\world!"#);
    }

    #[test]
    fn test_raw_delimited_empty() {
        let (_, res) = parse_raw(r#""""#).expect("Valid variable");
        assert_eq!(res, String::new().into());
    }

    #[test]
    fn test_literal_empty() {
        let (_, res) = SString::parse_caos(r#""""#).expect("Valid variable");
        assert_eq!(res, String::new().into());
    }

    #[test]
    fn test_literal_single_escape() {
        let (_, res) = SString::parse_caos(r#""\"""#).expect("Valid variable");
        assert_eq!(res, "\"".to_owned().into());
    }

    #[test]
    fn test_literal() {
        let (_, res) = SString::parse_caos(r#""Hello world!""#).expect("Valid variable");
        assert_eq!(res, "Hello world!".to_owned().into());
    }

    #[test]
    fn test_end() {
        let (_, res) = SString::parse_caos(r#""Hello " world!""#).expect("Valid variable");
        assert_eq!(res, "Hello ".to_owned().into());
    }

    #[test]
    fn test_escape() {
        let (_, res) = SString::parse_caos(r#""Hello \" world!""#).expect("Valid variable");
        assert_eq!(res, "Hello \" world!".to_owned().into());
    }
}

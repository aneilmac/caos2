use super::{Anything, Decimal, Integer, Variable};
use crate::parser::CaosParsable;
use caos_macros::{CaosParsable, CommandList};
use nom::bytes::complete::tag;
use nom::{
    bytes::complete::escaped,
    character::complete::{none_of, one_of},
    combinator::map,
    sequence::delimited,
};

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum SString {
    #[syntax(with_parser = "parse_variable")]
    Raw(String),
    #[syntax(with_parser = "parse_raw")]
    Variable(Box<Variable>),
    #[syntax]
    Catx { category_id: Box<Integer> },
    #[syntax]
    Hand,
    #[syntax]
    Wild {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
        tag_stub: Box<SString>,
        offset: Box<Integer>,
    },
    #[syntax]
    Bkgd { metaroom_id: Box<Integer> },
    #[syntax]
    Ptxt,
    #[syntax]
    Face,
    #[syntax(name = "dbg#")]
    Dbg { variable: Box<Integer> },
    #[syntax]
    Dbga { variable: Box<Integer> },
    #[syntax]
    Fvwm { name: Box<SString> },
    #[syntax]
    Innl,
    #[syntax]
    Gtos { slot: Box<Integer> },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist mon1")]
    HistMon1 {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist mon2")]
    HistMon2 {
        moniker: Box<SString>,
        event_no: Box<Integer>,
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
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist wnam")]
    HistWnam {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax]
    Bkds { metaroom_id: Box<Integer> },
    #[syntax]
    Emid,
    #[syntax]
    Erid { metaroom_id: Box<Integer> },
    #[syntax]
    Mloc { metaroom_id: Box<Integer> },
    #[syntax]
    Rate {
        room_type: Box<Integer>,
        ca_index: Box<Integer>,
    },
    #[syntax]
    Rloc { room_id: Box<Integer> },
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
        inline: Box<Integer>,
        state_trans: Box<Integer>,
        p1: Box<Anything>,
        p2: Box<Anything>,
        commands: Box<SString>,
        throws: Box<Integer>,
        catches: Box<Integer>,
        report: Box<Variable>,
    },
    #[syntax]
    Mmsc {
        x: Box<Integer>,
        y: Box<Integer>,
        track_name: Box<SString>,
    },
    #[syntax]
    Rmsc { x: Box<Integer>, y: Box<Integer> },
    #[syntax]
    Vois,
    // Date
    #[syntax]
    Rtif {
        real_time: Box<Integer>,
        format: Box<SString>,
    },
    #[syntax]
    Gamn { previous: Box<SString> },
    #[syntax]
    Gnam,
    #[syntax]
    Read {
        catalogue_tag: Box<SString>,
        offset: Box<Integer>,
    },
    #[syntax]
    Subs {
        value: Box<SString>,
        start: Box<Integer>,
        count: Box<Integer>,
    },
    #[syntax]
    Vtos { value: Box<Decimal> },
    // World
    #[syntax]
    Pswd { world_index: Box<Integer> },
    #[syntax]
    Wnam,
    #[syntax]
    Wrld { world_index: Box<Integer> },
    #[syntax]
    Wuid,
}

impl From<String> for SString {
    fn from(s: String) -> Self {
        SString::Raw(s)
    }
}

fn parse_variable(input: &str) -> nom::IResult<&str, SString> {
    map(Variable::parse_caos, |v| SString::Variable(Box::new(v)))(input)
}

fn parse_raw(input: &str) -> nom::IResult<&str, SString> {
    let es = escaped(none_of("\\"), '\\', one_of(r#"\"\\\n"#));
    let delim = delimited(tag("\""), es, tag("\""));
    map(delim, |s: &str| SString::Raw(s.to_owned()))(input)
}

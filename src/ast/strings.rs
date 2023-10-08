use super::{Anything, Decimal, IntArg, Variable};
use caos_macros::CaosParsable;

#[derive(CaosParsable, Eq, PartialEq, Debug, Clone)]
pub enum SString {
    #[syntax(with_parser = "parse_raw")]
    Literal(String),
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
        SString::Literal(s)
    }
}

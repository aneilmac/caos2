use super::{AgentArg, Anything, DecimalArg, IntArg, SStringArg, Variable};
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
        tag_stub: Box<SStringArg>,
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
    Fvwm { name: Box<SStringArg> },
    #[syntax]
    Innl,
    #[syntax]
    Gtos { slot: Box<IntArg> },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist mon1")]
    HistMon1 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist mon2")]
    HistMon2 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist name")]
    HistName { moniker: Box<SStringArg> },
    #[syntax(name = "hist next")]
    HistNext { moniker: Box<SStringArg> },
    #[syntax(name = "hist prev")]
    HistPrev { moniker: Box<SStringArg> },
    #[syntax(name = "hist utxt")]
    HistUtxt {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist wnam")]
    HistWnam {
        moniker: Box<SStringArg>,
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
        resource_name: Box<SStringArg>,
        string_tag: Box<SStringArg>,
        default_value: Box<SStringArg>,
    },
    #[syntax(name = "pray next")]
    PrayNext {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
    #[syntax(name = "pray prev")]
    PrayPrev {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
    #[syntax]
    Caos {
        inline: Box<IntArg>,
        state_trans: Box<IntArg>,
        p1: Box<Anything>,
        p2: Box<Anything>,
        commands: Box<SStringArg>,
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
        format: Box<SStringArg>,
    },
    #[syntax]
    Gamn { previous: Box<SStringArg> },
    #[syntax]
    Gnam,
    #[syntax]
    Read {
        catalogue_tag: Box<SStringArg>,
        offset: Box<IntArg>,
    },
    #[syntax]
    Subs {
        value: Box<SStringArg>,
        start: Box<IntArg>,
        count: Box<IntArg>,
    },
    #[syntax]
    Vtos { value: Box<DecimalArg> },
    // World
    #[syntax]
    Pswd { world_index: Box<IntArg> },
    #[syntax]
    Wnam,
    #[syntax]
    Wrld { world_index: Box<IntArg> },
    #[syntax]
    Wuid,
    #[syntax(name = "prt: name")]
    PrtName {
        agent: Box<AgentArg>,
        in_or_out: Box<IntArg>,
        port_index: Box<IntArg>,
    },
}

impl From<String> for SString {
    fn from(s: String) -> Self {
        SString::Literal(s)
    }
}

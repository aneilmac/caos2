use super::{AgentArg, Anything, DecimalArg, IntArg, SStringArg, Variable};

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum SString {
    Literal(String),
    Catx { category_id: Box<IntArg> },
    Hand,
    Wild {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        tag_stub: Box<SStringArg>,
        offset: Box<IntArg>,
    },
    Bkgd { metaroom_id: Box<IntArg> },
    Ptxt,
    Face,
    Dbg { variable: Box<IntArg> },
    Dbga { variable: Box<IntArg> },
    Fvwm { name: Box<SStringArg> },
    Innl,
    Gtos { slot: Box<IntArg> },
    HistFoto {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    HistMon1 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    HistMon2 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    HistName { moniker: Box<SStringArg> },
    HistNext { moniker: Box<SStringArg> },
    HistPrev { moniker: Box<SStringArg> },
    HistUtxt {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    HistWnam {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    Bkds { metaroom_id: Box<IntArg> },
    Emid,
    Erid { metaroom_id: Box<IntArg> },
    Mloc { metaroom_id: Box<IntArg> },
    Rate {
        room_type: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    Rloc { room_id: Box<IntArg> },
    // Resource
    PrayAgts {
        resource_name: Box<SStringArg>,
        string_tag: Box<SStringArg>,
        default_value: Box<SStringArg>,
    },
    PrayNext {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
    PrayPrev {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
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
    Rmsc { x: Box<IntArg>, y: Box<IntArg> },
    Vois,
    // Date
    Rtif {
        real_time: Box<IntArg>,
        format: Box<SStringArg>,
    },
    Gamn { previous: Box<SStringArg> },
    Gnam,
    Read {
        catalogue_tag: Box<SStringArg>,
        offset: Box<IntArg>,
    },
    Subs {
        value: Box<SStringArg>,
        start: Box<IntArg>,
        count: Box<IntArg>,
    },
    Vtos { value: Box<DecimalArg> },
    // World
    Pswd { world_index: Box<IntArg> },
    Wnam,
    Wrld { world_index: Box<IntArg> },
    Wuid,
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

use super::{AgentArg, Anything, DecimalArg, IntArg, SStringArg, Variable};
use crate::Rule;
use caos_macros::ExpressionParser;

#[derive(Eq, PartialEq, Debug, Clone, ExpressionParser)]
pub enum SString {
    #[parse(ignore)]
    Literal(String),
    #[parse(rule=Rule::string_catx)]
    Catx { category_id: Box<IntArg> },
    #[parse(rule=Rule::overloaded_hand)]
    Hand,
    #[parse(rule=Rule::string_wild)]
    Wild {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        tag_stub: Box<SStringArg>,
        offset: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_bkgd)]
    Bkgd { metaroom_id: Box<IntArg> },
    #[parse(rule=Rule::overloaded_ptxt)]
    Ptxt,
    #[parse(rule=Rule::overloaded_face)]
    Face,
    #[parse(rule=Rule::string_dbg)]
    Dbg { variable: Box<IntArg> },
    #[parse(rule=Rule::string_dbga)]
    Dbga { variable: Box<IntArg> },
    #[parse(rule=Rule::string_fvwm)]
    Fvwm { name: Box<SStringArg> },
    #[parse(rule=Rule::string_innl)]
    Innl,
    #[parse(rule=Rule::string_gtos)]
    Gtos { slot: Box<IntArg> },
    #[parse(rule=Rule::overloaded_hist_foto)]
    HistFoto {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::string_hist_mon1)]
    HistMon1 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::string_hist_mon2)]
    HistMon2 {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_hist_name)]
    HistName { moniker: Box<SStringArg> },
    #[parse(rule=Rule::string_hist_next)]
    HistNext { moniker: Box<SStringArg> },
    #[parse(rule=Rule::string_hist_prev)]
    HistPrev { moniker: Box<SStringArg> },
    #[parse(rule=Rule::overloaded_hist_utxt)]
    HistUtxt {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::string_hist_wnam)]
    HistWnam {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::string_bkds)]
    Bkds { metaroom_id: Box<IntArg> },
    #[parse(rule=Rule::string_emid)]
    Emid,
    #[parse(rule=Rule::string_erid)]
    Erid { metaroom_id: Box<IntArg> },
    #[parse(rule=Rule::string_mloc)]
    Mloc { metaroom_id: Box<IntArg> },
    #[parse(rule=Rule::overloaded_rate)]
    Rate {
        room_type: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    #[parse(rule=Rule::string_rloc)]
    Rloc { room_id: Box<IntArg> },
    // Resource
    #[parse(rule=Rule::string_pray_agts)]
    PrayAgts {
        resource_name: Box<SStringArg>,
        string_tag: Box<SStringArg>,
        default_value: Box<SStringArg>,
    },
    #[parse(rule=Rule::string_pray_next)]
    PrayNext {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
    #[parse(rule=Rule::string_pray_prev)]
    PrayPrev {
        resource_type: Box<SStringArg>,
        last_known: Box<SStringArg>,
    },
    #[parse(rule=Rule::string_caos)]
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
    #[parse(rule=Rule::overloaded_rmsc)]
    Rmsc { x: Box<IntArg>, y: Box<IntArg> },
    #[parse(rule=Rule::overloaded_vois)]
    Vois,
    // Date
    #[parse(rule=Rule::string_rtif)]
    Rtif {
        real_time: Box<IntArg>,
        format: Box<SStringArg>,
    },
    #[parse(rule=Rule::string_gamn)]
    Gamn { previous: Box<SStringArg> },
    #[parse(rule=Rule::string_gnam)]
    Gnam,
    #[parse(rule=Rule::string_read)]
    Read {
        catalogue_tag: Box<SStringArg>,
        offset: Box<IntArg>,
    },
    #[parse(rule=Rule::string_subs)]
    Subs {
        value: Box<SStringArg>,
        start: Box<IntArg>,
        count: Box<IntArg>,
    },
    #[parse(rule=Rule::string_vtos)]
    Vtos { value: Box<DecimalArg> },
    // World
    #[parse(rule=Rule::overloaded_pswd)]
    Pswd { world_index: Box<IntArg> },
    #[parse(rule=Rule::string_wnam)]
    Wnam,
    #[parse(rule=Rule::overloaded_wrld)]
    Wrld { world_index: Box<IntArg> },
    #[parse(rule=Rule::string_wuid)]
    Wuid,
    #[parse(rule=Rule::string_prt_name)]
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

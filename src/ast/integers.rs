use super::{AgentArg, Anything, ByteString, FloatArg, IntArg, SStringArg, Variable};
use caos_macros::{CaosParsable, CommandList};

#[derive(CaosParsable, CommandList, Eq, PartialEq, Debug, Clone)]
pub enum Integer {
    #[syntax(with_parser = "parse_literal")]
    Literal(i32),
    #[syntax]
    Attr,
    #[syntax]
    Base,
    #[syntax]
    Bhvr,
    #[syntax]
    Cati {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[syntax]
    Clac,
    #[syntax]
    Clik { which_value: Box<IntArg> },
    #[syntax]
    Fmly,
    #[syntax]
    Gnus,
    #[syntax]
    Hght,
    #[syntax]
    Imsk,
    #[syntax]
    Mira,
    #[syntax]
    Mows,
    #[syntax]
    Paus,
    #[syntax]
    Plne,
    #[syntax]
    Pose,
    #[syntax]
    Puhl {
        pose: Box<IntArg>,
        x_or_y: Box<IntArg>,
    },
    #[syntax]
    Pupt {
        pose: Box<IntArg>,
        x_or_y: Box<IntArg>,
    },
    #[syntax]
    Seee {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[syntax]
    Spcs,
    #[syntax]
    Tick,
    #[syntax]
    Totl {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[syntax]
    Touc {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[syntax]
    Visi { check_all_cameras: Box<IntArg> },
    #[syntax]
    Wdth,
    #[syntax]
    Cmrx,
    #[syntax]
    Cmry,
    #[syntax]
    Loft { filename: Box<SStringArg> },
    #[syntax]
    Meta,
    #[syntax]
    Snax { filename: Box<SStringArg> },
    #[syntax]
    Wdow,
    #[syntax]
    Wndb,
    #[syntax]
    Wndh,
    #[syntax]
    Wndl,
    #[syntax]
    Wndr,
    #[syntax]
    Wndt,
    #[syntax]
    Wndw,
    #[syntax]
    Npgs,
    #[syntax]
    Page,
    #[syntax]
    Aslp,
    #[syntax]
    Attn,
    #[syntax]
    Body { body_part: Box<IntArg> },
    #[syntax]
    Bvar,
    #[syntax]
    Byit,
    #[syntax]
    Cage,
    #[syntax]
    Crea { agent: Box<AgentArg> },
    #[syntax]
    Dead,
    #[syntax]
    Decn,
    #[syntax]
    Dirn,
    #[syntax]
    Drea,
    #[syntax(name = "drv!")]
    Drv,
    #[syntax]
    Expr,
    #[syntax]
    Face,
    #[syntax(name = "ins#")]
    Ins,
    #[syntax]
    Orgi {
        organ_number: Box<IntArg>,
        data: Box<IntArg>,
    },
    #[syntax]
    Orgn,
    #[syntax]
    Tage,
    #[syntax]
    Uncs,
    #[syntax]
    Zomb,
    #[syntax]
    Code,
    #[syntax]
    Codf,
    #[syntax]
    Codg,
    #[syntax]
    Codp,
    #[syntax]
    Cods,
    #[syntax]
    Heap { index: Box<IntArg> },
    #[syntax]
    Paws,
    #[syntax]
    Unid,
    #[syntax]
    Inni,
    #[syntax]
    Inok,
    #[syntax(name = "hist cage")]
    HistCage {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist coun")]
    HistCoun { moniker: Box<SStringArg> },
    #[syntax(name = "hist cros")]
    HistCros { moniker: Box<SStringArg> },
    #[syntax(name = "hist find")]
    HistFind {
        moniker: Box<SStringArg>,
        event_type: Box<IntArg>,
        from_index: Box<IntArg>,
    },
    #[syntax(name = "hist finr")]
    HistFinr {
        moniker: Box<SStringArg>,
        event_type: Box<IntArg>,
        from_index: Box<IntArg>,
    },
    #[syntax(name = "hist gend")]
    HistGend { moniker: Box<SStringArg> },
    #[syntax(name = "hist gnus")]
    HistGnus { moniker: Box<SStringArg> },
    #[syntax(name = "hist mute")]
    HistMute { moniker: Box<SStringArg> },
    #[syntax(name = "hist rtim")]
    #[syntax(name = "hist prev")]
    HistPrev { moniker: Box<SStringArg> },
    HistRtim {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist tage")]
    HistTage {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist type")]
    HistType {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist vari")]
    HistVari { moniker: Box<SStringArg> },
    #[syntax(name = "hist wnam")]
    HistWnam {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    HistWtik {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax(name = "hist wuid")]
    HistWuid {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[syntax]
    Ooww { moniker: Box<SStringArg> },
    #[syntax]
    Keyd { key_code: Box<IntArg> },
    #[syntax]
    Mopx,
    #[syntax]
    Mopy,
    #[syntax]
    Pure,
    #[syntax]
    Addm {
        x: Box<IntArg>,
        y: Box<IntArg>,
        width: Box<IntArg>,
        height: Box<IntArg>,
        background: Box<SStringArg>,
    },
    #[syntax]
    Addr {
        metaroom_id: Box<IntArg>,
        x_left: Box<IntArg>,
        y_right: Box<IntArg>,
        y_left_ceiling: Box<IntArg>,
        y_right_ceiling: Box<IntArg>,
        y_left_floor: Box<IntArg>,
        y_right_floor: Box<IntArg>,
    },
    #[syntax]
    Door {
        room_id1: Box<IntArg>,
        room_id2: Box<IntArg>,
    },
    #[syntax]
    Down,
    #[syntax]
    Gmap { x: Box<FloatArg>, y: Box<FloatArg> },
    #[syntax]
    Grap { x: Box<FloatArg>, y: Box<FloatArg> },
    #[syntax]
    Grid {
        agent: Box<AgentArg>,
        direction: Box<IntArg>,
    },
    #[syntax]
    Hirp {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        directions: Box<IntArg>,
    },
    #[syntax]
    Left,
    #[syntax]
    Link {
        room1: Box<IntArg>,
        room2: Box<IntArg>,
    },
    #[syntax]
    Lorp {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        directions: Box<IntArg>,
    },
    #[syntax]
    Maph,
    #[syntax]
    Mapk,
    #[syntax]
    Mapw,
    #[syntax]
    Perm,
    #[syntax]
    Rght,
    #[syntax]
    Room { agent: Box<AgentArg> },
    #[syntax]
    Rtyp { room_id: Box<IntArg> },
    #[syntax(name = "_up_")]
    Up,
    //Motion
    #[syntax]
    Aero,
    #[syntax]
    Elas,
    #[syntax]
    Fall,
    #[syntax]
    Fric,
    #[syntax]
    Movs,
    #[syntax]
    Tmvb {
        delta_x: Box<FloatArg>,
        delta_y: Box<FloatArg>,
    },
    #[syntax]
    Tmvf { x: Box<FloatArg>, y: Box<FloatArg> },
    #[syntax]
    Tmvt { x: Box<FloatArg>, y: Box<FloatArg> },
    #[syntax]
    Wall,
    // Resources
    #[syntax(name = "pray agti")]
    PrayAgti {
        resource_name: Box<SStringArg>,
        integer_tag: Box<SStringArg>,
        default_value: Box<IntArg>,
    },
    #[syntax(name = "pray coun")]
    PrayCoun { resource_type: Box<SStringArg> },
    #[syntax(name = "pray deps")]
    PrayDeps {
        resource_name: Box<SStringArg>,
        dp_install: Box<IntArg>,
    },
    #[syntax(name = "pray expo")]
    PrayExpo { chunk_name: Box<SStringArg> },
    #[syntax(name = "pray file")]
    PrayFile {
        resource_name: Box<SStringArg>,
        resource_type: Box<IntArg>,
        do_install: Box<IntArg>,
    },
    #[syntax(name = "pray impo")]
    PrayImpo {
        moniker: Box<SStringArg>,
        actually_do_it: Box<IntArg>,
        keep_file: Box<IntArg>,
    },
    #[syntax(name = "pray injt")]
    PrayInjt {
        resource_name: Box<SStringArg>,
        do_install: Box<IntArg>,
        report_var: Box<Variable>,
    },
    #[syntax(name = "pray make")]
    PrayMake {
        which_journal_spot: Box<IntArg>,
        journal_name: Box<SStringArg>,
        which_pray_spot: Box<IntArg>,
        pray_name: Box<SStringArg>,
        report_destination: Box<Variable>,
    },
    #[syntax(name = "pray size")]
    PraySize { resource_name: Box<SStringArg> },
    #[syntax(name = "pray test")]
    PrayTest { resource_name: Box<SStringArg> },
    // Scripts
    #[syntax]
    Sorq {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        event: Box<IntArg>,
    },
    // Sounds
    #[syntax]
    Mute {
        and_mask: Box<IntArg>,
        eor_mask: Box<IntArg>,
    },
    // Time
    #[syntax]
    Date,
    #[syntax]
    Dayt,
    #[syntax]
    Etik,
    #[syntax(name = "hist date")]
    HistDate { world_tick: Box<IntArg> },
    #[syntax(name = "hist sean")]
    HistSean { world_tick: Box<IntArg> },
    #[syntax(name = "hist time")]
    HistTime { world_tick: Box<IntArg> },
    #[syntax(name = "hist year")]
    HistYear { world_tick: Box<IntArg> },
    #[syntax]
    Mont,
    #[syntax]
    Msec,
    #[syntax]
    Race,
    #[syntax]
    Rtim,
    #[syntax]
    Scol {
        and_mask: Box<IntArg>,
        eor_mask: Box<IntArg>,
        up_speeds: Box<ByteString>,
        down_speeds: Box<ByteString>,
    },
    #[syntax]
    Sean,
    #[syntax]
    Time,
    #[syntax]
    Wolf {
        kanga_mask: Box<IntArg>,
        eeyore_mask: Box<IntArg>,
    },
    #[syntax]
    Wpau,
    #[syntax]
    Wtik,
    #[syntax]
    Year,
    // Variables
    #[syntax]
    Char {
        string: Box<SStringArg>,
        index: Box<IntArg>,
    },
    #[syntax]
    Ftoi { number_to_convert: Box<FloatArg> },
    #[syntax]
    Rand {
        value1: Box<IntArg>,
        value2: Box<IntArg>,
    },
    #[syntax]
    Rean { catalogue_tag: Box<SStringArg> },
    #[syntax]
    Reaq { catalogue_tag: Box<SStringArg> },
    #[syntax]
    Stoi { value: Box<SStringArg> },
    #[syntax]
    Strl { value: Box<SStringArg> },
    #[syntax]
    Type { something: Box<Anything> },
    #[syntax]
    Vmjr,
    #[syntax]
    Vmnr,
    // Vehicles
    #[syntax]
    Cabb,
    #[syntax]
    Cabl,
    #[syntax]
    Cabp,
    #[syntax]
    Cabr,
    #[syntax]
    Cabt,
    #[syntax]
    Cabv,
    // World
    #[syntax]
    Nwld,
    #[syntax]
    Wnti { world: Box<SStringArg> },
    // Ports
    #[syntax(name = "prt: itot")]
    PrtItot,
    #[syntax(name = "prt: from")]
    PrtFrom { input_port: Box<IntArg> },
    #[syntax(name = "prt: otot")]
    PrtOtot,
}

impl From<i32> for Integer {
    fn from(i: i32) -> Self {
        Integer::Literal(i.into())
    }
}

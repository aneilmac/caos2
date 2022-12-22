use super::{Agent, Anything, ByteString, Float, SString, Variable};
use crate::parser::CaosParsable;
use caos_macros::{CaosParsable, CommandList};
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::map_res;

#[derive(CaosParsable, CommandList)]
pub enum Integer {
    #[syntax(with_parser = "parse_literal")]
    Raw(i32),
    #[syntax(with_parser = "parse_variable")]
    Variable(Variable),
    #[syntax]
    Attr,
    #[syntax]
    Base,
    #[syntax]
    Bhvr,
    #[syntax]
    Cati {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Clac,
    #[syntax]
    Clik { which_value: Box<Integer> },
    #[syntax]
    Enum {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Esee {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Etch {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
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
        pose: Box<Integer>,
        x_or_y: Box<Integer>,
    },
    #[syntax]
    Pupt {
        pose: Box<Integer>,
        x_or_y: Box<Integer>,
    },
    #[syntax]
    Seee {
        first: Box<Agent>,
        second: Box<Agent>,
    },
    #[syntax]
    Spcs,
    #[syntax]
    Tick,
    #[syntax]
    Totl {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
    },
    #[syntax]
    Touc {
        first: Box<Agent>,
        second: Box<Agent>,
    },
    #[syntax]
    Visi { check_all_cameras: Box<Integer> },
    #[syntax]
    Wdth,
    #[syntax]
    Cmrx,
    #[syntax]
    Cmry,
    #[syntax]
    Loft { filename: Box<SString> },
    #[syntax]
    Meta,
    #[syntax]
    Snax { filename: Box<SString> },
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
    Body { body_part: Box<Integer> },
    #[syntax]
    Bvar,
    #[syntax]
    Byit,
    #[syntax]
    Cage,
    #[syntax]
    Crea { agent: Box<Agent> },
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
        organ_number: Box<Integer>,
        data: Box<Integer>,
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
    Heap { index: Box<Integer> },
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
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist coun")]
    HistCoun { moniker: Box<SString> },
    #[syntax(name = "hist cros")]
    HistCros { moniker: Box<SString> },
    #[syntax(name = "hist find")]
    HistFind {
        moniker: Box<SString>,
        event_type: Box<Integer>,
        from_index: Box<Integer>,
    },
    #[syntax(name = "hist finr")]
    HistFinr {
        moniker: Box<SString>,
        event_type: Box<Integer>,
        from_index: Box<Integer>,
    },
    #[syntax(name = "hist gend")]
    HistGend { moniker: Box<SString> },
    #[syntax(name = "hist gnus")]
    HistGnus { moniker: Box<SString> },
    #[syntax(name = "hist mute")]
    HistMute { moniker: Box<SString> },
    #[syntax(name = "hist rtim")]
    HistPrev {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist tage")]
    HistTage {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist type")]
    HistType {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist vari")]
    HistVari { moniker: Box<SString> },
    #[syntax(name = "hist wtik")]
    HistWnam {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax(name = "hist wuid")]
    HistWuid {
        moniker: Box<SString>,
        event_no: Box<Integer>,
    },
    #[syntax]
    Ooww { moniker: Box<SString> },
    #[syntax]
    Keyd { key_code: Box<Integer> },
    #[syntax]
    Mopx,
    #[syntax]
    Mopy,
    #[syntax]
    Pure,
    #[syntax]
    Addm {
        x: Box<Integer>,
        y: Box<Integer>,
        width: Box<Integer>,
        height: Box<Integer>,
        background: Box<SString>,
    },
    #[syntax]
    Addr {
        metaroom_id: Box<Integer>,
        x_left: Box<Integer>,
        y_right: Box<Integer>,
        y_left_ceiling: Box<Integer>,
        y_right_ceiling: Box<Integer>,
        y_left_floor: Box<Integer>,
        y_right_floor: Box<Integer>,
    },
    #[syntax]
    Door {
        room_id1: Box<Integer>,
        room_id2: Box<Integer>,
    },
    #[syntax]
    Down,
    #[syntax]
    Gmap { x: Box<Float>, y: Box<Float> },
    #[syntax]
    Grap { x: Box<Float>, y: Box<Float> },
    #[syntax]
    Grid {
        agent: Box<Agent>,
        direction: Box<Integer>,
    },
    #[syntax]
    Hirp {
        room_id: Box<Integer>,
        ca_index: Box<Integer>,
        directions: Box<Integer>,
    },
    #[syntax]
    Left,
    #[syntax]
    Link {
        room1: Box<Integer>,
        room2: Box<Integer>,
    },
    #[syntax]
    Lorp {
        room_id: Box<Integer>,
        ca_index: Box<Integer>,
        directions: Box<Integer>,
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
    Right,
    #[syntax]
    Room { agent: Box<Agent> },
    #[syntax]
    Rtyp { room_id: Box<Integer> },
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
        delta_x: Box<Float>,
        delta_y: Box<Float>,
    },
    #[syntax]
    Tmvf { x: Box<Float>, y: Box<Float> },
    #[syntax]
    Tmvt { x: Box<Float>, y: Box<Float> },
    #[syntax]
    Wall,
    // Resources
    #[syntax(name = "pray agti")]
    PrayAgti {
        resource_name: Box<SString>,
        integer_tag: Box<SString>,
        default_value: Box<Integer>,
    },
    #[syntax(name = "pray coun")]
    PrayCoun { resource_type: Box<SString> },
    #[syntax(name = "pray deps")]
    PrayDeps {
        resource_name: Box<SString>,
        dp_install: Box<Integer>,
    },
    #[syntax(name = "pray expo")]
    PrayExpo { chunk_name: Box<SString> },
    #[syntax(name = "pray file")]
    PrayFile {
        resource_name: Box<SString>,
        resource_type: Box<Integer>,
        do_install: Box<Integer>,
    },
    #[syntax(name = "pray impo")]
    PrayImpo {
        moniker: Box<SString>,
        actually_do_it: Box<Integer>,
        keep_file: Box<Integer>,
    },
    #[syntax(name = "pray injt")]
    PrayInjt {
        resource_name: Box<SString>,
        do_install: Box<Integer>,
        report_var: Box<Variable>,
    },
    #[syntax(name = "pray make")]
    PrayMake {
        which_journal_spot: Box<Integer>,
        journal_name: Box<SString>,
        which_pray_spot: Box<Integer>,
        pray_name: Box<SString>,
        report_destination: Box<Variable>,
    },
    #[syntax(name = "pray size")]
    PraySize { resource_name: Box<SString> },
    #[syntax(name = "pray test")]
    PrayTest { resource_name: Box<SString> },
    // Scripts
    #[syntax]
    Sorq {
        family: Box<Integer>,
        genus: Box<Integer>,
        species: Box<Integer>,
        event: Box<Integer>,
    },
    // Sounds
    #[syntax]
    Mute {
        and_mask: Box<Integer>,
        eor_mask: Box<Integer>,
    },
    // Time
    #[syntax]
    Date,
    #[syntax]
    Dayt,
    #[syntax]
    Etik,
    #[syntax(name = "hist date")]
    HistDate { world_tick: Box<Integer> },
    #[syntax(name = "hist sean")]
    HistSean { world_tick: Box<Integer> },
    #[syntax(name = "hist time")]
    HistTime { world_tick: Box<Integer> },
    #[syntax(name = "hist year")]
    HistYear { world_tick: Box<Integer> },
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
        and_mask: Box<Integer>,
        eor_mask: Box<Integer>,
        up_speeds: Box<ByteString>,
        down_speeds: Box<ByteString>,
    },
    #[syntax]
    Sean,
    #[syntax]
    Time,
    #[syntax]
    Wolf {
        kanga_mask: Box<Integer>,
        eeyore_mask: Box<Integer>,
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
        string: Box<SString>,
        index: Box<Integer>,
    },
    #[syntax]
    Ftoi { number_to_convert: Box<Float> },
    #[syntax]
    Rand {
        value1: Box<Integer>,
        value2: Box<Integer>,
    },
    #[syntax]
    Rean { catalogue_tag: Box<SString> },
    #[syntax]
    Reaq { catalogue_tag: Box<SString> },
    #[syntax]
    Stoi { value: Box<SString> },
    #[syntax]
    Strl { value: Box<SString> },
    #[syntax]
    Type { sometime: Box<Anything> },
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
    Wnti { world: Box<SString> },
}

fn parse_variable(input: &str) -> nom::IResult<&str, Integer> {
    map(Variable::parse_caos, Integer::Variable)(input)
}

fn parse_literal(input: &str) -> nom::IResult<&str, Integer> {
    map_res(digit1, |i: &str| i.parse::<i32>().map(Integer::Raw))(input)
}

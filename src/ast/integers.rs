use super::{AgentArg, Anything, ByteString, FloatArg, IntArg, SStringArg, Variable};
use crate::Rule;
use caos_macros::ExpressionParser;

#[derive(Eq, PartialEq, Debug, Clone, ExpressionParser)]
pub enum Integer {
    #[parse(ignore)]
    Literal(i32),
    #[parse(rule=Rule::int_attr)]
    Attr,
    #[parse(rule=Rule::int_base)]
    Base,
    #[parse(rule=Rule::int_bhvr)]
    Bhvr,
    #[parse(rule=Rule::int_cati)]
    Cati {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::int_clac)]
    Clac,
    #[parse(rule=Rule::int_clik)]
    Clik { which_value: Box<IntArg> },
    #[parse(rule=Rule::int_fmly)]
    Fmly,
    #[parse(rule=Rule::int_gnus)]
    Gnus,
    #[parse(rule=Rule::int_hght)]
    Hght,
    #[parse(rule=Rule::int_imsk)]
    Imsk,
    #[parse(rule=Rule::int_mira)]
    Mira,
    #[parse(rule=Rule::int_mows)]
    Mows,
    #[parse(rule=Rule::int_paus)]
    Paus,
    #[parse(rule=Rule::int_plne)]
    Plne,
    #[parse(rule=Rule::int_pose)]
    Pose,
    #[parse(rule=Rule::int_puhl)]
    Puhl {
        pose: Box<IntArg>,
        x_or_y: Box<IntArg>,
    },
    #[parse(rule=Rule::int_pupt)]
    Pupt {
        pose: Box<IntArg>,
        x_or_y: Box<IntArg>,
    },
    #[parse(rule=Rule::int_seee)]
    Seee {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[parse(rule=Rule::int_spcs)]
    Spcs,
    #[parse(rule=Rule::int_tick)]
    Tick,
    #[parse(rule=Rule::int_totl)]
    Totl {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::int_touc)]
    Touc {
        first: Box<AgentArg>,
        second: Box<AgentArg>,
    },
    #[parse(rule=Rule::int_visi)]
    Visi { check_all_cameras: Box<IntArg> },
    #[parse(rule=Rule::int_wdth)]
    Wdth,
    #[parse(rule=Rule::int_cmrx)]
    Cmrx,
    #[parse(rule=Rule::int_cmry)]
    Cmry,
    #[parse(rule=Rule::int_loft)]
    Loft { filename: Box<SStringArg> },
    #[parse(rule=Rule::int_meta)]
    Meta,
    #[parse(rule=Rule::int_snax)]
    Snax { filename: Box<SStringArg> },
    #[parse(rule=Rule::int_wdow)]
    Wdow,
    #[parse(rule=Rule::int_wndb)]
    Wndb,
    #[parse(rule=Rule::int_wndh)]
    Wndh,
    #[parse(rule=Rule::int_wndl)]
    Wndl,
    #[parse(rule=Rule::int_wndr)]
    Wndr,
    #[parse(rule=Rule::int_wndt)]
    Wndt,
    #[parse(rule=Rule::int_wndw)]
    Wndw,
    #[parse(rule=Rule::int_npgs)]
    Npgs,
    #[parse(rule=Rule::int_page)]
    Page,
    #[parse(rule=Rule::int_aslp)]
    Aslp,
    #[parse(rule=Rule::int_attn)]
    Attn,
    #[parse(rule=Rule::int_body)]
    Body { body_part: Box<IntArg> },
    #[parse(rule=Rule::int_bvar)]
    Bvar,
    #[parse(rule=Rule::int_byit)]
    Byit,
    #[parse(rule=Rule::int_cage)]
    Cage,
    #[parse(rule=Rule::int_crea)]
    Crea { agent: Box<AgentArg> },
    #[parse(rule=Rule::int_dead)]
    Dead,
    #[parse(rule=Rule::int_decn)]
    Decn,
    #[parse(rule=Rule::int_dirn)]
    Dirn,
    #[parse(rule=Rule::int_drea)]
    Drea,
    #[parse(rule=Rule::int_drv)]
    Drv,
    #[parse(rule=Rule::int_expr)]
    Expr,
    #[parse(rule=Rule::int_face)]
    Face,
    #[parse(rule=Rule::int_ins)]
    Ins,
    #[parse(rule=Rule::int_orgi)]
    Orgi {
        organ_number: Box<IntArg>,
        data: Box<IntArg>,
    },
    #[parse(rule=Rule::int_orgn)]
    Orgn,
    #[parse(rule=Rule::int_tage)]
    Tage,
    #[parse(rule=Rule::int_uncs)]
    Uncs,
    #[parse(rule=Rule::int_zomb)]
    Zomb,
    #[parse(rule=Rule::int_code)]
    Code,
    #[parse(rule=Rule::int_codf)]
    Codf,
    #[parse(rule=Rule::int_codg)]
    Codg,
    #[parse(rule=Rule::int_codp)]
    Codp,
    #[parse(rule=Rule::int_cods)]
    Cods,
    #[parse(rule=Rule::int_heap)]
    Heap { index: Box<IntArg> },
    #[parse(rule=Rule::int_paws)]
    Paws,
    #[parse(rule=Rule::int_unid)]
    Unid,
    #[parse(rule=Rule::int_inni)]
    Inni,
    #[parse(rule=Rule::int_inok)]
    Inok,
    #[parse(rule=Rule::int_hist_cage)]
    HistCage {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_coun)]
    HistCoun { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_cros)]
    HistCros { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_find)]
    HistFind {
        moniker: Box<SStringArg>,
        event_type: Box<IntArg>,
        from_index: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_finr)]
    HistFinr {
        moniker: Box<SStringArg>,
        event_type: Box<IntArg>,
        from_index: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_gend)]
    HistGend { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_gnus)]
    HistGnus { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_mute)]
    HistMute { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_prev)]
    HistPrev { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_rtim)]
    HistRtim {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_tage)]
    HistTage {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_type)]
    HistType {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_vari)]
    HistVari { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_hist_wnam)]
    HistWnam {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_wtik)]
    HistWtik {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hist_wuid)]
    HistWuid {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
    },
    #[parse(rule=Rule::int_ooww)]
    Ooww { moniker: Box<SStringArg> },
    #[parse(rule=Rule::int_keyd)]
    Keyd { key_code: Box<IntArg> },
    #[parse(rule=Rule::int_mopx)]
    Mopx,
    #[parse(rule=Rule::int_mopy)]
    Mopy,
    #[parse(rule=Rule::int_pure)]
    Pure,
    #[parse(rule=Rule::int_addm)]
    Addm {
        x: Box<IntArg>,
        y: Box<IntArg>,
        width: Box<IntArg>,
        height: Box<IntArg>,
        background: Box<SStringArg>,
    },
    #[parse(rule=Rule::int_addr)]
    Addr {
        metaroom_id: Box<IntArg>,
        x_left: Box<IntArg>,
        y_right: Box<IntArg>,
        y_left_ceiling: Box<IntArg>,
        y_right_ceiling: Box<IntArg>,
        y_left_floor: Box<IntArg>,
        y_right_floor: Box<IntArg>,
    },
    #[parse(rule=Rule::int_door)]
    Door {
        room_id1: Box<IntArg>,
        room_id2: Box<IntArg>,
    },
    #[parse(rule=Rule::int_down)]
    Down,
    #[parse(rule=Rule::int_gmap)]
    Gmap { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::int_grap)]
    Grap { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::int_grid)]
    Grid {
        agent: Box<AgentArg>,
        direction: Box<IntArg>,
    },
    #[parse(rule=Rule::int_hirp)]
    Hirp {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        directions: Box<IntArg>,
    },
    #[parse(rule=Rule::int_left)]
    Left,
    #[parse(rule=Rule::int_link)]
    Link {
        room1: Box<IntArg>,
        room2: Box<IntArg>,
    },
    #[parse(rule=Rule::int_lorp)]
    Lorp {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        directions: Box<IntArg>,
    },
    #[parse(rule=Rule::int_maph)]
    Maph,
    #[parse(rule=Rule::int_mapk)]
    Mapk,
    #[parse(rule=Rule::int_mapw)]
    Mapw,
    #[parse(rule=Rule::int_perm)]
    Perm,
    #[parse(rule=Rule::int_rght)]
    Rght,
    #[parse(rule=Rule::int_room)]
    Room { agent: Box<AgentArg> },
    #[parse(rule=Rule::int_rtyp)]
    Rtyp { room_id: Box<IntArg> },
    #[parse(rule=Rule::int_up)]
    Up,
    //Motion
    #[parse(rule=Rule::int_aero)]
    Aero,
    #[parse(rule=Rule::int_elas)]
    Elas,
    #[parse(rule=Rule::int_fall)]
    Fall,
    #[parse(rule=Rule::int_fric)]
    Fric,
    #[parse(rule=Rule::int_movs)]
    Movs,
    #[parse(rule=Rule::int_tmvb)]
    Tmvb {
        delta_x: Box<FloatArg>,
        delta_y: Box<FloatArg>,
    },
    #[parse(rule=Rule::int_tmvf)]
    Tmvf { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::int_tmvt)]
    Tmvt { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::int_wall)]
    Wall,
    // Resources
    #[parse(rule=Rule::int_pray_agti)]
    PrayAgti {
        resource_name: Box<SStringArg>,
        integer_tag: Box<SStringArg>,
        default_value: Box<IntArg>,
    },
    #[parse(rule=Rule::int_pray_coun)]
    PrayCoun { resource_type: Box<SStringArg> },
    #[parse(rule=Rule::int_pray_deps)]
    PrayDeps {
        resource_name: Box<SStringArg>,
        dp_install: Box<IntArg>,
    },
    #[parse(rule=Rule::int_pray_expo)]
    PrayExpo { chunk_name: Box<SStringArg> },
    #[parse(rule=Rule::int_pray_file)]
    PrayFile {
        resource_name: Box<SStringArg>,
        resource_type: Box<IntArg>,
        do_install: Box<IntArg>,
    },
    #[parse(rule=Rule::int_pray_impo)]
    PrayImpo {
        moniker: Box<SStringArg>,
        actually_do_it: Box<IntArg>,
        keep_file: Box<IntArg>,
    },
    #[parse(rule=Rule::int_pray_injt)]
    PrayInjt {
        resource_name: Box<SStringArg>,
        do_install: Box<IntArg>,
        report_var: Box<Variable>,
    },
    #[parse(rule=Rule::int_pray_make)]
    PrayMake {
        which_journal_spot: Box<IntArg>,
        journal_name: Box<SStringArg>,
        which_pray_spot: Box<IntArg>,
        pray_name: Box<SStringArg>,
        report_destination: Box<Variable>,
    },
    #[parse(rule=Rule::int_pray_size)]
    PraySize { resource_name: Box<SStringArg> },
    #[parse(rule=Rule::int_pray_test)]
    PrayTest { resource_name: Box<SStringArg> },
    // Scripts
    #[parse(rule=Rule::int_sorq)]
    Sorq {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        event: Box<IntArg>,
    },
    // Sounds
    #[parse(rule=Rule::int_mute)]
    Mute {
        and_mask: Box<IntArg>,
        eor_mask: Box<IntArg>,
    },
    // Time
    #[parse(rule=Rule::int_date)]
    Date,
    #[parse(rule=Rule::int_dayt)]
    Dayt,
    #[parse(rule=Rule::int_etik)]
    Etik,
    #[parse(rule=Rule::int_hist_date)]
    HistDate { world_tick: Box<IntArg> },
    #[parse(rule=Rule::int_hist_sean)]
    HistSean { world_tick: Box<IntArg> },
    #[parse(rule=Rule::int_hist_time)]
    HistTime { world_tick: Box<IntArg> },
    #[parse(rule=Rule::int_hist_year)]
    HistYear { world_tick: Box<IntArg> },
    #[parse(rule=Rule::int_mont)]
    Mont,
    #[parse(rule=Rule::int_msec)]
    Msec,
    #[parse(rule=Rule::int_race)]
    Race,
    #[parse(rule=Rule::int_rtim)]
    Rtim,
    #[parse(rule=Rule::int_scol)]
    Scol {
        and_mask: Box<IntArg>,
        eor_mask: Box<IntArg>,
        up_speeds: Box<ByteString>,
        down_speeds: Box<ByteString>,
    },
    #[parse(rule=Rule::int_sean)]
    Sean,
    #[parse(rule=Rule::int_time)]
    Time,
    #[parse(rule=Rule::int_wolf)]
    Wolf {
        kanga_mask: Box<IntArg>,
        eeyore_mask: Box<IntArg>,
    },
    #[parse(rule=Rule::int_wpau)]
    Wpau,
    #[parse(rule=Rule::int_wtik)]
    Wtik,
    #[parse(rule=Rule::int_year)]
    Year,
    // Variables
    #[parse(rule=Rule::int_char)]
    Char {
        string: Box<SStringArg>,
        index: Box<IntArg>,
    },
    #[parse(rule=Rule::int_ftoi)]
    Ftoi { number_to_convert: Box<FloatArg> },
    #[parse(rule=Rule::int_rand)]
    Rand {
        value1: Box<IntArg>,
        value2: Box<IntArg>,
    },
    #[parse(rule=Rule::int_rean)]
    Rean { catalogue_tag: Box<SStringArg> },
    #[parse(rule=Rule::int_reaq)]
    Reaq { catalogue_tag: Box<SStringArg> },
    #[parse(rule=Rule::int_stoi)]
    Stoi { value: Box<SStringArg> },
    #[parse(rule=Rule::int_strl)]
    Strl { value: Box<SStringArg> },
    #[parse(rule=Rule::int_type)]
    Type { something: Box<Anything> },
    #[parse(rule=Rule::int_vmjr)]
    Vmjr,
    #[parse(rule=Rule::int_vmnr)]
    Vmnr,
    // Vehicles
    #[parse(rule=Rule::int_cabb)]
    Cabb,
    #[parse(rule=Rule::int_cabl)]
    Cabl,
    #[parse(rule=Rule::int_cabp)]
    Cabp,
    #[parse(rule=Rule::int_cabr)]
    Cabr,
    #[parse(rule=Rule::int_cabt)]
    Cabt,
    #[parse(rule=Rule::int_cabv)]
    Cabv,
    // World
    #[parse(rule=Rule::int_nwld)]
    Nwld,
    #[parse(rule=Rule::int_wnti)]
    Wnti { world: Box<SStringArg> },
    // Ports
    #[parse(rule=Rule::int_prt_itot)]
    PrtItot,
    #[parse(rule=Rule::int_prt_from)]
    PrtFrom { input_port: Box<IntArg> },
    #[parse(rule=Rule::int_prt_otot)]
    PrtOtot,
}

impl From<i32> for Integer {
    fn from(i: i32) -> Self {
        Integer::Literal(i.into())
    }
}

use caos_macros::CommandParser;

use super::{
    AgentArg, Anything, ByteString, Condition, DecimalArg, FloatArg, IntArg, Label, SStringArg,
    ScriptDefinition, Variable,
};
use crate::Rule;

#[derive(Eq, PartialEq, Clone, Debug, CommandParser)]
pub enum Command {
    #[parse(ignore)]
    Gsub { destination: Label },
    #[parse(ignore)]
    DbgAsrt { condition: Condition },
    #[parse(ignore)]
    Goto { destination: Label },
    #[parse(ignore)]
    Doif {
        condition: Condition,
        definition: ScriptDefinition,
        elif_definitions: Vec<(Condition, ScriptDefinition)>,
        else_definition: Option<ScriptDefinition>,
    },
    #[parse(ignore)]
    Subr {
        label: Label,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    Reps {
        count: Box<IntArg>,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    LoopEver { definition: ScriptDefinition },
    #[parse(ignore)]
    LoopUntl {
        definition: ScriptDefinition,
        condition: Condition,
    },
    #[parse(ignore)]
    Econ {
        agent: Box<AgentArg>,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    Enum {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    Etch {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    Esee {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        definition: ScriptDefinition,
    },
    #[parse(ignore)]
    Epas {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        definition: ScriptDefinition,
    },
    // Box<AgentArg>s
    #[parse(rule=Rule::command_anim)]
    Anim { pose_list: Box<ByteString> },
    #[parse(rule=Rule::command_anms)]
    Anms { anim_string: Box<SStringArg> },
    #[parse(rule=Rule::overloaded_attr)]
    Attr { attributes: Box<IntArg> },
    #[parse(rule=Rule::overloaded_base)]
    Base { index: Box<IntArg> },
    #[parse(rule=Rule::overloaded_bhvr)]
    Bhvr { permissions: Box<IntArg> },
    #[parse(rule=Rule::command_frat)]
    Frat { framerate: Box<IntArg> },
    #[parse(rule=Rule::command_gait)]
    Gait { gait_number: Box<IntArg> },
    #[parse(rule=Rule::command_gall)]
    Gall {
        sprite_file: Box<SStringArg>,
        first_image: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_hand)]
    Hand { name_for_the_hand: Box<SStringArg> },
    #[parse(rule=Rule::command_kill)]
    Kill { agent: Box<AgentArg> },
    #[parse(rule=Rule::command_mesg_writ)]
    MesgWrit {
        agent: Box<AgentArg>,
        message_id: Box<IntArg>,
    },
    #[parse(rule=Rule::command_mesg_wrt)]
    MesgWritPlus {
        agent: Box<AgentArg>,
        message_id: Box<IntArg>,
        param_1: Box<Anything>,
        param_2: Box<Anything>,
        delay: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_mira)]
    Mira { on_off: Box<IntArg> },
    #[parse(rule=Rule::command_new_simp)]
    NewSimp {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        image_count: Box<IntArg>,
        first_image: Box<IntArg>,
        plane: Box<IntArg>,
    },
    #[parse(rule=Rule::command_nohh)]
    Nohh,
    #[parse(rule=Rule::command_over)]
    Over,
    #[parse(rule=Rule::overloaded_paus)]
    Paus { paused: Box<IntArg> },
    #[parse(rule=Rule::overloaded_plne)]
    Plne { plane: Box<IntArg> },
    #[parse(rule=Rule::overloaded_pose)]
    Pose { pose: Box<IntArg> },
    #[parse(rule=Rule::overloaded_puhl)]
    Puhl {
        pose: Box<IntArg>,
        x: Box<IntArg>,
        y: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_pupt)]
    Pupt {
        pose: Box<IntArg>,
        x: Box<IntArg>,
        y: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_rnge)]
    Rnge { distance: Box<FloatArg> },
    #[parse(rule=Rule::command_rtar)]
    Rtar {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::command_show)]
    Show { visibility: Box<IntArg> },
    #[parse(rule=Rule::command_star)]
    Star {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_tick)]
    Tick { tick_rate: Box<IntArg> },
    #[parse(rule=Rule::command_tint)]
    Tint {
        red_tint: Box<IntArg>,
        green_tint: Box<IntArg>,
        blue_tint: Box<IntArg>,
        rotation: Box<IntArg>,
        swap: Box<IntArg>,
    },
    #[parse(rule=Rule::command_ttar)]
    Ttar {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    // Brain
    #[parse(rule=Rule::command_brn_dmpb)]
    BrnDmpb,
    #[parse(rule=Rule::command_brn_dmpd)]
    BrnDmpd {
        tract_number: Box<IntArg>,
        dendrite_number: Box<IntArg>,
    },
    #[parse(rule=Rule::command_brn_dmpl)]
    BrnDmpl { lobe_number: Box<IntArg> },
    #[parse(rule=Rule::command_brn_dmpn)]
    BrnDmpn {
        lobe_number: Box<IntArg>,
        neuron_number: Box<IntArg>,
    },
    #[parse(rule=Rule::command_brn_dmpt)]
    BrnDmpt { tract_number: Box<IntArg> },
    #[parse(rule=Rule::command_brn_setd)]
    BrnSetd {
        tract_number: Box<IntArg>,
        dendrite_number: Box<IntArg>,
        weight_number: Box<IntArg>,
        new_value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_brn_setl)]
    BrnSetl {
        lobe_number: Box<IntArg>,
        line_number: Box<IntArg>,
        new_value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_brn_setn)]
    BrnSetn {
        lobe_number: Box<IntArg>,
        neuron_number: Box<IntArg>,
        state_number: Box<IntArg>,
        new_value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_brn_sett)]
    BrnSett {
        tract_number: Box<IntArg>,
        line_number: Box<IntArg>,
        new_value: Box<FloatArg>,
    },
    // Camera
    #[parse(rule=Rule::overloaded_bkgd)]
    Bkgd {
        metaroom_id: Box<IntArg>,
        background: Box<SStringArg>,
        transition: Box<IntArg>,
    },
    #[parse(rule=Rule::command_brmi)]
    Brmi {
        mearoom_base: Box<IntArg>,
        room_base: Box<IntArg>,
    },
    #[parse(rule=Rule::command_cmra)]
    Cmra {
        x: Box<IntArg>,
        y: Box<IntArg>,
        pan: Box<IntArg>,
    },
    #[parse(rule=Rule::command_cmrp)]
    Cmrp {
        x: Box<IntArg>,
        y: Box<IntArg>,
        pan: Box<IntArg>,
    },
    #[parse(rule=Rule::command_cmrt)]
    Cmrt { pan: Box<IntArg> },
    #[parse(rule=Rule::command_frsh)]
    Frsh,
    #[parse(rule=Rule::command_line)]
    Line {
        x1: Box<IntArg>,
        y1: Box<IntArg>,
        x2: Box<IntArg>,
        y2: Box<IntArg>,
        r: Box<IntArg>,
        g: Box<IntArg>,
        b: Box<IntArg>,
        stipple_on: Box<IntArg>,
        stipple_off: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_meta)]
    Meta {
        metaroom_id: Box<IntArg>,
        camera_x: Box<IntArg>,
        camera_y: Box<IntArg>,
        transition: Box<IntArg>,
    },
    #[parse(rule=Rule::command_scam)]
    Scam {
        compound_agent: Box<AgentArg>,
        part_number: Box<IntArg>,
    },
    #[parse(rule=Rule::command_snap)]
    Snap {
        filename: Box<SStringArg>,
        x_centre: Box<IntArg>,
        y_centre: Box<IntArg>,
        width: Box<IntArg>,
        height: Box<IntArg>,
        zoom_factor: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_trck)]
    Trck {
        agent: Box<AgentArg>,
        x_percent: Box<IntArg>,
        y_percent: Box<IntArg>,
        style: Box<IntArg>,
        transition: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_wdow)]
    Wdow,
    #[parse(rule=Rule::command_zoom)]
    Zoom {
        pixels: Box<IntArg>,
        x: Box<IntArg>,
        y: Box<IntArg>,
    },
    // Compounds
    #[parse(rule=Rule::command_fcus)]
    Fcus,
    #[parse(rule=Rule::command_frmt)]
    Frmt {
        left_margin: Box<IntArg>,
        top_margin: Box<IntArg>,
        right_margin: Box<IntArg>,
        bottom_margin: Box<IntArg>,
        line_spacing: Box<IntArg>,
        character_spacing: Box<IntArg>,
        justification: Box<IntArg>,
    },
    #[parse(rule=Rule::command_grpl)]
    Grpl {
        red: Box<IntArg>,
        green: Box<IntArg>,
        blue: Box<IntArg>,
        min_y: Box<FloatArg>,
        max_y: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_grpv)]
    Grpv {
        line_index: Box<IntArg>,
        value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_new_comp)]
    NewComp {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        image_count: Box<IntArg>,
        first_image: Box<IntArg>,
        plane: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_page)]
    Page { page: Box<IntArg> },
    #[parse(rule=Rule::command_part)]
    Part { part_id: Box<IntArg> },
    #[parse(rule=Rule::command_pat_butt)]
    PatButt {
        part_id: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        first_image: Box<IntArg>,
        image_count: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
        anim_hover: Box<ByteString>,
        message_id: Box<IntArg>,
        option: Box<IntArg>,
    },
    #[parse(rule=Rule::command_pat_cmra)]
    PatCmra {
        part_id: Box<IntArg>,
        overlay_sprite: Box<SStringArg>,
        base_image: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
        view_width: Box<IntArg>,
        view_height: Box<IntArg>,
        camera_width: Box<IntArg>,
        camera_height: Box<IntArg>,
    },
    #[parse(rule=Rule::command_pat_dull)]
    PatDull {
        part_id: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        first_image: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
    },
    #[parse(rule=Rule::command_pat_fixd)]
    PatFixd {
        part_id: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        first_image: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
        font_sprite: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_pat_grph)]
    PatGrph {
        part_id: Box<IntArg>,
        overlay_sprite: Box<SStringArg>,
        base_image: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
        num_values: Box<IntArg>,
    },
    #[parse(rule=Rule::command_pat_kill)]
    PatKill { part_id: Box<IntArg> },
    #[parse(rule=Rule::command_pat_text)]
    PatText {
        part_id: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        first_image: Box<IntArg>,
        rel_x: Box<DecimalArg>,
        rel_y: Box<DecimalArg>,
        rel_plane: Box<IntArg>,
        message_id: Box<IntArg>,
        font_sprite: Box<SStringArg>,
    },
    #[parse(rule=Rule::overloaded_ptxt)]
    Ptxt { text: Box<SStringArg> },
    // Creates
    #[parse(rule=Rule::command_ages)]
    Ages { times: Box<IntArg> },
    #[parse(rule=Rule::command_appr)]
    Appr,
    #[parse(rule=Rule::overloaded_aslp)]
    Aslp { asleep: Box<IntArg> },
    #[parse(rule=Rule::overloaded_body)]
    Body {
        set_number: Box<IntArg>,
        layer: Box<IntArg>,
    },
    #[parse(rule=Rule::command_born)]
    Born,
    #[parse(rule=Rule::overloaded_chem)]
    Chem {
        chemical: Box<IntArg>,
        adjustment: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_dead)]
    Dead,
    #[parse(rule=Rule::overloaded_dirn)]
    Dirn { direction: Box<IntArg> },
    #[parse(rule=Rule::command_done)]
    Done,
    #[parse(rule=Rule::overloaded_drea)]
    Drea { dream: Box<IntArg> },
    #[parse(rule=Rule::overloaded_driv)]
    Driv {
        drive: Box<IntArg>,
        adjustment: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_face)]
    Face { set_number: Box<IntArg> },
    #[parse(rule=Rule::command_forf)]
    Forf {
        creature_to_learn_about: Box<AgentArg>,
    },
    #[parse(rule=Rule::command_hair)]
    Hair { stage: Box<IntArg> },
    #[parse(rule=Rule::command_injr)]
    Injr {
        organ: Box<IntArg>,
        amount: Box<IntArg>,
    },
    #[parse(rule=Rule::command_like)]
    Like {
        creature_state_opinion_about: Box<AgentArg>,
    },
    #[parse(rule=Rule::overloaded_loci)]
    Loci {
        r#type: Box<IntArg>,
        organ: Box<IntArg>,
        tissue: Box<IntArg>,
        id: Box<IntArg>,
        new_value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_ltcy)]
    Ltcy {
        action: Box<IntArg>,
        min: Box<IntArg>,
        max: Box<IntArg>,
    },
    #[parse(rule=Rule::command_mate)]
    Mate,
    #[parse(rule=Rule::command_mvft)]
    Mvft { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::command_new_crea)]
    NewCrea {
        family: Box<IntArg>,
        gene_agent: Box<AgentArg>,
        gene_slot: Box<IntArg>,
        sex: Box<IntArg>,
        variant: Box<IntArg>,
    },
    #[parse(rule=Rule::command_newc)]
    Newc {
        family: Box<IntArg>,
        gene_agent: Box<AgentArg>,
        gene_slot: Box<IntArg>,
        sex: Box<IntArg>,
        variant: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_norn)]
    Norn { creature: Box<AgentArg> },
    #[parse(rule=Rule::command_nude)]
    Nude,
    #[parse(rule=Rule::command_ordr_shou)]
    OrdrShou { speech: Box<SStringArg> },
    #[parse(rule=Rule::command_ordr_sign)]
    OrdrSign { speech: Box<SStringArg> },
    #[parse(rule=Rule::command_ordr_writ)]
    OrdrWrit {
        creature: Box<AgentArg>,
        speech: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_sayn)]
    Sayn,
    #[parse(rule=Rule::command_spnl)]
    Spnl {
        lobe_monkier: Box<SStringArg>,
        neuron_id: Box<IntArg>,
        value: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_stim_shou)]
    StimShou {
        stimulus: Box<IntArg>,
        strength: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_stim_sign)]
    StimSign {
        stimulus: Box<IntArg>,
        strength: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_stim_tact)]
    StimTact {
        stimulus: Box<IntArg>,
        strength: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_stim_writ)]
    StimWrit {
        creature: Box<AgentArg>,
        stimulus: Box<IntArg>,
        strength: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_sway_shou)]
    SwayShou {
        drive1: Box<IntArg>,
        adjust1: Box<FloatArg>,
        drive2: Box<IntArg>,
        adjust2: Box<FloatArg>,
        drive3: Box<IntArg>,
        adjust3: Box<FloatArg>,
        drive4: Box<IntArg>,
        adjust4: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_sway_sign)]
    SwaySign {
        drive1: Box<IntArg>,
        adjust1: Box<FloatArg>,
        drive2: Box<IntArg>,
        adjust2: Box<FloatArg>,
        drive3: Box<IntArg>,
        adjust3: Box<FloatArg>,
        drive4: Box<IntArg>,
        adjust4: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_sway_tact)]
    SwayTact {
        drive1: Box<IntArg>,
        adjust1: Box<FloatArg>,
        drive2: Box<IntArg>,
        adjust2: Box<FloatArg>,
        drive3: Box<IntArg>,
        adjust3: Box<FloatArg>,
        drive4: Box<IntArg>,
        adjust4: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_sway_writ)]
    SwayWrit {
        creature: Box<AgentArg>,
        drive1: Box<IntArg>,
        adjust1: Box<FloatArg>,
        drive2: Box<IntArg>,
        adjust2: Box<FloatArg>,
        drive3: Box<IntArg>,
        adjust3: Box<FloatArg>,
        drive4: Box<IntArg>,
        adjust4: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_touc)]
    Touc,
    #[parse(rule=Rule::overloaded_uncs)]
    Uncs { unconscious: Box<IntArg> },
    #[parse(rule=Rule::command_urge_shou)]
    UrgeShou {
        noun_stim: Box<FloatArg>,
        verb_id: Box<IntArg>,
        verb_stim: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_urge_sign)]
    UrgeSign {
        noun_stim: Box<FloatArg>,
        verb_id: Box<IntArg>,
        verb_stim: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_urge_tact)]
    UrgeTact {
        noun_stim: Box<FloatArg>,
        verb_id: Box<IntArg>,
        verb_stim: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_urge_writ)]
    UrgeWrit {
        creature: Box<AgentArg>,
        noun_id: Box<IntArg>,
        noun_stim: Box<FloatArg>,
        verb_id: Box<IntArg>,
        verb_stim: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_vocb)]
    Vocb,
    #[parse(rule=Rule::command_walk)]
    Walk,
    #[parse(rule=Rule::command_wear)]
    Wear {
        body_id: Box<IntArg>,
        set_number: Box<IntArg>,
        layer: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_zomb)]
    Zomb { zombie: Box<IntArg> },
    #[parse(rule=Rule::command_apro)]
    // Debug
    Apro { search_text: Box<SStringArg> },
    #[parse(rule=Rule::command_dbg_cpro)]
    DbgCpro,
    #[parse(rule=Rule::command_dbg_flsh)]
    DbgFlsh,
    #[parse(rule=Rule::command_dbg_html)]
    DbgHtml { sort_order: Box<IntArg> },
    #[parse(rule=Rule::command_dbg_outs)]
    DbgOuts { value: Box<SStringArg> },
    #[parse(rule=Rule::command_dbg_outv)]
    DbgOutv { value: Box<DecimalArg> },
    #[parse(rule=Rule::command_dbg_paws)]
    DbgPaws,
    #[parse(rule=Rule::command_dbg_play)]
    DbgPlay,
    #[parse(rule=Rule::command_dbg_poll)]
    DbgPoll,
    #[parse(rule=Rule::command_dbg_prof)]
    DbgProf,
    #[parse(rule=Rule::command_dbg_tack)]
    DbgTack { follow: Box<AgentArg> },
    #[parse(rule=Rule::command_dbg_tock)]
    DbgTock,
    #[parse(rule=Rule::command_dbg_wtik)]
    DbgWtik { new_world_tick: Box<IntArg> },
    #[parse(rule=Rule::command_help)]
    Help,
    #[parse(rule=Rule::command_mann)]
    Mann { command: Box<SStringArg> },
    #[parse(rule=Rule::command_memx)]
    Memx,
    // Files
    #[parse(rule=Rule::command_file_glob)]
    FileGlob {
        directory: Box<IntArg>,
        file_spec: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_file_iclo)]
    FileIclo,
    #[parse(rule=Rule::command_file_iope)]
    FileIope {
        directory: Box<IntArg>,
        filename: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_file_jdel)]
    FileJdel {
        directory: Box<IntArg>,
        filename: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_file_oclo)]
    FileOclo,
    #[parse(rule=Rule::command_file_oflu)]
    FileOflu,
    #[parse(rule=Rule::command_file_oope)]
    FileOope {
        directory: Box<IntArg>,
        filename: Box<SStringArg>,
        append: Box<IntArg>,
    },
    #[parse(rule=Rule::command_outs)]
    Outs { text: Box<SStringArg> },
    #[parse(rule=Rule::command_outv)]
    Outv { value: Box<DecimalArg> },
    #[parse(rule=Rule::command_outx)]
    Outx { text: Box<SStringArg> },
    // Genetics
    #[parse(rule=Rule::command_gene_clon)]
    GeneClon {
        dest_agent: Box<AgentArg>,
        dest_slot: Box<IntArg>,
        source_agent: Box<AgentArg>,
        source_slot: Box<IntArg>,
    },
    #[parse(rule=Rule::command_gene_cros)]
    GeneCros {
        child_agent: Box<AgentArg>,
        child_slot: Box<IntArg>,
        mum_agent: Box<AgentArg>,
        mum_slot: Box<IntArg>,
        dad_agent: Box<AgentArg>,
        dad_slot: Box<IntArg>,
        mum_chance_of_mutation: Box<IntArg>,
        mum_degree_of_mutation: Box<IntArg>,
        dad_chance_of_mutation: Box<IntArg>,
        dad_degree_of_mutation: Box<IntArg>,
    },
    #[parse(rule=Rule::command_gene_kill)]
    GeneKill {
        agent: Box<AgentArg>,
        slot: Box<IntArg>,
    },
    #[parse(rule=Rule::command_gene_load)]
    GeneLoad {
        agent: Box<AgentArg>,
        slot: Box<IntArg>,
        gene_file: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_gene_move)]
    GeneMove {
        dest_agent: Box<AgentArg>,
        dest_slot: Box<IntArg>,
        source_agent: Box<AgentArg>,
        source_slot: Box<IntArg>,
    },
    // History
    #[parse(rule=Rule::command_hist_evnt)]
    HistEvnt {
        moniker: Box<SStringArg>,
        event_type: Box<IntArg>,
        related_moniker_1: Box<SStringArg>,
        related_moniker_2: Box<SStringArg>,
    },
    #[parse(rule=Rule::overloaded_hist_foto)]
    HistFoto {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
        new_value: Box<SStringArg>,
    },
    #[parse(rule=Rule::overloaded_hist_name)]
    HistName {
        moniker: Box<SStringArg>,
        new_name: Box<SStringArg>,
    },
    #[parse(rule=Rule::overloaded_hist_utxt)]
    HistUtxt {
        moniker: Box<SStringArg>,
        event_no: Box<IntArg>,
        new_value: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_hist_wipe)]
    HistWipe { moniker: Box<SStringArg> },
    #[parse(rule=Rule::overloaded_clac)]
    Clac { message: Box<IntArg> },
    #[parse(rule=Rule::overloaded_clik)]
    Clik {
        message_1: Box<IntArg>,
        message_2: Box<IntArg>,
        message_3: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_imsk)]
    Imsk { mask: Box<IntArg> },
    #[parse(rule=Rule::command_mous)]
    Mous { behaviour: Box<IntArg> },
    #[parse(rule=Rule::overloaded_pure)]
    Pure { value: Box<IntArg> },
    #[parse(rule=Rule::command_tran)]
    Tran {
        transparency: Box<IntArg>,
        part_no: Box<IntArg>,
    },
    // Map
    #[parse(rule=Rule::command_addb)]
    Addb {
        metaroom_id: Box<IntArg>,
        background_file: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_altr)]
    Altr {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        ca_delta: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_cacl)]
    Cacl {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        ca_index: Box<IntArg>,
    },
    #[parse(rule=Rule::command_calc)]
    Calc,
    #[parse(rule=Rule::command_delm)]
    Delm { metaroom_id: Box<IntArg> },
    #[parse(rule=Rule::command_delr)]
    Delr { room_id: Box<IntArg> },
    #[parse(rule=Rule::command_dmap)]
    Dmap { debug_map: Box<IntArg> },
    #[parse(rule=Rule::command_doca)]
    Doca { no_of_updates: Box<IntArg> },
    #[parse(rule=Rule::overloaded_door)]
    Door {
        room_id1: Box<IntArg>,
        room_id2: Box<IntArg>,
        permiability: Box<IntArg>,
    },
    #[parse(rule=Rule::command_emit)]
    Emit {
        ca_index: Box<IntArg>,
        amount: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_link)]
    Link {
        room1: Box<IntArg>,
        room2: Box<IntArg>,
        permiability: Box<IntArg>,
    },
    #[parse(rule=Rule::command_mapd)]
    Mapd {
        width: Box<IntArg>,
        height: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_mapk)]
    Mapk,
    #[parse(rule=Rule::overloaded_perm)]
    Perm { permiability: Box<IntArg> },
    #[parse(rule=Rule::overloaded_prop)]
    Prop {
        room_id: Box<IntArg>,
        ca_index: Box<IntArg>,
        ca_value: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_rate)]
    Rate {
        room_type: Box<IntArg>,
        ca_index: Box<IntArg>,
        gain: Box<FloatArg>,
        loss: Box<FloatArg>,
        diffusion: Box<FloatArg>,
    },
    #[parse(rule=Rule::overloaded_rtyp)]
    Rtyp {
        room_id: Box<IntArg>,
        room_type: Box<IntArg>,
    },
    // Motion
    #[parse(rule=Rule::overloaded_accg)]
    Accg { acceleration: Box<FloatArg> },
    #[parse(rule=Rule::overloaded_aero)]
    Aero { aerodynamics: Box<IntArg> },
    #[parse(rule=Rule::overloaded_elas)]
    Elas { elasticity: Box<IntArg> },
    #[parse(rule=Rule::command_flto)]
    Flto {
        screen_x: Box<FloatArg>,
        screen_y: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_frel)]
    Frel { relative: Box<AgentArg> },
    #[parse(rule=Rule::overloaded_fric)]
    Fric { friction: Box<IntArg> },
    #[parse(rule=Rule::command_mvby)]
    Mvby {
        delta_x: Box<FloatArg>,
        delta_y: Box<FloatArg>,
    },
    #[parse(rule=Rule::command_mvsf)]
    Mvsf { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::command_mvto)]
    Mvto { x: Box<FloatArg>, y: Box<FloatArg> },
    #[parse(rule=Rule::command_velo)]
    Velo {
        x_velocity: Box<FloatArg>,
        y_velocity: Box<FloatArg>,
    },
    // Ports
    #[parse(rule=Rule::command_prt_bang)]
    PrtBang { bang_strength: Box<IntArg> },
    #[parse(rule=Rule::command_prt_inew)]
    PrtInew {
        id: Box<IntArg>,
        name: Box<SStringArg>,
        description: Box<SStringArg>,
        x: Box<IntArg>,
        y: Box<IntArg>,
        message_num: Box<IntArg>,
    },
    #[parse(rule=Rule::command_prt_izap)]
    PrtIzap { id: Box<IntArg> },
    #[parse(rule=Rule::command_prt_join)]
    PrtJoin {
        source_agent: Box<AgentArg>,
        output_id: Box<IntArg>,
        dest_agent: Box<AgentArg>,
        input_id: Box<IntArg>,
    },
    #[parse(rule=Rule::command_prt_krak)]
    PrtKrak {
        agent: Box<AgentArg>,
        in_or_out: Box<IntArg>,
        port_index: Box<IntArg>,
    },
    #[parse(rule=Rule::command_prt_onew)]
    PrtOnew {
        id: Box<IntArg>,
        name: Box<SStringArg>,
        description: Box<SStringArg>,
        x: Box<IntArg>,
        y: Box<IntArg>,
    },
    #[parse(rule=Rule::command_prt_ozap)]
    PrtOzap { id: Box<IntArg> },
    #[parse(rule=Rule::command_prt_send)]
    PrtSend {
        id: Box<IntArg>,
        data: Box<Anything>,
    },
    // Resources
    #[parse(rule=Rule::command_pray_garb)]
    PrayGarb { force: Box<IntArg> },
    #[parse(rule=Rule::command_pray_refr)]
    PrayRefr,
    // Scripts
    #[parse(rule=Rule::command_gids_fmly)]
    GidsFmly { family: Box<IntArg> },
    #[parse(rule=Rule::command_gids_gnus)]
    GidsGnus {
        family: Box<IntArg>,
        genus: Box<IntArg>,
    },
    #[parse(rule=Rule::command_gids_root)]
    GidsRoot,
    #[parse(rule=Rule::command_gids_spcs)]
    GidsSpcs {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::command_inst)]
    Inst,
    #[parse(rule=Rule::command_lock)]
    Lock,
    #[parse(rule=Rule::command_scrx)]
    Scrx {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        event: Box<IntArg>,
    },
    #[parse(rule=Rule::command_slow)]
    Slow,
    #[parse(rule=Rule::command_sorc)]
    Sorc {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        event: Box<IntArg>,
    },
    #[parse(rule=Rule::command_stop)]
    Stop,
    #[parse(rule=Rule::command_stpt)]
    Stpt,
    #[parse(rule=Rule::command_unlk)]
    Unlk,
    #[parse(rule=Rule::command_wait)]
    Wait { ticks: Box<IntArg> },
    // Sounds
    #[parse(rule=Rule::command_fade)]
    Fade,
    #[parse(rule=Rule::command_mclr)]
    Mclr { x: Box<IntArg>, y: Box<IntArg> },
    #[parse(rule=Rule::command_midi)]
    Midi { midi_file: Box<SStringArg> },
    #[parse(rule=Rule::command_mmsc)]
    Mmsc {
        x: Box<IntArg>,
        y: Box<IntArg>,
        track_name: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_rclr)]
    Rclr { x: Box<IntArg>, y: Box<IntArg> },
    #[parse(rule=Rule::overloaded_rmsc)]
    Rmsc {
        x: Box<IntArg>,
        y: Box<IntArg>,
        track_name: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_sezz)]
    Sezz { text: Box<SStringArg> },
    #[parse(rule=Rule::command_sndc)]
    Sndc { sound_file: Box<SStringArg> },
    #[parse(rule=Rule::command_snde)]
    Snde { sound_file: Box<SStringArg> },
    #[parse(rule=Rule::command_sndl)]
    Sndl { sound_file: Box<SStringArg> },
    #[parse(rule=Rule::command_sndq)]
    Sndq {
        sound_file: Box<SStringArg>,
        delay: Box<IntArg>,
    },
    #[parse(rule=Rule::command_stpc)]
    Stpc,
    #[parse(rule=Rule::command_strk)]
    Strk {
        latency: Box<IntArg>,
        track: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_voic)]
    Voic {
        genus: Box<IntArg>,
        gender: Box<IntArg>,
        age: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_vois)]
    Vois { voice_name: Box<SStringArg> },
    #[parse(rule=Rule::command_volm)]
    Volm { volume: Box<IntArg> },
    // Date
    #[parse(rule=Rule::overloaded_wpau)]
    Wpau { paused: Box<IntArg> },
    // Box<Variable>s
    #[parse(rule=Rule::command_absv)]
    Absv { var: Box<Variable> },
    #[parse(rule=Rule::command_adds)]
    Adds {
        var: Box<Variable>,
        append: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_addv)]
    Addv {
        var: Box<Variable>,
        sum: Box<DecimalArg>,
    },
    #[parse(rule=Rule::command_andv)]
    Andv {
        var: Box<Variable>,
        value: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_char)]
    Char {
        string: Box<Variable>,
        index: Box<IntArg>,
        character: Box<IntArg>,
    },
    #[parse(rule=Rule::command_delg)]
    Delg { variable_name: Box<SStringArg> },
    #[parse(rule=Rule::command_divv)]
    Divv {
        var: Box<Variable>,
        div: Box<DecimalArg>,
    },
    #[parse(rule=Rule::command_modv)]
    Modv {
        var: Box<Variable>,
        r#mod: Box<IntArg>,
    },
    #[parse(rule=Rule::command_mulv)]
    Mulv {
        var: Box<Variable>,
        mul: Box<DecimalArg>,
    },
    #[parse(rule=Rule::command_negv)]
    Negv { var: Box<Variable> },
    #[parse(rule=Rule::command_orrv)]
    Orrv {
        var: Box<Variable>,
        value: Box<IntArg>,
    },
    #[parse(rule=Rule::command_reaf)]
    Reaf,
    #[parse(rule=Rule::command_seta)]
    Seta {
        var: Box<Variable>,
        value: Box<AgentArg>,
    },
    #[parse(rule=Rule::command_sets)]
    Sets {
        var: Box<Variable>,
        value: Box<SStringArg>,
    },
    #[parse(rule=Rule::command_setv)]
    Setv {
        var: Box<Variable>,
        value: Box<DecimalArg>,
    },
    #[parse(rule=Rule::command_subv)]
    Subv {
        var: Box<Variable>,
        sub: Box<DecimalArg>,
    },
    #[parse(rule=Rule::overloaded_targ)]
    Targ { agent: Box<AgentArg> },
    // Vehicles
    #[parse(rule=Rule::command_cabn)]
    Cabn {
        left: Box<IntArg>,
        top: Box<IntArg>,
        right: Box<IntArg>,
        bottom: Box<IntArg>,
    },
    #[parse(rule=Rule::overloaded_cabp)]
    Cabp { plane: Box<IntArg> },
    #[parse(rule=Rule::overloaded_cabv)]
    Cabv { cabin_room_id: Box<IntArg> },
    #[parse(rule=Rule::command_cabw)]
    Cabw { cabin_capacity: Box<IntArg> },
    #[parse(rule=Rule::command_dpas)]
    Dpas {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
    },
    #[parse(rule=Rule::command_gpas)]
    Gpas {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        rect_to_use: Box<IntArg>,
    },
    #[parse(rule=Rule::command_new_vhcl)]
    NewVhcl {
        family: Box<IntArg>,
        genus: Box<IntArg>,
        species: Box<IntArg>,
        sprite_file: Box<SStringArg>,
        image_count: Box<IntArg>,
        first_image: Box<IntArg>,
        plane: Box<IntArg>,
    },
    #[parse(rule=Rule::command_rpas)]
    Rpas {
        vehicle: Box<AgentArg>,
        passenger: Box<AgentArg>,
    },
    #[parse(rule=Rule::command_spas)]
    Spas {
        vehicle: Box<AgentArg>,
        new_passenger: Box<AgentArg>,
    },
    // World
    #[parse(rule=Rule::command_delw)]
    Delw { world_name: Box<SStringArg> },
    #[parse(rule=Rule::command_load)]
    Load { world_name: Box<SStringArg> },
    #[parse(rule=Rule::overloaded_pswd)]
    Pswd { world_name: Box<SStringArg> },
    #[parse(rule=Rule::command_quit)]
    Quit,
    #[parse(rule=Rule::command_rgam)]
    Rgam,
    #[parse(rule=Rule::command_save)]
    Save,
    #[parse(rule=Rule::command_tntw)]
    Tntw { index: Box<IntArg> },
    #[parse(rule=Rule::overloaded_wrld)]
    Wrld { world_name: Box<SStringArg> },
    #[parse(rule=Rule::command_wtnt)]
    Wtnt {
        index: Box<IntArg>,
        red_tint: Box<IntArg>,
        green_tint: Box<IntArg>,
        blue_tint: Box<IntArg>,
        rotation: Box<IntArg>,
        swap: Box<IntArg>,
    },
}

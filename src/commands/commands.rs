mod evaluators;

use super::{
    Agent, Anything, ByteString, Condition, Decimal, FloatArg, IntArg, Label, SString, Variable,
};
use crate::parser::{caos_skippable1, CaosParsable, CaosParseResult, Script, ScriptDefinition};
use caos_macros::{CaosParsable, CommandList, EvaluateCommand};
use evaluators::*;
use nom::{
    bytes::complete::tag_no_case,
    combinator::{cut, opt},
    sequence::{terminated, tuple},
};

#[derive(CaosParsable, CommandList, EvaluateCommand, Eq, PartialEq, Clone, Debug)]
#[return_type(())]
pub enum Command {
    // Flow -- keep this near the top to keep stack size reduced in debug builds.
    #[syntax(with_parser = "parse_doif")]
    Doif {
        condition: Condition,
        definition: ScriptDefinition,
        else_definition: Option<ScriptDefinition>,
    },
    #[syntax(with_parser = "parse_subr")]
    Subr {
        label: Label,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_reps")]
    Reps {
        count: IntArg,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_econ")]
    Econ {
        agent: Agent,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_enum")]
    Enum {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_etch")]
    Etch {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_esee")]
    Esee {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    #[syntax(with_parser = "parse_epas")]
    Epas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    // Agents
    #[syntax]
    Anim { pose_list: ByteString },
    #[syntax]
    Anms { anim_string: SString },
    #[syntax]
    Attr { attributes: IntArg },
    #[syntax]
    Base { index: IntArg },
    #[syntax]
    Bhvr { permissions: IntArg },
    #[syntax]
    Frat { framerate: IntArg },
    #[syntax]
    Gait { gait_number: IntArg },
    #[syntax]
    Gall {
        sprite_file: SString,
        first_image: IntArg,
    },
    #[syntax]
    Hand { name_for_the_hand: SString },
    #[syntax]
    Kill { agent: Agent },
    #[syntax(name = "mesg writ")]
    MesgWrit { command: Agent, message_id: IntArg },
    #[syntax(name = "mesg wrt+")]
    MesgWritPlus {
        agent: Agent,
        message_id: IntArg,
        param_1: Anything,
        param_2: Anything,
        delay: IntArg,
    },
    #[syntax]
    Mira { on_off: IntArg },
    #[syntax(name = "new: simp")]
    NewSimp {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SString,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    #[syntax]
    Nohh,
    #[syntax]
    Over,
    #[syntax]
    Paus { paused: IntArg },
    #[syntax]
    Plne { plane: IntArg },
    #[syntax]
    Pose { pose: IntArg },
    #[syntax]
    Puhl { pose: IntArg, x: IntArg, y: IntArg },
    #[syntax]
    Pupt { pose: IntArg, x: IntArg, y: IntArg },
    #[syntax]
    Rnge { distance: FloatArg },
    #[syntax]
    Rtar {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    #[syntax]
    Show { visibility: IntArg },
    #[syntax]
    Star {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    #[syntax]
    Tick { tick_rate: IntArg },
    #[syntax]
    Tint {
        red_tint: IntArg,
        green_tint: IntArg,
        blue_tint: IntArg,
        rotation: IntArg,
        swap: IntArg,
    },
    #[syntax]
    Ttar {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    // Brain
    #[syntax(name = "brn: dmpb")]
    BrnDmpb,
    #[syntax(name = "brn: dmpd")]
    BrnDmpd {
        tract_number: IntArg,
        dendrite_number: IntArg,
    },
    #[syntax(name = "brn: dmpl")]
    BrnDmpl { lobe_number: IntArg },
    #[syntax(name = "brn: dmpn")]
    BrnDmpn {
        lobe_number: IntArg,
        neuron_number: IntArg,
    },
    #[syntax(name = "brn: dmpt")]
    BrnDmpt { tract_number: IntArg },
    #[syntax(name = "brn: setd")]
    BrnSetd {
        tract_number: IntArg,
        dendrite_number: IntArg,
        weight_number: IntArg,
        new_value: FloatArg,
    },
    #[syntax(name = "brn: setl")]
    BrntSetl {
        lobe_number: IntArg,
        line_number: IntArg,
        new_value: FloatArg,
    },
    #[syntax(name = "brn: setn")]
    BrnSetn {
        lobe_number: IntArg,
        neuron_number: IntArg,
        state_number: IntArg,
        new_value: FloatArg,
    },
    #[syntax(name = "brn: sett")]
    BrnSett {
        tract_number: IntArg,
        line_number: IntArg,
        new_value: FloatArg,
    },
    // Camera
    #[syntax]
    Bkgd {
        metaroom_id: IntArg,
        background: SString,
        transition: IntArg,
    },
    #[syntax]
    Brmi {
        mearoom_base: IntArg,
        room_base: IntArg,
    },
    #[syntax]
    Cmra { x: IntArg, y: IntArg, pan: IntArg },
    #[syntax]
    Cmrp { x: IntArg, y: IntArg, pan: IntArg },
    #[syntax]
    Cmrt { pan: IntArg },
    #[syntax]
    Frsh,
    #[syntax]
    Line {
        x1: IntArg,
        y1: IntArg,
        x2: IntArg,
        y2: IntArg,
        r: IntArg,
        g: IntArg,
        b: IntArg,
        stipple_on: IntArg,
        stipple_off: IntArg,
    },
    #[syntax]
    Meta {
        metaroom_id: IntArg,
        camera_x: IntArg,
        camera_y: IntArg,
        transition: IntArg,
    },
    #[syntax]
    Scam {
        compound_agent: Agent,
        part_number: IntArg,
    },
    #[syntax]
    Snap {
        filename: SString,
        x_centre: IntArg,
        y_centre: IntArg,
        width: IntArg,
        height: IntArg,
        zoom_factor: IntArg,
    },
    #[syntax]
    Trck {
        agent: Agent,
        x_percent: IntArg,
        y_percent: IntArg,
        style: IntArg,
        transition: IntArg,
    },
    #[syntax]
    Wdow,
    #[syntax]
    Zoom {
        pixels: IntArg,
        x: IntArg,
        y: IntArg,
    },
    // Compounds
    #[syntax]
    Fcus,
    #[syntax]
    Frmt {
        left_margin: IntArg,
        top_margin: IntArg,
        right_margin: IntArg,
        bottom_margin: IntArg,
        line_spacing: IntArg,
        character_spacing: IntArg,
        justification: IntArg,
    },
    #[syntax]
    Grpl {
        red: IntArg,
        green: IntArg,
        blue: IntArg,
        min_y: FloatArg,
        max_y: FloatArg,
    },
    #[syntax]
    Grpv { line_index: IntArg, value: FloatArg },
    #[syntax(name = "new: comp")]
    NewComp {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SString,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    #[syntax]
    Page { page: IntArg },
    #[syntax]
    Part { part_id: IntArg },
    #[syntax(name = "pat: butt")]
    PatButt {
        part_id: IntArg,
        sprite_file: SString,
        first_image: IntArg,
        image_count: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: IntArg,
        anim_hover: ByteString,
        message_id: IntArg,
        option: IntArg,
    },
    #[syntax(name = "pat: cmra")]
    PatCmra {
        part_id: IntArg,
        overlay_sprinte: SString,
        base_image: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: IntArg,
        view_width: IntArg,
        view_height: IntArg,
        camera_width: IntArg,
        camera_height: IntArg,
    },
    #[syntax(name = "pat: dull")]
    PatDull {
        part_id: IntArg,
        sprite_file: SString,
        first_image: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: IntArg,
    },
    #[syntax(name = "pat: fixd")]
    PatFixd {
        part_id: IntArg,
        sprinte_file: SString,
        first_image: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: IntArg,
        font_sprite: SString,
    },
    #[syntax(name = "pat: grph")]
    PatGrph {
        part_id: IntArg,
        overlay_sprite: SString,
        base_image: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Decimal,
        num_values: IntArg,
    },
    #[syntax(name = "pat: kill")]
    PatKill { part_id: IntArg },
    #[syntax(name = "pat: text")]
    PatText {
        part_id: IntArg,
        sprite_file: SString,
        first_image: IntArg,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: IntArg,
        message_id: IntArg,
        font_sprite: SString,
    },
    #[syntax]
    Ptxt { text: SString },
    // Creates
    #[syntax]
    Ages { times: IntArg },
    #[syntax]
    Appr,
    #[syntax]
    Aslp { asleep: IntArg },
    #[syntax]
    Body { set_number: IntArg, layer: IntArg },
    #[syntax]
    Born,
    #[syntax]
    Chem {
        chemical: IntArg,
        adjustment: FloatArg,
    },
    #[syntax]
    Dead,
    #[syntax]
    Dirn { direction: IntArg },
    #[syntax]
    Done,
    #[syntax]
    Drea { dream: IntArg },
    #[syntax]
    Driv { drive: IntArg, adjustment: FloatArg },
    #[syntax]
    Face { set_number: IntArg },
    #[syntax]
    Forf { creature_to_learn_about: Agent },
    #[syntax]
    Hair { stage: IntArg },
    #[syntax]
    Injr { organ: IntArg, amount: IntArg },
    #[syntax]
    Like { creature_state_opinion_about: Agent },
    #[syntax]
    Loci {
        r#type: IntArg,
        organ: IntArg,
        tissue: IntArg,
        id: IntArg,
        new_value: FloatArg,
    },
    #[syntax]
    Ltcy {
        action: IntArg,
        min: IntArg,
        max: IntArg,
    },
    #[syntax]
    Mate,
    #[syntax]
    Mvft { x: FloatArg, y: FloatArg },
    #[syntax(name = "new: crea")]
    NewCrea {
        family: IntArg,
        gene_agent: Agent,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    #[syntax]
    Newc {
        family: IntArg,
        gene_agent: Agent,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    #[syntax]
    Norn { creature: Agent },
    #[syntax]
    Nude,
    #[syntax(name = "ordr shou")]
    OrdrShou { speech: SString },
    #[syntax(name = "ordr sign")]
    OrdrSign { speech: SString },
    #[syntax(name = "ordr writ")]
    OrdrWrit { crature: Agent, speech: SString },
    #[syntax]
    Sayn,
    #[syntax]
    Spnl {
        lobe_monkier: SString,
        neuron_id: IntArg,
        value: FloatArg,
    },
    #[syntax(name = "stim shou")]
    StimShou {
        stimulus: IntArg,
        strength: FloatArg,
    },
    #[syntax(name = "stim sign")]
    StimSign {
        stimulus: IntArg,
        strength: FloatArg,
    },
    #[syntax(name = "stim tact")]
    StimTact {
        stimulus: IntArg,
        strength: FloatArg,
    },
    #[syntax(name = "stim writ")]
    StimWrit {
        creature: Agent,
        stimulus: IntArg,
        strength: FloatArg,
    },
    #[syntax(name = "sway shou")]
    SwayShou {
        drive1: IntArg,
        adjust1: FloatArg,
        drive2: IntArg,
        adjust2: FloatArg,
        drive3: IntArg,
        adjust3: FloatArg,
        drive4: IntArg,
        adjust4: FloatArg,
    },
    #[syntax(name = "sway sign")]
    SwaySign {
        drive1: IntArg,
        adjust1: FloatArg,
        drive2: IntArg,
        adjust2: FloatArg,
        drive3: IntArg,
        adjust3: FloatArg,
        drive4: IntArg,
        adjust4: FloatArg,
    },
    #[syntax(name = "sway tact")]
    SwayTact {
        drive1: IntArg,
        adjust1: FloatArg,
        drive2: IntArg,
        adjust2: FloatArg,
        drive3: IntArg,
        adjust3: FloatArg,
        drive4: IntArg,
        adjust4: FloatArg,
    },
    #[syntax(name = "sway writ")]
    SwayWrit {
        creature: Agent,
        drive1: IntArg,
        adjust1: FloatArg,
        drive2: IntArg,
        adjust2: FloatArg,
        drive3: IntArg,
        adjust3: FloatArg,
        drive4: IntArg,
        adjust4: FloatArg,
    },
    #[syntax]
    Touc,
    #[syntax]
    Uncs { unconscious: IntArg },
    #[syntax(name = "urge shou")]
    UrgeShou {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    #[syntax(name = "urge sign")]
    UrgeSign {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    #[syntax(name = "urge tact")]
    UrgeTact {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    #[syntax(name = "urge writ")]
    UrgeWrit {
        creature: Agent,
        noun_id: IntArg,
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    #[syntax]
    Vocb,
    #[syntax]
    Walk,
    #[syntax]
    Wear {
        body_id: IntArg,
        set_number: IntArg,
        layer: IntArg,
    },
    #[syntax]
    Zomb { zombie: IntArg },
    // Debug
    #[syntax]
    Apro { search_text: SString },
    #[syntax(name = "dbg: asrt")]
    DbgAsrt { condition: Condition },
    #[syntax(name = "dbg: cpro")]
    DbgCpro,
    #[syntax(name = "dbg: flsh")]
    DbgFlsh,
    #[syntax(name = "dbg: html")]
    DbgHtml { sort_order: IntArg },
    #[syntax(name = "dbg: outs")]
    DbgOuts { value: SString },
    #[syntax(name = "dbg: outv")]
    DbgOutv { value: Decimal },
    #[syntax(name = "dbg: paws")]
    DbgPaws,
    #[syntax(name = "dbg: play")]
    DbgPlay,
    #[syntax(name = "dbg: poll")]
    DbgPoll,
    #[syntax(name = "dbg: prof")]
    DbgProf,
    #[syntax(name = "dbg: tack")]
    DbgTack { follow: Agent },
    #[syntax(name = "dbg: tock")]
    DbTock,
    #[syntax(name = "dbg: wtik")]
    DbgWtik { new_world_tick: IntArg },
    #[syntax]
    Help,
    #[syntax]
    Mann { command: SString },
    #[syntax]
    Memx,
    // Files
    #[syntax(name = "file glob")]
    FileGlob {
        directory: IntArg,
        file_spec: SString,
    },
    #[syntax(name = "file iclo")]
    FileIclo,
    #[syntax(name = "file iope")]
    FileIope {
        directory: IntArg,
        filename: SString,
    },
    #[syntax(name = "file jdel")]
    FileJdel {
        directory: IntArg,
        filename: SString,
    },
    #[syntax(name = "file oclo")]
    FileOclo,
    #[syntax(name = "file oflu")]
    FileOflu,
    #[syntax(name = "file oope")]
    FileOope {
        directory: IntArg,
        filename: SString,
        append: IntArg,
    },
    #[syntax]
    Outs { text: SString },
    #[syntax]
    Outv { value: Decimal },
    #[syntax]
    Outx { text: SString },
    #[syntax]
    Ever,
    #[syntax]
    Goto { destination: Label },
    #[syntax]
    Gsub { destination: Label },
    #[syntax]
    Loop,
    #[syntax]
    Untl { condition: Condition },
    // Genetics
    #[syntax(name = "gene clon")]
    GeneClon {
        dest_agent: Agent,
        dest_slot: IntArg,
        source_agent: Agent,
        source_slot: IntArg,
    },
    #[syntax(name = "gene cros")]
    GeneCros {
        child_agent: Agent,
        child_slot: IntArg,
        mum_agent: Agent,
        mum_slot: IntArg,
        dad_agent: Agent,
        dad_slot: IntArg,
        mum_chance_of_mutation: IntArg,
        mum_degree_of_mutation: IntArg,
        dad_chance_of_mutation: IntArg,
        dad_degree_of_mutation: IntArg,
    },
    #[syntax(name = "gene kill")]
    GeneKill { agent: Agent, slot: IntArg },
    #[syntax(name = "gene load")]
    GeneLoad {
        agent: Agent,
        slot: IntArg,
        gene_file: SString,
    },
    #[syntax(name = "gene move")]
    GeneMove {
        dest_agent: Agent,
        dest_slot: IntArg,
        source_agent: Agent,
        source_slot: IntArg,
    },
    // History
    #[syntax(name = "hist evnt")]
    HistEvnt {
        moniker: SString,
        event_type: IntArg,
        related_moniker_1: SString,
        related_moniker_2: SString,
    },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: SString,
        event_no: IntArg,
        new_value: SString,
    },
    #[syntax(name = "hist name")]
    HistName { moniker: SString, new_name: SString },
    #[syntax(name = "hist utxt")]
    HistUtxt {
        moniker: SString,
        event_no: IntArg,
        new_value: SString,
    },
    #[syntax(name = "hist wipe")]
    HistWipe { moniker: SString },
    #[syntax]
    Clac { message: IntArg },
    #[syntax]
    Clik {
        message_1: IntArg,
        message_2: IntArg,
        message_3: IntArg,
    },
    #[syntax]
    Imsk { mask: IntArg },
    #[syntax]
    Mous { behaviour: IntArg },
    #[syntax]
    Pure { value: IntArg },
    #[syntax]
    Tran {
        transparency: IntArg,
        part_no: IntArg,
    },
    // Map
    #[syntax]
    Addb {
        metaroom_id: IntArg,
        background_file: SString,
    },
    #[syntax]
    Altr {
        room_id: IntArg,
        ca_index: IntArg,
        ca_delta: FloatArg,
    },
    #[syntax]
    Cacl {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        ca_index: IntArg,
    },
    #[syntax]
    Delm { metaroom_id: IntArg },
    #[syntax]
    Delr { room_id: IntArg },
    #[syntax]
    Dmap { debug_map: IntArg },
    #[syntax]
    Doca { no_of_updates: IntArg },
    #[syntax]
    Door {
        room_id1: IntArg,
        room_id2: IntArg,
        permiability: IntArg,
    },
    #[syntax]
    Emit { ca_index: IntArg, amount: FloatArg },
    #[syntax]
    Link {
        room1: IntArg,
        room2: IntArg,
        permiability: IntArg,
    },
    #[syntax]
    Mapd { width: IntArg, height: IntArg },
    #[syntax]
    Mapk,
    #[syntax]
    Perm { permiability: IntArg },
    #[syntax]
    Prop {
        room_id: IntArg,
        ca_index: IntArg,
        ca_value: FloatArg,
    },
    #[syntax]
    Rate {
        room_type: IntArg,
        ca_index: IntArg,
        gain: FloatArg,
        loss: FloatArg,
        diffusion: FloatArg,
    },
    #[syntax]
    Rtyp { room_id: IntArg, room_type: IntArg },
    // Motion
    #[syntax]
    Accg { acceleration: FloatArg },
    #[syntax]
    Aero { aerodynamics: IntArg },
    #[syntax]
    Elas { elasticity: IntArg },
    #[syntax]
    Flto {
        screen_x: FloatArg,
        screen_y: FloatArg,
    },
    #[syntax]
    Frel { relative: Agent },
    #[syntax]
    Fric { friction: IntArg },
    #[syntax]
    Mvby {
        delta_x: FloatArg,
        delta_y: FloatArg,
    },
    #[syntax]
    Mvsf { x: FloatArg, y: FloatArg },
    #[syntax]
    Mvto { x: FloatArg, y: FloatArg },
    #[syntax]
    Velo {
        x_velocity: FloatArg,
        y_velocity: FloatArg,
    },
    // Ports
    #[syntax(name = "prt: bang")]
    PrtBang { bang_strength: IntArg },
    #[syntax(name = "prt: inew")]
    PrtInew {
        id: IntArg,
        name: SString,
        description: SString,
        x: IntArg,
        y: IntArg,
        message_num: IntArg,
    },
    #[syntax(name = "prt: izap")]
    PrtIzap { id: IntArg },
    #[syntax(name = "prt: join")]
    PrtJoin {
        source_agent: Agent,
        output_id: IntArg,
        dest_agent: IntArg,
        input_id: IntArg,
    },
    #[syntax(name = "prt: krak")]
    PrtKrak {
        agent: Agent,
        in_or_out: IntArg,
        port_index: IntArg,
    },
    #[syntax(name = "prt: name")]
    PrtName {
        agent: Agent,
        in_or_out: IntArg,
        port_index: IntArg,
    },
    #[syntax(name = "prt: onew")]
    PrtOnew {
        id: IntArg,
        name: SString,
        description: SString,
        x: IntArg,
        y: IntArg,
    },
    #[syntax(name = "prt: otot")]
    PrtOtot,
    #[syntax(name = "prt: ozap")]
    PrtOzap { id: IntArg },
    #[syntax(name = "prt: send")]
    PrtSend { id: IntArg, data: Anything },
    // Resources
    #[syntax(name = "pray grab")]
    PrayGrab { force: IntArg },
    #[syntax(name = "pray refr")]
    PrayRefr,
    // Scripts
    #[syntax(name = "gids fmly")]
    GidsFmly { family: IntArg },
    #[syntax(name = "gids gnus")]
    GidsGnus { family: IntArg, genus: IntArg },
    #[syntax(name = "gids root")]
    GidsRoot,
    #[syntax(name = "gids spcs")]
    GidsSpcs {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    #[syntax]
    Inst,
    #[syntax]
    Lock,
    #[syntax]
    Scrx {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        event: IntArg,
    },
    #[syntax]
    Slow,
    #[syntax]
    Sorc {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        event: IntArg,
    },
    #[syntax]
    Stop,
    #[syntax]
    Stpt,
    #[syntax]
    Unlk,
    #[syntax]
    Wait { ticks: IntArg },
    // Sounds
    #[syntax]
    Fade,
    #[syntax]
    Mclr { x: IntArg, y: IntArg },
    #[syntax]
    Midi { midi_file: SString },
    #[syntax]
    Mmsc {
        x: IntArg,
        y: IntArg,
        track_name: SString,
    },
    #[syntax]
    Rclr { x: IntArg, y: IntArg },
    #[syntax]
    Rmsc {
        x: IntArg,
        y: IntArg,
        track_name: SString,
    },
    #[syntax]
    Sezz { text: SString },
    #[syntax]
    Sndc { sound_file: SString },
    #[syntax]
    Snde { sound_file: SString },
    #[syntax]
    Sndl { sound_file: SString },
    #[syntax]
    Sndq { sound_file: SString, delay: IntArg },
    #[syntax]
    Stpc,
    #[syntax]
    Strk { latency: IntArg, track: SString },
    #[syntax]
    Voic {
        genus: IntArg,
        gender: IntArg,
        age: IntArg,
    },
    #[syntax]
    Vois { voice_name: SString },
    #[syntax]
    Volm { volume: IntArg },
    // Date
    #[syntax]
    Wpau { paused: IntArg },
    // Variables
    #[syntax]
    Absv { var: Variable },
    #[syntax]
    Adds { var: Variable, append: SString },
    #[syntax]
    Addv { var: Variable, sum: Decimal },
    #[syntax]
    Andv { var: Variable, value: IntArg },
    #[syntax]
    Char {
        string: Variable,
        index: IntArg,
        character: IntArg,
    },
    #[syntax]
    Delg { variable_name: SString },
    #[syntax]
    Divv { var: Variable, div: Decimal },
    #[syntax]
    Modv { var: Variable, r#mod: IntArg },
    #[syntax]
    Mulv { var: Variable, mul: Decimal },
    #[syntax]
    Negv { var: Variable },
    #[syntax]
    Orrv { var: Variable, value: IntArg },
    #[syntax]
    Reaf,
    #[syntax]
    Seta { var: Variable, value: Agent },
    #[syntax]
    Sets { var: Variable, value: SString },
    #[syntax]
    Setv { var: Variable, value: Decimal },
    #[syntax]
    Subv { var: Variable, sub: Decimal },
    #[syntax]
    Targ { agent: Agent },
    // Vehicles
    #[syntax]
    Cabn {
        left: IntArg,
        top: IntArg,
        right: IntArg,
        bottom: IntArg,
    },
    #[syntax]
    Cabp { plane: IntArg },
    #[syntax]
    Cabv { cabin_room_id: IntArg },
    #[syntax]
    Cabw { cabin_capacity: IntArg },
    #[syntax]
    Dpas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    #[syntax]
    Gpas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        rect_to_use: IntArg,
    },
    #[syntax(name = "new: vhcl")]
    NewVhcl {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SString,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    #[syntax]
    Rpas { vehicle: Agent, passenger: Agent },
    #[syntax]
    Spas {
        vehicle: Agent,
        new_passanger: Agent,
    },
    // World
    #[syntax]
    Delw { world_name: SString },
    #[syntax]
    Load { world_name: SString },
    #[syntax]
    Pswd { world_name: SString },
    #[syntax]
    Quit,
    #[syntax]
    Rgam,
    #[syntax]
    Save,
    #[syntax]
    Tntw { index: IntArg },
    #[syntax]
    Wrld { world_name: SString },
    #[syntax]
    Wtnt {
        index: IntArg,
        red_tint: IntArg,
        green_tint: IntArg,
        blue_tint: IntArg,
        rotation: IntArg,
        swap: IntArg,
    },
}

fn parse_doif(input: &str) -> CaosParseResult<&str, Command> {
    let (input, command) = parse_doif_impl(input, "doif")?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, _) = cut(tag_no_case("endi"))(input)?;
    Ok((input, command))
}

fn parse_elif(input: &str) -> CaosParseResult<&str, Command> {
    parse_doif_impl(input, "elif")
}

fn parse_doif_impl<'a>(input: &'a str, tag: &str) -> CaosParseResult<&'a str, Command> {
    let (input, _) = tag_no_case(tag)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (mut input, condition) = cut(Condition::parse_caos)(input)?;
    let (input2, _) = cut(caos_skippable1)(input)?;
    let (input2, definition) = cut(Script::parse_definition)(input2)?;

    // Don't overconsume spaces.
    if !definition.is_empty() {
        input = input2;
    }

    let mut else_definition: Option<ScriptDefinition> = None;

    let (mut input, elif) = opt(tuple((caos_skippable1, parse_elif)))(input)?;

    if let Some((_, elif)) = elif {
        else_definition = Some(ScriptDefinition {
            commands: vec![elif],
        });
    } else {
        let (input2, e) = opt(tuple((caos_skippable1, tag_no_case("else"))))(input)?;
        if e.is_some() {
            let (input3, _) = cut(caos_skippable1)(input2)?;
            let (input3, d) = cut(Script::parse_definition)(input3)?;
            if !d.is_empty() {
                else_definition = Some(d);
                input = input3;
            } else {
                input = input2;
            }
        }
    }
    Ok((
        input,
        Command::Doif {
            condition,
            definition,
            else_definition,
        },
    ))
}

fn parse_subr(input: &str) -> CaosParseResult<&str, Command> {
    let (input, _) = tag_no_case("subr")(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, label) = cut(Label::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, definition) = cut(terminated(
        Script::parse_definition,
        tuple((caos_skippable1, tag_no_case("retn"))),
    ))(input)?;
    Ok((input, Command::Subr { label, definition }))
}

fn parse_reps(input: &str) -> CaosParseResult<&str, Command> {
    let (input, _) = tag_no_case("reps")(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, count) = cut(IntArg::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, definition) = cut(terminated(
        Script::parse_definition,
        tuple((caos_skippable1, tag_no_case("repe"))),
    ))(input)?;
    Ok((input, Command::Reps { count, definition }))
}

fn parse_enum_impl<'a>(
    input: &'a str,
    tag: &str,
) -> CaosParseResult<&'a str, (IntArg, IntArg, IntArg, ScriptDefinition)> {
    let (input, _) = tag_no_case(tag)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, family) = cut(IntArg::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, genus) = cut(IntArg::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, species) = cut(IntArg::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, definition) = cut(terminated(
        Script::parse_definition,
        tuple((caos_skippable1, tag_no_case("next"))),
    ))(input)?;
    Ok((input, (family, genus, species, definition)))
}

fn parse_enum<'a>(input: &'a str) -> CaosParseResult<&'a str, Command> {
    let (input, (family, genus, species, definition)) = parse_enum_impl(input, "enum")?;
    Ok((
        input,
        Command::Enum {
            family,
            genus,
            species,
            definition,
        },
    ))
}

fn parse_etch<'a>(input: &'a str) -> CaosParseResult<&'a str, Command> {
    let (input, (family, genus, species, definition)) = parse_enum_impl(input, "etch")?;
    Ok((
        input,
        Command::Etch {
            family,
            genus,
            species,
            definition,
        },
    ))
}

fn parse_esee<'a>(input: &'a str) -> CaosParseResult<&'a str, Command> {
    let (input, (family, genus, species, definition)) = parse_enum_impl(input, "esee")?;
    Ok((
        input,
        Command::Esee {
            family,
            genus,
            species,
            definition,
        },
    ))
}

fn parse_epas<'a>(input: &'a str) -> CaosParseResult<&'a str, Command> {
    let (input, (family, genus, species, definition)) = parse_enum_impl(input, "epas")?;
    Ok((
        input,
        Command::Epas {
            family,
            genus,
            species,
            definition,
        },
    ))
}

fn parse_econ(input: &str) -> CaosParseResult<&str, Command> {
    let (input, _) = tag_no_case("econ")(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, agent) = cut(Agent::parse_caos)(input)?;
    let (input, _) = cut(caos_skippable1)(input)?;
    let (input, definition) = cut(terminated(
        Script::parse_definition,
        tuple((caos_skippable1, tag_no_case("next"))),
    ))(input)?;
    Ok((input, Command::Econ { agent, definition }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_subr() {
        let input = "subr foo inst retn";
        let (_, result) = Command::parse_caos(input).expect("Successful parse");

        assert_eq!(
            result,
            Command::Subr {
                label: Label::from(String::from("foo")),
                definition: ScriptDefinition {
                    commands: vec![Command::Inst]
                }
            }
        )
    }

    #[test]
    fn test_parse_subr_recusrive() {
        let input = "subr foo subr bar inst retn retn";
        let (_, result) = Command::parse_caos(input).expect("Successful parse");

        assert_eq!(
            result,
            Command::Subr {
                label: Label::from(String::from("foo")),
                definition: ScriptDefinition {
                    commands: vec![Command::Subr {
                        label: Label::from(String::from("bar")),
                        definition: ScriptDefinition {
                            commands: vec![Command::Inst]
                        }
                    }]
                }
            }
        )
    }
}

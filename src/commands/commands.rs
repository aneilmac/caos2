use super::{
    Agent, Anything, ByteString, Condition, Decimal, Float, Integer, Label, LiteralInt, SString,
    Variable,
};
use crate::parser::CaosParseResult;
use caos_macros::{CaosParsable, CommandList};

#[derive(CaosParsable, CommandList, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    // Script
    #[syntax]
    Scrp {
        family: LiteralInt,
        genus: LiteralInt,
        species: LiteralInt,
        script_number: LiteralInt,
    },
    #[syntax]
    Endm,
    #[syntax]
    Iscr,
    #[syntax]
    Rscr,
    #[syntax]
    Drft,
    // Agents
    #[syntax]
    Anim { pose_list: ByteString },
    #[syntax]
    Anms { anim_string: SString },
    #[syntax]
    Attr { attributes: Integer },
    #[syntax]
    Base { index: Integer },
    #[syntax]
    Bhvr { permissions: Integer },
    #[syntax]
    #[syntax]
    Enum {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Esee {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Etch {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Frat { framerate: Integer },
    #[syntax]
    Gait { gait_number: Integer },
    #[syntax]
    Gall {
        sprite_file: SString,
        first_image: Integer,
    },
    #[syntax]
    Hand { name_for_the_hand: SString },
    #[syntax]
    Kill { agent: Agent },
    #[syntax(name = "mesg writ")]
    MesgWrit { command: Agent, message_id: Integer },
    #[syntax(name = "mesg wrt+")]
    MesgWritPlus {
        agent: Agent,
        message_id: Integer,
        param_1: Anything,
        param_2: Anything,
        delay: Integer,
    },
    #[syntax]
    Mira { on_off: Integer },
    #[syntax(name = "new: simp")]
    NewSimp {
        family: Integer,
        genus: Integer,
        species: Integer,
        sprite_file: SString,
        image_count: Integer,
        first_image: Integer,
        plane: Integer,
    },
    #[syntax]
    Next,
    #[syntax]
    Nohh,
    #[syntax]
    Over,
    #[syntax]
    Paus { paused: Integer },
    #[syntax]
    Plne { plane: Integer },
    #[syntax]
    Pose { pose: Integer },
    #[syntax]
    Puhl {
        pose: Integer,
        x: Integer,
        y: Integer,
    },
    #[syntax]
    Pupt {
        pose: Integer,
        x: Integer,
        y: Integer,
    },
    #[syntax]
    Rnge { distance: Float },
    #[syntax]
    Rtar {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Show { visibility: Integer },
    #[syntax]
    Star {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Tick { tick_rate: Integer },
    #[syntax]
    Tint {
        red_tint: Integer,
        green_tint: Integer,
        blue_tint: Integer,
        rotation: Integer,
        swap: Integer,
    },
    #[syntax]
    Ttar {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    // Brain
    #[syntax(name = "brn: dmpb")]
    BrnDmpb,
    #[syntax(name = "brn: dmpd")]
    BrnDmpd {
        tract_number: Integer,
        dendrite_number: Integer,
    },
    #[syntax(name = "brn: dmpl")]
    BrnDmpl { lobe_number: Integer },
    #[syntax(name = "brn: dmpn")]
    BrnDmpn {
        lobe_number: Integer,
        neuron_number: Integer,
    },
    #[syntax(name = "brn: dmpt")]
    BrnDmpt { tract_number: Integer },
    #[syntax(name = "brn: setd")]
    BrnSetd {
        tract_number: Integer,
        dendrite_number: Integer,
        weight_number: Integer,
        new_value: Float,
    },
    #[syntax(name = "brn: setl")]
    BrntSetl {
        lobe_number: Integer,
        line_number: Integer,
        new_value: Float,
    },
    #[syntax(name = "brn: setn")]
    BrnSetn {
        lobe_number: Integer,
        neuron_number: Integer,
        state_number: Integer,
        new_value: Float,
    },
    #[syntax(name = "brn: sett")]
    BrnSett {
        tract_number: Integer,
        line_number: Integer,
        new_value: Float,
    },
    // Camera
    #[syntax]
    Bkgd {
        metaroom_id: Integer,
        background: SString,
        transition: Integer,
    },
    #[syntax]
    Brmi {
        mearoom_base: Integer,
        room_base: Integer,
    },
    #[syntax]
    Cmra {
        x: Integer,
        y: Integer,
        pan: Integer,
    },
    #[syntax]
    Cmrp {
        x: Integer,
        y: Integer,
        pan: Integer,
    },
    #[syntax]
    Cmrt { pan: Integer },
    #[syntax]
    Frsh,
    #[syntax]
    Line {
        x1: Integer,
        y1: Integer,
        x2: Integer,
        y2: Integer,
        r: Integer,
        g: Integer,
        b: Integer,
        stipple_on: Integer,
        stipple_off: Integer,
    },
    #[syntax]
    Meta {
        metaroom_id: Integer,
        camera_x: Integer,
        camera_y: Integer,
        transition: Integer,
    },
    #[syntax]
    Scam {
        compound_agent: Agent,
        part_number: Integer,
    },
    #[syntax]
    Snap {
        filename: SString,
        x_centre: Integer,
        y_centre: Integer,
        width: Integer,
        height: Integer,
        zoom_factor: Integer,
    },
    #[syntax]
    Trck {
        agent: Agent,
        x_percent: Integer,
        y_percent: Integer,
        style: Integer,
        transition: Integer,
    },
    #[syntax]
    Wdow,
    #[syntax]
    Zoom {
        pixels: Integer,
        x: Integer,
        y: Integer,
    },
    // Compounds
    #[syntax]
    Fcus,
    #[syntax]
    Frmt {
        left_margin: Integer,
        top_margin: Integer,
        right_margin: Integer,
        bottom_margin: Integer,
        line_spacing: Integer,
        character_spacing: Integer,
        justification: Integer,
    },
    #[syntax]
    Grpl {
        red: Integer,
        green: Integer,
        blue: Integer,
        min_y: Float,
        max_y: Float,
    },
    #[syntax]
    Grpv { line_index: Integer, value: Float },
    #[syntax(name = "new: comp")]
    NewComp {
        family: Integer,
        genus: Integer,
        species: Integer,
        sprite_file: SString,
        image_count: Integer,
        first_image: Integer,
        plane: Integer,
    },
    #[syntax]
    Page { page: Integer },
    #[syntax]
    Part { part_id: Integer },
    #[syntax(name = "pat: butt")]
    PatButt {
        part_id: Integer,
        sprite_file: SString,
        first_image: Integer,
        image_count: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Integer,
        anim_hover: ByteString,
        message_id: Integer,
        option: Integer,
    },
    #[syntax(name = "pat: cmra")]
    PatCmra {
        part_id: Integer,
        overlay_sprinte: SString,
        base_image: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Integer,
        view_width: Integer,
        view_height: Integer,
        camera_width: Integer,
        camera_height: Integer,
    },
    #[syntax(name = "pat: dull")]
    PatDull {
        part_id: Integer,
        sprite_file: SString,
        first_image: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Integer,
    },
    #[syntax(name = "pat: fixd")]
    PatFixd {
        part_id: Integer,
        sprinte_file: SString,
        first_image: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Integer,
        font_sprite: SString,
    },
    #[syntax(name = "pat: grph")]
    PatGrph {
        part_id: Integer,
        overlay_sprite: SString,
        base_image: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Decimal,
        num_values: Integer,
    },
    #[syntax(name = "pat: kill")]
    PatKill { part_id: Integer },
    #[syntax(name = "pat: text")]
    PatText {
        part_id: Integer,
        sprite_file: SString,
        first_image: Integer,
        rel_x: Decimal,
        rel_y: Decimal,
        rel_plane: Integer,
        message_id: Integer,
        font_sprite: SString,
    },
    #[syntax]
    Ptxt { text: SString },
    // Creates
    #[syntax]
    Ages { times: Integer },
    #[syntax]
    Appr,
    #[syntax]
    Aslp { asleep: Integer },
    #[syntax]
    Body { set_number: Integer, layer: Integer },
    #[syntax]
    Born,
    #[syntax]
    Chem {
        chemical: Integer,
        adjustment: Float,
    },
    #[syntax]
    Dead,
    #[syntax]
    Dirn { direction: Integer },
    #[syntax]
    Done,
    #[syntax]
    Drea { dream: Integer },
    #[syntax]
    Driv { drive: Integer, adjustment: Float },
    #[syntax]
    Face { set_number: Integer },
    #[syntax]
    Forf { creature_to_learn_about: Agent },
    #[syntax]
    Hair { stage: Integer },
    #[syntax]
    Injr { organ: Integer, amount: Integer },
    #[syntax]
    Like { creature_state_opinion_about: Agent },
    #[syntax]
    Loci {
        r#type: Integer,
        organ: Integer,
        tissue: Integer,
        id: Integer,
        new_value: Float,
    },
    #[syntax]
    Ltcy {
        action: Integer,
        min: Integer,
        max: Integer,
    },
    #[syntax]
    Mate,
    #[syntax]
    Mvft { x: Float, y: Float },
    #[syntax(name = "new: crea")]
    NewCrea {
        family: Integer,
        gene_agent: Agent,
        gene_slot: Integer,
        sex: Integer,
        variant: Integer,
    },
    #[syntax]
    Newc {
        family: Integer,
        gene_agent: Agent,
        gene_slot: Integer,
        sex: Integer,
        variant: Integer,
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
        neuron_id: Integer,
        value: Float,
    },
    #[syntax(name = "stim shou")]
    StimShou { stimulus: Integer, strength: Float },
    #[syntax(name = "stim sign")]
    StimSign { stimulus: Integer, strength: Float },
    #[syntax(name = "stim tact")]
    StimTact { stimulus: Integer, strength: Float },
    #[syntax(name = "stim writ")]
    StimWrit {
        creature: Agent,
        stimulus: Integer,
        strength: Float,
    },
    #[syntax(name = "sway shou")]
    SwayShou {
        drive1: Integer,
        adjust1: Float,
        drive2: Integer,
        adjust2: Float,
        drive3: Integer,
        adjust3: Float,
        drive4: Integer,
        adjust4: Float,
    },
    #[syntax(name = "sway sign")]
    SwaySign {
        drive1: Integer,
        adjust1: Float,
        drive2: Integer,
        adjust2: Float,
        drive3: Integer,
        adjust3: Float,
        drive4: Integer,
        adjust4: Float,
    },
    #[syntax(name = "sway tact")]
    SwayTact {
        drive1: Integer,
        adjust1: Float,
        drive2: Integer,
        adjust2: Float,
        drive3: Integer,
        adjust3: Float,
        drive4: Integer,
        adjust4: Float,
    },
    #[syntax(name = "sway tact")]
    SwayWrit {
        creature: Agent,
        drive1: Integer,
        adjust1: Float,
        drive2: Integer,
        adjust2: Float,
        drive3: Integer,
        adjust3: Float,
        drive4: Integer,
        adjust4: Float,
    },
    #[syntax]
    Touc,
    #[syntax]
    Uncs { unconscious: Integer },
    #[syntax(name = "urge shou")]
    UrgeShou {
        noun_stim: Float,
        verb_id: Integer,
        verb_stim: Float,
    },
    #[syntax(name = "urge sign")]
    UrgeSign {
        noun_stim: Float,
        verb_id: Integer,
        verb_stim: Float,
    },
    #[syntax(name = "urge tact")]
    UrgeTact {
        noun_stim: Float,
        verb_id: Integer,
        verb_stim: Float,
    },
    #[syntax(name = "urge writ")]
    UrgeWrit {
        creature: Agent,
        noun_id: Integer,
        noun_stim: Float,
        verb_id: Integer,
        verb_stim: Float,
    },
    #[syntax]
    Vocab,
    #[syntax]
    Walk,
    #[syntax]
    Wear {
        body_id: Integer,
        set_number: Integer,
        layer: Integer,
    },
    #[syntax]
    Zomb { zombie: Integer },
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
    DbgHtml { sort_order: Integer },
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
    DbgWtik { new_world_tick: Integer },
    #[syntax]
    Help,
    #[syntax]
    Mann { command: SString },
    #[syntax]
    Memx,
    // Files
    #[syntax(name = "file glob")]
    FileGlob {
        directory: Integer,
        file_spec: SString,
    },
    #[syntax(name = "file iclo")]
    FileIclo,
    #[syntax(name = "file iope")]
    FileIope {
        directory: Integer,
        filename: SString,
    },
    #[syntax(name = "file jdel")]
    FileJdel {
        directory: Integer,
        filename: SString,
    },
    #[syntax(name = "file oclo")]
    FileOclo,
    #[syntax(name = "file oflu")]
    FileOflu,
    #[syntax(name = "file oope")]
    FileOope {
        directory: Integer,
        filename: SString,
        append: Integer,
    },
    #[syntax]
    Outs { text: SString },
    #[syntax]
    Outv { value: Decimal },
    #[syntax]
    Outx { text: SString },
    // Flow
    #[syntax]
    Doif { condition: Condition },
    #[syntax]
    Elif { condition: Condition },
    #[syntax]
    Else,
    #[syntax]
    Endi,
    #[syntax]
    Ever,
    #[syntax]
    Goto { destination: Label },
    #[syntax]
    Gsub { destination: Label },
    #[syntax]
    Loop,
    #[syntax]
    Repe,
    #[syntax]
    Reps { count: Integer },
    #[syntax]
    Retn,
    #[syntax]
    Subr { label: Label },
    #[syntax]
    Untl { condition: Condition },
    // Genetics
    #[syntax(name = "gene clon")]
    GeneClon {
        dest_agent: Agent,
        dest_slot: Integer,
        source_agent: Agent,
        source_slot: Integer,
    },
    #[syntax(name = "gene cros")]
    GeneCros {
        child_agent: Agent,
        child_slot: Integer,
        mum_agent: Agent,
        mum_slot: Integer,
        dad_agent: Agent,
        dad_slot: Integer,
        mum_chance_of_mutation: Integer,
        mum_degree_of_mutation: Integer,
        dad_chance_of_mutation: Integer,
        dad_degree_of_mutation: Integer,
    },
    #[syntax(name = "gene kill")]
    GeneKill { agent: Agent, slot: Integer },
    #[syntax(name = "gene load")]
    GeneLoad {
        agent: Agent,
        slot: Integer,
        gene_file: SString,
    },
    #[syntax(name = "gene move")]
    GeneMove {
        dest_agent: Agent,
        dest_slot: Integer,
        source_agent: Agent,
        source_slot: Integer,
    },
    // History
    #[syntax(name = "hist evnt")]
    HistEvnt {
        moniker: SString,
        event_type: Integer,
        related_moniker_1: SString,
        related_moniker_2: SString,
    },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: SString,
        event_no: Integer,
        new_value: SString,
    },
    #[syntax(name = "hist name")]
    HistName { moniker: SString, new_name: SString },
    #[syntax(name = "hist utxt")]
    HistUtxt {
        moniker: SString,
        event_no: Integer,
        new_value: SString,
    },
    #[syntax(name = "hist wipe")]
    HistWipe { moniker: SString },
    #[syntax]
    Clac { message: Integer },
    #[syntax]
    Clik {
        message_1: Integer,
        message_2: Integer,
        message_3: Integer,
    },
    #[syntax]
    Imsk { mask: Integer },
    #[syntax]
    Mous { behaviour: Integer },
    #[syntax]
    Pure { value: Integer },
    #[syntax]
    Tran {
        transparency: Integer,
        part_no: Integer,
    },
    // Map
    #[syntax]
    Addb {
        metaroom_id: Integer,
        background_file: SString,
    },
    #[syntax]
    Altr {
        room_id: Integer,
        ca_index: Integer,
        ca_delta: Float,
    },
    #[syntax]
    Calc {
        family: Integer,
        genus: Integer,
        species: Integer,
        ca_index: Integer,
    },
    #[syntax]
    Delm { metaroom_id: Integer },
    #[syntax]
    Delr { room_id: Integer },
    #[syntax]
    Dmap { debug_map: Integer },
    #[syntax]
    Doca { no_of_updates: Integer },
    #[syntax]
    Door {
        room_id1: Integer,
        room_id2: Integer,
        permiability: Integer,
    },
    #[syntax]
    Emit { ca_index: Integer, amount: Float },
    #[syntax]
    Link {
        room1: Integer,
        room2: Integer,
        permiability: Integer,
    },
    #[syntax]
    Mapd { width: Integer, height: Integer },
    #[syntax]
    Mapk,
    #[syntax]
    Perm { permiability: Integer },
    #[syntax]
    Prop {
        room_id: Integer,
        ca_index: Integer,
        ca_value: Float,
    },
    #[syntax]
    Rate {
        room_type: Integer,
        ca_index: Integer,
        gain: Float,
        loss: Float,
        diffusion: Float,
    },
    #[syntax]
    Rtyp {
        room_id: Integer,
        room_type: Integer,
    },
    // Motion
    #[syntax]
    Accg { acceleration: Float },
    #[syntax]
    Aero { aerodynamics: Integer },
    #[syntax]
    Elas { elasticity: Integer },
    #[syntax]
    Flto { screen_x: Float, screen_y: Float },
    #[syntax]
    Frel { relative: Agent },
    #[syntax]
    Fric { friction: Integer },
    #[syntax]
    Mvby { delta_x: Float, delta_y: Float },
    #[syntax]
    Mvsf { x: Float, y: Float },
    #[syntax]
    Mvto { x: Float, y: Float },
    #[syntax]
    Velo {
        x_velocity: Float,
        y_velocity: Float,
    },
    // Ports
    #[syntax]
    Econ { agent: Agent },
    #[syntax(name = "prt: bang")]
    PrtBang { bang_strength: Integer },
    #[syntax(name = "prt: inew")]
    PrtInew {
        id: Integer,
        name: SString,
        description: SString,
        x: Integer,
        y: Integer,
        message_num: Integer,
    },
    #[syntax(name = "prt: izap")]
    PrtIzap { id: Integer },
    #[syntax(name = "prt: join")]
    PrtJoin {
        source_agent: Agent,
        output_id: Integer,
        dest_agent: Integer,
        input_id: Integer,
    },
    #[syntax(name = "prt: krak")]
    PrtKrak {
        agent: Agent,
        in_or_out: Integer,
        port_index: Integer,
    },
    #[syntax(name = "prt: name")]
    PrtName {
        agent: Agent,
        in_or_out: Integer,
        port_index: Integer,
    },
    #[syntax(name = "prt: onew")]
    PrtOnew {
        id: Integer,
        name: SString,
        description: SString,
        x: Integer,
        y: Integer,
    },
    #[syntax(name = "prt: otot")]
    PrtOtot,
    #[syntax(name = "prt: ozap")]
    PrtOzap { id: Integer },
    #[syntax(name = "prt: send")]
    PrtSend { id: Integer, data: Anything },
    // Resources
    #[syntax(name = "pray grab")]
    PrayGrab { force: Integer },
    #[syntax(name = "pray refr")]
    PrayRefr,
    // Scripts
    #[syntax(name = "gids fmly")]
    GidsFmly { family: Integer },
    #[syntax(name = "gids gnus")]
    GidsGnus { family: Integer, genus: Integer },
    #[syntax(name = "gids root")]
    GidsRoot,
    #[syntax(name = "gids spcs")]
    GidsSpcs {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Inst,
    #[syntax]
    Lock,
    #[syntax]
    Scrx {
        family: Integer,
        genus: Integer,
        species: Integer,
        event: Integer,
    },
    #[syntax]
    Slow,
    #[syntax]
    Sorc {
        family: Integer,
        genus: Integer,
        species: Integer,
        event: Integer,
    },
    #[syntax]
    Stop,
    #[syntax]
    Stpt,
    #[syntax]
    Unlk,
    #[syntax]
    Wait { ticks: Integer },
    // Sounds
    #[syntax]
    Fade,
    #[syntax]
    Mclr { x: Integer, y: Integer },
    #[syntax]
    Midi { midi_file: SString },
    #[syntax]
    Mmsc {
        x: Box<Integer>,
        y: Box<Integer>,
        track_name: Box<SString>,
    },
    #[syntax]
    Rclr { x: Integer, y: Integer },
    #[syntax]
    Rmsc {
        x: Integer,
        y: Integer,
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
    Sndq { sound_file: SString, delay: Integer },
    #[syntax]
    Stpc,
    #[syntax]
    Strk { latency: Integer, track: SString },
    #[syntax]
    Voic { voice_name: SString },
    #[syntax]
    Volm { volume: Integer },
    // Date
    #[syntax]
    Wpau { paused: Integer },
    // Variables
    #[syntax]
    Absv { var: Variable },
    #[syntax]
    Adds { var: Variable, append: SString },
    #[syntax]
    Addv { var: Variable, sum: Decimal },
    #[syntax]
    Andv { var: Variable, value: Integer },
    #[syntax]
    Char {
        string: Variable,
        index: Integer,
        character: Integer,
    },
    #[syntax]
    Delg { variable_name: SString },
    #[syntax]
    Divv { var: Variable, div: Decimal },
    #[syntax]
    Modv { var: Variable, r#mod: Integer },
    #[syntax]
    Mulv { var: Variable, mul: Decimal },
    #[syntax]
    Negv { var: Variable },
    #[syntax]
    Orrv { var: Variable, value: Integer },
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
        left: Integer,
        top: Integer,
        right: Integer,
        bottom: Integer,
    },
    #[syntax]
    Cabp { plane: Integer },
    #[syntax]
    Cabv { cabin_room_id: Integer },
    #[syntax]
    Cabw { cabin_capacity: Integer },
    #[syntax]
    Dpas {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Epas {
        family: Integer,
        genus: Integer,
        species: Integer,
    },
    #[syntax]
    Gpas {
        family: Integer,
        genus: Integer,
        species: Integer,
        rect_to_use: Integer,
    },
    #[syntax(name = "new: vhcl")]
    NewVhcl {
        family: Integer,
        genus: Integer,
        species: Integer,
        sprite_file: SString,
        image_count: Integer,
        first_image: Integer,
        plane: Integer,
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
    Tntw { index: Integer },
    #[syntax]
    Wrld { world_name: SString },
    #[syntax]
    Wtnt {
        index: Integer,
        red_tint: Integer,
        green_tint: Integer,
        blue_tint: Integer,
        rotation: Integer,
        swap: Integer,
    },
}

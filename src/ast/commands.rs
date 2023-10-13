use super::{
    AgentArg, Anything, ByteString, Condition, DecimalArg, FloatArg, IntArg, Label, SStringArg,
    ScriptDefinition, Variable,
};
use caos_macros::{CaosParsable, CommandList, EvaluateCommand};

#[derive(CaosParsable, Eq, PartialEq, Clone, Debug)]
pub enum Command {
    // Flow -- keep this near the top to keep stack size reduced in debug builds.
    #[syntax(with_parser = "parse_doif")]
    Doif {
        condition: Condition,
        definition: ScriptDefinition,
        elif_definitions: Vec<(Condition, ScriptDefinition)>,
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
    #[syntax]
    LoopEver { definition: ScriptDefinition },
    #[syntax]
    LoopUntl {
        definition: ScriptDefinition,
        condition: Condition,
    },
    #[syntax(with_parser = "parse_econ")]
    Econ {
        agent: AgentArg,
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
    // AgentArgs
    #[syntax]
    Anim { pose_list: ByteString },
    #[syntax]
    Anms { anim_string: SStringArg },
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
        sprite_file: SStringArg,
        first_image: IntArg,
    },
    #[syntax]
    Hand { name_for_the_hand: SStringArg },
    #[syntax]
    Kill { agent: AgentArg },
    #[syntax(name = "mesg writ")]
    MesgWrit { agent: AgentArg, message_id: IntArg },
    #[syntax(name = "mesg wrt+")]
    MesgWritPlus {
        agent: AgentArg,
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
        sprite_file: SStringArg,
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
    BrnSetl {
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
        background: SStringArg,
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
        compound_agent: AgentArg,
        part_number: IntArg,
    },
    #[syntax]
    Snap {
        filename: SStringArg,
        x_centre: IntArg,
        y_centre: IntArg,
        width: IntArg,
        height: IntArg,
        zoom_factor: IntArg,
    },
    #[syntax]
    Trck {
        agent: AgentArg,
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
        sprite_file: SStringArg,
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
        sprite_file: SStringArg,
        first_image: IntArg,
        image_count: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        anim_hover: ByteString,
        message_id: IntArg,
        option: IntArg,
    },
    #[syntax(name = "pat: cmra")]
    PatCmra {
        part_id: IntArg,
        overlay_sprite: SStringArg,
        base_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        view_width: IntArg,
        view_height: IntArg,
        camera_width: IntArg,
        camera_height: IntArg,
    },
    #[syntax(name = "pat: dull")]
    PatDull {
        part_id: IntArg,
        sprite_file: SStringArg,
        first_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
    },
    #[syntax(name = "pat: fixd")]
    PatFixd {
        part_id: IntArg,
        sprite_file: SStringArg,
        first_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        font_sprite: SStringArg,
    },
    #[syntax(name = "pat: grph")]
    PatGrph {
        part_id: IntArg,
        overlay_sprite: SStringArg,
        base_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        num_values: IntArg,
    },
    #[syntax(name = "pat: kill")]
    PatKill { part_id: IntArg },
    #[syntax(name = "pat: text")]
    PatText {
        part_id: IntArg,
        sprite_file: SStringArg,
        first_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        message_id: IntArg,
        font_sprite: SStringArg,
    },
    #[syntax]
    Ptxt { text: SStringArg },
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
    Forf { creature_to_learn_about: AgentArg },
    #[syntax]
    Hair { stage: IntArg },
    #[syntax]
    Injr { organ: IntArg, amount: IntArg },
    #[syntax]
    Like {
        creature_state_opinion_about: AgentArg,
    },
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
        gene_agent: AgentArg,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    #[syntax]
    Newc {
        family: IntArg,
        gene_agent: AgentArg,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    #[syntax]
    Norn { creature: AgentArg },
    #[syntax]
    Nude,
    #[syntax(name = "ordr shou")]
    OrdrShou { speech: SStringArg },
    #[syntax(name = "ordr sign")]
    OrdrSign { speech: SStringArg },
    #[syntax(name = "ordr writ")]
    OrdrWrit {
        creature: AgentArg,
        speech: SStringArg,
    },
    #[syntax]
    Sayn,
    #[syntax]
    Spnl {
        lobe_monkier: SStringArg,
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
        creature: AgentArg,
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
        creature: AgentArg,
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
        creature: AgentArg,
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
    Apro { search_text: SStringArg },
    #[syntax(name = "dbg: asrt")]
    DbgAsrt { condition: Condition },
    #[syntax(name = "dbg: cpro")]
    DbgCpro,
    #[syntax(name = "dbg: flsh")]
    DbgFlsh,
    #[syntax(name = "dbg: html")]
    DbgHtml { sort_order: IntArg },
    #[syntax(name = "dbg: outs")]
    DbgOuts { value: SStringArg },
    #[syntax(name = "dbg: outv")]
    DbgOutv { value: DecimalArg },
    #[syntax(name = "dbg: paws")]
    DbgPaws,
    #[syntax(name = "dbg: play")]
    DbgPlay,
    #[syntax(name = "dbg: poll")]
    DbgPoll,
    #[syntax(name = "dbg: prof")]
    DbgProf,
    #[syntax(name = "dbg: tack")]
    DbgTack { follow: AgentArg },
    #[syntax(name = "dbg: tock")]
    DbgTock,
    #[syntax(name = "dbg: wtik")]
    DbgWtik { new_world_tick: IntArg },
    #[syntax]
    Help,
    #[syntax]
    Mann { command: SStringArg },
    #[syntax]
    Memx,
    // Files
    #[syntax(name = "file glob")]
    FileGlob {
        directory: IntArg,
        file_spec: SStringArg,
    },
    #[syntax(name = "file iclo")]
    FileIclo,
    #[syntax(name = "file iope")]
    FileIope {
        directory: IntArg,
        filename: SStringArg,
    },
    #[syntax(name = "file jdel")]
    FileJdel {
        directory: IntArg,
        filename: SStringArg,
    },
    #[syntax(name = "file oclo")]
    FileOclo,
    #[syntax(name = "file oflu")]
    FileOflu,
    #[syntax(name = "file oope")]
    FileOope {
        directory: IntArg,
        filename: SStringArg,
        append: IntArg,
    },
    #[syntax]
    Outs { text: SStringArg },
    #[syntax]
    Outv { value: DecimalArg },
    #[syntax]
    Outx { text: SStringArg },
    #[syntax]
    Goto { destination: Label },
    #[syntax]
    Gsub { destination: Label },
    // Genetics
    #[syntax(name = "gene clon")]
    GeneClon {
        dest_agent: AgentArg,
        dest_slot: IntArg,
        source_agent: AgentArg,
        source_slot: IntArg,
    },
    #[syntax(name = "gene cros")]
    GeneCros {
        child_agent: AgentArg,
        child_slot: IntArg,
        mum_agent: AgentArg,
        mum_slot: IntArg,
        dad_agent: AgentArg,
        dad_slot: IntArg,
        mum_chance_of_mutation: IntArg,
        mum_degree_of_mutation: IntArg,
        dad_chance_of_mutation: IntArg,
        dad_degree_of_mutation: IntArg,
    },
    #[syntax(name = "gene kill")]
    GeneKill { agent: AgentArg, slot: IntArg },
    #[syntax(name = "gene load")]
    GeneLoad {
        agent: AgentArg,
        slot: IntArg,
        gene_file: SStringArg,
    },
    #[syntax(name = "gene move")]
    GeneMove {
        dest_agent: AgentArg,
        dest_slot: IntArg,
        source_agent: AgentArg,
        source_slot: IntArg,
    },
    // History
    #[syntax(name = "hist evnt")]
    HistEvnt {
        moniker: SStringArg,
        event_type: IntArg,
        related_moniker_1: SStringArg,
        related_moniker_2: SStringArg,
    },
    #[syntax(name = "hist foto")]
    HistFoto {
        moniker: SStringArg,
        event_no: IntArg,
        new_value: SStringArg,
    },
    #[syntax(name = "hist name")]
    HistName {
        moniker: SStringArg,
        new_name: SStringArg,
    },
    #[syntax(name = "hist utxt")]
    HistUtxt {
        moniker: SStringArg,
        event_no: IntArg,
        new_value: SStringArg,
    },
    #[syntax(name = "hist wipe")]
    HistWipe { moniker: SStringArg },
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
        background_file: SStringArg,
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
    Frel { relative: AgentArg },
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
        name: SStringArg,
        description: SStringArg,
        x: IntArg,
        y: IntArg,
        message_num: IntArg,
    },
    #[syntax(name = "prt: izap")]
    PrtIzap { id: IntArg },
    #[syntax(name = "prt: join")]
    PrtJoin {
        source_agent: AgentArg,
        output_id: IntArg,
        dest_agent: AgentArg,
        input_id: IntArg,
    },
    #[syntax(name = "prt: krak")]
    PrtKrak {
        agent: AgentArg,
        in_or_out: IntArg,
        port_index: IntArg,
    },
    #[syntax(name = "prt: onew")]
    PrtOnew {
        id: IntArg,
        name: SStringArg,
        description: SStringArg,
        x: IntArg,
        y: IntArg,
    },
    #[syntax(name = "prt: ozap")]
    PrtOzap { id: IntArg },
    #[syntax(name = "prt: send")]
    PrtSend { id: IntArg, data: Anything },
    // Resources
    #[syntax(name = "pray grab")]
    PrayGarb { force: IntArg },
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
    Midi { midi_file: SStringArg },
    #[syntax]
    Mmsc {
        x: IntArg,
        y: IntArg,
        track_name: SStringArg,
    },
    #[syntax]
    Rclr { x: IntArg, y: IntArg },
    #[syntax]
    Rmsc {
        x: IntArg,
        y: IntArg,
        track_name: SStringArg,
    },
    #[syntax]
    Sezz { text: SStringArg },
    #[syntax]
    Sndc { sound_file: SStringArg },
    #[syntax]
    Snde { sound_file: SStringArg },
    #[syntax]
    Sndl { sound_file: SStringArg },
    #[syntax]
    Sndq {
        sound_file: SStringArg,
        delay: IntArg,
    },
    #[syntax]
    Stpc,
    #[syntax]
    Strk { latency: IntArg, track: SStringArg },
    #[syntax]
    Voic {
        genus: IntArg,
        gender: IntArg,
        age: IntArg,
    },
    #[syntax]
    Vois { voice_name: SStringArg },
    #[syntax]
    Volm { volume: IntArg },
    // Date
    #[syntax]
    Wpau { paused: IntArg },
    // Variables
    #[syntax]
    Absv { var: Variable },
    #[syntax]
    Adds { var: Variable, append: SStringArg },
    #[syntax]
    Addv { var: Variable, sum: DecimalArg },
    #[syntax]
    Andv { var: Variable, value: IntArg },
    #[syntax]
    Char {
        string: Variable,
        index: IntArg,
        character: IntArg,
    },
    #[syntax]
    Delg { variable_name: SStringArg },
    #[syntax]
    Divv { var: Variable, div: DecimalArg },
    #[syntax]
    Modv { var: Variable, r#mod: IntArg },
    #[syntax]
    Mulv { var: Variable, mul: DecimalArg },
    #[syntax]
    Negv { var: Variable },
    #[syntax]
    Orrv { var: Variable, value: IntArg },
    #[syntax]
    Reaf,
    #[syntax]
    Seta { var: Variable, value: AgentArg },
    #[syntax]
    Sets { var: Variable, value: SStringArg },
    #[syntax]
    Setv { var: Variable, value: DecimalArg },
    #[syntax]
    Subv { var: Variable, sub: DecimalArg },
    #[syntax]
    Targ { agent: AgentArg },
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
        sprite_file: SStringArg,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    #[syntax]
    Rpas {
        vehicle: AgentArg,
        passenger: AgentArg,
    },
    #[syntax]
    Spas {
        vehicle: AgentArg,
        new_passenger: AgentArg,
    },
    // World
    #[syntax]
    Delw { world_name: SStringArg },
    #[syntax]
    Load { world_name: SStringArg },
    #[syntax]
    Pswd { world_name: SStringArg },
    #[syntax]
    Quit,
    #[syntax]
    Rgam,
    #[syntax]
    Save,
    #[syntax]
    Tntw { index: IntArg },
    #[syntax]
    Wrld { world_name: SStringArg },
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

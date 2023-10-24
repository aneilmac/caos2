use super::{
    AgentArg, Anything, ByteString, Condition, DecimalArg, FloatArg, IntArg, Label, SStringArg,
    ScriptDefinition, Variable,
};
#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Command {
    // Flow -- keep this near the top to keep stack size reduced in debug builds.
    Doif {
        condition: Condition,
        definition: ScriptDefinition,
        elif_definitions: Vec<(Condition, ScriptDefinition)>,
        else_definition: Option<ScriptDefinition>,
    },
    Subr {
        label: Label,
        definition: ScriptDefinition,
    },
    Reps {
        count: IntArg,
        definition: ScriptDefinition,
    },
    LoopEver {
        definition: ScriptDefinition,
    },
    LoopUntl {
        definition: ScriptDefinition,
        condition: Condition,
    },
    Econ {
        agent: AgentArg,
        definition: ScriptDefinition,
    },
    Enum {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    Etch {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    Esee {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    Epas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        definition: ScriptDefinition,
    },
    // AgentArgs
    Anim {
        pose_list: ByteString,
    },
    Anms {
        anim_string: SStringArg,
    },
    Attr {
        attributes: IntArg,
    },
    Base {
        index: IntArg,
    },
    Bhvr {
        permissions: IntArg,
    },
    Frat {
        framerate: IntArg,
    },
    Gait {
        gait_number: IntArg,
    },
    Gall {
        sprite_file: SStringArg,
        first_image: IntArg,
    },
    Hand {
        name_for_the_hand: SStringArg,
    },
    Kill {
        agent: AgentArg,
    },
    MesgWrit {
        agent: AgentArg,
        message_id: IntArg,
    },
    MesgWritPlus {
        agent: AgentArg,
        message_id: IntArg,
        param_1: Anything,
        param_2: Anything,
        delay: IntArg,
    },
    Mira {
        on_off: IntArg,
    },
    NewSimp {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SStringArg,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    Nohh,
    Over,
    Paus {
        paused: IntArg,
    },
    Plne {
        plane: IntArg,
    },
    Pose {
        pose: IntArg,
    },
    Puhl {
        pose: IntArg,
        x: IntArg,
        y: IntArg,
    },
    Pupt {
        pose: IntArg,
        x: IntArg,
        y: IntArg,
    },
    Rnge {
        distance: FloatArg,
    },
    Rtar {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    Show {
        visibility: IntArg,
    },
    Star {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    Tick {
        tick_rate: IntArg,
    },
    Tint {
        red_tint: IntArg,
        green_tint: IntArg,
        blue_tint: IntArg,
        rotation: IntArg,
        swap: IntArg,
    },
    Ttar {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    // Brain
    BrnDmpb,
    BrnDmpd {
        tract_number: IntArg,
        dendrite_number: IntArg,
    },
    BrnDmpl {
        lobe_number: IntArg,
    },
    BrnDmpn {
        lobe_number: IntArg,
        neuron_number: IntArg,
    },
    BrnDmpt {
        tract_number: IntArg,
    },
    BrnSetd {
        tract_number: IntArg,
        dendrite_number: IntArg,
        weight_number: IntArg,
        new_value: FloatArg,
    },
    BrnSetl {
        lobe_number: IntArg,
        line_number: IntArg,
        new_value: FloatArg,
    },
    BrnSetn {
        lobe_number: IntArg,
        neuron_number: IntArg,
        state_number: IntArg,
        new_value: FloatArg,
    },
    BrnSett {
        tract_number: IntArg,
        line_number: IntArg,
        new_value: FloatArg,
    },
    // Camera
    Bkgd {
        metaroom_id: IntArg,
        background: SStringArg,
        transition: IntArg,
    },
    Brmi {
        mearoom_base: IntArg,
        room_base: IntArg,
    },
    Cmra {
        x: IntArg,
        y: IntArg,
        pan: IntArg,
    },
    Cmrp {
        x: IntArg,
        y: IntArg,
        pan: IntArg,
    },
    Cmrt {
        pan: IntArg,
    },
    Frsh,
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
    Meta {
        metaroom_id: IntArg,
        camera_x: IntArg,
        camera_y: IntArg,
        transition: IntArg,
    },
    Scam {
        compound_agent: AgentArg,
        part_number: IntArg,
    },
    Snap {
        filename: SStringArg,
        x_centre: IntArg,
        y_centre: IntArg,
        width: IntArg,
        height: IntArg,
        zoom_factor: IntArg,
    },
    Trck {
        agent: AgentArg,
        x_percent: IntArg,
        y_percent: IntArg,
        style: IntArg,
        transition: IntArg,
    },
    Wdow,
    Zoom {
        pixels: IntArg,
        x: IntArg,
        y: IntArg,
    },
    // Compounds
    Fcus,
    Frmt {
        left_margin: IntArg,
        top_margin: IntArg,
        right_margin: IntArg,
        bottom_margin: IntArg,
        line_spacing: IntArg,
        character_spacing: IntArg,
        justification: IntArg,
    },
    Grpl {
        red: IntArg,
        green: IntArg,
        blue: IntArg,
        min_y: FloatArg,
        max_y: FloatArg,
    },
    Grpv {
        line_index: IntArg,
        value: FloatArg,
    },
    NewComp {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SStringArg,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    Page {
        page: IntArg,
    },
    Part {
        part_id: IntArg,
    },
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
    PatDull {
        part_id: IntArg,
        sprite_file: SStringArg,
        first_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
    },
    PatFixd {
        part_id: IntArg,
        sprite_file: SStringArg,
        first_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        font_sprite: SStringArg,
    },
    PatGrph {
        part_id: IntArg,
        overlay_sprite: SStringArg,
        base_image: IntArg,
        rel_x: DecimalArg,
        rel_y: DecimalArg,
        rel_plane: IntArg,
        num_values: IntArg,
    },
    PatKill {
        part_id: IntArg,
    },
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
    Ptxt {
        text: SStringArg,
    },
    // Creates
    Ages {
        times: IntArg,
    },
    Appr,
    Aslp {
        asleep: IntArg,
    },
    Body {
        set_number: IntArg,
        layer: IntArg,
    },
    Born,
    Chem {
        chemical: IntArg,
        adjustment: FloatArg,
    },
    Dead,
    Dirn {
        direction: IntArg,
    },
    Done,
    Drea {
        dream: IntArg,
    },
    Driv {
        drive: IntArg,
        adjustment: FloatArg,
    },
    Face {
        set_number: IntArg,
    },
    Forf {
        creature_to_learn_about: AgentArg,
    },
    Hair {
        stage: IntArg,
    },
    Injr {
        organ: IntArg,
        amount: IntArg,
    },
    Like {
        creature_state_opinion_about: AgentArg,
    },
    Loci {
        r#type: IntArg,
        organ: IntArg,
        tissue: IntArg,
        id: IntArg,
        new_value: FloatArg,
    },
    Ltcy {
        action: IntArg,
        min: IntArg,
        max: IntArg,
    },
    Mate,
    Mvft {
        x: FloatArg,
        y: FloatArg,
    },
    NewCrea {
        family: IntArg,
        gene_agent: AgentArg,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    Newc {
        family: IntArg,
        gene_agent: AgentArg,
        gene_slot: IntArg,
        sex: IntArg,
        variant: IntArg,
    },
    Norn {
        creature: AgentArg,
    },
    Nude,
    OrdrShou {
        speech: SStringArg,
    },
    OrdrSign {
        speech: SStringArg,
    },
    OrdrWrit {
        creature: AgentArg,
        speech: SStringArg,
    },
    Sayn,
    Spnl {
        lobe_monkier: SStringArg,
        neuron_id: IntArg,
        value: FloatArg,
    },
    StimShou {
        stimulus: IntArg,
        strength: FloatArg,
    },
    StimSign {
        stimulus: IntArg,
        strength: FloatArg,
    },
    StimTact {
        stimulus: IntArg,
        strength: FloatArg,
    },
    StimWrit {
        creature: AgentArg,
        stimulus: IntArg,
        strength: FloatArg,
    },
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
    Touc,
    Uncs {
        unconscious: IntArg,
    },
    UrgeShou {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    UrgeSign {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    UrgeTact {
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    UrgeWrit {
        creature: AgentArg,
        noun_id: IntArg,
        noun_stim: FloatArg,
        verb_id: IntArg,
        verb_stim: FloatArg,
    },
    Vocb,
    Walk,
    Wear {
        body_id: IntArg,
        set_number: IntArg,
        layer: IntArg,
    },
    Zomb {
        zombie: IntArg,
    },
    // Debug
    Apro {
        search_text: SStringArg,
    },
    DbgAsrt {
        condition: Condition,
    },
    DbgCpro,
    DbgFlsh,
    DbgHtml {
        sort_order: IntArg,
    },
    DbgOuts {
        value: SStringArg,
    },
    DbgOutv {
        value: DecimalArg,
    },
    DbgPaws,
    DbgPlay,
    DbgPoll,
    DbgProf,
    DbgTack {
        follow: AgentArg,
    },
    DbgTock,
    DbgWtik {
        new_world_tick: IntArg,
    },
    Help,
    Mann {
        command: SStringArg,
    },
    Memx,
    // Files
    FileGlob {
        directory: IntArg,
        file_spec: SStringArg,
    },
    FileIclo,
    FileIope {
        directory: IntArg,
        filename: SStringArg,
    },
    FileJdel {
        directory: IntArg,
        filename: SStringArg,
    },
    FileOclo,
    FileOflu,
    FileOope {
        directory: IntArg,
        filename: SStringArg,
        append: IntArg,
    },
    Outs {
        text: SStringArg,
    },
    Outv {
        value: DecimalArg,
    },
    Outx {
        text: SStringArg,
    },
    Goto {
        destination: Label,
    },
    Gsub {
        destination: Label,
    },
    // Genetics
    GeneClon {
        dest_agent: AgentArg,
        dest_slot: IntArg,
        source_agent: AgentArg,
        source_slot: IntArg,
    },
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
    GeneKill {
        agent: AgentArg,
        slot: IntArg,
    },
    GeneLoad {
        agent: AgentArg,
        slot: IntArg,
        gene_file: SStringArg,
    },
    GeneMove {
        dest_agent: AgentArg,
        dest_slot: IntArg,
        source_agent: AgentArg,
        source_slot: IntArg,
    },
    // History
    HistEvnt {
        moniker: SStringArg,
        event_type: IntArg,
        related_moniker_1: SStringArg,
        related_moniker_2: SStringArg,
    },
    HistFoto {
        moniker: SStringArg,
        event_no: IntArg,
        new_value: SStringArg,
    },
    HistName {
        moniker: SStringArg,
        new_name: SStringArg,
    },
    HistUtxt {
        moniker: SStringArg,
        event_no: IntArg,
        new_value: SStringArg,
    },
    HistWipe {
        moniker: SStringArg,
    },
    Clac {
        message: IntArg,
    },
    Clik {
        message_1: IntArg,
        message_2: IntArg,
        message_3: IntArg,
    },
    Imsk {
        mask: IntArg,
    },
    Mous {
        behaviour: IntArg,
    },
    Pure {
        value: IntArg,
    },
    Tran {
        transparency: IntArg,
        part_no: IntArg,
    },
    // Map
    Addb {
        metaroom_id: IntArg,
        background_file: SStringArg,
    },
    Altr {
        room_id: IntArg,
        ca_index: IntArg,
        ca_delta: FloatArg,
    },
    Cacl {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        ca_index: IntArg,
    },
    Delm {
        metaroom_id: IntArg,
    },
    Delr {
        room_id: IntArg,
    },
    Dmap {
        debug_map: IntArg,
    },
    Doca {
        no_of_updates: IntArg,
    },
    Door {
        room_id1: IntArg,
        room_id2: IntArg,
        permiability: IntArg,
    },
    Emit {
        ca_index: IntArg,
        amount: FloatArg,
    },
    Link {
        room1: IntArg,
        room2: IntArg,
        permiability: IntArg,
    },
    Mapd {
        width: IntArg,
        height: IntArg,
    },
    Mapk,
    Perm {
        permiability: IntArg,
    },
    Prop {
        room_id: IntArg,
        ca_index: IntArg,
        ca_value: FloatArg,
    },
    Rate {
        room_type: IntArg,
        ca_index: IntArg,
        gain: FloatArg,
        loss: FloatArg,
        diffusion: FloatArg,
    },
    Rtyp {
        room_id: IntArg,
        room_type: IntArg,
    },
    // Motion
    Accg {
        acceleration: FloatArg,
    },
    Aero {
        aerodynamics: IntArg,
    },
    Elas {
        elasticity: IntArg,
    },
    Flto {
        screen_x: FloatArg,
        screen_y: FloatArg,
    },
    Frel {
        relative: AgentArg,
    },
    Fric {
        friction: IntArg,
    },
    Mvby {
        delta_x: FloatArg,
        delta_y: FloatArg,
    },
    Mvsf {
        x: FloatArg,
        y: FloatArg,
    },
    Mvto {
        x: FloatArg,
        y: FloatArg,
    },
    Velo {
        x_velocity: FloatArg,
        y_velocity: FloatArg,
    },
    // Ports
    PrtBang {
        bang_strength: IntArg,
    },
    PrtInew {
        id: IntArg,
        name: SStringArg,
        description: SStringArg,
        x: IntArg,
        y: IntArg,
        message_num: IntArg,
    },
    PrtIzap {
        id: IntArg,
    },
    PrtJoin {
        source_agent: AgentArg,
        output_id: IntArg,
        dest_agent: AgentArg,
        input_id: IntArg,
    },
    PrtKrak {
        agent: AgentArg,
        in_or_out: IntArg,
        port_index: IntArg,
    },
    PrtOnew {
        id: IntArg,
        name: SStringArg,
        description: SStringArg,
        x: IntArg,
        y: IntArg,
    },
    PrtOzap {
        id: IntArg,
    },
    PrtSend {
        id: IntArg,
        data: Anything,
    },
    // Resources
    PrayGarb {
        force: IntArg,
    },
    PrayRefr,
    // Scripts
    GidsFmly {
        family: IntArg,
    },
    GidsGnus {
        family: IntArg,
        genus: IntArg,
    },
    GidsRoot,
    GidsSpcs {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    Inst,
    Lock,
    Scrx {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        event: IntArg,
    },
    Slow,
    Sorc {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        event: IntArg,
    },
    Stop,
    Stpt,
    Unlk,
    Wait {
        ticks: IntArg,
    },
    // Sounds
    Fade,
    Mclr {
        x: IntArg,
        y: IntArg,
    },
    Midi {
        midi_file: SStringArg,
    },
    Mmsc {
        x: IntArg,
        y: IntArg,
        track_name: SStringArg,
    },
    Rclr {
        x: IntArg,
        y: IntArg,
    },
    Rmsc {
        x: IntArg,
        y: IntArg,
        track_name: SStringArg,
    },
    Sezz {
        text: SStringArg,
    },
    Sndc {
        sound_file: SStringArg,
    },
    Snde {
        sound_file: SStringArg,
    },
    Sndl {
        sound_file: SStringArg,
    },
    Sndq {
        sound_file: SStringArg,
        delay: IntArg,
    },
    Stpc,
    Strk {
        latency: IntArg,
        track: SStringArg,
    },
    Voic {
        genus: IntArg,
        gender: IntArg,
        age: IntArg,
    },
    Vois {
        voice_name: SStringArg,
    },
    Volm {
        volume: IntArg,
    },
    // Date
    Wpau {
        paused: IntArg,
    },
    // Variables
    Absv {
        var: Variable,
    },
    Adds {
        var: Variable,
        append: SStringArg,
    },
    Addv {
        var: Variable,
        sum: DecimalArg,
    },
    Andv {
        var: Variable,
        value: IntArg,
    },
    Char {
        string: Variable,
        index: IntArg,
        character: IntArg,
    },
    Delg {
        variable_name: SStringArg,
    },
    Divv {
        var: Variable,
        div: DecimalArg,
    },
    Modv {
        var: Variable,
        r#mod: IntArg,
    },
    Mulv {
        var: Variable,
        mul: DecimalArg,
    },
    Negv {
        var: Variable,
    },
    Orrv {
        var: Variable,
        value: IntArg,
    },
    Reaf,
    Seta {
        var: Variable,
        value: AgentArg,
    },
    Sets {
        var: Variable,
        value: SStringArg,
    },
    Setv {
        var: Variable,
        value: DecimalArg,
    },
    Subv {
        var: Variable,
        sub: DecimalArg,
    },
    Targ {
        agent: AgentArg,
    },
    // Vehicles
    Cabn {
        left: IntArg,
        top: IntArg,
        right: IntArg,
        bottom: IntArg,
    },
    Cabp {
        plane: IntArg,
    },
    Cabv {
        cabin_room_id: IntArg,
    },
    Cabw {
        cabin_capacity: IntArg,
    },
    Dpas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
    },
    Gpas {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        rect_to_use: IntArg,
    },
    NewVhcl {
        family: IntArg,
        genus: IntArg,
        species: IntArg,
        sprite_file: SStringArg,
        image_count: IntArg,
        first_image: IntArg,
        plane: IntArg,
    },
    Rpas {
        vehicle: AgentArg,
        passenger: AgentArg,
    },
    Spas {
        vehicle: AgentArg,
        new_passenger: AgentArg,
    },
    // World
    Delw {
        world_name: SStringArg,
    },
    Load {
        world_name: SStringArg,
    },
    Pswd {
        world_name: SStringArg,
    },
    Quit,
    Rgam,
    Save,
    Tntw {
        index: IntArg,
    },
    Wrld {
        world_name: SStringArg,
    },
    Wtnt {
        index: IntArg,
        red_tint: IntArg,
        green_tint: IntArg,
        blue_tint: IntArg,
        rotation: IntArg,
        swap: IntArg,
    },
}

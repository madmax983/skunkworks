use crate::matrix_rain::MatrixRain;
use ratatui::widgets::ListState;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ViewMode {
    Genome,
    Grid,
    Microscope,
    #[cfg(feature = "biophysics")]
    Cortex,
    #[cfg(feature = "resonance")]
    Resonance,
    #[cfg(feature = "nova")]
    Grimoire,
    #[cfg(feature = "nova")]
    Laboratory,
    #[cfg(feature = "nova")]
    Topology,
    #[cfg(feature = "nova")]
    Graveyard,
    #[cfg(feature = "nova")]
    PianoRoll,
    #[cfg(feature = "nova")]
    Retina,
    #[cfg(feature = "nova")]
    Quantum,
    #[cfg(feature = "nova")]
    Dream,
    #[cfg(feature = "nova")]
    Phylogeny,
    #[cfg(feature = "nova")]
    Alchemy,
    #[cfg(feature = "nova")]
    Memetics,
    #[cfg(feature = "nova")]
    Egregore,
    #[cfg(feature = "nova")]
    Bestiary,
    #[cfg(feature = "nova")]
    Kaleidoscope,
    #[cfg(feature = "nova")]
    Void,
    #[cfg(feature = "nova")]
    Signals,
    #[cfg(feature = "nova")]
    Sovereignty,
    #[cfg(feature = "nova")]
    Spectrogram,
    #[cfg(feature = "nova")]
    Market,
    #[cfg(feature = "nova")]
    Ballistics,
    #[cfg(feature = "nova")]
    Scent,
    Heatmap,
    #[cfg(feature = "silicon")]
    Schematic,
    #[cfg(feature = "silicon")]
    Foundry,
    #[cfg(feature = "elektra")]
    Elektra,
    #[cfg(feature = "nova")]
    Fishing,
    #[cfg(feature = "nova")]
    Arena,
    #[cfg(feature = "nova")]
    Garden,
    #[cfg(feature = "nova")]
    Orca,
    #[cfg(feature = "nova")]
    Babel,
    #[cfg(feature = "nova")]
    Strings,
    #[cfg(feature = "nova")]
    Quipu,
    #[cfg(feature = "nova")]
    Hydra,
    #[cfg(feature = "nova")]
    Chronos,
    #[cfg(feature = "nova")]
    Logos,
    #[cfg(feature = "nova")]
    Pandemonium,
    BioticChaos,
    Catalyst,
    #[cfg(feature = "nova")]
    Hyperspace,
    #[cfg(feature = "nova")]
    Hologram,
    #[cfg(feature = "nova")]
    Weaver,
    #[cfg(feature = "nova")]
    Terminal,
    #[cfg(feature = "nova")]
    Attractor,
    #[cfg(feature = "nova")]
    Virology,
    #[cfg(feature = "nova")]
    BioMesh,
    #[cfg(feature = "nova")]
    Crispr,
    #[cfg(feature = "nova")]
    Reactor,
    #[cfg(feature = "nova")]
    Biolum,
    Evolution,
    #[cfg(feature = "nova")]
    Ecology,
    #[cfg(feature = "nova")]
    LifeCycle,
    #[cfg(feature = "nova")]
    Semiotics,
    #[cfg(feature = "nova")]
    Fractal,
    #[cfg(feature = "nova")]
    Metazoa,
    #[cfg(feature = "nova")]
    Genesis,
    #[cfg(feature = "nova")]
    Cambrian,
    #[cfg(feature = "nova")]
    Savant,
    #[cfg(feature = "nova")]
    Akashic,
    #[cfg(feature = "nova")]
    Prologue,
    #[cfg(feature = "nova")]
    Lexicon,
    #[cfg(feature = "nova")]
    Narrative,
    Sequencer,
    Mutagen,
    Forge,
    #[cfg(feature = "nova")]
    Tesseract,
    #[cfg(feature = "nova")]
    Choir,
    #[cfg(feature = "nova")]
    Paradox,
    #[cfg(feature = "nova")]
    Codex,
    #[cfg(feature = "nova")]
    Verbum,
}

pub enum InputMode {
    Normal,
    Editing,
    Injection,
}

pub(crate) struct EvolutionState {
    pub(crate) engine: Option<crate::vm::evolution::EvolutionEngine>,
    pub(crate) challenge: crate::vm::evolution::Challenge,
    pub(crate) auto_run: bool,
}

impl EvolutionState {
    fn new() -> Self {
        Self {
            engine: None,
            challenge: crate::vm::evolution::Challenge::default(),
            auto_run: false,
        }
    }
}

pub(crate) struct SequencerState {
    pub(crate) playing: bool,
    pub(crate) bpm: u64,
    pub(crate) tick: usize,
    pub(crate) scroll_x: usize,
}

impl SequencerState {
    fn new() -> Self {
        Self {
            playing: false,
            bpm: 120,
            tick: 0,
            scroll_x: 0,
        }
    }
}

pub(crate) struct AppState {
    pub(crate) view_mode: ViewMode,
    pub(crate) input_mode: InputMode,
    pub(crate) selected_strand: usize,
    pub(crate) selected_gene: usize,
    pub(crate) grid_cursor: (usize, usize),
    pub(crate) input_buffer: String,
    pub(crate) status_msg: String,
    #[cfg(feature = "biophysics")]
    pub(crate) voltage_history: Vec<u64>,
    #[cfg(feature = "biophysics")]
    pub(crate) selected_neuron_coords: Option<(usize, usize)>,
    #[cfg(feature = "nova")]
    pub(crate) lab_parent_a: usize,
    #[cfg(feature = "nova")]
    pub(crate) lab_parent_b: usize,
    #[cfg(feature = "nova")]
    pub(crate) lab_method: usize,
    #[cfg(feature = "nova")]
    pub(crate) selected_graveyard_strand: usize,
    #[cfg(feature = "nova")]
    pub(crate) selected_sigil_index: usize,
    #[cfg(feature = "nova")]
    pub(crate) selected_dream_trace: usize,
    #[cfg(feature = "nova")]
    pub(crate) alchemy_selection: usize, // 0=Shelf, 1=Strands
    #[cfg(feature = "nova")]
    pub(crate) alchemy_shelf_idx: usize,
    #[cfg(feature = "nova")]
    pub(crate) alchemy_strand_idx: usize,
    #[cfg(feature = "nova")]
    pub(crate) selected_organelle_index: usize,
    #[cfg(feature = "nova")]
    pub(crate) kaleidoscope_hue_idx: usize,
    #[cfg(feature = "nova")]
    pub(crate) kaleidoscope_light_idx: usize,
    #[cfg(feature = "nova")]
    pub(crate) fishing_bobber_y: f64,
    #[cfg(feature = "nova")]
    pub(crate) fishing_tension: f64,
    #[cfg(feature = "nova")]
    pub(crate) fishing_hooked: bool,
    #[cfg(feature = "nova")]
    pub(crate) fishing_cast: bool,
    #[cfg(feature = "nova")]
    pub(crate) fishing_fish_y: f64,
    #[cfg(feature = "oracle")]
    pub(crate) query_input: String,
    #[cfg(feature = "oracle")]
    pub(crate) query_mode: bool,
    #[cfg(feature = "oracle")]
    pub(crate) query_results: Vec<String>,
    pub(crate) palette_open: bool,
    pub(crate) palette_idx: usize,
    pub(crate) palette_char: Option<char>,
    #[cfg(feature = "nova")]
    pub(crate) babel_pattern: String,
    #[cfg(feature = "nova")]
    pub(crate) babel_input: String,
    #[cfg(feature = "nova")]
    pub(crate) babel_result: String,
    #[cfg(feature = "nova")]
    pub(crate) babel_focus: usize, // 0=Pattern, 1=Input
    #[cfg(feature = "nova")]
    pub(crate) babel_ast: Option<crate::vm::Value>,
    pub(crate) pandemonium_cursor: (f64, f64),
    pub(crate) pandemonium_radius: f64,
    pub(crate) pandemonium_selected_tool: usize, // 0=Mutate, 1=Scramble, 2=Purge, 3=Duplicate
    pub(crate) catalyst_scroll: usize,
    pub(crate) show_view_selector: bool,
    pub(crate) view_selector_state: std::cell::RefCell<ListState>,
    #[cfg(feature = "nova")]
    pub(crate) terminal_input: String,
    #[cfg(feature = "nova")]
    pub(crate) terminal_history: Vec<String>,
    #[cfg(feature = "nova")]
    pub(crate) terminal_history_idx: usize,
    #[cfg(feature = "nova")]
    pub(crate) crispr_target_strand: usize,
    #[cfg(feature = "nova")]
    pub(crate) crispr_guide: String,
    #[cfg(feature = "nova")]
    pub(crate) crispr_replace: String,
    #[cfg(feature = "nova")]
    pub(crate) crispr_focus: usize,
    #[cfg(feature = "nova")]
    pub(crate) crispr_result: String,
    #[cfg(feature = "nova")]
    pub(crate) genesis_editor_buffer: String,
    #[cfg(feature = "nova")]
    pub(crate) genesis_grammar_buffer: String,
    #[cfg(feature = "nova")]
    pub(crate) genesis_focus: u8, // 0=Editor, 1=Grammar, 2=Grid
    #[cfg(feature = "nova")]
    pub(crate) grimoire_scroll: u16,
    #[cfg(feature = "nova")]
    pub(crate) virus_design_name: String,
    #[cfg(feature = "nova")]
    pub(crate) virus_design_pattern: String,
    #[cfg(feature = "nova")]
    pub(crate) virus_design_rate: u8,
    #[cfg(feature = "nova")]
    pub(crate) virus_design_payload: i64,
    #[cfg(feature = "nova")]
    pub(crate) virus_design_mode: usize, // 0=Overwrite, 1=RewriteGrid, 2=RewriteDNA
    #[cfg(feature = "nova")]
    pub(crate) virus_design_focus: u8, // 0=Name, 1=Pattern, 2=Rate, 3=Payload, 4=Mode
    #[cfg(feature = "nova")]
    pub(crate) forge_selected_rule: String,
    #[cfg(feature = "nova")]
    pub(crate) forge_editor_buffer: String,
    #[cfg(feature = "nova")]
    pub(crate) forge_test_input: String,
    #[cfg(feature = "nova")]
    pub(crate) forge_test_output: String,
    #[cfg(feature = "nova")]
    pub(crate) forge_focus: u8, // 0=List, 1=Editor, 2=TestInput
    #[cfg(feature = "nova")]
    pub(crate) paradox_editor_buffer: String,
    #[cfg(feature = "nova")]
    pub(crate) codex_selected_spell: usize,
    pub(crate) evolution_state: EvolutionState,
    pub(crate) sequencer_state: SequencerState,
    pub(crate) matrix_rain: MatrixRain,
    pub(crate) screen_shake: f32,
    pub(crate) chaos_mode: bool,
    pub(crate) source_path: Option<std::path::PathBuf>,
    pub(crate) last_modified: Option<std::time::SystemTime>,
    pub(crate) last_check_tick: u64,
}

impl AppState {
    pub(crate) fn new(
        initial_view: Option<ViewMode>,
        source_path: Option<std::path::PathBuf>,
    ) -> Self {
        let mut view_selector_state = ListState::default();
        view_selector_state.select(Some(0));
        let last_modified = if let Some(path) = &source_path {
            std::fs::metadata(path).ok().and_then(|m| m.modified().ok())
        } else {
            None
        };

        Self {
            view_mode: initial_view.unwrap_or(ViewMode::Genome),
            show_view_selector: false,
            view_selector_state: std::cell::RefCell::new(view_selector_state),
            input_mode: InputMode::Normal,
            selected_strand: 0,
            selected_gene: 0,
            grid_cursor: (0, 0),
            input_buffer: String::new(),
            status_msg: String::new(),
            #[cfg(feature = "biophysics")]
            voltage_history: Vec::with_capacity(100),
            #[cfg(feature = "biophysics")]
            selected_neuron_coords: None,
            #[cfg(feature = "nova")]
            lab_parent_a: 0,
            #[cfg(feature = "nova")]
            lab_parent_b: 0,
            #[cfg(feature = "nova")]
            lab_method: 0,
            #[cfg(feature = "nova")]
            selected_graveyard_strand: 0,
            #[cfg(feature = "nova")]
            selected_sigil_index: 0,
            #[cfg(feature = "nova")]
            selected_dream_trace: 0,
            #[cfg(feature = "nova")]
            alchemy_selection: 0,
            #[cfg(feature = "nova")]
            alchemy_shelf_idx: 0,
            #[cfg(feature = "nova")]
            alchemy_strand_idx: 0,
            #[cfg(feature = "nova")]
            selected_organelle_index: 0,
            #[cfg(feature = "nova")]
            kaleidoscope_hue_idx: 0,
            #[cfg(feature = "nova")]
            kaleidoscope_light_idx: 1, // Normal
            #[cfg(feature = "nova")]
            fishing_bobber_y: 50.0,
            #[cfg(feature = "nova")]
            fishing_tension: 0.0,
            #[cfg(feature = "nova")]
            fishing_hooked: false,
            #[cfg(feature = "nova")]
            fishing_cast: false,
            #[cfg(feature = "nova")]
            fishing_fish_y: 80.0,
            #[cfg(feature = "oracle")]
            query_input: String::new(),
            #[cfg(feature = "oracle")]
            query_mode: false,
            #[cfg(feature = "oracle")]
            query_results: Vec::new(),
            palette_open: false,
            palette_idx: 0,
            palette_char: None,
            #[cfg(feature = "nova")]
            babel_pattern: String::from("[a-z]+"),
            #[cfg(feature = "nova")]
            babel_input: String::from("hello"),
            #[cfg(feature = "nova")]
            babel_result: String::new(),
            #[cfg(feature = "nova")]
            babel_focus: 0,
            #[cfg(feature = "nova")]
            babel_ast: None,
            pandemonium_cursor: (0.0, 0.0),
            pandemonium_radius: 5.0,
            pandemonium_selected_tool: 0,
            catalyst_scroll: 0,
            #[cfg(feature = "nova")]
            terminal_input: String::new(),
            #[cfg(feature = "nova")]
            terminal_history: Vec::new(),
            #[cfg(feature = "nova")]
            terminal_history_idx: 0,
            #[cfg(feature = "nova")]
            crispr_target_strand: 0,
            #[cfg(feature = "nova")]
            crispr_guide: String::new(),
            #[cfg(feature = "nova")]
            crispr_replace: String::new(),
            #[cfg(feature = "nova")]
            crispr_focus: 0,
            #[cfg(feature = "nova")]
            crispr_result: String::from("Ready to edit."),
            #[cfg(feature = "nova")]
            genesis_editor_buffer: String::new(),
            #[cfg(feature = "nova")]
            genesis_grammar_buffer: String::new(),
            #[cfg(feature = "nova")]
            genesis_focus: 0,
            #[cfg(feature = "nova")]
            grimoire_scroll: 0,
            #[cfg(feature = "nova")]
            virus_design_name: String::from("NewVirus"),
            #[cfg(feature = "nova")]
            virus_design_pattern: String::from(".*"),
            #[cfg(feature = "nova")]
            virus_design_rate: 50,
            #[cfg(feature = "nova")]
            virus_design_payload: -1,
            #[cfg(feature = "nova")]
            virus_design_mode: 0,
            #[cfg(feature = "nova")]
            virus_design_focus: 0,
            #[cfg(feature = "nova")]
            forge_selected_rule: String::new(),
            #[cfg(feature = "nova")]
            forge_editor_buffer: String::new(),
            #[cfg(feature = "nova")]
            forge_test_input: String::new(),
            #[cfg(feature = "nova")]
            forge_test_output: String::new(),
            #[cfg(feature = "nova")]
            forge_focus: 0,
            #[cfg(feature = "nova")]
            paradox_editor_buffer: String::new(),
            #[cfg(feature = "nova")]
            codex_selected_spell: 0,
            evolution_state: EvolutionState::new(),
            sequencer_state: SequencerState::new(),
            matrix_rain: MatrixRain::new(),
            screen_shake: 0.0,
            chaos_mode: false,
            source_path,
            last_modified,
            last_check_tick: 0,
        }
    }

    pub fn get_render_area(&self, full_area: ratatui::layout::Rect) -> ratatui::layout::Rect {
        if self.screen_shake > 0.1 {
            let mut rng = rand::thread_rng();
            use rand::Rng;
            let dx = (rng.gen::<f32>() - 0.5) * self.screen_shake;
            let dy = (rng.gen::<f32>() - 0.5) * self.screen_shake;

            let new_x = (full_area.x as f32 + dx).clamp(0.0, full_area.width as f32);
            let new_y = (full_area.y as f32 + dy).clamp(0.0, full_area.height as f32);

            ratatui::layout::Rect {
                x: new_x as u16,
                y: new_y as u16,
                width: full_area.width.saturating_sub(dx.abs() as u16),
                height: full_area.height.saturating_sub(dy.abs() as u16),
            }
        } else {
            full_area
        }
    }
}

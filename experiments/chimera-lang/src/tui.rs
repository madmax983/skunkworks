use crate::ast::Gene;
use crate::vm::ChimeraVM;
use crate::{ChimeraParser, Rule};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use pest::Parser;
#[cfg(feature = "nova")]
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, ListState, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum ViewMode {
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
}

enum InputMode {
    Normal,
    Editing,
    Injection,
}

pub(crate) struct AppState {
    pub(crate) view_mode: ViewMode,
    input_mode: InputMode,
    selected_strand: usize,
    selected_gene: usize,
    grid_cursor: (usize, usize),
    input_buffer: String,
    status_msg: String,
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
    pub(crate) show_view_selector: bool,
    pub(crate) view_selector_state: std::cell::RefCell<ListState>,
}

impl AppState {
    pub(crate) fn new() -> Self {
        let mut view_selector_state = ListState::default();
        view_selector_state.select(Some(0));
        Self {
            view_mode: ViewMode::Genome,
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
        }
    }
}

pub fn run_tui(mut vm: ChimeraVM) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app_state = AppState::new();

    let res = run_app(&mut terminal, &mut vm, &mut app_state);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn parse_grid_value(s: &str) -> crate::vm::Value {
    if s.is_empty() {
        return crate::vm::Value::Int(0);
    }
    if let Ok(i) = s.parse::<i64>() {
        crate::vm::Value::Int(i)
    } else {
        crate::vm::Value::Str(s.to_string())
    }
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    vm: &mut ChimeraVM,
    app_state: &mut AppState,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        #[cfg(feature = "nova")]
        if let ViewMode::Fishing = app_state.view_mode {
            if app_state.fishing_cast {
                // Bobber float animation
                app_state.fishing_bobber_y += (rand::random::<f64>() - 0.5) * 0.5;

                // Random Hook
                if !app_state.fishing_hooked && rand::random::<f64>() < 0.01 {
                    app_state.fishing_hooked = true;
                    app_state.status_msg = "FISH HOOKED! REEL IT IN!".to_string();
                }

                if app_state.fishing_hooked {
                    // Fish fights back (Randomly pulls)
                    if rand::random::<f64>() < 0.2 {
                        app_state.fishing_tension += 0.02;
                    } else {
                        // Passive decay when not being pulled
                        app_state.fishing_tension -= 0.002;
                    }
                    app_state.fishing_fish_y =
                        app_state.fishing_bobber_y + (rand::random::<f64>() - 0.5) * 2.0;
                } else {
                    app_state.fishing_tension -= 0.01;
                }

                // Clamp tension
                app_state.fishing_tension = app_state.fishing_tension.clamp(0.0, 1.1); // Allow slight over for snap check

                if app_state.fishing_tension >= 1.0 {
                    app_state.status_msg = "SNAP! Line broke.".to_string();
                    app_state.fishing_cast = false;
                    app_state.fishing_hooked = false;
                    app_state.fishing_tension = 0.0;
                }
            }
        }

        #[cfg(feature = "biophysics")]
        if let Some(coord) = app_state.selected_neuron_coords {
            if let Some(neuron) = vm.neurons.get(&coord) {
                // Push voltage (mapped to u64 for Sparkline)
                // V is approx -100 to +50. Shift by +100.
                let v_norm = (neuron.v + 100.0).clamp(0.0, 200.0) as u64;
                if app_state.voltage_history.len() >= 100 {
                    app_state.voltage_history.remove(0);
                }
                app_state.voltage_history.push(v_norm);
            }
        }

        terminal.draw(|f| {
            if let ViewMode::Microscope = app_state.view_mode {
                render_microscope(f, vm, app_state);
                return;
            }

            #[cfg(feature = "biophysics")]
            if let ViewMode::Cortex = app_state.view_mode {
                render_cortex(f, vm, app_state);
                return;
            }

            #[cfg(feature = "resonance")]
            if let ViewMode::Resonance = app_state.view_mode {
                render_resonance(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Grimoire = app_state.view_mode {
                render_grimoire(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Topology = app_state.view_mode {
                render_topology(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Laboratory = app_state.view_mode {
                render_laboratory(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Graveyard = app_state.view_mode {
                render_graveyard(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Retina = app_state.view_mode {
                render_retina(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Quantum = app_state.view_mode {
                render_quantum(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Dream = app_state.view_mode {
                render_dream(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Phylogeny = app_state.view_mode {
                render_phylogeny(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Alchemy = app_state.view_mode {
                render_alchemy(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::PianoRoll = app_state.view_mode {
                render_piano_roll(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Memetics = app_state.view_mode {
                render_memetics(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Egregore = app_state.view_mode {
                render_egregore(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Bestiary = app_state.view_mode {
                render_bestiary(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Kaleidoscope = app_state.view_mode {
                render_kaleidoscope(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Void = app_state.view_mode {
                render_void(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Signals = app_state.view_mode {
                render_signals(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Sovereignty = app_state.view_mode {
                render_sovereignty(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Spectrogram = app_state.view_mode {
                render_spectrogram(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Market = app_state.view_mode {
                render_market(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Ballistics = app_state.view_mode {
                render_ballistics(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Scent = app_state.view_mode {
                render_scent(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Fishing = app_state.view_mode {
                render_fishing(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Arena = app_state.view_mode {
                render_arena(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Garden = app_state.view_mode {
                render_garden(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Orca = app_state.view_mode {
                render_orca(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Babel = app_state.view_mode {
                render_babel(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Strings = app_state.view_mode {
                render_strings(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Quipu = app_state.view_mode {
                render_quipu(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Hydra = app_state.view_mode {
                render_hydra(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Chronos = app_state.view_mode {
                render_chronos(f, vm, app_state);
                return;
            }

            #[cfg(feature = "nova")]
            if let ViewMode::Logos = app_state.view_mode {
                render_logos(f, vm, app_state);
                return;
            }

            if let ViewMode::Heatmap = app_state.view_mode {
                render_heatmap(f, vm, app_state);
                return;
            }

            #[cfg(feature = "silicon")]
            if let ViewMode::Schematic = app_state.view_mode {
                render_schematic(f, vm, app_state);
                return;
            }

            #[cfg(feature = "silicon")]
            if let ViewMode::Foundry = app_state.view_mode {
                render_foundry(f, vm, app_state);
                return;
            }

            #[cfg(feature = "elektra")]
            if let ViewMode::Elektra = app_state.view_mode {
                render_elektra(f, vm, app_state);
                return;
            }

            render_genome_and_grid(f, vm, app_state);

            if vm.glitch_level > 0.01 {
                apply_glitch_fx(f.buffer_mut(), vm.glitch_level);
            }
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if app_state.show_view_selector {
                    let views = get_all_views();
                    let mut list_state = app_state.view_selector_state.borrow_mut();
                    let selected = list_state.selected().unwrap_or(0);

                    match key.code {
                        KeyCode::Esc => app_state.show_view_selector = false,
                        KeyCode::Up => {
                            if selected > 0 {
                                list_state.select(Some(selected - 1));
                            } else {
                                list_state.select(Some(views.len().saturating_sub(1)));
                            }
                        }
                        KeyCode::Down => {
                            if selected + 1 < views.len() {
                                list_state.select(Some(selected + 1));
                            } else {
                                list_state.select(Some(0));
                            }
                        }
                        KeyCode::Enter => {
                            if selected < views.len() {
                                app_state.view_mode = views[selected].0;
                            }
                            app_state.show_view_selector = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                if app_state.palette_open {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('p') => app_state.palette_open = false,
                        KeyCode::Up => {
                            if app_state.palette_idx >= 4 {
                                app_state.palette_idx -= 4;
                            }
                        }
                        KeyCode::Down => {
                            if app_state.palette_idx + 4 < 16 {
                                app_state.palette_idx += 4;
                            }
                        }
                        KeyCode::Left => {
                            if app_state.palette_idx > 0 {
                                app_state.palette_idx -= 1;
                            }
                        }
                        KeyCode::Right => {
                            if app_state.palette_idx + 1 < 16 {
                                app_state.palette_idx += 1;
                            }
                        }
                        KeyCode::Enter => {
                            let chars = [
                                '*', 'o', 'x', '^', 'v', '<', '>', '+', '-', '/', '%', '!', '=',
                                ':', ';', '?',
                            ];
                            if app_state.palette_idx < chars.len() {
                                app_state.palette_char = Some(chars[app_state.palette_idx]);
                            }
                            app_state.palette_open = false;
                        }
                        _ => {}
                    }
                    continue;
                }

                #[cfg(feature = "oracle")]
                if app_state.query_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let query_str = &app_state.query_input;
                            match ChimeraParser::parse(Rule::gene, query_str) {
                                Ok(mut pairs) => {
                                    let pair = pairs.next().unwrap();
                                    match Gene::try_from_pair(pair) {
                                        Ok(gene) => {
                                            let op_name = gene.op.to_string();
                                            let mut terms = vec![crate::vm::Value::Str(op_name)];

                                            fn nuc_to_val(
                                                n: &crate::ast::Nucleotide,
                                            ) -> crate::vm::Value
                                            {
                                                match n {
                                                    crate::ast::Nucleotide::Number(i) => {
                                                        crate::vm::Value::Int(*i)
                                                    }
                                                    crate::ast::Nucleotide::String(s) => {
                                                        crate::vm::Value::Str(s.clone())
                                                    }
                                                    crate::ast::Nucleotide::Identifier(s) => {
                                                        crate::vm::Value::Str(s.clone())
                                                    }
                                                    crate::ast::Nucleotide::Junction(t, args) => {
                                                        crate::vm::Value::Junction(
                                                            *t,
                                                            args.iter().map(nuc_to_val).collect(),
                                                        )
                                                    }
                                                    _ => crate::vm::Value::Str("?".to_string()),
                                                }
                                            }

                                            for arg in gene.args {
                                                terms.push(nuc_to_val(&arg));
                                            }

                                            let goal = crate::vm::Value::Junction(
                                                crate::ast::JunctionType::Any,
                                                terms,
                                            );
                                            let mut solutions = Vec::new();
                                            crate::vm::oracle::solve(
                                                &[goal],
                                                std::collections::HashMap::new(),
                                                &vm.knowledge_base,
                                                vm,
                                                &mut solutions,
                                                0,
                                            );

                                            app_state.query_results.clear();
                                            if solutions.is_empty() {
                                                app_state.query_results.push("No.".to_string());
                                            } else {
                                                app_state.query_results.push(format!(
                                                    "Yes ({} solutions):",
                                                    solutions.len()
                                                ));
                                                for (i, sol) in solutions.iter().enumerate() {
                                                    let mut s = format!("{}: ", i + 1);
                                                    for (k, v) in sol {
                                                        s.push_str(&format!("{}={} ", k, v));
                                                    }
                                                    if sol.is_empty() {
                                                        s.push_str("true");
                                                    }
                                                    app_state.query_results.push(s);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            app_state.query_results.clear();
                                            app_state
                                                .query_results
                                                .push(format!("Parse Error: {}", e));
                                        }
                                    }
                                }
                                Err(e) => {
                                    app_state.query_results.clear();
                                    app_state.query_results.push(format!("Syntax Error: {}", e));
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app_state.query_mode = false;
                            app_state.query_input.clear();
                            app_state.query_results.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.query_input.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.query_input.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Spirit Request
                #[cfg(feature = "nova")]
                if vm.spirit_request {
                    match key.code {
                        KeyCode::Enter => {
                            let val = parse_grid_value(&app_state.input_buffer);
                            vm.spirit_value = Some(val);
                            vm.step(); // Resume
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Esc => {
                            vm.spirit_value = Some(crate::vm::Value::Int(0));
                            vm.step();
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.input_buffer.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Injection Mode
                if let InputMode::Injection = app_state.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            let src = format!("strand injection {{ {} }}", app_state.input_buffer);
                            match crate::compiler::compile(&src, None) {
                                Ok(dna) => {
                                    if let Some(strand) = dna.helix.strands.first() {
                                        vm.inject_genes(strand.genes.clone());
                                        app_state.status_msg = "Injection Successful".to_string();
                                    } else {
                                        app_state.status_msg =
                                            "Injection Failed: No genes".to_string();
                                    }
                                }
                                Err(e) => {
                                    app_state.status_msg = format!("Injection Error: {}", e);
                                }
                            }
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Esc => {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Char(c) => {
                            app_state.input_buffer.push(c);
                        }
                        KeyCode::Backspace => {
                            app_state.input_buffer.pop();
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Editing Mode
                if let InputMode::Editing = app_state.input_mode {
                    match key.code {
                        KeyCode::Enter => {
                            match app_state.view_mode {
                                ViewMode::Genome => {
                                    // Genome Editing Logic
                                    match ChimeraParser::parse(Rule::gene, &app_state.input_buffer)
                                    {
                                        Ok(mut pairs) => {
                                            let pair = pairs.next().unwrap();
                                            match Gene::try_from_pair(pair) {
                                                Ok(gene) => {
                                                    if app_state.selected_strand
                                                        < vm.dna.helix.strands.len()
                                                        && app_state.selected_gene
                                                            < vm.dna.helix.strands
                                                                [app_state.selected_strand]
                                                                .genes
                                                                .len()
                                                    {
                                                        vm.dna.helix.strands
                                                            [app_state.selected_strand]
                                                            .genes[app_state.selected_gene] = gene;
                                                        app_state.status_msg =
                                                            "Gene updated successfully".to_string();
                                                    }
                                                    app_state.input_mode = InputMode::Normal;
                                                    app_state.input_buffer.clear();
                                                }
                                                Err(e) => {
                                                    app_state.status_msg =
                                                        format!("Parse Error: {}", e);
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            app_state.status_msg = format!("Parse Error: {}", e);
                                        }
                                    }
                                }
                                ViewMode::Grid => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Chronos => {
                                    // Enable editing grid from Chronos view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Logos => {
                                    // Enable editing grid from Logos view
                                    let (x, y) = app_state.grid_cursor;
                                    // Should parse as String usually for Atoms
                                    // parse_grid_value handles numbers.
                                    let val = if app_state.input_buffer.starts_with('?') {
                                        // Variable
                                        crate::vm::Value::Str(app_state.input_buffer.clone())
                                    } else {
                                        parse_grid_value(&app_state.input_buffer)
                                    };
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Orca => {
                                    // Grid Editing Logic
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Microscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "biophysics")]
                                ViewMode::Cortex => {
                                    // No editing for Cortex view yet
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "resonance")]
                                ViewMode::Resonance => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Grimoire => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Laboratory => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Topology => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Graveyard => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::PianoRoll => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Retina => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quantum => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                ViewMode::Heatmap => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Schematic => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Dream => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Phylogeny => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Alchemy => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Memetics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Egregore => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Bestiary => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Kaleidoscope => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Void => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Signals => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Sovereignty => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Spectrogram => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Market => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Ballistics => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Scent => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Fishing => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Garden => {
                                    // Enable editing for Garden (Sowing rules)
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "elektra")]
                                ViewMode::Elektra => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Arena => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Babel => {
                                    // In Babel, Enter in Normal mode enters Editing mode.
                                    // Editing happens directly on the strings, no buffer commit needed here.
                                    // But we use input_buffer as scratchpad in other modes.
                                    // Here we edit in place.
                                    // So we just clear buffer and exit?
                                    // Wait, if we are in Editing mode, keys append to buffer.
                                    // We need to implement custom handling for Babel in Editing mode loop.
                                    // See below.
                                    app_state.input_mode = InputMode::Normal;
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Strings => {
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Quipu => {
                                    // Edit cord value?
                                    // Let's allow setting value of active cord
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    if let crate::vm::Value::Int(n) = val {
                                        if let Some(cord) =
                                            vm.quipu.cords.get_mut(vm.quipu.active_cord)
                                        {
                                            // Reset cord to this value?
                                            // Tie replaces it?
                                            // Let's reuse tie logic by clearing first?
                                            // Or just make tie set it. My tie logic replaces.
                                            cord.tie(n);
                                            app_state.status_msg = format!(
                                                "Cord {} set to {}",
                                                vm.quipu.active_cord, n
                                            );
                                        }
                                    }
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "nova")]
                                ViewMode::Hydra => {
                                    // Enable editing grid from Hydra view
                                    let (x, y) = app_state.grid_cursor;
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                                #[cfg(feature = "silicon")]
                                ViewMode::Foundry => {
                                    // Same as Schematic/Grid?
                                    // Allow editing grid in Foundry
                                    let val = parse_grid_value(&app_state.input_buffer);
                                    let (x, y) = app_state.grid_cursor;
                                    vm.grid[y][x] = val;
                                    app_state.status_msg = format!("Grid updated at {},{}", x, y);
                                    app_state.input_mode = InputMode::Normal;
                                    app_state.input_buffer.clear();
                                }
                            }
                        }
                        KeyCode::Tab =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                            }
                        }
                        KeyCode::Esc => {
                            app_state.input_mode = InputMode::Normal;
                            app_state.input_buffer.clear();
                        }
                        KeyCode::Char(c) =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.push(c);
                            } else {
                                app_state.input_buffer.push(c);
                            }
                        }
                        KeyCode::Backspace =>
                        {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Babel = app_state.view_mode {
                                let target = if app_state.babel_focus == 0 {
                                    &mut app_state.babel_pattern
                                } else {
                                    &mut app_state.babel_input
                                };
                                target.pop();
                            } else {
                                app_state.input_buffer.pop();
                            }
                        }
                        _ => {}
                    }
                    continue;
                }

                // Handle Normal Mode
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
                    if app_state.view_mode == ViewMode::Orca {
                        if c == ' ' {
                            // Let Space fall through
                        } else if c.is_ascii_graphic() {
                            let (x, y) = app_state.grid_cursor;
                            vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            continue;
                        }
                    }

                    if c != 'q' && c != ' ' && c != 'm' && c != 'c' && vm.handle_input(c) {
                        continue;
                    }
                }

                match key.code {
                    KeyCode::Tab => {
                        app_state.view_mode = match app_state.view_mode {
                            ViewMode::Genome => ViewMode::Grid,
                            ViewMode::Grid => ViewMode::Microscope,
                            ViewMode::Microscope => {
                                #[cfg(feature = "biophysics")]
                                {
                                    ViewMode::Cortex
                                }
                                #[cfg(not(feature = "biophysics"))]
                                {
                                    #[cfg(feature = "resonance")]
                                    {
                                        ViewMode::Resonance
                                    }
                                    #[cfg(not(feature = "resonance"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Grimoire
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Heatmap
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                #[cfg(feature = "resonance")]
                                {
                                    ViewMode::Resonance
                                }
                                #[cfg(not(feature = "resonance"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Grimoire
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Heatmap
                                    }
                                }
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Grimoire
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Heatmap
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => ViewMode::Topology,
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => ViewMode::Graveyard,
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => ViewMode::PianoRoll,
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => ViewMode::Retina,
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => ViewMode::Quantum,
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => ViewMode::Dream,
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => ViewMode::Phylogeny,
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => ViewMode::Alchemy,
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => ViewMode::Memetics,
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => ViewMode::Egregore,
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => ViewMode::Bestiary,
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => ViewMode::Kaleidoscope,
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => ViewMode::Void,
                            #[cfg(feature = "nova")]
                            ViewMode::Void => ViewMode::Signals,
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => ViewMode::Sovereignty,
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => ViewMode::Spectrogram,
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => ViewMode::Market,
                            #[cfg(feature = "nova")]
                            ViewMode::Market => ViewMode::Ballistics,
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => ViewMode::Scent,
                            #[cfg(feature = "nova")]
                            ViewMode::Scent => ViewMode::Fishing,
                            #[cfg(feature = "nova")]
                            ViewMode::Fishing => ViewMode::Arena,
                            #[cfg(feature = "nova")]
                            ViewMode::Arena => ViewMode::Garden,
                            #[cfg(feature = "nova")]
                            ViewMode::Garden => ViewMode::Orca,
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => ViewMode::Babel,
                            #[cfg(feature = "nova")]
                            ViewMode::Babel => ViewMode::Strings,
                            #[cfg(feature = "nova")]
                            ViewMode::Strings => ViewMode::Quipu,
                            #[cfg(feature = "nova")]
                            ViewMode::Quipu => ViewMode::Hydra,
                            #[cfg(feature = "nova")]
                            ViewMode::Hydra => ViewMode::Chronos,
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => ViewMode::Logos,
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => ViewMode::Heatmap,
                            ViewMode::Heatmap => {
                                #[cfg(feature = "silicon")]
                                {
                                    ViewMode::Schematic
                                }
                                #[cfg(not(feature = "silicon"))]
                                {
                                    #[cfg(feature = "elektra")]
                                    {
                                        ViewMode::Elektra
                                    }
                                    #[cfg(not(feature = "elektra"))]
                                    {
                                        #[cfg(feature = "nova")]
                                        {
                                            ViewMode::Laboratory
                                        }
                                        #[cfg(not(feature = "nova"))]
                                        {
                                            ViewMode::Genome
                                        }
                                    }
                                }
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => ViewMode::Foundry,
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                #[cfg(feature = "elektra")]
                                {
                                    ViewMode::Elektra
                                }
                                #[cfg(not(feature = "elektra"))]
                                {
                                    #[cfg(feature = "nova")]
                                    {
                                        ViewMode::Laboratory
                                    }
                                    #[cfg(not(feature = "nova"))]
                                    {
                                        ViewMode::Genome
                                    }
                                }
                            }
                            #[cfg(feature = "elektra")]
                            ViewMode::Elektra => {
                                #[cfg(feature = "nova")]
                                {
                                    ViewMode::Laboratory
                                }
                                #[cfg(not(feature = "nova"))]
                                {
                                    ViewMode::Genome
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => ViewMode::Genome,
                        };
                    }
                    KeyCode::Char('h') => app_state.view_mode = ViewMode::Heatmap,
                    #[cfg(feature = "silicon")]
                    KeyCode::Char('F') => app_state.view_mode = ViewMode::Foundry,
                    #[cfg(feature = "elektra")]
                    KeyCode::Char('E') => app_state.view_mode = ViewMode::Elektra,
                    KeyCode::Char('p') => {
                        if let ViewMode::Grid = app_state.view_mode {
                            app_state.palette_open = !app_state.palette_open;
                        } else {
                            #[cfg(feature = "nova")]
                            {
                                app_state.view_mode = ViewMode::PianoRoll;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('z') => app_state.view_mode = ViewMode::Bestiary,
                    KeyCode::Char('i') => {
                        app_state.input_mode = InputMode::Injection;
                        app_state.input_buffer.clear();
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('r') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            match vm.resurrect_from_graveyard(app_state.selected_graveyard_strand) {
                                Ok(idx) => {
                                    app_state.status_msg = format!("Resurrected strand {}!", idx);
                                    if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                        && !vm.graveyard.is_empty()
                                    {
                                        app_state.selected_graveyard_strand =
                                            vm.graveyard.len() - 1;
                                    }
                                }
                                Err(e) => app_state.status_msg = format!("Error: {}", e),
                            }
                        }
                    }
                    #[cfg(feature = "biophysics")]
                    KeyCode::Char('b') => app_state.view_mode = ViewMode::Cortex,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('a') => {
                        if let ViewMode::Alchemy = app_state.view_mode {
                            // Add to Crucible
                            match app_state.alchemy_selection {
                                0 => {
                                    // Shelf
                                    let elements = [
                                        "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead",
                                        "Energy",
                                    ];
                                    if app_state.alchemy_shelf_idx < elements.len() {
                                        vm.crucible.add(crate::vm::Value::Str(
                                            elements[app_state.alchemy_shelf_idx].to_string(),
                                        ));
                                    }
                                }
                                1 => {
                                    // Strands
                                    if app_state.alchemy_strand_idx < vm.dna.helix.strands.len() {
                                        vm.crucible.add(crate::vm::Value::Int(
                                            app_state.alchemy_strand_idx as i64,
                                        ));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('x') => {
                        if let ViewMode::Graveyard = app_state.view_mode {
                            if app_state.selected_graveyard_strand < vm.graveyard.len() {
                                vm.graveyard.remove(app_state.selected_graveyard_strand);
                                app_state.status_msg = "Exterminated strand.".to_string();
                                if app_state.selected_graveyard_strand >= vm.graveyard.len()
                                    && !vm.graveyard.is_empty()
                                {
                                    app_state.selected_graveyard_strand = vm.graveyard.len() - 1;
                                }
                            }
                        } else if let ViewMode::Alchemy = app_state.view_mode {
                            vm.crucible.clear();
                            app_state.status_msg = "Crucible emptied.".to_string();
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('t') => {
                        if let ViewMode::Alchemy = app_state.view_mode {
                            crate::vm::alchemy::transmute_crucible(vm);
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('k') => app_state.view_mode = ViewMode::Kaleidoscope,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('$') => app_state.view_mode = ViewMode::Market,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('!') => app_state.view_mode = ViewMode::Ballistics,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('~') => app_state.view_mode = ViewMode::Scent,
                    KeyCode::Char('f') => {
                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Fabricate current strand
                            let (x, y) = app_state.grid_cursor;
                            let s_idx = app_state.selected_strand;
                            vm.stack.push(crate::vm::Value::Int(s_idx as i64));
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Fabricate,
                                &[],
                            );
                            app_state.status_msg =
                                format!("Fabricated strand {} at {},{}", s_idx, x, y);
                            continue;
                        }

                        #[cfg(feature = "nova")]
                        {
                            app_state.view_mode = ViewMode::Fishing;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('V') => app_state.view_mode = ViewMode::Arena,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('G') => app_state.view_mode = ViewMode::Garden,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('O') => app_state.view_mode = ViewMode::Orca,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('L') => app_state.view_mode = ViewMode::Babel,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('=') => app_state.view_mode = ViewMode::Strings,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('Y') => app_state.view_mode = ViewMode::Hydra,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('T') => app_state.view_mode = ViewMode::Chronos,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('U') => app_state.view_mode = ViewMode::Logos,
                    #[cfg(all(feature = "oracle", feature = "nova"))]
                    KeyCode::Char('/') => {
                        if let ViewMode::Grimoire = app_state.view_mode {
                            app_state.query_mode = true;
                            app_state.query_input.clear();
                            app_state.query_results.clear();
                        }
                    }
                    KeyCode::Char('?') => {
                        app_state.show_view_selector = !app_state.show_view_selector;
                        // Reset index when opening
                        if app_state.show_view_selector {
                            app_state.view_selector_state.borrow_mut().select(Some(0));
                        }
                    }
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Babel = app_state.view_mode {
                            // Run Parse
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_pattern.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::ParserRegex,
                                &[],
                            );
                            vm.stack
                                .push(crate::vm::Value::Str(app_state.babel_input.clone()));
                            let _ = crate::vm::babel::exec_babel_op(
                                vm,
                                crate::opcode::OpCode::Parse,
                                &[],
                            );

                            if let Some(res) = vm.stack.pop() {
                                app_state.babel_result = format!("{}", res);
                            } else {
                                app_state.babel_result = "Stack Empty/Error".to_string();
                            }
                            continue;
                        }

                        if let ViewMode::Grid = app_state.view_mode {
                            if let Some(c) = app_state.palette_char {
                                let (x, y) = app_state.grid_cursor;
                                vm.grid[y][x] = crate::vm::Value::Str(c.to_string());
                            } else {
                                vm.step();
                            }
                        } else {
                            #[cfg(feature = "nova")]
                            if let ViewMode::Arena = app_state.view_mode {
                                if let Some(arena) = &mut vm.arena {
                                    arena.tick();
                                }
                            } else if let ViewMode::Fishing = app_state.view_mode {
                                if app_state.fishing_cast {
                                    // Reel
                                    if app_state.fishing_hooked {
                                        app_state.fishing_bobber_y -= 4.0;
                                        app_state.fishing_tension += 0.05; // Reeling increases tension

                                        if app_state.fishing_bobber_y < 10.0 {
                                            // Caught!
                                            app_state.status_msg = "CAUGHT A FISH!".to_string();
                                            app_state.fishing_cast = false;
                                            app_state.fishing_hooked = false;
                                            app_state.fishing_tension = 0.0;
                                            // Maybe give energy?
                                            vm.energy += 10;
                                        }
                                    } else {
                                        // Just pull empty line
                                        app_state.fishing_cast = false;
                                        app_state.status_msg = "Reeled in empty.".to_string();
                                    }
                                } else {
                                    // Cast
                                    app_state.fishing_cast = true;
                                    app_state.fishing_bobber_y = 50.0;
                                    app_state.fishing_tension = 0.0;
                                    app_state.status_msg = "Casted line...".to_string();
                                }
                            } else if let ViewMode::Kaleidoscope = app_state.view_mode {
                                // Paint
                                let (x, y) = app_state.grid_cursor;
                                let r = match app_state.kaleidoscope_hue_idx {
                                    0 => 255,
                                    1 => 255,
                                    2 => 0,
                                    3 => 0,
                                    4 => 0,
                                    5 => 255,
                                    _ => 255,
                                };
                                let g = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 255,
                                    2 => 255,
                                    3 => 255,
                                    4 => 0,
                                    5 => 0,
                                    _ => 255,
                                };
                                let b = match app_state.kaleidoscope_hue_idx {
                                    0 => 0,
                                    1 => 0,
                                    2 => 0,
                                    3 => 255,
                                    4 => 255,
                                    5 => 255,
                                    _ => 255,
                                };

                                // Adjust for lightness (Light=0, Normal=1, Dark=2)
                                let (r, g, b) = match app_state.kaleidoscope_light_idx {
                                    0 => (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2), // Light
                                    2 => (r / 2, g / 2, b / 2), // Dark
                                    _ => (r, g, b),             // Normal
                                };

                                vm.chroma_grid[y][x].fg = Some((r as u8, g as u8, b as u8));
                            } else {
                                vm.step();
                            }
                            #[cfg(not(feature = "nova"))]
                            vm.step();
                        }
                    }
                    KeyCode::Char('s') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            // Step Piet
                            if vm.piet_state.is_none() {
                                vm.piet_state = Some(crate::vm::piet::init_piet(vm));
                            }
                            if let Some(mut state) = vm.piet_state.take() {
                                crate::vm::piet::step_piet_once(vm, &mut state);
                                vm.piet_state = Some(state);
                            }
                            continue;
                        }

                        #[cfg(feature = "silicon")]
                        {
                            app_state.view_mode = ViewMode::Schematic;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('R') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            vm.piet_state = None;
                            app_state.status_msg = "Piet State Reset".to_string();
                        } else if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                arena.reset();
                                app_state.status_msg = "Arena Reset".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('S') => {
                        if let ViewMode::Arena = app_state.view_mode {
                            if let Some(arena) = &mut vm.arena {
                                // Add random gladiators if empty
                                if arena.combatants.is_empty() {
                                    // Use some existing strands or random
                                    let mut rng = rand::thread_rng();
                                    use rand::Rng;
                                    if !vm.dna.helix.strands.is_empty() {
                                        let s1 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        let s2 = vm.dna.helix.strands
                                            [rng.gen_range(0..vm.dna.helix.strands.len())]
                                        .clone();
                                        arena.add_gladiator(s1, rng.gen());
                                        arena.add_gladiator(s2, rng.gen());
                                    }
                                }
                                arena.start();
                                app_state.status_msg = "Arena Started!".to_string();
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('[') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_hue_idx > 0 {
                                app_state.kaleidoscope_hue_idx -= 1;
                            } else {
                                app_state.kaleidoscope_hue_idx = 5;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char(']') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_hue_idx =
                                (app_state.kaleidoscope_hue_idx + 1) % 6;
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('{') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            if app_state.kaleidoscope_light_idx > 0 {
                                app_state.kaleidoscope_light_idx -= 1;
                            } else {
                                app_state.kaleidoscope_light_idx = 2;
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('}') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            app_state.kaleidoscope_light_idx =
                                (app_state.kaleidoscope_light_idx + 1) % 3;
                        }
                    }
                    KeyCode::Char('m') => vm.mutate(),
                    KeyCode::Char('c') => vm.chaos_mode = !vm.chaos_mode,
                    KeyCode::Down => match app_state.view_mode {
                        ViewMode::Genome => {
                            let s_len = vm.dna.helix.strands.len();
                            if s_len > 0 {
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if app_state.selected_gene + 1 < g_len {
                                    app_state.selected_gene += 1;
                                } else if app_state.selected_strand + 1 < s_len {
                                    app_state.selected_strand += 1;
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            app_state.babel_focus = (app_state.babel_focus + 1) % 2;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand + 1 < vm.graveyard.len() {
                                app_state.selected_graveyard_strand += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace + 1 < vm.dream_traces.len() {
                                app_state.selected_dream_trace += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx < 7 {
                                    // 8 items
                                    app_state.alchemy_shelf_idx += 1;
                                }
                            } else {
                                if app_state.alchemy_strand_idx + 1 < vm.dna.helix.strands.len() {
                                    app_state.alchemy_strand_idx += 1;
                                }
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 < 15 {
                                app_state.grid_cursor.1 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            match app_state.selected_strand {
                                // 0=A, 1=B, 2=Method
                                0 => {
                                    if app_state.lab_parent_a > 0 {
                                        app_state.lab_parent_a -= 1;
                                    }
                                }
                                1 => {
                                    if app_state.lab_parent_b > 0 {
                                        app_state.lab_parent_b -= 1;
                                    }
                                }
                                2 => {
                                    if app_state.lab_method > 0 {
                                        app_state.lab_method -= 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index + 1 < vm.sigil_registry.len() {
                                app_state.selected_sigil_index += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if !vm.organelles.is_empty() {
                                if app_state.selected_organelle_index + 1 < vm.organelles.len() {
                                    app_state.selected_organelle_index += 1;
                                }
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos + 1 < neurons_sorted.len() {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos + 1]);
                                        app_state.voltage_history.clear(); // Reset history on switch
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                    },
                    KeyCode::Up => match app_state.view_mode {
                        ViewMode::Genome => {
                            if app_state.selected_gene > 0 {
                                app_state.selected_gene -= 1;
                            } else if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                                let g_len =
                                    vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                if g_len > 0 {
                                    app_state.selected_gene = g_len - 1;
                                } else {
                                    app_state.selected_gene = 0;
                                }
                            }
                        }
                        ViewMode::Grid => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {
                            if app_state.selected_sigil_index > 0 {
                                app_state.selected_sigil_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            let max_strand = vm.dna.helix.strands.len().saturating_sub(1);
                            match app_state.selected_strand {
                                // 0=A, 1=B, 2=Method
                                0 => {
                                    if app_state.lab_parent_a < max_strand {
                                        app_state.lab_parent_a += 1;
                                    }
                                }
                                1 => {
                                    if app_state.lab_parent_b < max_strand {
                                        app_state.lab_parent_b += 1;
                                    }
                                }
                                2 => {
                                    if app_state.lab_method < 2 {
                                        app_state.lab_method += 1;
                                    }
                                }
                                _ => {}
                            }
                        }
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {
                            let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
                            neurons_sorted.sort();
                            if let Some(current) = app_state.selected_neuron_coords {
                                if let Some(pos) =
                                    neurons_sorted.iter().position(|&c| *c == current)
                                {
                                    if pos > 0 {
                                        app_state.selected_neuron_coords =
                                            Some(*neurons_sorted[pos - 1]);
                                        app_state.voltage_history.clear();
                                    }
                                }
                            } else if !neurons_sorted.is_empty() {
                                app_state.selected_neuron_coords = Some(*neurons_sorted[0]);
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {
                            if app_state.selected_graveyard_strand > 0 {
                                app_state.selected_graveyard_strand -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {
                            if app_state.selected_dream_trace > 0 {
                                app_state.selected_dream_trace -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            if app_state.alchemy_selection == 0 {
                                if app_state.alchemy_shelf_idx > 0 {
                                    app_state.alchemy_shelf_idx -= 1;
                                }
                            } else {
                                if app_state.alchemy_strand_idx > 0 {
                                    app_state.alchemy_strand_idx -= 1;
                                }
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {
                            if app_state.selected_organelle_index > 0 {
                                app_state.selected_organelle_index -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {
                            if app_state.babel_focus > 0 {
                                app_state.babel_focus -= 1;
                            } else {
                                app_state.babel_focus = 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.1 > 0 {
                                app_state.grid_cursor.1 -= 1;
                            }
                        }
                    },
                    KeyCode::Right => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            if app_state.selected_strand < 2 {
                                app_state.selected_strand += 1;
                            } else {
                                app_state.selected_strand = 0;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 1;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord + 1 < vm.quipu.cords.len() {
                                vm.quipu.active_cord += 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 < 15 {
                                app_state.grid_cursor.0 += 1;
                            }
                        }
                    },
                    KeyCode::Left => match app_state.view_mode {
                        #[cfg(feature = "nova")]
                        ViewMode::Babel => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Strings => {}
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Chronos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Logos => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "silicon")]
                        ViewMode::Foundry => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "elektra")]
                        ViewMode::Elektra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        ViewMode::Microscope => {}
                        #[cfg(feature = "resonance")]
                        ViewMode::Resonance => {}
                        #[cfg(feature = "biophysics")]
                        ViewMode::Cortex => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Grimoire => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Graveyard => {}
                        #[cfg(feature = "nova")]
                        ViewMode::PianoRoll => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Retina => {}
                        #[cfg(feature = "silicon")]
                        ViewMode::Schematic => {}
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                            } else {
                                app_state.selected_strand = 2;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quantum => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Dream => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Phylogeny => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Alchemy => {
                            app_state.alchemy_selection = 0;
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Memetics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Egregore => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Bestiary => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Kaleidoscope => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Void => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Signals => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Sovereignty => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Spectrogram => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Market => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Ballistics => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Scent => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Fishing => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Arena => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Garden => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Orca => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Quipu => {
                            if vm.quipu.active_cord > 0 {
                                vm.quipu.active_cord -= 1;
                            }
                        }
                        #[cfg(feature = "nova")]
                        ViewMode::Hydra => {
                            if app_state.grid_cursor.0 > 0 {
                                app_state.grid_cursor.0 -= 1;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        #[cfg(feature = "silicon")]
                        if let ViewMode::Foundry = app_state.view_mode {
                            // Trace current circuit
                            let (x, y) = app_state.grid_cursor;
                            vm.stack.push(crate::vm::Value::Int(y as i64));
                            vm.stack.push(crate::vm::Value::Int(x as i64));
                            crate::vm::silicon::exec_silicon_op(
                                vm,
                                crate::opcode::OpCode::Trace,
                                &[],
                            );
                            if let Some(crate::vm::Value::Int(idx)) = vm.stack.last() {
                                app_state.status_msg = format!("Traced circuit to strand {}", idx);
                                app_state.selected_strand = *idx as usize;
                            }
                            continue;
                        }

                        app_state.input_mode = InputMode::Editing;
                        match app_state.view_mode {
                            #[cfg(feature = "nova")]
                            ViewMode::Phylogeny => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Alchemy => {
                                // Prevent entering edit mode for Alchemy (uses keys instead)
                                app_state.input_mode = InputMode::Normal;
                            }
                            ViewMode::Genome => {
                                if app_state.selected_strand < vm.dna.helix.strands.len() {
                                    let g_len =
                                        vm.dna.helix.strands[app_state.selected_strand].genes.len();
                                    if app_state.selected_gene < g_len {
                                        let gene = &vm.dna.helix.strands[app_state.selected_strand]
                                            .genes[app_state.selected_gene];
                                        let mut s = format!("{}(", gene.op);
                                        for (i, arg) in gene.args.iter().enumerate() {
                                            if i > 0 {
                                                s.push(' ');
                                            }
                                            match arg {
                                                crate::ast::Nucleotide::Number(n) => {
                                                    s.push_str(&n.to_string())
                                                }
                                                crate::ast::Nucleotide::String(str_val) => {
                                                    s.push_str(&format!("\"{}\"", str_val))
                                                }
                                                crate::ast::Nucleotide::Identifier(id) => {
                                                    s.push_str(id)
                                                }
                                                crate::ast::Nucleotide::Junction(t, vals) => {
                                                    let t_str = match t {
                                                        crate::ast::JunctionType::Any => "any",
                                                        crate::ast::JunctionType::All => "all",
                                                        crate::ast::JunctionType::Dish => "dish",
                                                    };
                                                    s.push_str(t_str);
                                                    s.push('(');
                                                    for (k, v) in vals.iter().enumerate() {
                                                        if k > 0 {
                                                            s.push(' ');
                                                        }
                                                        match v {
                                                            crate::ast::Nucleotide::Number(n) => {
                                                                s.push_str(&n.to_string())
                                                            }
                                                            crate::ast::Nucleotide::String(
                                                                str_val,
                                                            ) => s.push_str(&format!(
                                                                "\"{}\"",
                                                                str_val
                                                            )),
                                                            crate::ast::Nucleotide::Identifier(
                                                                id,
                                                            ) => s.push_str(id),
                                                            crate::ast::Nucleotide::Junction(
                                                                _,
                                                                _,
                                                            ) => s.push_str("nested"),
                                                        }
                                                    }
                                                    s.push(')');
                                                }
                                            }
                                        }
                                        s.push(')');
                                        app_state.input_buffer = s;
                                    }
                                }
                            }
                            ViewMode::Grid => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Chronos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Logos => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                app_state.input_buffer = format!("{}", val);
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Foundry => {
                                // Prepare input buffer for editing
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Orca => {
                                // Enable editing grid from Orca view
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            ViewMode::Microscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "biophysics")]
                            ViewMode::Cortex => {
                                // Prevent entering edit mode for Cortex
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "resonance")]
                            ViewMode::Resonance => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Grimoire => {
                                app_state.input_mode = InputMode::Normal;
                                let mut registry: Vec<_> =
                                    vm.sigil_registry.keys().cloned().collect();
                                registry.sort();
                                if app_state.selected_sigil_index < registry.len() {
                                    let key = &registry[app_state.selected_sigil_index];
                                    if let Some(sigil) = vm.sigil_registry.get_mut(key) {
                                        sigil.auto_cast = !sigil.auto_cast;
                                        let status = if sigil.auto_cast {
                                            "ENABLED"
                                        } else {
                                            "DISABLED"
                                        };
                                        app_state.status_msg =
                                            format!("{} Auto-Cast: {}", key, status);
                                    }
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Laboratory => {
                                app_state.input_mode = InputMode::Normal;
                                let splice_op = crate::opcode::OpCode::Splice;
                                let args = vec![
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_a as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_parent_b as i64),
                                    crate::ast::Nucleotide::Number(app_state.lab_method as i64),
                                ];
                                vm.execute_gene_inner(splice_op, &args);
                                app_state.status_msg = format!(
                                    "Spliced {} & {} (Method {})",
                                    app_state.lab_parent_a,
                                    app_state.lab_parent_b,
                                    app_state.lab_method
                                );
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Topology => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Graveyard => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::PianoRoll => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Retina => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Quantum => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Dream => {
                                app_state.input_mode = InputMode::Normal;
                                if app_state.selected_dream_trace < vm.dream_traces.len() {
                                    let (target_idx, mutated_strand) = {
                                        let trace =
                                            &vm.dream_traces[app_state.selected_dream_trace];
                                        (trace.strand_idx, trace.mutated_strand.clone())
                                    };

                                    if let Some(strand) = mutated_strand {
                                        // Lucid Dreaming: Inject the strand
                                        if target_idx < vm.dna.helix.strands.len() {
                                            vm.dna.helix.strands[target_idx] = strand;
                                            app_state.status_msg = format!(
                                                "LUCID DREAM: Realized mutations for strand {}",
                                                target_idx
                                            );
                                        }
                                    }
                                }
                            }
                            ViewMode::Heatmap => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Memetics => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Egregore => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Bestiary => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Kaleidoscope => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Void => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Signals => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Sovereignty => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Spectrogram => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Market => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Ballistics => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Scent => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Fishing => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Arena => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Garden => {
                                // Enable editing grid from Garden view
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Babel => {
                                // No buffer prep needed, editing in place
                            }
                            #[cfg(feature = "elektra")]
                            ViewMode::Elektra => {
                                // Enable editing grid from Elektra view (like Grid view)
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Strings => {
                                app_state.input_mode = InputMode::Normal;
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Quipu => {
                                // Prepare buffer with current value
                                if vm.quipu.active_cord < vm.quipu.cords.len() {
                                    let val = vm.quipu.cords[vm.quipu.active_cord].read();
                                    app_state.input_buffer = val.to_string();
                                }
                            }
                            #[cfg(feature = "nova")]
                            ViewMode::Hydra => {
                                let (x, y) = app_state.grid_cursor;
                                let val = &vm.grid[y][x];
                                match val {
                                    crate::vm::Value::Int(n) => {
                                        app_state.input_buffer = n.to_string()
                                    }
                                    crate::vm::Value::Str(s) => app_state.input_buffer = s.clone(),
                                    _ => app_state.input_buffer = String::new(),
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn apply_glitch_fx(buffer: &mut ratatui::buffer::Buffer, intensity: f32) {
    let area = *buffer.area();
    let mut rng = rand::thread_rng();
    use rand::Rng;

    for y in area.y..area.height {
        for x in area.x..area.width {
            if rng.gen::<f32>() < intensity {
                let cell = &mut buffer[(x, y)];
                match rng.gen_range(0..4) {
                    0 => {
                        let chars = ['@', '#', '$', '%', '&', '!', '?', 'X', '.', ':', ';', '~'];
                        cell.set_char(chars[rng.gen_range(0..chars.len())]);
                    }
                    1 => {
                        let fg = cell.fg;
                        cell.fg = cell.bg;
                        cell.bg = fg;
                    }
                    2 => {
                        cell.fg = Color::DarkGray;
                    }
                    3 => {
                        cell.set_char('?');
                        cell.set_style(Style::default().fg(Color::Red).bg(Color::Black));
                    }
                    _ => {}
                }
            }
        }
    }
}

#[cfg(feature = "nova")]
fn render_signals(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Signal Grid
    let mut grid_lines = Vec::new();
    for y in 0..crate::vm::GRID_SIZE {
        let mut line_spans = Vec::new();
        for x in 0..crate::vm::GRID_SIZE {
            let signal = vm.signal_grid[y][x];
            let trail = vm.execution_trail[y * crate::vm::GRID_SIZE + x];
            let mut style = Style::default();

            // Background for Execution Trail
            if trail > 0 {
                let intensity = trail as u8;
                // Fade from white (255) to dark blue
                style = style.bg(Color::Rgb(0, 0, intensity.min(150)));
            }

            // Foreground for Signal
            let ch = if signal > 0 {
                // Directional hint? No simple way without storing direction in signal_grid.
                // Just use intensity.
                if signal < 50 {
                    "·"
                } else if signal < 100 {
                    "+"
                } else if signal < 200 {
                    "*"
                } else {
                    "#"
                }
            } else {
                " "
            };

            if signal > 0 {
                if signal < 50 {
                    style = style.fg(Color::Cyan);
                } else if signal < 150 {
                    style = style.fg(Color::Yellow);
                } else {
                    style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                }
            } else {
                style = style.fg(Color::DarkGray);
            }

            // Cursor
            if app_state.grid_cursor == (x, y) {
                style = style.bg(Color::White).fg(Color::Black);
            }

            line_spans.push(Span::styled(ch.to_string(), style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Signal & Execution Heatmap"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Legend
    let legend_text = vec![
        Line::from("SIGNALS"),
        Line::from(Span::styled(
            "· Low Intensity",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::styled(
            "+ Med Intensity",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(Span::styled(
            "* High Intensity",
            Style::default().fg(Color::Red),
        )),
        Line::from(" "),
        Line::from("EXECUTION TRAIL"),
        Line::from(Span::styled(
            "Background (Blue Fade)",
            Style::default().bg(Color::Blue),
        )),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  - Signals propagate via 'nova_signals.rs'"),
        Line::from("  - Trail marks recent gene execution sites"),
    ];

    let legend_widget =
        Paragraph::new(legend_text).block(Block::default().borders(Borders::ALL).title("Legend"));
    f.render_widget(legend_widget, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_fishing(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    let mut block = Block::default()
        .borders(Borders::ALL)
        .title("Fishing Minigame");

    // Flash background if tension is critical
    if app_state.fishing_tension > 0.9 && vm.tick_counter % 4 < 2 {
        block = block.style(Style::default().bg(Color::Red));
    }

    let canvas = Canvas::default()
        .block(block)
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Sky Gradient
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 50.0,
                width: 100.0,
                height: 50.0,
                color: Color::Cyan,
            });
            // Top Sky (Darker)
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 75.0,
                width: 100.0,
                height: 25.0,
                color: Color::Blue,
            });

            // Water Gradient
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 50.0,
                color: Color::Blue,
            });
            // Deep Water
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 25.0,
                color: Color::DarkGray,
            });

            // Water Ripples
            for i in 0..20 {
                let speed = vm.tick_counter as f64 * 0.2;
                let rx = (speed + i as f64 * 13.0) % 95.0 + 2.0;
                let ry = 5.0 + (i as f64 * 7.0) % 40.0;
                let ch = if i % 2 == 0 { "~" } else { "-" };
                ctx.print(rx, ry, ch);
            }

            if app_state.fishing_cast {
                // Splash / Ripple around bobber
                if app_state.fishing_bobber_y < 50.0 {
                    // Bobber is underwater/surface
                    let phase = (vm.tick_counter % 6) / 2;
                    let (left, right) = match phase {
                        0 => ("(", ")"),
                        1 => ("<", ">"),
                        _ => ("{", "}"),
                    };
                    ctx.print(48.0, app_state.fishing_bobber_y, left);
                    ctx.print(51.0, app_state.fishing_bobber_y, right);
                }

                // Fishing Line
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: 50.0,
                    y1: 100.0, // Top center (approx rod tip)
                    x2: 50.0,
                    y2: app_state.fishing_bobber_y,
                    color: Color::White,
                });

                // Bobber (Icon)
                ctx.print(49.0, app_state.fishing_bobber_y, "🔴");

                // Fish (Icon)
                if app_state.fishing_fish_y > 0.0 && app_state.fishing_fish_y < 100.0 {
                    let fish_icon = if app_state.fishing_tension > 0.8 {
                        "🦈"
                    } else {
                        "🐟"
                    };
                    ctx.print(48.0, app_state.fishing_fish_y, fish_icon);
                }

                // Instructions Overlay (Bottom Right)
                ctx.print(60.0, 5.0, "SPACE: Reel");
            } else {
                ctx.print(35.0, 60.0, "Press SPACE to Cast");
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Tension Bar
    let tension = app_state.fishing_tension;
    let (tension_color, label) = if tension < 0.5 {
        (Color::Green, "SAFE")
    } else if tension < 0.8 {
        (Color::Yellow, "WARNING")
    } else {
        (Color::Red, "CRITICAL")
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Line Tension"))
        .gauge_style(Style::default().fg(tension_color))
        .ratio(tension.clamp(0.0, 1.0))
        .label(format!("{} ({:.0}%)", label, tension * 100.0));
    f.render_widget(gauge, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_sovereignty(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.area());

    // Left: Sovereignty Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let owner = vm.sovereignty_grid[y][x];
            let mut style = Style::default();
            let mut ch = " ".to_string();

            if let Some(id) = owner {
                // Generate color from id
                let colors = [
                    Color::Red,
                    Color::Green,
                    Color::Blue,
                    Color::Yellow,
                    Color::Magenta,
                    Color::Cyan,
                    Color::White,
                ];
                let bg = colors[id % colors.len()];
                style = style.bg(bg).fg(Color::Black);
                ch = format!("{:X}", id % 16);
            } else {
                style = style.fg(Color::DarkGray);
                ch = "·".to_string();
            }

            // Highlight cursor
            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Sovereignty Map (Territory)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Tax Rates & Info
    let (cx, cy) = app_state.grid_cursor;
    let owner = vm.sovereignty_grid[cy][cx];

    let mut info_lines = Vec::new();

    if let Some(id) = owner {
        info_lines.push(Line::from(format!("Owner: Strand {}", id)));
        if let Some(rate) = vm.tax_rates.get(&id) {
            info_lines.push(Line::from(format!("Tax Rate: {} Energy/tick", rate)));
        } else {
            info_lines.push(Line::from("Tax Rate: 0 (Free)"));
        }
    } else {
        info_lines.push(Line::from("Owner: None (Wilderness)"));
        info_lines.push(Line::from("Tax Rate: 0"));
    }

    info_lines.push(Line::from(""));
    info_lines.push(Line::from("Mechanics:"));
    info_lines.push(Line::from("  - claim(radius): Claim empty cells"));
    info_lines.push(Line::from("  - cede(y, x): Release cells"));
    info_lines.push(Line::from("  - tax(rate): Set tax for your land"));
    info_lines.push(Line::from("  - sovereignty(y, x): Check owner"));

    let info_widget = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Territory Info"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "silicon")]
fn render_foundry(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(f.area());

    // Left: Schematic Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            // Reuse schematic styling logic
            let (ch, style) = match val {
                crate::vm::Value::Int(0) => (" ".to_string(), Style::default().fg(Color::DarkGray)),
                crate::vm::Value::Int(1) => ("┼".to_string(), Style::default().fg(Color::DarkGray)), // Wire
                crate::vm::Value::Int(2) => (
                    "⚡".to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ), // Head
                crate::vm::Value::Int(3) => (".".to_string(), Style::default().fg(Color::Red)), // Tail
                crate::vm::Value::Str(s) => {
                    if s.starts_with("G:") {
                        let parts: Vec<&str> = s.split(':').collect();
                        let sym = if parts.len() >= 2 {
                            match parts[1] {
                                "AND" => "&",
                                "OR" => "≥",
                                "XOR" => "=",
                                "NAND" => "!",
                                "NOT" => "¬",
                                _ => "?",
                            }
                        } else {
                            "G"
                        };
                        (
                            sym.to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("LATCH:") {
                        let state = s.trim_start_matches("LATCH:");
                        (
                            format!("L{}", state),
                            Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("EMIT:") {
                        (
                            "E".to_string(),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("RECV:") {
                        (
                            "R".to_string(),
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s == "PIN:IN" {
                        (
                            "I".to_string(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s == "PIN:OUT" {
                        (
                            "O".to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ("?".to_string(), Style::default().fg(Color::White))
                    }
                }
                _ => ("?".to_string(), Style::default().fg(Color::White)),
            };

            let mut final_style = style;
            if app_state.grid_cursor == (x, y) {
                final_style = final_style.bg(Color::White).fg(Color::Black);
            }

            line_spans.push(Span::styled(ch, final_style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Foundry (Silicon Grid)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Genetic Library (Strands)
    // Allows selecting a strand to Fabricate
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(6)].as_ref())
        .split(chunks[1]);

    let mut strand_items = Vec::new();
    for (i, strand) in vm.dna.helix.strands.iter().enumerate() {
        let is_selected = i == app_state.selected_strand;
        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        strand_items.push(
            ListItem::new(format!("Strand {} ({} genes)", i, strand.genes.len())).style(style),
        );
    }

    let strand_list =
        List::new(strand_items).block(Block::default().borders(Borders::ALL).title("DNA Library"));
    f.render_widget(strand_list, right_chunks[0]);

    // Help / Status
    let help_text = vec![
        Line::from("Foundry Operations:"),
        Line::from("  Enter: TRACE (Circuit -> DNA)"),
        Line::from("  F: FABRICATE (DNA -> Circuit)"),
        Line::from("  Nav: Arrows (Move Cursor)"),
        Line::from("  (Select Strand in Genome View)"),
    ];

    let help_widget =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(help_widget, right_chunks[1]);
}

#[cfg(feature = "oracle")]
fn render_wisdom(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Left: Knowledge Base (Facts & Rules)
    let mut kb_items = Vec::new();
    if vm.knowledge_base.is_empty() {
        kb_items.push(ListItem::new("Knowledge Base is empty."));
    } else {
        for (i, fact) in vm.knowledge_base.iter().enumerate() {
            kb_items.push(
                ListItem::new(format!("{}: {}", i, fact)).style(Style::default().fg(Color::Cyan)),
            );
        }
    }
    let kb_list = List::new(kb_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Knowledge Base (Facts & Rules)"),
    );
    f.render_widget(kb_list, chunks[0]);

    // Right: Omens & Query Results
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Omens
    let mut omen_items = Vec::new();
    if vm.omens.is_empty() {
        omen_items.push(ListItem::new("No active prophecies (Omens)."));
    } else {
        for (i, omen) in vm.omens.iter().enumerate() {
            omen_items.push(
                ListItem::new(format!("{}: If {} Then {}", i, omen.condition, omen.effect))
                    .style(Style::default().fg(Color::Yellow)),
            );
        }
    }
    let omen_list = List::new(omen_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Prophecies (Omens)"),
    );
    f.render_widget(omen_list, right_chunks[0]);

    // Query Results
    let mut result_items = Vec::new();
    if app_state.query_results.is_empty() {
        result_items.push(ListItem::new("No query results. Press '/' to query."));
    } else {
        for res in &app_state.query_results {
            result_items.push(ListItem::new(res.clone()).style(Style::default().fg(Color::Green)));
        }
    }
    let result_list = List::new(result_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Query: {}", app_state.query_input)),
    );
    f.render_widget(result_list, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_babel(f: &mut Frame, _vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Pattern
                Constraint::Length(3), // Input
                Constraint::Min(0),    // Result
            ]
            .as_ref(),
        )
        .split(f.area());

    let pattern_style = if app_state.babel_focus == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let input_style = if app_state.babel_focus == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let pattern_widget = Paragraph::new(app_state.babel_pattern.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Regex Pattern (Edit)"),
        )
        .style(pattern_style);
    f.render_widget(pattern_widget, chunks[0]);

    let input_widget = Paragraph::new(app_state.babel_input.clone())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Test String (Edit)"),
        )
        .style(input_style);
    f.render_widget(input_widget, chunks[1]);

    let result_widget = Paragraph::new(app_state.babel_result.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Match Result (Enter to Run)"),
    );
    f.render_widget(result_widget, chunks[2]);
}

fn render_microscope(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(f.area());

    // Scan Data
    let (cx, cy) = app_state.grid_cursor;
    let data = crate::vm::microscope::scan(vm, cy, cx);

    // Header
    let header = Paragraph::new(format!(
        "Microscope: Cell ({}, {}) - Value: {}",
        cx, cy, data.value
    ))
    .block(Block::default().borders(Borders::ALL).title("Inspection"));
    f.render_widget(header, chunks[0]);

    let main_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Left: Environment
    let env_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Hormones
                Constraint::Length(3), // Waste
                Constraint::Length(3), // Mutagen
                Constraint::Length(3), // Light
            ]
            .as_ref(),
        )
        .split(main_split[0]);

    // Hormones (RGB)
    let h = data.hormone_levels;
    let h_label = format!("Hormones [R:{} G:{} B:{}]", h[0], h[1], h[2]);
    let h_ratio = ((h[0] + h[1] + h[2]) as f64 / 765.0).clamp(0.0, 1.0);
    let h_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Hormones"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .ratio(h_ratio)
        .label(h_label);
    f.render_widget(h_gauge, env_chunks[0]);

    // Waste
    let w_ratio = (data.waste_level as f64 / 100.0).clamp(0.0, 1.0);
    let w_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Waste"))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(w_ratio)
        .label(format!("{} / 100", data.waste_level));
    f.render_widget(w_gauge, env_chunks[1]);

    // Mutagen
    let m_ratio = (data.mutagen_level as f64 / 100.0).clamp(0.0, 1.0);
    let m_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Mutagen"))
        .gauge_style(Style::default().fg(Color::Red))
        .ratio(m_ratio)
        .label(format!("{} / 100", data.mutagen_level));
    f.render_widget(m_gauge, env_chunks[2]);

    // Light
    let l_ratio = (data.light_level as f64 / 100.0).clamp(0.0, 1.0);
    let l_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Light"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(l_ratio)
        .label(format!("{} / 100", data.light_level));
    f.render_widget(l_gauge, env_chunks[3]);

    // Right: Organelles
    let rows: Vec<Row> = data
        .organelles
        .iter()
        .map(|org| {
            Row::new(vec![
                org.kind.clone(),
                format!("{:?}", org.ip),
                org.stack_depth.to_string(),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ],
    )
    .header(Row::new(vec!["Type", "IP", "Stack"]))
    .block(Block::default().borders(Borders::ALL).title("Inhabitants"));

    f.render_widget(table, main_split[1]);
}

#[cfg(feature = "nova")]
fn render_spectrogram(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Frequency/Amp Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let (freq, amp) = vm.resonance_grid[y][x];
            let mut style = Style::default();

            // Character based on amplitude
            let ch = if amp < 0.1 {
                " "
            } else if amp < 5.0 {
                "·"
            } else if amp < 20.0 {
                "~"
            } else if amp < 50.0 {
                "≈"
            } else if amp < 100.0 {
                "%"
            } else {
                "#"
            };

            // Color based on Frequency (Hue mapping)
            // Visible spectrum approx 400-700THz, audio 20-20kHz.
            // Let's map arbitrary frequency range to colors.
            // Low = Red, Mid = Green, High = Blue
            let color = if amp < 0.1 {
                Color::DarkGray
            } else if freq < 100.0 {
                Color::Red
            } else if freq < 300.0 {
                Color::Yellow
            } else if freq < 600.0 {
                Color::Green
            } else if freq < 1000.0 {
                Color::Cyan
            } else if freq < 5000.0 {
                Color::Blue
            } else {
                Color::Magenta
            };

            style = style.fg(color);

            // Cursor
            if app_state.grid_cursor == (x, y) {
                style = style.bg(Color::White).fg(Color::Black);
            }

            line_spans.push(Span::styled(ch.to_string(), style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Resonance Spectrogram"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let (freq, amp) = vm.resonance_grid[cy][cx];

    let info_text = vec![
        Line::from("SONIC FIELD"),
        Line::from(" "),
        Line::from(format!("Frequency: {:.2} Hz", freq)),
        Line::from(format!("Amplitude: {:.2}", amp)),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  - resonate(freq, amp): Emit continuous wave"),
        Line::from("  - sonic_claim(freq): Claim territory if resonant"),
        Line::from("  - dampen(amount, radius): Reduce amplitude"),
        Line::from("  - Waves diffuse and mix frequencies"),
    ];

    let info_widget = Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Wave Analysis"),
    );
    f.render_widget(info_widget, chunks[1]);
}

fn render_heatmap(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.area());

    let mut max_count = 1;
    for count in vm.gene_execution_counts.values() {
        if *count > max_count {
            max_count = *count;
        }
    }

    let mut items = Vec::new();
    for (s_idx, strand) in vm.dna.helix.strands.iter().enumerate() {
        items.push(ListItem::new(Span::styled(
            format!("Strand {}", s_idx),
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )));

        for (g_idx, gene) in strand.genes.iter().enumerate() {
            let count = vm.gene_execution_counts.get(&(s_idx, g_idx)).unwrap_or(&0);
            let ratio = (*count as f64) / (max_count as f64);
            let color = if ratio < 0.01 {
                Color::DarkGray
            } else if ratio < 0.3 {
                Color::Blue
            } else if ratio < 0.6 {
                Color::Green
            } else if ratio < 0.9 {
                Color::Yellow
            } else {
                Color::Red
            };

            let content = format!("  {}({:?}) - Exec: {}", gene.op, gene.args, count);
            items.push(ListItem::new(Span::styled(
                content,
                Style::default().fg(color),
            )));
        }
        items.push(ListItem::new(""));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Gene Expression Heatmap (Max: {})", max_count)),
    );
    f.render_widget(list, chunks[0]);
}
fn render_genome_and_grid(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunks[1]);

    // Genome View
    let helix = &vm.dna.helix;
    let mut strand_items = Vec::new();

    for (s_idx, strand) in helix.strands.iter().enumerate() {
        #[cfg(feature = "cortex")]
        {
            let mut header = format!("Strand {}", s_idx);
            if s_idx < vm.activation_levels.len() {
                header.push_str(&format!(" ⚡{}", vm.activation_levels[s_idx]));
            }
            if s_idx < vm.synapse_map.len() && !vm.synapse_map[s_idx].is_empty() {
                header.push_str(&format!(" -> {:?}", vm.synapse_map[s_idx]));
            }
            strand_items.push(ListItem::new(Span::styled(
                header,
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        #[cfg(not(feature = "cortex"))]
        {
            strand_items.push(ListItem::new(Span::styled(
                format!("Strand {}", s_idx),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
        }

        for (g_idx, gene) in strand.genes.iter().enumerate() {
            let content = format!("{}({:?})", gene.op, gene.args);
            let mut style = Style::default();
            let mut prefix = "  ";

            // Logic for execution highlighting
            #[cfg(feature = "nova")]
            if vm.epigenome.contains(&(s_idx, g_idx)) {
                style = style.fg(Color::Blue);
            }

            if s_idx == vm.ip.0 && g_idx == vm.ip.1 {
                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                #[cfg(feature = "nova")]
                if vm.epigenome.contains(&(s_idx, g_idx)) {
                    style = style.bg(Color::Blue);
                }
                prefix = "> ";
            } else if (s_idx < vm.ip.0 || (s_idx == vm.ip.0 && g_idx < vm.ip.1))
                && style.fg != Some(Color::Blue)
            {
                style = style.fg(Color::DarkGray);
            }

            // Logic for Editor Selection highlighting (Only if ViewMode::Genome)
            if app_state.view_mode == ViewMode::Genome
                && s_idx == app_state.selected_strand
                && g_idx == app_state.selected_gene
            {
                if let InputMode::Editing = app_state.input_mode {
                    style = style.bg(Color::Red).fg(Color::White);
                    prefix = "E ";
                } else {
                    style = style.bg(Color::White).fg(Color::Black);
                    prefix = "* ";
                }
            }

            strand_items.push(ListItem::new(format!("{}{}", prefix, content)).style(style));
        }
        strand_items.push(ListItem::new("-------------------"));
    }

    let chaos_status = if vm.chaos_mode { "ON" } else { "OFF" };
    let mode_str = match app_state.view_mode {
        ViewMode::Genome => "GENOME",
        ViewMode::Grid => "GRID",
        ViewMode::Microscope => "MICROSCOPE",
        #[cfg(feature = "biophysics")]
        ViewMode::Cortex => "CORTEX",
        #[cfg(feature = "resonance")]
        ViewMode::Resonance => "RESONANCE",
        #[cfg(feature = "nova")]
        ViewMode::Grimoire => "GRIMOIRE",
        #[cfg(feature = "nova")]
        ViewMode::Laboratory => "LABORATORY",
        #[cfg(feature = "nova")]
        ViewMode::Topology => "TOPOLOGY",
        #[cfg(feature = "nova")]
        ViewMode::Graveyard => "GRAVEYARD",
        #[cfg(feature = "nova")]
        ViewMode::PianoRoll => "PIANO ROLL",
        #[cfg(feature = "nova")]
        ViewMode::Retina => "RETINA",
        #[cfg(feature = "nova")]
        ViewMode::Quantum => "QUANTUM",
        #[cfg(feature = "nova")]
        ViewMode::Dream => "DREAM CATCHER",
        #[cfg(feature = "nova")]
        ViewMode::Phylogeny => "PHYLOGENY",
        #[cfg(feature = "nova")]
        ViewMode::Alchemy => "THE ALCHEMIST'S TABLE",
        #[cfg(feature = "nova")]
        ViewMode::Memetics => "MEMETICS",
        #[cfg(feature = "nova")]
        ViewMode::Egregore => "THE EGREGORE",
        #[cfg(feature = "nova")]
        ViewMode::Bestiary => "BESTIARY",
        #[cfg(feature = "nova")]
        ViewMode::Kaleidoscope => "KALEIDOSCOPE",
        #[cfg(feature = "nova")]
        ViewMode::Void => "VOID (ENTROPY)",
        #[cfg(feature = "nova")]
        ViewMode::Signals => "SIGNALS & TRAILS",
        #[cfg(feature = "nova")]
        ViewMode::Sovereignty => "SOVEREIGNTY (TERRITORY)",
        #[cfg(feature = "nova")]
        ViewMode::Spectrogram => "SPECTROGRAM (RESONANCE)",
        #[cfg(feature = "nova")]
        ViewMode::Market => "MARKET (EXCHANGE)",
        #[cfg(feature = "nova")]
        ViewMode::Ballistics => "BALLISTICS (TRAJECTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Scent => "SCENT (OLFACTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Fishing => "FISHING (MINIGAME)",
        #[cfg(feature = "nova")]
        ViewMode::Arena => "ARENA (COLOSSEUM)",
        #[cfg(feature = "nova")]
        ViewMode::Garden => "THE GARDEN OF EDEN (Cellular Automata)",
        #[cfg(feature = "nova")]
        ViewMode::Orca => "ORCA (SIGNAL GRID)",
        ViewMode::Heatmap => "HEATMAP",
        #[cfg(feature = "silicon")]
        ViewMode::Schematic => "SCHEMATIC",
        #[cfg(feature = "elektra")]
        ViewMode::Elektra => "ELEKTRA (ANALOG SIMULATION)",
        #[cfg(feature = "nova")]
        ViewMode::Babel => "BABEL (REGEX LAB)",
        #[cfg(feature = "nova")]
        ViewMode::Strings => "COSMIC STRINGS (VIBRATION)",
        #[cfg(feature = "nova")]
        ViewMode::Quipu => "QUIPU (TOPOLOGICAL MEMORY)",
        #[cfg(feature = "nova")]
        ViewMode::Hydra => "HYDRA (FLUIDIC LOGIC)",
        #[cfg(feature = "nova")]
        ViewMode::Chronos => "CHRONOS (TIME DILATION & HISTORY)",
        #[cfg(feature = "nova")]
        ViewMode::Logos => "LOGOS (LOGIC CHEMISTRY)",
        #[cfg(feature = "silicon")]
        ViewMode::Foundry => "FOUNDRY (GENETIC CIRCUITRY)",
    };

    let title = match app_state.input_mode {
                InputMode::Normal => format!(
                    "{} (Tab: Switch View, Space: Step, M: Mutate, C: Chaos[{}], I: Inject, Arrows: Nav, Enter: Edit, Q: Quit)",
                    mode_str, chaos_status
                ),
                InputMode::Editing => format!(
                    "EDITING {} (Enter: Commit, Esc: Cancel) - {}",
                    mode_str, app_state.input_buffer
                ),
                InputMode::Injection => "INJECTION (Enter: Splice, Esc: Cancel)".to_string(),
            };

    let genome_block = Block::default().borders(Borders::ALL).title("Genome");
    let genome_style = if app_state.view_mode == ViewMode::Genome {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    // Highlight the active block borders/title
    let genome_list = List::new(strand_items).block(
        genome_block.border_style(genome_style).title(title.clone()), // Show controls in main title usually
    );
    f.render_widget(genome_list, left_chunks[0]);

    // Petri Dish (Grid)
    let mut grid_lines = Vec::new();

    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];

            #[allow(unused_mut)]
            let (mut char_rep, mut style) = match val {
                crate::vm::Value::Int(0) => (".".to_string(), Style::default().fg(Color::DarkGray)),
                crate::vm::Value::Int(n) => {
                    #[cfg(feature = "silicon")]
                    if vm.silicon_mode {
                        match n {
                            1 => ("#".to_string(), Style::default().fg(Color::Yellow)), // Conductor
                            2 => (
                                "@".to_string(),
                                Style::default().fg(Color::White).bg(Color::Cyan),
                            ), // Head
                            3 => ("~".to_string(), Style::default().fg(Color::Red)),    // Tail
                            _ => (
                                format!("{}", (n.abs() % 10)),
                                Style::default().fg(Color::Green),
                            ),
                        }
                    } else {
                        (
                            format!("{}", (n.abs() % 10)),
                            Style::default().fg(Color::Green),
                        )
                    }
                    #[cfg(not(feature = "silicon"))]
                    (
                        format!("{}", (n.abs() % 10)),
                        Style::default().fg(Color::Green),
                    )
                }
                crate::vm::Value::Junction(_, _) => {
                    ("J".to_string(), Style::default().fg(Color::Yellow))
                }
                crate::vm::Value::Superposition(_) => (
                    "Ψ".to_string(),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ),
                crate::vm::Value::Str(s) => {
                    let mut symbol = if s.starts_with("G:") {
                        let parts: Vec<&str> = s.split(':').collect();
                        if parts.len() >= 2 {
                            match parts[1] {
                                "AND" => "&",
                                "OR" => "|",
                                "XOR" => "^",
                                "NAND" => "!",
                                "NOT" => "~",
                                _ => "G",
                            }
                        } else {
                            "G"
                        }
                    } else {
                        match s.as_str() {
                            "virus" => "V",
                            "incubate" => "I",
                            "push" => "^",
                            "add" => "+",
                            "sub" => "-",
                            "mul" => "*",
                            "div" => "/",
                            "jump" | "jump_s" => "J",
                            "brz" | "brz_s" => "?",
                            "photosynthesize" => "P",
                            "consume" => "C",
                            "g_read" => "R",
                            "g_write" => "W",
                            "mitosis" => "M",
                            "apoptosis" => "X",
                            "fire" => "F",
                            "water" => "W",
                            "earth" => "E",
                            "air" => "A",
                            "steam" => "S",
                            "lava" => "L",
                            "cloud" => "C",
                            "spirit" => "S",
                            "gold" => "G",
                            "lead" => "L",
                            _ => &s[0..1],
                        }
                    };
                    let style = if s.starts_with("G:") {
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        match s.as_str() {
                            "fire" => Style::default().fg(Color::Red),
                            "water" => Style::default().fg(Color::Blue),
                            "earth" => Style::default().fg(Color::Yellow),
                            "air" => Style::default().fg(Color::Cyan),
                            "steam" => Style::default().fg(Color::White),
                            "lava" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            "cloud" => Style::default()
                                .fg(Color::White)
                                .add_modifier(Modifier::BOLD),
                            "spirit" => Style::default().fg(Color::Magenta),
                            "gold" => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            "lead" => Style::default().fg(Color::DarkGray),
                            "PIN:IN" => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            "PIN:OUT" => Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                            s if s.starts_with("EMIT:") => Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                            s if s.starts_with("RECV:") => Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::BOLD),
                            _ => Style::default().fg(Color::Cyan),
                        }
                    };
                    if s == "PIN:IN" {
                        symbol = "I";
                    } else if s == "PIN:OUT" {
                        symbol = "O";
                    } else if s.starts_with("EMIT:") {
                        symbol = "E";
                    } else if s.starts_with("RECV:") {
                        symbol = "R";
                    }
                    (symbol.to_string(), style)
                }
            };

            #[cfg(feature = "nova")]
            {
                let h = vm.hormone_grid[y][x];
                let r = h[0].clamp(0, 255) as u8;
                let g = h[1].clamp(0, 255) as u8;
                let b = h[2].clamp(0, 255) as u8;
                if r > 0 || g > 0 || b > 0 {
                    style = style.bg(Color::Rgb(r, g, b));
                    if (r as u16 + g as u16 + b as u16) > 300 {
                        style = style.fg(Color::Black);
                    }
                }

                if vm.signal_grid[y][x] > 0 {
                    // Signal active!
                    style = style
                        .bg(Color::White)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                }

                // Chromatophores (Nova)
                let chroma = &vm.chroma_grid[y][x];
                if let Some(c) = chroma.char {
                    char_rep = c.to_string();
                }
                if let Some((r, g, b)) = chroma.fg {
                    style = style.fg(Color::Rgb(r, g, b));
                }

                if vm.waste_grid[y][x] > 50 {
                    style = style.add_modifier(Modifier::CROSSED_OUT);
                    if vm.waste_grid[y][x] > 100 {
                        style = style.fg(Color::Red);
                    }
                }

                #[cfg(feature = "nova")]
                if vm.mutagen_grid[y][x] > 20 {
                    // Purple haze for radiation
                    if vm.mutagen_grid[y][x] > 50 {
                        style = style.bg(Color::Magenta).fg(Color::White);
                    } else {
                        style = style.fg(Color::Magenta);
                    }
                }

                if let Some(organelle) = vm.organelles.iter().find(|o| o.context_loc == (y, x)) {
                    let mut color = match organelle.kind {
                        crate::vm::nova::OrganelleType::Chloroplast => Color::Green,
                        crate::vm::nova::OrganelleType::Mitochondria => Color::Red,
                        crate::vm::nova::OrganelleType::Lysosome => Color::Magenta,
                        crate::vm::nova::OrganelleType::Ribosome => Color::Cyan,
                        crate::vm::nova::OrganelleType::Void => Color::DarkGray,
                        crate::vm::nova::OrganelleType::Alchemist => Color::Yellow,
                        crate::vm::nova::OrganelleType::Seed => Color::Green,
                        crate::vm::nova::OrganelleType::Choir => Color::Blue,
                        crate::vm::nova::OrganelleType::Wisp => Color::Yellow,
                        crate::vm::nova::OrganelleType::Worker => Color::White,
                    };
                    let char_code = match organelle.kind {
                        crate::vm::nova::OrganelleType::Chloroplast => "C",
                        crate::vm::nova::OrganelleType::Mitochondria => "M",
                        crate::vm::nova::OrganelleType::Lysosome => "L",
                        crate::vm::nova::OrganelleType::Ribosome => "R",
                        crate::vm::nova::OrganelleType::Void => "Ø",
                        crate::vm::nova::OrganelleType::Alchemist => "A",
                        crate::vm::nova::OrganelleType::Seed => "S",
                        crate::vm::nova::OrganelleType::Choir => "♫",
                        crate::vm::nova::OrganelleType::Wisp => "*",
                        crate::vm::nova::OrganelleType::Worker => "O",
                    };

                    if organelle.ttl.is_some() {
                        color = Color::Yellow;
                    }

                    style = style
                        .bg(color)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD);
                    if char_rep == "." {
                        char_rep = char_code.to_string();
                    }
                }

                if let Some(target) = vm.sonar_target {
                    if target == (y, x) {
                        style = style
                            .bg(Color::Yellow)
                            .fg(Color::Black)
                            .add_modifier(Modifier::SLOW_BLINK);
                    }
                }

                if vm.portals.contains_key(&(y, x)) {
                    char_rep = "@".to_string();
                    style = style
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
                }

                if (vm.membranes[y][x] & 2) != 0 {
                    style = style.add_modifier(Modifier::UNDERLINED);
                }

                // Viral Infection
                if let Some(state) = &vm.viral_grid[y][x] {
                    if state.virus_id < vm.virus_library.len() {
                        let virus = &vm.virus_library[state.virus_id];
                        let (r, g, b) = virus.color;
                        style = style.bg(Color::Rgb(r, g, b)).fg(Color::Black);

                        if state.infection_level > 150 {
                            let chars = ['@', '#', '$', '%', '&', '!', '?', 'X'];
                            let idx = (x + y + vm.tick_counter as usize) % chars.len();
                            char_rep = chars[idx].to_string();
                            style = style.add_modifier(Modifier::RAPID_BLINK);
                        }
                    }
                }

                // Projectiles (Top Layer)
                for p in &vm.projectiles {
                    if (p.x as usize) == x && (p.y as usize) == y {
                        style = style
                            .fg(Color::Red)
                            .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                        if p.vx.abs() > p.vy.abs() {
                            if p.vx > 0.0 {
                                char_rep = "→".to_string();
                            } else {
                                char_rep = "←".to_string();
                            }
                        } else {
                            if p.vy > 0.0 {
                                char_rep = "↓".to_string();
                            } else {
                                char_rep = "↑".to_string();
                            }
                        }
                    }
                }
            }

            // Highlight Cursor in Grid Mode
            if app_state.view_mode == ViewMode::Grid && app_state.grid_cursor == (x, y) {
                if let InputMode::Editing = app_state.input_mode {
                    style = style.bg(Color::Red).fg(Color::White);
                    // If editing, maybe show first char of input buffer?
                    // But input buffer might be long string "add".
                    // Let's just highlight the cell.
                } else {
                    style = style.bg(Color::White).fg(Color::Black);
                }
            }

            line_spans.push(Span::styled(char_rep, style));

            #[allow(unused_mut)]
            let mut spacer = " ";
            #[cfg(feature = "nova")]
            if (vm.membranes[y][x] & 4) != 0 {
                spacer = "|";
            }
            line_spans.push(Span::raw(spacer));
        }
        grid_lines.push(Line::from(line_spans));
    }

    #[cfg(feature = "nova")]
    let topology_name = format!("{:?}", vm.topology);
    #[cfg(not(feature = "nova"))]
    let topology_name = "Classic";

    let grid_title = format!("Petri Dish (16x16) - {}", topology_name);
    let grid_style = if app_state.view_mode == ViewMode::Grid {
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let grid_paragraph = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(grid_title)
            .border_style(grid_style),
    );
    f.render_widget(grid_paragraph, left_chunks[1]);

    // Cytoplasm (Stack)
    let stack_items: Vec<ListItem> = vm
        .stack
        .iter()
        .rev()
        .map(|val| ListItem::new(format!("{}", val)))
        .collect();

    let stack_list = List::new(stack_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Cytoplasm (Stack) - Energy: {}", vm.energy)),
    );
    f.render_widget(stack_list, right_chunks[0]);

    // Output
    let mut output_items: Vec<ListItem> = vm
        .output
        .iter()
        .rev()
        .map(|val| ListItem::new(val.clone()))
        .collect();

    if !app_state.status_msg.is_empty() {
        output_items.insert(
            0,
            ListItem::new(Span::styled(
                format!("STATUS: {}", app_state.status_msg),
                Style::default().fg(Color::Yellow),
            )),
        );
    }
    if let InputMode::Editing = app_state.input_mode {
        if app_state.view_mode == ViewMode::Grid {
            output_items.insert(
                0,
                ListItem::new(Span::styled(
                    format!(
                        "EDIT GRID [{},{}]: {}",
                        app_state.grid_cursor.0, app_state.grid_cursor.1, app_state.input_buffer
                    ),
                    Style::default().fg(Color::Cyan),
                )),
            );
        }
    }

    let output_list =
        List::new(output_items).block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_widget(output_list, right_chunks[1]);

    // Draw Injection Popup
    if let InputMode::Injection = app_state.input_mode {
        let area = f.area();
        let popup_area = ratatui::layout::Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        f.render_widget(ratatui::widgets::Clear, popup_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title("Viral Injection Vector (ChimeraScript)")
            .style(Style::default().fg(Color::Green));
        let text = Paragraph::new(app_state.input_buffer.clone())
            .block(block)
            .wrap(ratatui::widgets::Wrap { trim: true });
        f.render_widget(text, popup_area);
    }

    // Draw Spirit Popup on top
    #[cfg(feature = "nova")]
    if vm.spirit_request {
        let area = f.area();
        let popup_area = ratatui::layout::Rect {
            x: area.width / 4,
            y: area.height / 3,
            width: area.width / 2,
            height: 5,
        };
        f.render_widget(ratatui::widgets::Clear, popup_area);

        let prompt = vm.spirit_message.as_deref().unwrap_or("SPIRIT SUMMONING");
        let text = format!("{}\n\n> {}", prompt, app_state.input_buffer);
        let popup = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Spirit Communication")
                .style(Style::default().fg(Color::Cyan)),
        );
        f.render_widget(popup, popup_area);
    }

    if app_state.palette_open {
        render_palette(f, app_state);
    }

    if app_state.show_view_selector {
        render_view_selector(f, app_state);
    }
}

fn get_all_views() -> Vec<(ViewMode, &'static str, &'static str)> {
    let mut views = vec![
        (ViewMode::Genome, "Genome", "Tab"),
        (ViewMode::Grid, "Grid", "Tab"),
        (ViewMode::Microscope, "Microscope", "Tab"),
        (ViewMode::Heatmap, "Heatmap", "h"),
    ];

    #[cfg(feature = "biophysics")]
    views.push((ViewMode::Cortex, "Cortex", "b"));

    #[cfg(feature = "resonance")]
    views.push((ViewMode::Resonance, "Resonance", "Tab"));

    #[cfg(feature = "elektra")]
    views.push((ViewMode::Elektra, "Elektra", "E"));

    #[cfg(feature = "silicon")]
    {
        views.push((ViewMode::Schematic, "Schematic", "s"));
        views.push((ViewMode::Foundry, "Foundry", "F"));
    }

    #[cfg(feature = "nova")]
    {
        views.push((ViewMode::Grimoire, "Grimoire", "Tab"));
        views.push((ViewMode::Laboratory, "Laboratory", "Tab"));
        views.push((ViewMode::Topology, "Topology", "Tab"));
        views.push((ViewMode::Graveyard, "Graveyard", "Tab"));
        views.push((ViewMode::PianoRoll, "Piano Roll", "p"));
        views.push((ViewMode::Retina, "Retina", "Tab"));
        views.push((ViewMode::Quantum, "Quantum", "Tab"));
        views.push((ViewMode::Dream, "Dream Catcher", "Tab"));
        views.push((ViewMode::Phylogeny, "Phylogeny", "Tab"));
        views.push((ViewMode::Alchemy, "Alchemy", "Tab"));
        views.push((ViewMode::Memetics, "Memetics", "Tab"));
        views.push((ViewMode::Egregore, "Egregore", "Tab"));
        views.push((ViewMode::Bestiary, "Bestiary", "z"));
        views.push((ViewMode::Kaleidoscope, "Kaleidoscope", "k"));
        views.push((ViewMode::Void, "Void", "Tab"));
        views.push((ViewMode::Signals, "Signals", "Tab"));
        views.push((ViewMode::Sovereignty, "Sovereignty", "Tab"));
        views.push((ViewMode::Spectrogram, "Spectrogram", "Tab"));
        views.push((ViewMode::Market, "Market", "$"));
        views.push((ViewMode::Ballistics, "Ballistics", "!"));
        views.push((ViewMode::Scent, "Scent", "~"));
        views.push((ViewMode::Fishing, "Fishing", "f"));
        views.push((ViewMode::Arena, "Arena", "V"));
        views.push((ViewMode::Garden, "Garden", "G"));
        views.push((ViewMode::Orca, "Orca", "O"));
        views.push((ViewMode::Babel, "Babel", "L"));
        views.push((ViewMode::Strings, "Strings", "="));
        views.push((ViewMode::Quipu, "Quipu", "Tab"));
        views.push((ViewMode::Hydra, "Hydra", "Y"));
        views.push((ViewMode::Chronos, "Chronos", "T"));
        views.push((ViewMode::Logos, "Logos", "U"));
    }
    views
}

fn render_view_selector(f: &mut Frame, app_state: &AppState) {
    let area = f.area();
    let width = 60;
    let height = 30;
    let x = (area.width.saturating_sub(width)) / 2;
    let y = (area.height.saturating_sub(height)) / 2;
    let rect = ratatui::layout::Rect {
        x,
        y,
        width,
        height,
    };

    f.render_widget(ratatui::widgets::Clear, rect);

    let views = get_all_views();

    let current_selected = app_state
        .view_selector_state
        .borrow()
        .selected()
        .unwrap_or(0);

    let items: Vec<ListItem> = views
        .iter()
        .enumerate()
        .map(|(i, (_mode, name, key))| {
            let style = if i == current_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };
            ListItem::new(format!("{:<20} [{}]", name, key)).style(style)
        })
        .collect();

    let block = Block::default()
        .borders(Borders::ALL)
        .title("View Selector (? to Toggle)");

    let list = List::new(items)
        .block(block)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    let mut state = app_state.view_selector_state.borrow_mut();
    f.render_stateful_widget(list, rect, &mut *state);
}

fn render_palette(f: &mut Frame, app_state: &AppState) {
    let area = f.area();
    let width = 30;
    let height = 10;
    let x = (area.width - width) / 2;
    let y = (area.height - height) / 2;
    let rect = ratatui::layout::Rect {
        x,
        y,
        width,
        height,
    };

    f.render_widget(ratatui::widgets::Clear, rect);

    let chars = [
        '*', 'o', 'x', '^', 'v', '<', '>', '+', '-', '/', '%', '!', '=', ':', ';', '?',
    ];
    let mut lines = Vec::new();

    for row in 0..4 {
        let mut spans = Vec::new();
        for col in 0..4 {
            let idx = row * 4 + col;
            if idx < chars.len() {
                let ch = chars[idx];
                let style = if idx == app_state.palette_idx {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Cyan)
                };
                spans.push(Span::styled(format!(" {} ", ch), style));
                spans.push(Span::raw(" "));
            }
        }
        lines.push(Line::from(spans));
        lines.push(Line::from(""));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Operator Palette (Enter)");

    let p = Paragraph::new(lines)
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, rect);
}

#[cfg(feature = "biophysics")]
fn render_cortex(f: &mut Frame, vm: &mut ChimeraVM, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    let right_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Neuron List
    let mut neuron_items = Vec::new();
    let mut neurons_sorted: Vec<_> = vm.neurons.keys().collect();
    neurons_sorted.sort();

    for (i, coord) in neurons_sorted.iter().enumerate() {
        let neuron = &vm.neurons[coord];
        let is_selected = app_state.selected_neuron_coords == Some(**coord);

        let style = if is_selected {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        // Auto-select first if none selected
        if app_state.selected_neuron_coords.is_none() && i == 0 {
            app_state.selected_neuron_coords = Some(**coord);
        }

        neuron_items.push(ListItem::new(Span::styled(
            format!("({}, {}) - {:.2}mV", coord.1, coord.0, neuron.v),
            style,
        )));
    }

    let neuron_list = List::new(neuron_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Neurons (Cortex)"),
    );
    f.render_widget(neuron_list, chunks[0]);

    // Details & Oscilloscope
    if let Some(coord) = app_state.selected_neuron_coords {
        if let Some(neuron) = vm.neurons.get(&coord) {
            // Sparkline
            // ratatui::widgets::Sparkline is what we need but we must import it if not present.
            // Wait, previous code used Sparkline but it wasn't in imports in my `read_file`.
            // It must be there or I missed it.
            // I'll assume it works as it was existing code.
            let history = &app_state.voltage_history;
            let sparkline = ratatui::widgets::Sparkline::default()
                .block(
                    Block::default()
                        .title("Voltage Trace")
                        .borders(Borders::ALL),
                )
                .data(history)
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(sparkline, right_split[1]);

            // Details
            let details = vec![
                Line::from(format!("Membrane Potential (v): {:.2} mV", neuron.v)),
                Line::from(format!("Injected Current (i_inj): {:.2}", neuron.i_inj)),
                Line::from(" "),
                Line::from(format!("Na Activation (m): {:.4}", neuron.m)),
                Line::from(format!("Na Inactivation (h): {:.4}", neuron.h)),
                Line::from(format!("K Activation (n): {:.4}", neuron.n)),
            ];
            let info = Paragraph::new(details).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Neuron Details [{}, {}]", coord.1, coord.0)),
            );
            f.render_widget(info, right_split[0]);
        }
    }
}
#[cfg(feature = "resonance")]
fn render_resonance(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Wave Grid
    let mut lines = Vec::new();
    for y in 0..16 {
        let mut spans = Vec::new();
        for x in 0..16 {
            let idx = y * 16 + x;
            let val = if idx < vm.audio_snapshot.len() {
                vm.audio_snapshot[idx]
            } else {
                0.0
            };

            // Visualizing -1.0 to 1.0
            let abs_val = val.abs();
            let ch = if abs_val < 0.1 {
                "·"
            } else if abs_val < 0.3 {
                "~"
            } else if abs_val < 0.6 {
                "*"
            } else {
                "@"
            };

            let color = if val > 0.0 {
                if val > 0.5 {
                    Color::Cyan
                } else {
                    Color::Blue
                }
            } else if val < 0.0 {
                if val < -0.5 {
                    Color::Red
                } else {
                    Color::Magenta
                }
            } else {
                Color::DarkGray
            };

            spans.push(Span::styled(ch, Style::default().fg(color)));
            spans.push(Span::raw(" "));
        }
        lines.push(Line::from(spans));
    }

    let wave_grid = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Resonance Wave Function"),
    );
    f.render_widget(wave_grid, chunks[0]);

    // Help / Status
    let help_text = "Physics Simulation Active.\nUse Pluck(str), Oscillate(freq, str), Hear() ops.\n\nLeft: Wavefront Visualization\nRight: (Reserved for Spectrum Analysis)";
    let help =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Cymatics"));
    f.render_widget(help, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_memetics(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)].as_ref())
        .split(f.area());

    // Left: Meme Pool
    let mut meme_items = Vec::new();
    if vm.meme_pool.memes.is_empty() {
        meme_items.push(ListItem::new("No memes in the pool."));
    } else {
        for (i, meme) in vm.meme_pool.memes.iter().enumerate() {
            let content = format!(
                "Meme #{}: {} (Vir: {} Fid: {}) [{} genes]",
                i,
                meme.description,
                meme.virulence,
                meme.fidelity,
                meme.genes.len()
            );
            meme_items.push(ListItem::new(content).style(Style::default().fg(Color::Cyan)));
        }
    }
    let meme_list =
        List::new(meme_items).block(Block::default().borders(Borders::ALL).title("Meme Pool"));
    f.render_widget(meme_list, chunks[0]);

    // Middle: Dialect (Shibboleths)
    // Show dialect for CURRENT strand (ip.0)
    let s_idx = vm.ip.0;
    let mut dialect_items = Vec::new();

    if let Some(dialect) = vm.dialects.get(&s_idx) {
        if dialect.is_empty() {
            dialect_items.push(ListItem::new("Standard Dialect (No deviations)"));
        } else {
            for (from, to) in dialect {
                dialect_items.push(
                    ListItem::new(format!("{} -> {}", from, to))
                        .style(Style::default().fg(Color::Yellow)),
                );
            }
        }
    } else {
        dialect_items.push(ListItem::new("Standard Dialect"));
    }

    let dialect_list = List::new(dialect_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Dialect (Strand {})", s_idx)),
    );
    f.render_widget(dialect_list, chunks[1]);

    // Right: Vocabulary (Global Drift)
    let mut vocab_items = Vec::new();
    if vm.vocabulary.is_empty() {
        vocab_items.push(ListItem::new("Standard Vocabulary"));
    } else {
        let mut keys: Vec<_> = vm.vocabulary.keys().collect();
        keys.sort();
        for key in keys {
            if let Some(op) = vm.vocabulary.get(key) {
                vocab_items.push(ListItem::new(format!("{} -> {}", key, op))
                    .style(Style::default().fg(Color::Green)));
            }
        }
    }
    let vocab_list = List::new(vocab_items).block(
        Block::default().borders(Borders::ALL).title("Global Vocabulary (Drift)")
    );
    f.render_widget(vocab_list, chunks[2]);
}

#[cfg(feature = "nova")]
fn render_alchemy(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20), // Shelf
                Constraint::Percentage(40), // Crucible
                Constraint::Percentage(40), // Strands
            ]
            .as_ref(),
        )
        .split(f.area());

    // Shelf
    let elements = [
        "Fire", "Water", "Earth", "Air", "Life", "Death", "Lead", "Energy",
    ];
    let mut shelf_items = Vec::new();
    for (i, elem) in elements.iter().enumerate() {
        let mut style = Style::default().fg(Color::Cyan);
        if app_state.alchemy_selection == 0 && i == app_state.alchemy_shelf_idx {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        shelf_items.push(ListItem::new(Span::styled(*elem, style)));
    }

    let shelf_border_style = if app_state.alchemy_selection == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let shelf_list = List::new(shelf_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Reagent Shelf")
            .border_style(shelf_border_style),
    );
    f.render_widget(shelf_list, chunks[0]);

    // Crucible
    let crucible_items: Vec<ListItem> = vm
        .crucible
        .contents
        .iter()
        .map(|v| ListItem::new(format!("{}", v)).style(Style::default().fg(Color::Magenta)))
        .collect();

    let crucible_list =
        List::new(crucible_items).block(Block::default().borders(Borders::ALL).title(
            "Crucible (A: Add, X: Clear, T: Transmute) [Recipes: Fire/Water/Void/Life + Strand]",
        ));
    f.render_widget(crucible_list, chunks[1]);

    // Strands
    let mut strand_items = Vec::new();
    for (i, strand) in vm.dna.helix.strands.iter().enumerate() {
        let mut style = Style::default().fg(Color::White);
        if app_state.alchemy_selection == 1 && i == app_state.alchemy_strand_idx {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
        }
        strand_items.push(
            ListItem::new(format!("Strand {} ({} genes)", i, strand.genes.len())).style(style),
        );
    }

    let strand_border_style = if app_state.alchemy_selection == 1 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };

    let strand_list = List::new(strand_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("DNA Inventory")
            .border_style(strand_border_style),
    );
    f.render_widget(strand_list, chunks[2]);
}
#[cfg(feature = "nova")]
fn render_grimoire(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Ether (IPC)
    let ether_items: Vec<ListItem> = vm
        .ether
        .iter()
        .map(|(ch, queue)| ListItem::new(format!("Channel {}: {} msgs", ch, queue.len())))
        .collect();
    let ether_list =
        List::new(ether_items).block(Block::default().borders(Borders::ALL).title("Ether (IPC)"));
    f.render_widget(ether_list, chunks[0]);

    // Oracle (KB)
    #[cfg(feature = "oracle")]
    {
        let mut kb_items: Vec<ListItem> = vm
            .knowledge_base
            .iter()
            .take(20)
            .map(|fact| ListItem::new(format!("{}", fact)))
            .collect();

        if !app_state.query_results.is_empty() {
            kb_items.push(
                ListItem::new("--- Query Results ---").style(Style::default().fg(Color::Yellow)),
            );
            for res in &app_state.query_results {
                kb_items.push(ListItem::new(res.clone()).style(Style::default().fg(Color::Cyan)));
            }
        }

        let title = if app_state.query_mode {
            format!("Oracle (Query Mode: {})", app_state.query_input)
        } else {
            "Oracle (Knowledge Base) - Press '/' to Query".to_string()
        };

        let border_style = if app_state.query_mode {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let oracle_list = List::new(kb_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(border_style),
        );
        f.render_widget(oracle_list, chunks[1]);
    }
    #[cfg(not(feature = "oracle"))]
    {
        let oracle_list = Paragraph::new("Oracle feature disabled")
            .block(Block::default().borders(Borders::ALL).title("Oracle"));
        f.render_widget(&oracle_list, chunks[1]);
    }

    // Sigil Registry (The Grimoire)
    #[cfg(feature = "nova")]
    {
        let mut registry: Vec<_> = vm.sigil_registry.iter().collect();
        registry.sort_by_key(|(k, _)| *k);

        let sigil_items: Vec<ListItem> = registry
            .iter()
            .enumerate()
            .map(|(i, (name, sigil))| {
                let status = if sigil.auto_cast {
                    "⚡ AUTO"
                } else {
                    "○ MANU"
                };
                let style = if i == app_state.selected_sigil_index {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };
                ListItem::new(format!(
                    "{} | {} ({} cells) -> Strand {}",
                    status,
                    name,
                    sigil.pattern.len(),
                    sigil.strand_idx
                ))
                .style(style)
            })
            .collect();

        let sigil_list = List::new(sigil_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("The Grimoire (Select & Enter to Toggle Auto-Cast)"),
        );
        f.render_widget(sigil_list, chunks[2]);
    }

    // Bard (Score)
    let score_text: String = crate::vm::bard::score_to_abc(&vm.score);
    let bard_paragraph = Paragraph::new(score_text)
        .block(Block::default().borders(Borders::ALL).title("Bard (Score)"));
    f.render_widget(bard_paragraph, chunks[3]);
}

#[cfg(feature = "nova")]
fn render_topology(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(f.area());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ]
            .as_ref(),
        )
        .split(chunks[0]);

    // Portals
    let portal_items: Vec<ListItem> = vm
        .portals
        .iter()
        .map(|(k, v)| ListItem::new(format!("Portal: ({},{}) -> ({},{})", k.1, k.0, v.1, v.0)))
        .collect();
    let portal_list =
        List::new(portal_items).block(Block::default().borders(Borders::ALL).title("Wormholes"));
    f.render_widget(portal_list, left_chunks[0]);

    // Entanglements
    let ent_items: Vec<ListItem> = vm
        .entangled_pairs
        .iter()
        .map(|(k, v)| ListItem::new(format!("Entangled: Strand {} <-> {}", k, v)))
        .collect();
    let ent_list = List::new(ent_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Spooky Action"),
    );
    f.render_widget(ent_list, left_chunks[1]);

    // Mycelium
    let myc_items: Vec<ListItem> = vm
        .mycelium
        .iter()
        .map(|(k, v)| {
            let neighbors: Vec<String> = v.iter().map(|n| format!("({},{})", n.1, n.0)).collect();
            ListItem::new(format!("Hyphae ({},{}): {:?}", k.1, k.0, neighbors))
        })
        .collect();
    let myc_list = List::new(myc_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Fungal Network"),
    );
    f.render_widget(myc_list, left_chunks[2]);

    // Topology Map
    let mut map_lines = Vec::new();
    for y in 0..16 {
        let mut spans = Vec::new();
        for x in 0..16 {
            let mut ch = "·".to_string();
            let mut style = Style::default().fg(Color::DarkGray);

            if vm.mycelium.contains_key(&(y, x)) {
                ch = "▓".to_string();
                style = style.fg(Color::Green);
            }
            if vm.portals.contains_key(&(y, x)) {
                ch = "Ω".to_string();
                style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD);
            }
            if vm.organelles.iter().any(|o| o.context_loc == (y, x)) {
                ch = "o".to_string();
                style = style.fg(Color::Yellow);
            }

            spans.push(Span::styled(ch, style));
            spans.push(Span::raw(" "));
        }
        map_lines.push(Line::from(spans));
    }
    let map = Paragraph::new(map_lines)
        .block(Block::default().borders(Borders::ALL).title("Topology Map"));
    f.render_widget(map, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_laboratory(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(30),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Helper to render strand preview
    let render_strand = |idx: usize, title: &str, is_focused: bool| {
        let mut items = Vec::new();
        if idx < vm.dna.helix.strands.len() {
            let strand = &vm.dna.helix.strands[idx];
            for gene in &strand.genes {
                items.push(ListItem::new(format!("{}", gene.op)));
            }
        } else {
            items.push(ListItem::new("Invalid Strand"));
        }

        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("{} (Idx: {})", title, idx))
                .border_style(border_style),
        )
    };

    // Parent A
    f.render_widget(
        render_strand(
            app_state.lab_parent_a,
            "Parent A",
            app_state.selected_strand == 0,
        ),
        chunks[0],
    );

    // Parent B
    f.render_widget(
        render_strand(
            app_state.lab_parent_b,
            "Parent B",
            app_state.selected_strand == 1,
        ),
        chunks[1],
    );

    // Child / Method
    let method_name = match app_state.lab_method {
        0 => "Interleave",
        1 => "Uniform Crossover",
        2 => "Midpoint Split",
        _ => "Unknown",
    };

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks[2]);

    let method_border = if app_state.selected_strand == 2 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let method_widget = Paragraph::new(method_name).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Splice Method")
            .border_style(method_border),
    );
    f.render_widget(method_widget, right_chunks[0]);

    // Preview Child
    // We simulate the splice to show preview
    // This is a bit expensive to do every frame but OK for TUI.
    let mut preview_items = Vec::new();

    let idx_a = app_state.lab_parent_a;
    let idx_b = app_state.lab_parent_b;
    let helix_len = vm.dna.helix.strands.len();

    if idx_a < helix_len && idx_b < helix_len {
        let genes_a = &vm.dna.helix.strands[idx_a].genes;
        let genes_b = &vm.dna.helix.strands[idx_b].genes;
        let len_a = genes_a.len();
        let len_b = genes_b.len();
        let max_len = len_a.max(len_b);

        // Simple simulation for preview (deterministic only)
        match app_state.lab_method {
            0 => {
                // Interleave
                for i in 0..max_len {
                    if i < len_a {
                        preview_items.push(
                            ListItem::new(format!("{}", genes_a[i].op))
                                .style(Style::default().fg(Color::Cyan)),
                        );
                    }
                    if i < len_b {
                        preview_items.push(
                            ListItem::new(format!("{}", genes_b[i].op))
                                .style(Style::default().fg(Color::Magenta)),
                        );
                    }
                }
            }
            1 => {
                // Uniform
                preview_items.push(
                    ListItem::new("Randomized Result").style(Style::default().fg(Color::DarkGray)),
                );
            }
            2 => {
                // Midpoint
                let mid_a = len_a / 2;
                let mid_b = len_b / 2;
                for gene in genes_a.iter().take(mid_a) {
                    preview_items.push(
                        ListItem::new(format!("{}", gene.op))
                            .style(Style::default().fg(Color::Cyan)),
                    );
                }
                for gene in genes_b.iter().skip(mid_b) {
                    preview_items.push(
                        ListItem::new(format!("{}", gene.op))
                            .style(Style::default().fg(Color::Magenta)),
                    );
                }
            }
            _ => {}
        }
    }

    let preview_list = List::new(preview_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Child Preview (Enter to Splice)"),
    );
    f.render_widget(preview_list, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_graveyard(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[0]);

    // Graveyard List
    let mut grave_items = Vec::new();
    if vm.graveyard.is_empty() {
        grave_items.push(ListItem::new("The Graveyard is empty."));
    } else {
        for (i, strand) in vm.graveyard.iter().enumerate() {
            let is_selected = i == app_state.selected_graveyard_strand;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            grave_items.push(
                ListItem::new(format!("Strand {} (Len: {})", i, strand.genes.len())).style(style),
            );
        }
    }
    let grave_list = List::new(grave_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Graveyard (Necropolis)"),
    );
    f.render_widget(grave_list, top_chunks[0]);

    // Strand Preview
    let mut gene_items = Vec::new();
    if !vm.graveyard.is_empty() && app_state.selected_graveyard_strand < vm.graveyard.len() {
        let strand = &vm.graveyard[app_state.selected_graveyard_strand];
        for gene in &strand.genes {
            gene_items.push(
                ListItem::new(format!("{}", gene.op)).style(Style::default().fg(Color::Cyan)),
            );
        }
    } else if !vm.graveyard.is_empty() {
        gene_items.push(ListItem::new("Invalid Selection"));
    } else {
        gene_items.push(ListItem::new("No souls to display."));
    }
    let preview_list = List::new(gene_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Genome of the Departed"),
    );
    f.render_widget(preview_list, top_chunks[1]);

    // Help / Status
    let help_text = "Controls:\n↑/↓: Navigate\nR: Resurrect (Exhume to Helix)\nX: Exterminate (Permanent Deletion)\nTab: Switch View";
    let help_para =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Necromancy"));
    f.render_widget(help_para, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_retina(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Retina Display
    let mut lines = Vec::new();
    for row in &vm.retina.buffer {
        let mut spans = Vec::new();
        for (ch, (r, g, b)) in row {
            spans.push(Span::styled(
                ch.to_string(),
                Style::default().fg(Color::Rgb(*r, *g, *b)),
            ));
        }
        lines.push(Line::from(spans));
    }

    let retina_widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Retina ({}x{})", vm.retina.width, vm.retina.height)),
    );
    f.render_widget(retina_widget, chunks[0]);

    let help = Paragraph::new(
        "Retina Display Active.\nControl via `retina_draw`, `retina_clear` opcodes.",
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_quantum(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Left: Entanglements
    let mut ent_items = Vec::new();
    if vm.entangled_pairs.is_empty() {
        ent_items.push(ListItem::new("No entanglement detected."));
    } else {
        let mut pairs: Vec<_> = vm.entangled_pairs.iter().collect();
        pairs.sort_by_key(|(k, _)| **k);

        // Deduplicate pairs (A<->B is same as B<->A) for display
        let mut seen = std::collections::HashSet::new();

        for (k, v) in pairs {
            let min = std::cmp::min(*k, *v);
            let max = std::cmp::max(*k, *v);
            if !seen.contains(&(min, max)) {
                seen.insert((min, max));
                ent_items.push(
                    ListItem::new(format!("Strand {} <===> Strand {}", min, max))
                        .style(Style::default().fg(Color::Cyan)),
                );
            }
        }
    }

    let ent_list = List::new(ent_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Quantum Entanglement State"),
    );
    f.render_widget(ent_list, chunks[0]);

    // Right: Superpositions
    let mut sup_items = Vec::new();
    let mut found_sup = false;
    for (i, val) in vm.stack.iter().enumerate() {
        if let crate::vm::Value::Superposition(states) = val {
            found_sup = true;
            let mut desc = format!("Stack[{}]: Ψ = {{ ", i);
            for (j, (v, p)) in states.iter().enumerate() {
                if j > 0 {
                    desc.push_str(" | ");
                }
                desc.push_str(&format!("{}: {:.2}", v, p));
            }
            desc.push_str(" }");
            sup_items.push(ListItem::new(desc).style(Style::default().fg(Color::Magenta)));
        }
    }

    if !found_sup {
        sup_items.push(ListItem::new(
            "Wavefunction has collapsed (No superpositions).",
        ));
    }

    let sup_list = List::new(sup_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Superpositions"),
    );
    f.render_widget(sup_list, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_dream(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(f.area());

    // Trace List
    let mut trace_items = Vec::new();
    if vm.dream_traces.is_empty() {
        trace_items.push(ListItem::new("No dreams recorded."));
    } else {
        for (i, trace) in vm.dream_traces.iter().enumerate() {
            let is_selected = i == app_state.selected_dream_trace;
            let mut style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            if trace.accepted {
                style = style.fg(Color::Green);
            } else {
                style = style.fg(Color::Magenta); // Discarded dreams
            }

            let mut label_suffix = "";
            if trace.is_nightmare {
                style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                label_suffix = " (NIGHTMARE)";
            }

            if is_selected {
                style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            }

            let icon = if trace.accepted { "✔" } else { "✖" };
            trace_items.push(
                ListItem::new(format!(
                    "{} Dream #{} (Strand {}) - {} Ticks{}",
                    icon, i, trace.strand_idx, trace.duration, label_suffix
                ))
                .style(style),
            );
        }
    }

    let trace_list =
        List::new(trace_items).block(Block::default().borders(Borders::ALL).title("Dream Log"));
    f.render_widget(trace_list, chunks[0]);

    // Details
    if !vm.dream_traces.is_empty() && app_state.selected_dream_trace < vm.dream_traces.len() {
        let trace = &vm.dream_traces[app_state.selected_dream_trace];

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
            .split(chunks[1]);

        let mut info_text = vec![
            Line::from(format!("Mutation: {}", trace.mutation_desc)),
            Line::from(format!(
                "Energy: {} -> {} (Cost: {})",
                trace.result_energy + trace.energy_cost, // Approx start
                trace.result_energy,
                trace.energy_cost
            )),
            Line::from(format!(
                "Status: {}",
                if trace.status == 1 { "Alive" } else { "Dead" }
            )),
            Line::from(format!("Accepted: {}", trace.accepted)),
        ];

        if trace.is_nightmare {
            info_text.push(Line::from(Span::styled(
                "TYPE: NIGHTMARE (FORCED)",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
        }

        info_text.push(Line::from(""));
        info_text.push(Line::from(Span::styled(
            "Press ENTER to Realize (Lucid Dreaming)",
            Style::default().fg(Color::Cyan),
        )));

        let info = Paragraph::new(info_text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dream Details"),
        );
        f.render_widget(info, right_chunks[0]);

        // Output Log
        let log_items: Vec<ListItem> = trace
            .output_log
            .iter()
            .map(|s| ListItem::new(s.clone()).style(Style::default().fg(Color::DarkGray)))
            .collect();

        let log_list = List::new(log_items)
            .block(Block::default().borders(Borders::ALL).title("Dream Output"));
        f.render_widget(log_list, right_chunks[1]);
    } else {
        let info = Paragraph::new("Select a dream to view details.")
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(info, chunks[1]);
    }
}

#[cfg(feature = "nova")]
fn render_piano_roll(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    // Calculate total duration to define the time window
    let mut total_duration = 0;
    for note in &vm.score {
        total_duration += note.duration as u64;
    }

    let window_size = 64; // 4 measures of 16th notes
    let window_end = total_duration as f64;
    let window_start = (total_duration as f64 - window_size as f64).max(0.0);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Piano Roll (MIDI Visualization)"),
        )
        .x_bounds([window_start, window_end.max(window_start + 1.0)])
        .y_bounds([20.0, 108.0]) // MIDI 21 (A0) to 108 (C8) covers most piano range
        .paint(|ctx| {
            // Draw grid lines (measures)
            // Every 16 ticks is a measure
            let start_measure = (window_start as u64 / 16) * 16;
            let end_measure = window_end as u64 + 16;
            for t in (start_measure..end_measure).step_by(16) {
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: t as f64,
                    y1: 20.0,
                    x2: t as f64,
                    y2: 108.0,
                    color: Color::DarkGray,
                });
            }

            // Draw notes
            let mut current_time = 0;
            for note in &vm.score {
                let start = current_time as f64;
                let end = start + note.duration as f64;
                current_time += note.duration as u64;

                // Only draw if in window
                if end > window_start && start < window_end && note.pitch > 0 {
                    // Not a rest
                    let color = match note.velocity {
                        0..=40 => Color::Blue,
                        41..=80 => Color::Cyan,
                        81..=100 => Color::Green,
                        _ => Color::Yellow, // Loud
                    };

                    ctx.draw(&Rectangle {
                        x: start,
                        y: note.pitch as f64,
                        width: note.duration as f64,
                        height: 1.0,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let help = Paragraph::new("Visualizing MIDI Score.\nX-Axis: Time (16th notes)\nY-Axis: Pitch")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}

#[cfg(feature = "nova")]
fn layout_tree_node(
    node_id: usize,
    depth: f64,
    current_y: &mut f64,
    positions: &mut std::collections::HashMap<usize, (f64, f64)>,
    vm: &ChimeraVM,
    max_depth: &mut f64,
) -> f64 {
    if depth > *max_depth {
        *max_depth = depth;
    }

    let mut my_y = *current_y;

    if let Some(node) = vm.cladistics.nodes.get(&node_id) {
        if node.children.is_empty() {
            *current_y += 1.0;
        } else {
            let mut sum_y = 0.0;
            let count = node.children.len() as f64;
            for child_id in &node.children {
                sum_y +=
                    layout_tree_node(*child_id, depth + 1.0, current_y, positions, vm, max_depth);
            }
            my_y = sum_y / count;
        }
        positions.insert(node_id, (depth, my_y));
    }
    my_y
}

#[cfg(feature = "nova")]
fn render_phylogeny(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    use ratatui::widgets::canvas::{Canvas, Line, Rectangle};

    // Layout Calculation
    // Map node_id -> (x, y)
    let mut positions: std::collections::HashMap<usize, (f64, f64)> =
        std::collections::HashMap::new();
    let mut max_depth = 0.0;

    let roots = vm.cladistics.get_roots();
    let mut current_y = 0.0;

    for root in roots {
        layout_tree_node(
            root,
            0.0,
            &mut current_y,
            &mut positions,
            vm,
            &mut max_depth,
        );
    }
    let max_height = current_y;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Phylogeny (Tree of Life)"),
        )
        .x_bounds([-1.0, max_depth + 5.0])
        .y_bounds([-1.0, max_height + 1.0])
        .paint(|ctx| {
            for (id, (x, y)) in &positions {
                if let Some(node) = vm.cladistics.nodes.get(id) {
                    // Draw node
                    let color = if node.death_tick.is_some() {
                        Color::DarkGray
                    } else {
                        Color::Green
                    };

                    ctx.draw(&Rectangle {
                        x: *x - 0.2,
                        y: *y - 0.2,
                        width: 0.4,
                        height: 0.4,
                        color,
                    });

                    // Draw link to parent
                    if let Some(pid) = node.parent_id {
                        if let Some((px, py)) = positions.get(&pid) {
                            ctx.draw(&Line {
                                x1: *px,
                                y1: *py,
                                x2: *x,
                                y2: *y,
                                color: Color::White,
                            });
                        }
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let help = Paragraph::new(format!(
        "Nodes: {} | Roots: {} | Generations: {}",
        vm.cladistics.nodes.len(),
        vm.cladistics.get_roots().len(),
        max_depth
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_egregore(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    let face_str = if vm.egregore.alignment < -20 {
        // Demon
        r#"
      / \
     |o o|
      \=/
        "#
    } else if vm.egregore.alignment > 20 {
        // Angel
        r#"
      O
    .-^-.
   (o   o)
    \ - /
        "#
    } else {
        // Neutral
        r#"
      .
     .-.
    ( - )
     '-'
        "#
    };

    let face_style = if vm.egregore.alignment < -20 {
        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
    } else if vm.egregore.alignment > 20 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let face = Paragraph::new(face_str)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Manifestation"),
        )
        .style(face_style);
    f.render_widget(face, chunks[0]);

    // Stats
    let faith = vm.egregore.faith;
    let alignment = vm.egregore.alignment;
    let timer = vm.egregore.manifestation_timer;

    let stats = vec![
        Line::from(format!("Faith: {}", faith)),
        Line::from(format!("Alignment: {} (Chaos <-> Order)", alignment)),
        Line::from(format!("Manifestation: {} ticks", timer)),
        Line::from(" "),
        Line::from("Rituals:"),
        Line::from("  pray(n) - Order"),
        Line::from("  sacrifice - Chaos"),
        Line::from("  egregore_summon(s) - Global Effect"),
    ];

    let info =
        Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title("The Covenant"));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "elektra")]
fn render_elektra(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Voltage Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let v = vm.voltage_grid[y][x];
            let r = vm.resistance_grid[y][x];
            let mut style = Style::default();

            // Voltage visualization: Yellow intensity
            // Assuming range 0-100V typically
            let intensity = (v.abs() * 2.55).clamp(0.0, 255.0) as u8;
            if v > 0.1 {
                style = style.fg(Color::Rgb(intensity, intensity, 0));
            } else if v < -0.1 {
                // Negative voltage? Blue
                style = style.fg(Color::Rgb(0, 0, intensity));
            } else {
                style = style.fg(Color::DarkGray);
            }

            // Fixed nodes
            let ch = if r == -1.0 {
                style = style
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD);
                "+".to_string() // Battery
            } else if r == -2.0 {
                style = style
                    .fg(Color::White)
                    .bg(Color::Blue)
                    .add_modifier(Modifier::BOLD);
                "-".to_string() // Ground
            } else {
                // Current flow?
                let i = vm.current_grid[y][x];
                if i > 0.1 {
                    style = style.bg(Color::Rgb(
                        0,
                        (i * 10.0).clamp(0.0, 100.0) as u8,
                        (i * 20.0).clamp(0.0, 255.0) as u8,
                    ));
                }

                if v.abs() < 0.1 {
                    "·".to_string()
                } else if v.abs() < 10.0 {
                    "~".to_string()
                } else if v.abs() < 50.0 {
                    "≈".to_string()
                } else {
                    "⚡".to_string()
                }
            };

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Voltage Field"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let v = vm.voltage_grid[cy][cx];
    let c = vm.current_grid[cy][cx];
    let r = vm.resistance_grid[cy][cx];

    let r_status = if r == -1.0 {
        "BATTERY (Source)"
    } else if r == -2.0 {
        "GROUND (Sink)"
    } else {
        "Passive"
    };

    let info_text = vec![
        Line::from("ELEKTRA PROBE"),
        Line::from(" "),
        Line::from(format!("Voltage: {:.2} V", v)),
        Line::from(format!("Current: {:.2} A", c)),
        Line::from(format!("Node Type: {}", r_status)),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  battery(v, y, x) - Set Source"),
        Line::from("  ground(y, x) - Set Sink"),
        Line::from("  sense_volt(y, x) - Read Voltage"),
        Line::from("  shock(p, r) - Discharge"),
        Line::from(" "),
        Line::from("Physics:"),
        Line::from("  V propagates via Grid neighbors."),
        Line::from("  Values 1, 2, 3 (Silicon) act as Wires (Low R)."),
        Line::from("  Empty space acts as Air (High R)."),
    ];

    let info_widget = Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Circuit Analyzer"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_market(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(34),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Asks
    let mut ask_items = Vec::new();
    if vm.market.asks.is_empty() {
        ask_items.push(ListItem::new("No active asks."));
    } else {
        for order in &vm.market.asks {
            ask_items.push(ListItem::new(format!(
                "#{}: {} @ {} (Seller: {})",
                order.id, order.item, order.price, order.trader_id
            )));
        }
    }
    let asks_list = List::new(ask_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Order Book (Asks)"),
    );
    f.render_widget(asks_list, chunks[0]);

    // History
    let mut hist_items = Vec::new();
    if vm.market.history.is_empty() {
        hist_items.push(ListItem::new("No recent transactions."));
    } else {
        for (item, price) in &vm.market.history {
            hist_items.push(ListItem::new(format!("{} sold for {}", item, price)));
        }
    }
    let hist_list =
        List::new(hist_items).block(Block::default().borders(Borders::ALL).title("Ticker"));
    f.render_widget(hist_list, chunks[1]);

    // Wallets
    let mut wallet_items = Vec::new();
    for (id, balance) in vm.market.wallets.iter().enumerate() {
        if *balance > 0 {
            wallet_items.push(ListItem::new(format!("Strand {}: {}", id, balance)));
        }
    }
    if wallet_items.is_empty() {
        wallet_items.push(ListItem::new("No funds."));
    }
    let wallet_list = List::new(wallet_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Wealth Leaderboard"),
    );
    f.render_widget(wallet_list, chunks[2]);
}

#[cfg(feature = "nova")]
fn render_ballistics(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Projectile List
    let mut items = Vec::new();
    if vm.projectiles.is_empty() {
        items.push(ListItem::new("No active projectiles."));
    } else {
        for (i, p) in vm.projectiles.iter().enumerate() {
            items.push(ListItem::new(format!(
                "#{}: Pos({:.1}, {:.1}) Vel({:.1}, {:.1}) Pow:{} TTL:{}",
                i, p.x, p.y, p.vx, p.vy, p.power, p.ttl
            )));
        }
    }
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Active Trajectories"),
    );
    f.render_widget(list, chunks[0]);

    // Info
    let info = Paragraph::new("Visualizing ballistic objects.\n\nOpcodes:\n- fire(pow, dy, dx)\n- salvo(pow, count)\n- reflector(mode)\n- prism\n- lens(pow)")
        .block(Block::default().borders(Borders::ALL).title("Ballistics Control"));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_scent(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Scent List
    let mut items = Vec::new();
    if vm.pheromones.is_empty() {
        items.push(ListItem::new("No active scents."));
    } else {
        // Show top 20 strongest
        let mut sorted_scents: Vec<_> = vm.pheromones.iter().collect();
        sorted_scents.sort_by(|a, b| {
            b.intensity
                .partial_cmp(&a.intensity)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        for p in sorted_scents.iter().take(20) {
            items.push(ListItem::new(format!(
                "'{}': Pos({:.1}, {:.1}) Int:{:.2} Age:{}",
                p.signature, p.x, p.y, p.intensity, p.age
            )));
        }
        if vm.pheromones.len() > 20 {
            items.push(ListItem::new(format!(
                "... and {} more",
                vm.pheromones.len() - 20
            )));
        }
    }
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Olfactory Sensors"),
    );
    f.render_widget(list, chunks[0]);

    // Info
    let info = Paragraph::new("Visualizing airborne chemicals.\n\nOpcodes:\n- emit(int, sig)\n- smell() -> [dy, dx, int, sig]\n- track(sig) -> [dy, dx]\n\nMechanics:\n- Diffusion via Brownian motion\n- Wind influence\n- Decay over time")
        .block(Block::default().borders(Borders::ALL).title("Pheromone Analysis"));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_bestiary(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.area());

    // Organelle List
    let mut items = Vec::new();
    if vm.organelles.is_empty() {
        items.push(ListItem::new("No active organelles."));
    } else {
        for (i, org) in vm.organelles.iter().enumerate() {
            let style = if i == app_state.selected_organelle_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            items.push(ListItem::new(format!("{} [{:?}]", org.name, org.kind)).style(style));
        }
    }
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Bestiary (Active Agents)"),
    );
    f.render_widget(list, chunks[0]);

    // Details
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    if !vm.organelles.is_empty() && app_state.selected_organelle_index < vm.organelles.len() {
        let org = &vm.organelles[app_state.selected_organelle_index];

        // Face
        let face_lines = crate::vm::nova_bestiary::generate_face(org.genome_id, &org.traits);
        let mut face_text = Vec::new();
        for line in face_lines {
            face_text.push(Line::from(Span::styled(
                line,
                Style::default().fg(Color::Cyan),
            )));
        }
        let face_widget = Paragraph::new(face_text)
            .block(Block::default().borders(Borders::ALL).title("Portrait"));
        f.render_widget(face_widget, right_chunks[0]);

        // Stats
        let stats = vec![
            Line::from(format!("Name: {}", org.name)),
            Line::from(format!("Type: {:?}", org.kind)),
            Line::from(format!("Genome ID: {:x}", org.genome_id)),
            Line::from(format!("Traits: {:?}", org.traits)),
            Line::from(format!("Location: {:?}", org.context_loc)),
            Line::from(format!("Stack Depth: {}", org.stack.len())),
            Line::from(format!("IP: {:?}", org.ip)),
            Line::from(format!("Direction: {:?}", org.direction)),
        ];

        let stats_widget =
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title("Vitals"));
        f.render_widget(stats_widget, right_chunks[1]);
    } else {
        let info = Paragraph::new("Select an organelle to inspect.")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(info, chunks[1]);
    }
}

#[cfg(feature = "nova")]
fn render_kaleidoscope(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Left: Grid (Piet Canvas)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();

            // Background Color from Chroma
            let chroma = &vm.chroma_grid[y][x];
            if let Some((r, g, b)) = chroma.fg {
                style = style.bg(Color::Rgb(r, g, b));
            } else {
                style = style.bg(Color::White); // Default white canvas
            }

            // Cursor
            let mut ch = "  ".to_string();
            if app_state.grid_cursor == (x, y) {
                ch = "[]".to_string();
                style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
            }

            // Piet DP/CC if active
            if let Some(state) = &vm.piet_state {
                if state.y == y && state.x == x {
                    let arrow = match state.dp {
                        crate::vm::piet::Direction::Right => ">",
                        crate::vm::piet::Direction::Down => "v",
                        crate::vm::piet::Direction::Left => "<",
                        crate::vm::piet::Direction::Up => "^",
                    };
                    ch = format!(
                        "{}{}",
                        arrow,
                        if state.cc == crate::vm::piet::CodelChooser::Left {
                            "L"
                        } else {
                            "R"
                        }
                    );
                    style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
                }
            }

            line_spans.push(Span::styled(ch, style));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Kaleidoscope (Piet Canvas)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Palette & State
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    // Palette
    let mut palette_lines = Vec::new();
    let hues = ["Red", "Yellow", "Green", "Cyan", "Blue", "Magenta"];
    let lights = ["Light", "Normal", "Dark"];

    for (l_idx, light) in lights.iter().enumerate() {
        let mut spans = Vec::new();
        spans.push(Span::raw(format!("{:<8}", light)));

        for (h_idx, _hue) in hues.iter().enumerate() {
            let r = match h_idx {
                0 => 255,
                1 => 255,
                2 => 0,
                3 => 0,
                4 => 0,
                5 => 255,
                _ => 0,
            };
            let g = match h_idx {
                0 => 0,
                1 => 255,
                2 => 255,
                3 => 255,
                4 => 0,
                5 => 0,
                _ => 0,
            };
            let b = match h_idx {
                0 => 0,
                1 => 0,
                2 => 0,
                3 => 255,
                4 => 255,
                5 => 255,
                _ => 0,
            };

            let (r, g, b) = match l_idx {
                0 => (r + (255 - r) / 2, g + (255 - g) / 2, b + (255 - b) / 2),
                2 => (r / 2, g / 2, b / 2),
                _ => (r, g, b),
            };

            let mut style = Style::default().bg(Color::Rgb(r as u8, g as u8, b as u8));
            let mut text = "  ".to_string();

            if app_state.kaleidoscope_hue_idx == h_idx && app_state.kaleidoscope_light_idx == l_idx
            {
                style = style.fg(Color::Black).add_modifier(Modifier::BOLD);
                text = "XX".to_string();
            }

            spans.push(Span::styled(text, style));
            spans.push(Span::raw(" "));
        }
        palette_lines.push(Line::from(spans));
    }

    // Black & White
    let mut bw_spans = Vec::new();
    bw_spans.push(Span::raw("Special:  "));
    bw_spans.push(Span::styled("  ", Style::default().bg(Color::White))); // White
    bw_spans.push(Span::raw(" "));
    bw_spans.push(Span::styled("  ", Style::default().bg(Color::Black))); // Black
    palette_lines.push(Line::from(bw_spans));

    let palette_widget = Paragraph::new(palette_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Palette (Space: Paint, []: Hue, {}: Light)"),
    );
    f.render_widget(palette_widget, right_chunks[0]);

    // State Info
    let mut info_lines = Vec::new();
    info_lines.push(Line::from("Controls: S: Step, R: Reset, Arrows: Move"));

    if let Some(state) = &vm.piet_state {
        info_lines.push(Line::from(""));
        info_lines.push(Line::from(format!("Steps: {}", state.steps)));
        info_lines.push(Line::from(format!(
            "DP: {:?} | CC: {:?}",
            state.dp, state.cc
        )));
        info_lines.push(Line::from(format!("Pos: {},{}", state.x, state.y)));

        info_lines.push(Line::from(""));
        info_lines.push(Line::from("Stack (Top):"));
        for val in state.stack.iter().rev().take(10) {
            info_lines.push(Line::from(format!("  {}", val)));
        }
    } else {
        info_lines.push(Line::from(""));
        info_lines.push(Line::from("Piet Interpreter Inactive. Press 'S' to start."));
    }

    let info_widget = Paragraph::new(info_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Interpreter State"),
    );
    f.render_widget(info_widget, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_void(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Void Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let entropy = vm.entropy_grid[y][x];
            let mut style = Style::default();

            // Entropy visualization
            // 0-10: Space
            // 10-30: Light Shade
            // 30-60: Medium Shade
            // 60-80: Dark Shade
            // 80+: Full Block
            let ch = if entropy < 10 {
                " ".to_string()
            } else if entropy < 30 {
                "░".to_string()
            } else if entropy < 60 {
                "▒".to_string()
            } else if entropy < 80 {
                "▓".to_string()
            } else {
                "█".to_string()
            };

            // Color: Dark Gray to White to Red
            if entropy < 30 {
                style = style.fg(Color::DarkGray);
            } else if entropy < 60 {
                style = style.fg(Color::Gray);
            } else if entropy < 80 {
                style = style.fg(Color::White);
            } else {
                style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
            }

            // Overlay Void Organelles
            let mut is_void = false;
            let mut is_wisp = false;
            if let Some(org) = vm.organelles.iter().find(|o| o.context_loc == (y, x)) {
                if org.kind == crate::vm::nova::OrganelleType::Void {
                    is_void = true;
                } else if org.kind == crate::vm::nova::OrganelleType::Wisp {
                    is_wisp = true;
                }
            }

            if is_void {
                style = style
                    .bg(Color::Red)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
                line_spans.push(Span::styled("Ø", style));
            } else if is_wisp {
                style = style
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                line_spans.push(Span::styled("*", style));
            } else {
                // If cursor
                if app_state.grid_cursor == (x, y) {
                    style = style.bg(Color::White).fg(Color::Black);
                }
                line_spans.push(Span::styled(ch, style));
            }
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines)
        .block(Block::default().borders(Borders::ALL).title("Entropy Grid"));
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let local_entropy = vm.entropy_grid[cy][cx];

    let info_text = vec![
        Line::from("THE VOID"),
        Line::from(" "),
        Line::from(format!("Local Entropy: {} / 100", local_entropy)),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  - Entropy > 50 causes Reality Decay (Glitches)"),
        Line::from("  - Void Organelles (Ø) generate Entropy"),
        Line::from("  - stabilize(n) reduces Entropy"),
        Line::from("  - disintegrate(y, x) creates Entropy"),
    ];

    let info_widget =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "silicon")]
fn render_schematic(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Schematic Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let (ch, style) = match val {
                crate::vm::Value::Int(0) => (" ".to_string(), Style::default().fg(Color::DarkGray)),
                crate::vm::Value::Int(1) => ("┼".to_string(), Style::default().fg(Color::DarkGray)), // Wire
                crate::vm::Value::Int(2) => (
                    "⚡".to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ), // Head
                crate::vm::Value::Int(3) => (".".to_string(), Style::default().fg(Color::Red)), // Tail
                crate::vm::Value::Str(s) => {
                    if s.starts_with("G:") {
                        let parts: Vec<&str> = s.split(':').collect();
                        let sym = if parts.len() >= 2 {
                            match parts[1] {
                                "AND" => "&",
                                "OR" => "≥",
                                "XOR" => "=",
                                "NAND" => "!",
                                "NOT" => "¬",
                                _ => "?",
                            }
                        } else {
                            "G"
                        };
                        (
                            sym.to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("LATCH:") {
                        let state = s.trim_start_matches("LATCH:");
                        (
                            format!("L{}", state),
                            Style::default()
                                .fg(Color::Magenta)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("EMIT:") {
                        (
                            "E".to_string(),
                            Style::default()
                                .fg(Color::Green)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s.starts_with("RECV:") {
                        (
                            "R".to_string(),
                            Style::default()
                                .fg(Color::Blue)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s == "PIN:IN" {
                        (
                            "I".to_string(),
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else if s == "PIN:OUT" {
                        (
                            "O".to_string(),
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        )
                    } else {
                        ("?".to_string(), Style::default().fg(Color::White))
                    }
                }
                _ => ("?".to_string(), Style::default().fg(Color::White)),
            };

            let mut final_style = style;
            if app_state.grid_cursor == (x, y) {
                final_style = final_style.bg(Color::White).fg(Color::Black);
            }

            line_spans.push(Span::styled(ch, final_style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Silicon Schematic"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Help / Info
    let info_text = vec![
        Line::from("Silicon Mode Active."),
        Line::from("Components:"),
        Line::from("  ┼ (Wire)"),
        Line::from("  ⚡ (Signal)"),
        Line::from("  L0/L1 (Latch)"),
        Line::from("  &, ≥, =, !, ¬ (Gates)"),
        Line::from("  E (Emitter), R (Receiver)"),
        Line::from("  I/O (Pins)"),
        Line::from(""),
        Line::from("Opcodes:"),
        Line::from("  latch(state) - Place Latch"),
        Line::from("  dac - Read neighbors -> Stack"),
        Line::from("  adc - Stack -> Write neighbors"),
    ];

    let info = Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Schematic Info"),
    );
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_arena(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    // Top: Combatants
    let arena = match &vm.arena {
        Some(a) => a,
        None => return,
    };

    let combat_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    for (i, gladiator) in arena.combatants.iter().enumerate() {
        if i >= 2 {
            break;
        } // Only show first 2 for now

        let hp_percent =
            (gladiator.stats.hp as f64 / gladiator.stats.max_hp as f64).clamp(0.0, 1.0);

        let stats_text = vec![
            Line::from(vec![
                Span::styled(
                    format!("{} ", gladiator.name),
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .fg(Color::Yellow),
                ),
                Span::raw(format!(
                    "(HP: {}/{})",
                    gladiator.stats.hp, gladiator.stats.max_hp
                )),
            ]),
            Line::from(format!(
                "ATK: {} | DEF: {} | SPD: {}",
                gladiator.stats.attack, gladiator.stats.defense, gladiator.stats.speed
            )),
            Line::from(format!("Traits: {:?}", gladiator.traits)),
            Line::from(""),
            Line::from(format!(
                "Action: {}",
                if arena.turn > 0 {
                    "Fighting"
                } else {
                    "Waiting"
                }
            )),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!("Fighter {}", i + 1));
        let paragraph = Paragraph::new(stats_text).block(block);

        f.render_widget(paragraph, combat_chunks[i]);

        // HP Bar gauge?
        // Overlay gauge on top? No, paragraph supports text.
        // Let's render gauge below.

        let gauge_area = ratatui::layout::Rect {
            x: combat_chunks[i].x + 1,
            y: combat_chunks[i].y + 5,
            width: combat_chunks[i].width - 2,
            height: 1,
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(if hp_percent > 0.5 {
                Color::Green
            } else {
                Color::Red
            }))
            .ratio(hp_percent);

        f.render_widget(gauge, gauge_area);
    }

    if arena.combatants.is_empty() {
        let center = Paragraph::new("Press 'S' to Start (Auto-Draft)")
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(center, chunks[0]);
    }

    // Bottom: Logs
    let log_items: Vec<ListItem> = arena
        .logs
        .iter()
        .rev()
        .map(|s| ListItem::new(s.clone()))
        .collect();
    let logs_list = List::new(log_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Battle Log (Space: Tick, R: Reset)"),
    );
    f.render_widget(logs_list, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_garden(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    // Garden Grid (Rainbow CA)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let mut style = Style::default();
            let mut ch = "·".to_string();

            if let crate::vm::Value::Int(n) = val {
                if *n > 0 {
                    // Color based on Species ID
                    let colors = [
                        Color::Red,
                        Color::Green,
                        Color::Blue,
                        Color::Yellow,
                        Color::Magenta,
                        Color::Cyan,
                        Color::White,
                    ];
                    let bg = colors[(*n as usize) % colors.len()];
                    style = style.bg(bg).fg(Color::Black);
                    ch = format!("{}", n % 10);
                } else {
                    style = style.fg(Color::DarkGray);
                }
            } else {
                style = style.fg(Color::Gray);
                ch = "?".to_string();
            }

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines)
        .block(Block::default().borders(Borders::ALL).title("The Garden"));
    f.render_widget(grid_widget, chunks[0]);

    // Right: Rules List
    let mut rules_items = Vec::new();
    if vm.garden.rules.is_empty() {
        rules_items.push(ListItem::new("No species defined."));
    } else {
        let mut keys: Vec<_> = vm.garden.rules.keys().collect();
        keys.sort();
        for k in keys {
            if let Some(rule) = vm.garden.rules.get(k) {
                let r_str = format!("B{:?}/S{:?}", rule.birth, rule.survival);
                let colors = [
                    Color::Red,
                    Color::Green,
                    Color::Blue,
                    Color::Yellow,
                    Color::Magenta,
                    Color::Cyan,
                    Color::White,
                ];
                let color = colors[(*k as usize) % colors.len()];
                rules_items.push(
                    ListItem::new(format!("Species {}: {}", k, r_str))
                        .style(Style::default().fg(color)),
                );
            }
        }
    }

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let rules_list = List::new(rules_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Active Species"),
    );
    f.render_widget(rules_list, right_chunks[0]);

    // Info
    let info = vec![
        Line::from("Controls:"),
        Line::from("  Sow(rule, id) - Define Species"),
        Line::from("  Evolve - Step Simulation"),
        Line::from("  Harvest(r) - Save Pattern"),
        Line::from("  Genesis(id) - Metamorphic Reboot"),
        Line::from(" "),
        Line::from("Default: Species 1 (Life B3/S23)"),
    ];
    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Guide"));
    f.render_widget(info_widget, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_orca(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    // Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let signal = vm.signal_grid[y][x];
            let mut style = Style::default();

            let (ch, base_color) = match val {
                crate::vm::Value::Str(s) => {
                    let c = s.chars().next().unwrap_or('.');
                    let color = match c {
                        '*' | '!' => Color::Red,
                        ':' | ';' => Color::Magenta,
                        '0'..='9' => Color::Cyan,
                        'a'..='z' => Color::Green,
                        'A'..='Z' => Color::Yellow,
                        _ => Color::DarkGray,
                    };
                    (c.to_string(), color)
                }
                crate::vm::Value::Int(n) => {
                    let v = (*n).rem_euclid(36);
                    let c = if v < 10 {
                        ((v as u8) + b'0') as char
                    } else {
                        ((v as u8 - 10) + b'a') as char
                    };
                    let color = if *n == 0 {
                        Color::DarkGray
                    } else {
                        Color::Cyan
                    };
                    (c.to_string(), color)
                }
                _ => ("?".to_string(), Color::White),
            };

            style = style.fg(base_color);

            if signal > 0 {
                style = style
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_block = Block::default()
        .borders(Borders::ALL)
        .title("ORCA GRID (Signal Processing)");

    let grid_widget = Paragraph::new(grid_lines).block(grid_block);
    f.render_widget(grid_widget, chunks[0]);

    // Right Panel: Split into Manual and MIDI Log
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(chunks[1]);

    // Info Panel
    let info = vec![
        Line::from("ORCA MODE"),
        Line::from(" "),
        Line::from("Operators:"),
        Line::from("  * ! Bang (Signal Source)"),
        Line::from("  :   MIDI Note (N, V, C, D)"),
        Line::from("  ;   MIDI CC   (K, V, C)"),
        Line::from("  ?   Random"),
        Line::from("  N/S/E/W (Directional I/O)"),
        Line::from("  A/B/D (Math: + - /)"),
        Line::from("  M (Mutate), C (Clock)"),
        Line::from("  Q (Query), H (Harvest)"),
        Line::from(" "),
        Line::from("Controls:"),
        Line::from("  Type to place operators."),
        Line::from("  Space to Step."),
        Line::from("  Arrow Keys to Move."),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Manual"));
    f.render_widget(info_widget, right_chunks[0]);

    // MIDI Log
    let mut midi_items = Vec::new();
    if vm.midi_messages.is_empty() {
        midi_items
            .push(ListItem::new("No MIDI Output").style(Style::default().fg(Color::DarkGray)));
    } else {
        for msg in &vm.midi_messages {
            let s = match msg {
                crate::vm::MidiEvent::NoteOn {
                    channel,
                    note,
                    velocity,
                    duration,
                } => {
                    format!(
                        "♪ Ch{} Note{} Vel{} Len{}",
                        channel, note, velocity, duration
                    )
                }
                crate::vm::MidiEvent::ControlChange {
                    channel,
                    controller,
                    value,
                } => {
                    format!("≡ Ch{} CC{} Val{}", channel, controller, value)
                }
            };
            midi_items.push(ListItem::new(s).style(Style::default().fg(Color::Magenta)));
        }
    }

    let midi_list = List::new(midi_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("MIDI Output Log"),
    );
    f.render_widget(midi_list, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_strings(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Cosmic Strings (Vibrating Entities)"),
        )
        .x_bounds([0.0, 16.0])
        .y_bounds([0.0, 16.0])
        .paint(|ctx| {
            // Draw Strings
            for s in &vm.strings {
                let amp = s.amplitude * (s.phase.sin());

                // Draw as a sine wave segment? Or just a line perturbed by sine?
                // Line segment from start to end
                // We can subdivide it to show vibration
                let steps = 20;
                let dx = (s.end.0 - s.start.0) / steps as f64;
                let dy = (s.end.1 - s.start.1) / steps as f64;

                // Normal vector for vibration
                let len = ((s.end.0 - s.start.0).powi(2) + (s.end.1 - s.start.1).powi(2)).sqrt();
                let nx = -(s.end.1 - s.start.1) / len;
                let ny = (s.end.0 - s.start.0) / len;

                let mut prev_x = s.start.0;
                let mut prev_y = s.start.1;

                for i in 1..=steps {
                    let t = i as f64 / steps as f64;
                    let base_x = s.start.0 + (s.end.0 - s.start.0) * t;
                    let base_y = s.start.1 + (s.end.1 - s.start.1) * t;

                    // Standing wave: sin(n * pi * t)
                    let wave = (t * std::f64::consts::PI).sin() * amp * 0.5; // Scale amp for visual

                    let curr_x = base_x + nx * wave;
                    let curr_y = base_y + ny * wave;

                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: prev_x,
                        y1: prev_y,
                        x2: curr_x,
                        y2: curr_y,
                        color: if s.amplitude > 5.0 {
                            Color::Red
                        } else {
                            Color::Cyan
                        },
                    });

                    prev_x = curr_x;
                    prev_y = curr_y;
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // String List
    let mut items = Vec::new();
    if vm.strings.is_empty() {
        items.push(ListItem::new("No strings exist."));
    } else {
        for (i, s) in vm.strings.iter().enumerate() {
            items.push(ListItem::new(format!(
                "#{}: L={:.1} T={:.1} Amp={:.2} Freq={:.2}",
                i,
                ((s.end.0 - s.start.0).powi(2) + (s.end.1 - s.start.1).powi(2)).sqrt(),
                s.tension,
                s.amplitude,
                s.frequency
            )));
        }
    }

    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("String Stats"));
    f.render_widget(list, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_quipu(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Quipu (Knot Memory)"),
        )
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 50.0])
        .paint(|ctx| {
            // Draw Main Cord (Horizontal)
            ctx.draw(&ratatui::widgets::canvas::Line {
                x1: 5.0,
                y1: 45.0,
                x2: 95.0,
                y2: 45.0,
                color: Color::White,
            });

            // Draw Pendant Cords
            let cord_count = vm.quipu.cords.len();
            let spacing = 90.0 / (cord_count as f64 + 1.0);

            for (i, cord) in vm.quipu.cords.iter().enumerate() {
                let x = 5.0 + spacing * (i as f64 + 1.0);

                // Draw Cord Line
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: x,
                    y1: 45.0,
                    x2: x,
                    y2: 5.0,
                    color: if i == vm.quipu.active_cord {
                        Color::Yellow
                    } else {
                        Color::Gray
                    },
                });

                // Draw Knots
                // Top-down visually means y decreasing from 45.
                let mut current_y = 40.0;

                for cluster in &cord.clusters {
                    for knot in cluster {
                        let _color = match knot {
                            crate::vm::nova_quipu::Knot::Simple => Color::Cyan,
                            crate::vm::nova_quipu::Knot::Long(_) => Color::Green,
                            crate::vm::nova_quipu::Knot::FigureEight => Color::Red,
                        };

                        let symbol = match knot {
                            crate::vm::nova_quipu::Knot::Simple => "o",
                            crate::vm::nova_quipu::Knot::Long(_v) => "L",
                            crate::vm::nova_quipu::Knot::FigureEight => "8",
                        };

                        ctx.print(x - 0.5, current_y, symbol);
                        current_y -= 2.0;
                    }
                    // Gap between clusters
                    current_y -= 3.0;
                }

                // Draw Value at bottom
                let val = cord.read();
                ctx.print(x - 1.0, 2.0, val.to_string());
            }
        });

    f.render_widget(canvas, chunks[0]);

    let info = Paragraph::new("Quipu Interface.\nActive Cord highlighted Yellow.\nOpcodes: Knot, Unknot, Cord, ReadCord, Tangle")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_hydra(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    // Fluid Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let wind = vm.wind_grid[y][x];
            let moisture = vm.moisture_grid[y][x];

            let mut style = Style::default();

            // Background color based on moisture (Blue)
            if moisture > 0 {
                let intensity = (moisture / 4).clamp(0, 255) as u8;
                style = style.bg(Color::Rgb(0, 0, intensity));
                if intensity > 128 {
                    style = style.fg(Color::White);
                } else {
                    style = style.fg(Color::Cyan);
                }
            } else {
                style = style.fg(Color::DarkGray);
            }

            let mut ch = "·".to_string();

            // Overlay Components
            if let crate::vm::Value::Str(s) = val {
                if matches!(
                    s.as_str(),
                    ">" | "<" | "^" | "v" | "@" | "~" | "#" | "!" | "X"
                ) {
                    ch = s.clone();
                    style = style.add_modifier(Modifier::BOLD);
                    if s == "@" {
                        style = style.fg(Color::Green);
                    } // Pump
                    if s == "~" {
                        style = style.fg(Color::Red);
                    } // Drain
                    if s == "#" {
                        style = style.fg(Color::White).bg(Color::DarkGray);
                    } // Wall
                    if s == "!" {
                        style = style.fg(Color::Yellow);
                    } // Sensor
                }
            } else if wind != (0, 0) {
                // Show wind direction if no component overlay
                // Wind vector (dy, dx)
                if wind.0.abs() > wind.1.abs() {
                    if wind.0 > 0 {
                        ch = "↓".to_string();
                    } else {
                        ch = "↑".to_string();
                    }
                } else {
                    if wind.1 > 0 {
                        ch = "→".to_string();
                    } else {
                        ch = "←".to_string();
                    }
                }
            }

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Hydra (Fluid Dynamics)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let w = vm.wind_grid[cy][cx];
    let m = vm.moisture_grid[cy][cx];

    let info = vec![
        Line::from("HYDRA SYSTEM"),
        Line::from(" "),
        Line::from(format!("Pressure: {}", m)),
        Line::from(format!("Flow: ({}, {})", w.1, w.0)), // dx, dy
        Line::from(" "),
        Line::from("Components:"),
        Line::from("  @  Pump (Source)"),
        Line::from("  ~  Drain (Sink)"),
        Line::from("  #  Wall (Block)"),
        Line::from("  > < ^ v  Fan (Direct Flow)"),
        Line::from("  !  Sensor (Trigger if Pressure > 100)"),
        Line::from("  X  Valve (Default Closed)"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Fluid Gauge"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
fn render_chronos(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    // Left: Time Grid (Dilation Factors)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let factor = vm.time_grid[y][x];
            let mut style = Style::default();

            // Dilation visualization
            // 0: Stasis (Blue/Black)
            // 1: Normal (Gray)
            // >1: Accelerated (Yellow/Red)
            let ch = match factor {
                0 => "ZZ",
                1 => " .",
                _ => ">>",
            };

            style = match factor {
                0 => style.fg(Color::Blue).bg(Color::Black),
                1 => style.fg(Color::DarkGray),
                n if n < 5 => style.fg(Color::Yellow),
                _ => style.fg(Color::Red).add_modifier(Modifier::BOLD),
            };

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch.to_string(), style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Time Dilation Field"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: History / Echoes
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // History Info
    let history_len = vm.grid_history.len();
    let (cx, cy) = app_state.grid_cursor;

    let info = vec![
        Line::from(format!(
            "History Depth: {} / {}",
            history_len,
            crate::vm::MAX_HISTORY_DEPTH
        )),
        Line::from(format!(
            "Chronostasis Timer: {} ticks",
            vm.chronostasis_timer
        )),
        Line::from(" "),
        Line::from(format!("Cursor: {},{}", cx, cy)),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  TimeWarp(factor, radius)"),
        Line::from("  Chronostasis(ticks)"),
        Line::from("  Retrograde(ticks)"),
        Line::from("  Sporulate / Germinate"),
    ];

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Chronos Status"),
    );
    f.render_widget(info_widget, right_chunks[0]);

    // Cell History (Echoes)
    // Show the history of the selected cell
    let mut echoes = Vec::new();
    for (i, snapshot) in vm.grid_history.iter().rev().enumerate() {
        let val = &snapshot[cy][cx];
        echoes.push(ListItem::new(format!("-{}: {}", i + 1, val)));
    }

    if echoes.is_empty() {
        echoes.push(ListItem::new("No history recorded."));
    }

    let echo_list = List::new(echoes).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Echoes at {},{}", cx, cy)),
    );
    f.render_widget(echo_list, right_chunks[1]);
}

#[cfg(feature = "nova")]
fn render_logos(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(f.area());

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let mut style = Style::default();

            // Highlight active atoms
            let s = match val {
                crate::vm::Value::Str(s) => s.clone(),
                crate::vm::Value::Int(n) => n.to_string(),
                crate::vm::Value::Junction(_, _) => "J".to_string(),
                _ => ".".to_string(),
            };

            // Color based on type?
            if matches!(val, crate::vm::Value::Junction(_, _)) {
                style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD);
            } else if matches!(val, crate::vm::Value::Str(_)) {
                style = style.fg(Color::Cyan);
            } else if matches!(val, crate::vm::Value::Int(0)) {
                style = style.fg(Color::DarkGray);
            }

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            // Truncate to 3 chars for grid alignment
            let display = format!("{:^3.3}", s);
            line_spans.push(Span::styled(display, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let status = if vm.logos_mode { "ON" } else { "OFF" };
    let color = if vm.logos_mode {
        Color::Green
    } else {
        Color::Red
    };

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default().borders(Borders::ALL).title(Span::styled(
            format!("LOGOS GRID (Mode: {})", status),
            Style::default().fg(color),
        )),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Info
    let (cx, cy) = app_state.grid_cursor;
    let val = &vm.grid[cy][cx];

    let info = vec![
        Line::from(format!("Cell: {},{}", cx, cy)),
        Line::from(format!("Value: {}", val)),
        Line::from(" "),
        Line::from("Logos Rules:"),
        Line::from("  reaction(A, B, C)"),
        Line::from("  If A, B adjacent -> A=C, B=0"),
        Line::from(" "),
        Line::from("Controls:"),
        Line::from("  U: Toggle View"),
        Line::from("  OpCode::Logos to Toggle Mode"),
    ];

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Logic Chemistry"),
    );
    f.render_widget(info_widget, chunks[1]);
}

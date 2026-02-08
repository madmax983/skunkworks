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
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Row, Table},
    Frame, Terminal,
};
use std::io;

#[derive(Debug, PartialEq)]
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
}

impl AppState {
    pub(crate) fn new() -> Self {
        Self {
            view_mode: ViewMode::Genome,
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

            if let ViewMode::Heatmap = app_state.view_mode {
                render_heatmap(f, vm, app_state);
                return;
            }

            #[cfg(feature = "silicon")]
            if let ViewMode::Schematic = app_state.view_mode {
                render_schematic(f, vm, app_state);
                return;
            }

            render_genome_and_grid(f, vm, app_state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
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
                            }
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

                // Handle Normal Mode
                #[cfg(feature = "nova")]
                if let KeyCode::Char(c) = key.code {
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
                            ViewMode::Scent => ViewMode::Heatmap,
                            ViewMode::Heatmap => {
                                #[cfg(feature = "silicon")]
                                {
                                    ViewMode::Schematic
                                }
                                #[cfg(not(feature = "silicon"))]
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
                            #[cfg(feature = "silicon")]
                            ViewMode::Schematic => {
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
                    KeyCode::Char('s') => app_state.view_mode = ViewMode::Schematic,
                    #[cfg(feature = "nova")]
                    KeyCode::Char('p') => app_state.view_mode = ViewMode::PianoRoll,
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
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        #[cfg(feature = "nova")]
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
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
                    #[cfg(feature = "nova")]
                    KeyCode::Char('s') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            // Step Piet
                            if vm.piet_state.is_none() {
                                vm.piet_state = Some(crate::vm::piet::init_piet(vm));
                            }
                            if let Some(mut state) = vm.piet_state.take() {
                                crate::vm::piet::step_piet_once(vm, &mut state);
                                vm.piet_state = Some(state);
                            }
                        }
                    }
                    #[cfg(feature = "nova")]
                    KeyCode::Char('R') => {
                        if let ViewMode::Kaleidoscope = app_state.view_mode {
                            vm.piet_state = None;
                            app_state.status_msg = "Piet State Reset".to_string();
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
                    },
                    KeyCode::Right => match app_state.view_mode {
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
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
                    },
                    KeyCode::Left => match app_state.view_mode {
                        ViewMode::Genome => {}
                        ViewMode::Grid => {
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
                    },
                    KeyCode::Enter => {
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
                        }
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
        ViewMode::Heatmap => "HEATMAP",
        #[cfg(feature = "silicon")]
        ViewMode::Schematic => "SCHEMATIC",
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

                // Projectiles (Top Layer)
                for p in &vm.projectiles {
                    if (p.x as usize) == x && (p.y as usize) == y {
                        style = style.fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                        if p.vx.abs() > p.vy.abs() {
                            if p.vx > 0.0 { char_rep = "→".to_string(); } else { char_rep = "←".to_string(); }
                        } else {
                            if p.vy > 0.0 { char_rep = "↓".to_string(); } else { char_rep = "↑".to_string(); }
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
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

    // Right: Dialect (Shibboleths)
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

    let crucible_list = List::new(crucible_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Crucible (A: Add, X: Clear, T: Transmute)"),
    );
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
        let kb_items: Vec<ListItem> = vm
            .knowledge_base
            .iter()
            .take(20)
            .map(|fact| ListItem::new(format!("{}", fact)))
            .collect();
        let oracle_list = List::new(kb_items).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Oracle (Knowledge Base)"),
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

            if is_selected {
                style = style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED);
            }

            let icon = if trace.accepted { "✔" } else { "✖" };
            trace_items.push(
                ListItem::new(format!(
                    "{} Dream #{} (Strand {}) - {} Ticks",
                    icon, i, trace.strand_idx, trace.duration
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

        let info_text = vec![
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
            Line::from(""),
            Line::from(Span::styled(
                "Press ENTER to Realize (Lucid Dreaming)",
                Style::default().fg(Color::Cyan),
            )),
        ];

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

#[cfg(feature = "nova")]
fn render_market(f: &mut Frame, vm: &mut ChimeraVM, _app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(34)].as_ref())
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
    let asks_list = List::new(ask_items).block(Block::default().borders(Borders::ALL).title("Order Book (Asks)"));
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
    let hist_list = List::new(hist_items).block(Block::default().borders(Borders::ALL).title("Ticker"));
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
    let wallet_list = List::new(wallet_items).block(Block::default().borders(Borders::ALL).title("Wealth Leaderboard"));
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
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Active Trajectories"));
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
        sorted_scents.sort_by(|a, b| b.intensity.partial_cmp(&a.intensity).unwrap_or(std::cmp::Ordering::Equal));

        for p in sorted_scents.iter().take(20) {
            items.push(ListItem::new(format!(
                "'{}': Pos({:.1}, {:.1}) Int:{:.2} Age:{}",
                p.signature, p.x, p.y, p.intensity, p.age
            )));
        }
        if vm.pheromones.len() > 20 {
            items.push(ListItem::new(format!("... and {} more", vm.pheromones.len() - 20)));
        }
    }
    let list = List::new(items).block(Block::default().borders(Borders::ALL).title("Olfactory Sensors"));
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
            if let Some(org) = vm.organelles.iter().find(|o| o.context_loc == (y, x)) {
                if org.kind == crate::vm::nova::OrganelleType::Void {
                    is_void = true;
                }
            }

            if is_void {
                style = style
                    .bg(Color::Red)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
                line_spans.push(Span::styled("Ø", style));
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

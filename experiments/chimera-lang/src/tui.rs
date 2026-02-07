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
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Row, Table},
    Terminal,
};
#[cfg(feature = "nova")]
use ratatui::widgets::canvas::{Canvas, Rectangle};
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
    Heatmap,
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
            // Handle Microscope View
            if let ViewMode::Microscope = app_state.view_mode {
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
                    .constraints([
                        Constraint::Length(3), // Hormones
                        Constraint::Length(3), // Waste
                        Constraint::Length(3), // Mutagen
                        Constraint::Length(3), // Light
                    ].as_ref())
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
                let rows: Vec<Row> = data.organelles.iter().map(|org| {
                    Row::new(vec![
                        org.kind.clone(),
                        format!("{:?}", org.ip),
                        org.stack_depth.to_string(),
                    ])
                }).collect();

                let table = Table::new(rows, [
                    Constraint::Percentage(40),
                    Constraint::Percentage(30),
                    Constraint::Percentage(30),
                ])
                .header(Row::new(vec!["Type", "IP", "Stack"]))
                .block(Block::default().borders(Borders::ALL).title("Inhabitants"));

                f.render_widget(table, main_split[1]);

                return;
            }

            // Handle Cortex View
            #[cfg(feature = "biophysics")]
            if let ViewMode::Cortex = app_state.view_mode {
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

                return; // Skip normal rendering
            }

            // Handle Resonance View
            #[cfg(feature = "resonance")]
            if let ViewMode::Resonance = app_state.view_mode {
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
                             if val > 0.5 { Color::Cyan } else { Color::Blue }
                        } else if val < 0.0 {
                             if val < -0.5 { Color::Red } else { Color::Magenta }
                        } else {
                             Color::DarkGray
                        };

                        spans.push(Span::styled(ch, Style::default().fg(color)));
                        spans.push(Span::raw(" "));
                    }
                    lines.push(Line::from(spans));
                }

                let wave_grid = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Resonance Wave Function"));
                f.render_widget(wave_grid, chunks[0]);

                // Help / Status
                let help_text = "Physics Simulation Active.\nUse Pluck(str), Oscillate(freq, str), Hear() ops.\n\nLeft: Wavefront Visualization\nRight: (Reserved for Spectrum Analysis)";
                let help = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Cymatics"));
                f.render_widget(help, chunks[1]);

                return;
            }

            // Handle Grimoire View
            #[cfg(feature = "nova")]
            if let ViewMode::Grimoire = app_state.view_mode {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(20), Constraint::Percentage(20), Constraint::Percentage(40), Constraint::Percentage(20)].as_ref())
                    .split(f.area());

                // Ether (IPC)
                let ether_items: Vec<ListItem> = vm.ether.iter().map(|(ch, queue)| {
                    ListItem::new(format!("Channel {}: {} msgs", ch, queue.len()))
                }).collect();
                let ether_list = List::new(ether_items).block(Block::default().borders(Borders::ALL).title("Ether (IPC)"));
                f.render_widget(ether_list, chunks[0]);

                // Oracle (KB)
                #[cfg(feature = "oracle")]
                {
                    let kb_items: Vec<ListItem> = vm.knowledge_base.iter().take(20).map(|fact| {
                        ListItem::new(format!("{}", fact))
                    }).collect();
                    let oracle_list = List::new(kb_items).block(Block::default().borders(Borders::ALL).title("Oracle (Knowledge Base)"));
                    f.render_widget(oracle_list, chunks[1]);
                }
                #[cfg(not(feature = "oracle"))]
                {
                    let oracle_list = Paragraph::new("Oracle feature disabled").block(Block::default().borders(Borders::ALL).title("Oracle"));
                    f.render_widget(&oracle_list, chunks[1]);
                }

                // Sigil Registry (The Grimoire)
                #[cfg(feature = "nova")]
                {
                    let mut registry: Vec<_> = vm.sigil_registry.iter().collect();
                    registry.sort_by_key(|(k, _)| *k);

                    let sigil_items: Vec<ListItem> = registry.iter().enumerate().map(|(i, (name, sigil))| {
                        let status = if sigil.auto_cast { "[AUTO]" } else { "[    ]" };
                        let style = if i == app_state.selected_sigil_index {
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        ListItem::new(format!("{} {} ({} cells) -> Strand {}", status, name, sigil.pattern.len(), sigil.strand_idx)).style(style)
                    }).collect();

                    let sigil_list = List::new(sigil_items).block(Block::default().borders(Borders::ALL).title("The Grimoire (Select & Enter to Toggle Auto-Cast)"));
                    f.render_widget(sigil_list, chunks[2]);
                }

                // Bard (Score)
                let score_text: String = crate::vm::bard::score_to_abc(&vm.score);
                let bard_paragraph = Paragraph::new(score_text).block(Block::default().borders(Borders::ALL).title("Bard (Score)"));
                f.render_widget(bard_paragraph, chunks[3]);

                return;
            }

            // Handle Topology View
            #[cfg(feature = "nova")]
            if let ViewMode::Topology = app_state.view_mode {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
                    .split(f.area());

                let left_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(33), Constraint::Percentage(33), Constraint::Percentage(33)].as_ref())
                    .split(chunks[0]);

                // Portals
                let portal_items: Vec<ListItem> = vm.portals.iter().map(|(k, v)| {
                    ListItem::new(format!("Portal: ({},{}) -> ({},{})", k.1, k.0, v.1, v.0))
                }).collect();
                let portal_list = List::new(portal_items).block(Block::default().borders(Borders::ALL).title("Wormholes"));
                f.render_widget(portal_list, left_chunks[0]);

                // Entanglements
                let ent_items: Vec<ListItem> = vm.entangled_pairs.iter().map(|(k, v)| {
                    ListItem::new(format!("Entangled: Strand {} <-> {}", k, v))
                }).collect();
                let ent_list = List::new(ent_items).block(Block::default().borders(Borders::ALL).title("Spooky Action"));
                f.render_widget(ent_list, left_chunks[1]);

                // Mycelium
                let myc_items: Vec<ListItem> = vm.mycelium.iter().map(|(k, v)| {
                    let neighbors: Vec<String> = v.iter().map(|n| format!("({},{})", n.1, n.0)).collect();
                    ListItem::new(format!("Hyphae ({},{}): {:?}", k.1, k.0, neighbors))
                }).collect();
                let myc_list = List::new(myc_items).block(Block::default().borders(Borders::ALL).title("Fungal Network"));
                f.render_widget(myc_list, left_chunks[2]);

                // Topology Map
                let mut map_lines = Vec::new();
                for y in 0..16 {
                    let mut spans = Vec::new();
                    for x in 0..16 {
                        let mut ch = "·".to_string();
                        let mut style = Style::default().fg(Color::DarkGray);

                        if vm.mycelium.contains_key(&(y, x)) {
                            ch = "*".to_string();
                            style = style.fg(Color::Green);
                        }
                        if vm.portals.contains_key(&(y, x)) {
                            ch = "@".to_string();
                            style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD);
                        }
                        if vm.organelles.iter().any(|o| o.context_loc == (y, x)) {
                            ch = "O".to_string();
                            style = style.fg(Color::Yellow);
                        }

                        spans.push(Span::styled(ch, style));
                        spans.push(Span::raw(" "));
                    }
                    map_lines.push(Line::from(spans));
                }
                let map = Paragraph::new(map_lines).block(Block::default().borders(Borders::ALL).title("Topology Map"));
                f.render_widget(map, chunks[1]);

                return;
            }

            // Handle Laboratory View
            #[cfg(feature = "nova")]
            if let ViewMode::Laboratory = app_state.view_mode {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(30),
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                    ].as_ref())
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
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(Color::White)
                    };

                    List::new(items).block(Block::default().borders(Borders::ALL).title(format!("{} (Idx: {})", title, idx)).border_style(border_style))
                };

                // Parent A
                f.render_widget(render_strand(app_state.lab_parent_a, "Parent A", app_state.selected_strand == 0), chunks[0]);

                // Parent B
                f.render_widget(render_strand(app_state.lab_parent_b, "Parent B", app_state.selected_strand == 1), chunks[1]);

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
                    Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White)
                };

                let method_widget = Paragraph::new(method_name)
                    .block(Block::default().borders(Borders::ALL).title("Splice Method").border_style(method_border));
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
                        0 => { // Interleave
                            for i in 0..max_len {
                                if i < len_a { preview_items.push(ListItem::new(format!("{}", genes_a[i].op)).style(Style::default().fg(Color::Cyan))); }
                                if i < len_b { preview_items.push(ListItem::new(format!("{}", genes_b[i].op)).style(Style::default().fg(Color::Magenta))); }
                            }
                        }
                        1 => { // Uniform
                            preview_items.push(ListItem::new("Randomized Result").style(Style::default().fg(Color::DarkGray)));
                        }
                        2 => { // Midpoint
                            let mid_a = len_a / 2;
                            let mid_b = len_b / 2;
                            for gene in genes_a.iter().take(mid_a) { preview_items.push(ListItem::new(format!("{}", gene.op)).style(Style::default().fg(Color::Cyan))); }
                            for gene in genes_b.iter().skip(mid_b) { preview_items.push(ListItem::new(format!("{}", gene.op)).style(Style::default().fg(Color::Magenta))); }
                        }
                        _ => {}
                    }
                }

                let preview_list = List::new(preview_items)
                    .block(Block::default().borders(Borders::ALL).title("Child Preview (Enter to Splice)"));
                f.render_widget(preview_list, right_chunks[1]);

                return;
            }

            // Handle Graveyard View
            #[cfg(feature = "nova")]
            if let ViewMode::Graveyard = app_state.view_mode {
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
                            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        grave_items.push(ListItem::new(format!("Strand {} (Len: {})", i, strand.genes.len())).style(style));
                    }
                }
                let grave_list = List::new(grave_items).block(Block::default().borders(Borders::ALL).title("Graveyard (Necropolis)"));
                f.render_widget(grave_list, top_chunks[0]);

                // Strand Preview
                let mut gene_items = Vec::new();
                if !vm.graveyard.is_empty() && app_state.selected_graveyard_strand < vm.graveyard.len() {
                    let strand = &vm.graveyard[app_state.selected_graveyard_strand];
                    for gene in &strand.genes {
                        gene_items.push(ListItem::new(format!("{}", gene.op)).style(Style::default().fg(Color::Cyan)));
                    }
                } else if !vm.graveyard.is_empty() {
                     gene_items.push(ListItem::new("Invalid Selection"));
                } else {
                     gene_items.push(ListItem::new("No souls to display."));
                }
                let preview_list = List::new(gene_items).block(Block::default().borders(Borders::ALL).title("Genome of the Departed"));
                f.render_widget(preview_list, top_chunks[1]);

                // Help / Status
                let help_text = "Controls:\n↑/↓: Navigate\nR: Resurrect (Exhume to Helix)\nX: Exterminate (Permanent Deletion)\nTab: Switch View";
                let help_para = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Necromancy"));
                f.render_widget(help_para, chunks[1]);

                return;
            }

            // Handle Retina View
            #[cfg(feature = "nova")]
            if let ViewMode::Retina = app_state.view_mode {
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
                return;
            }

            // Handle Piano Roll View
            #[cfg(feature = "nova")]
            if let ViewMode::PianoRoll = app_state.view_mode {
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
                    .block(Block::default().borders(Borders::ALL).title("Piano Roll (MIDI Visualization)"))
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
                             if end > window_start && start < window_end {
                                 if note.pitch > 0 { // Not a rest
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
                        }
                    });

                f.render_widget(canvas, chunks[0]);

                let help = Paragraph::new("Visualizing MIDI Score.\nX-Axis: Time (16th notes)\nY-Axis: Pitch").block(Block::default().borders(Borders::ALL));
                f.render_widget(help, chunks[1]);
                return;
            }

            // Handle Heatmap View
            if let ViewMode::Heatmap = app_state.view_mode {
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
                         Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)
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
                         items.push(ListItem::new(Span::styled(content, Style::default().fg(color))));
                     }
                     items.push(ListItem::new(""));
                 }

                 let list = List::new(items).block(Block::default().borders(Borders::ALL).title(format!("Gene Expression Heatmap (Max: {})", max_count)));
                 f.render_widget(list, chunks[0]);

                 return;
            }

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
                         Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
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
                    if app_state.view_mode == ViewMode::Genome && s_idx == app_state.selected_strand && g_idx == app_state.selected_gene {
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
                ViewMode::Heatmap => "HEATMAP",
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
                 Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
            } else {
                 Style::default().fg(Color::DarkGray)
            };

            // Highlight the active block borders/title
            let genome_list = List::new(strand_items).block(
                genome_block.border_style(genome_style).title(title.clone()) // Show controls in main title usually
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
                        crate::vm::Value::Int(0) => {
                            (".".to_string(), Style::default().fg(Color::DarkGray))
                        }
                        crate::vm::Value::Int(n) => {
                            #[cfg(feature = "silicon")]
                            if vm.silicon_mode {
                                match n {
                                    1 => ("#".to_string(), Style::default().fg(Color::Yellow)), // Conductor
                                    2 => ("@".to_string(), Style::default().fg(Color::White).bg(Color::Cyan)), // Head
                                    3 => ("~".to_string(), Style::default().fg(Color::Red)), // Tail
                                    _ => (format!("{}", (n.abs() % 10)), Style::default().fg(Color::Green)),
                                }
                            } else {
                                (format!("{}", (n.abs() % 10)), Style::default().fg(Color::Green))
                            }
                            #[cfg(not(feature = "silicon"))]
                            (format!("{}", (n.abs() % 10)), Style::default().fg(Color::Green))
                        },
                        crate::vm::Value::Junction(_, _) => (
                            "J".to_string(),
                            Style::default().fg(Color::Yellow),
                        ),
                        crate::vm::Value::Superposition(_) => (
                            "Ψ".to_string(),
                            Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                        ),
                        crate::vm::Value::Str(s) => {
                            let symbol = if s.starts_with("G:") {
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
                                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                            } else {
                                match s.as_str() {
                                    "fire" => Style::default().fg(Color::Red),
                                    "water" => Style::default().fg(Color::Blue),
                                "earth" => Style::default().fg(Color::Yellow),
                                "air" => Style::default().fg(Color::Cyan),
                                "steam" => Style::default().fg(Color::White),
                                "lava" => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                                "cloud" => Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                                "spirit" => Style::default().fg(Color::Magenta),
                                "gold" => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                                    "lead" => Style::default().fg(Color::DarkGray),
                                    _ => Style::default().fg(Color::Cyan),
                                }
                            };
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

                        if let Some(organelle) =
                            vm.organelles.iter().find(|o| o.context_loc == (y, x))
                        {
                            let mut color = match organelle.kind {
                                crate::vm::nova::OrganelleType::Chloroplast => Color::Green,
                                crate::vm::nova::OrganelleType::Mitochondria => Color::Red,
                                crate::vm::nova::OrganelleType::Lysosome => Color::Magenta,
                                crate::vm::nova::OrganelleType::Ribosome => Color::Cyan,
                                crate::vm::nova::OrganelleType::Void => Color::DarkGray,
                                crate::vm::nova::OrganelleType::Alchemist => Color::Yellow,
                                crate::vm::nova::OrganelleType::Worker => Color::White,
                            };
                            let char_code = match organelle.kind {
                                crate::vm::nova::OrganelleType::Chloroplast => "C",
                                crate::vm::nova::OrganelleType::Mitochondria => "M",
                                crate::vm::nova::OrganelleType::Lysosome => "L",
                                crate::vm::nova::OrganelleType::Ribosome => "R",
                                crate::vm::nova::OrganelleType::Void => "Ø",
                                crate::vm::nova::OrganelleType::Alchemist => "A",
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
                 Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
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
                output_items.insert(0, ListItem::new(Span::styled(
                    format!("STATUS: {}", app_state.status_msg),
                    Style::default().fg(Color::Yellow)
                )));
            }
            if let InputMode::Editing = app_state.input_mode {
                 if app_state.view_mode == ViewMode::Grid {
                      output_items.insert(0, ListItem::new(Span::styled(
                           format!("EDIT GRID [{},{}]: {}", app_state.grid_cursor.0, app_state.grid_cursor.1, app_state.input_buffer),
                           Style::default().fg(Color::Cyan)
                      )));
                 }
            }

            let output_list = List::new(output_items)
                .block(Block::default().borders(Borders::ALL).title("Output"));
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

                let block = Block::default().borders(Borders::ALL).title("Viral Injection Vector (ChimeraScript)").style(Style::default().fg(Color::Green));
                let text = Paragraph::new(app_state.input_buffer.clone()).block(block).wrap(ratatui::widgets::Wrap { trim: true });
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
                    height: 3,
                };
                f.render_widget(ratatui::widgets::Clear, popup_area);

                let text = format!("SPIRIT SUMMONING: {}", app_state.input_buffer);
                let popup = Paragraph::new(text)
                    .block(Block::default().borders(Borders::ALL).title("Enter Value").style(Style::default().fg(Color::Cyan)));
                f.render_widget(popup, popup_area);
            }
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
                                ViewMode::Heatmap => {
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
                            ViewMode::Retina => ViewMode::Heatmap,
                            ViewMode::Heatmap => {
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
                    #[cfg(feature = "nova")]
                    KeyCode::Char('p') => app_state.view_mode = ViewMode::PianoRoll,
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
                        }
                    }
                    #[cfg(feature = "biophysics")]
                    KeyCode::Char('b') => app_state.view_mode = ViewMode::Cortex,
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => vm.step(),
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
                        ViewMode::Heatmap => {}
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
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Topology => {}
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
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            if app_state.selected_strand < 2 {
                                app_state.selected_strand += 1;
                            } else {
                                app_state.selected_strand = 0;
                            }
                        }
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
                        ViewMode::Heatmap => {}
                        #[cfg(feature = "nova")]
                        ViewMode::Laboratory => {
                            if app_state.selected_strand > 0 {
                                app_state.selected_strand -= 1;
                            } else {
                                app_state.selected_strand = 2;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        app_state.input_mode = InputMode::Editing;
                        match app_state.view_mode {
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
                                let mut registry: Vec<_> = vm.sigil_registry.keys().cloned().collect();
                                registry.sort();
                                if app_state.selected_sigil_index < registry.len() {
                                    let key = &registry[app_state.selected_sigil_index];
                                    if let Some(sigil) = vm.sigil_registry.get_mut(key) {
                                        sigil.auto_cast = !sigil.auto_cast;
                                        let status = if sigil.auto_cast { "ENABLED" } else { "DISABLED" };
                                        app_state.status_msg = format!("{} Auto-Cast: {}", key, status);
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
                            ViewMode::Heatmap => {
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

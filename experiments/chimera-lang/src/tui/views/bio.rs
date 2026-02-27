use crate::tui::panel_block;
use crate::tui::state::AppState;
use crate::tui::{layout_tree_node, InputMode};
use crate::vm::ChimeraVM;
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Row, Table},
    Frame,
};

pub(crate) fn render_microscope(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Scan Data
    let (cx, cy) = app_state.grid_cursor;
    let data = crate::vm::microscope::scan(vm, cy, cx);

    // Header
    let header = Paragraph::new(format!(
        "Microscope: Cell ({}, {}) - Value: {}",
        cx, cy, data.value
    ))
    .block(panel_block("Inspection", true));
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

#[cfg(feature = "biophysics")]
pub(crate) fn render_cortex(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    use ratatui::widgets::canvas::{Canvas, Line as CanvasLine};

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Neural Map (Canvas)
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Connectome"))
        .x_bounds([0.0, 16.0])
        .y_bounds([0.0, 16.0])
        .paint(|ctx| {
            // Draw Synapses
            for (source, targets) in &vm.biophysics_synapses {
                let sx = source.1 as f64 + 0.5;
                let sy = 16.0 - (source.0 as f64 + 0.5);

                for (target, _) in targets {
                    let tx = target.1 as f64 + 0.5;
                    let ty = 16.0 - (target.0 as f64 + 0.5);

                    ctx.draw(&CanvasLine {
                        x1: sx,
                        y1: sy,
                        x2: tx,
                        y2: ty,
                        color: Color::DarkGray,
                    });
                }
            }

            // Draw Neurons
            for (coord, neuron) in &vm.neurons {
                let x = coord.1 as f64 + 0.5;
                let y = 16.0 - (coord.0 as f64 + 0.5);

                let _color = if neuron.v > 0.0 {
                    Color::Yellow
                } else if neuron.v > -50.0 {
                    Color::Cyan
                } else {
                    Color::Blue
                };

                let symbol = if neuron.v > 0.0 { "*" } else { "O" };
                ctx.print(x, y, symbol);
            }

            // Draw selection cursor
            if let Some((y, x)) = app_state.selected_neuron_coords {
                let cx = x as f64 + 0.5;
                let cy = 16.0 - (y as f64 + 0.5);
                ctx.print(cx, cy, "+");
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Right: Details
    let right_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    if let Some(coord) = app_state.selected_neuron_coords {
        if let Some(neuron) = vm.neurons.get(&coord) {
            let details = vec![
                Line::from(format!("Neuron [{}, {}]", coord.1, coord.0)),
                Line::from(format!("V: {:.2} mV", neuron.v)),
                Line::from(format!("I_inj: {:.2}", neuron.i_inj)),
                Line::from(" "),
                Line::from(format!("Last Spike: {}", neuron.last_spike)),
            ];
            let info = Paragraph::new(details)
                .block(Block::default().borders(Borders::ALL).title("Biophysics"));
            f.render_widget(info, right_split[0]);

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
        } else {
            let info = Paragraph::new("Selected neuron died or missing.")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, right_split[0]);
        }
    } else {
        let info = Paragraph::new("Select a neuron to view details.")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(info, right_split[0]);
    }
}

#[cfg(feature = "nova")]
pub(crate) fn render_crispr(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Top: Target Strand
    let idx = app_state.crispr_target_strand;
    let mut strand_items = Vec::new();
    let title = if idx < vm.dna.helix.strands.len() {
        let strand = &vm.dna.helix.strands[idx];
        for gene in &strand.genes {
            strand_items.push(ListItem::new(format!("{}", gene.op)));
        }
        format!("Target Strand {} ({} genes)", idx, strand.genes.len())
    } else {
        strand_items.push(ListItem::new("Invalid Strand Index"));
        format!("Target Strand {} (Invalid)", idx)
    };

    let strand_border = if app_state.crispr_focus == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };

    let strand_list = List::new(strand_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(strand_border),
    );
    f.render_widget(strand_list, chunks[0]);

    // Bottom: Editor
    let editor_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ]
            .as_ref(),
        )
        .split(chunks[1]);

    // Guide RNA (Pattern)
    let guide_border = if app_state.crispr_focus == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let guide_input = Paragraph::new(app_state.crispr_guide.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Guide Pattern (e.g. 'Push Add')")
            .border_style(guide_border),
    );
    f.render_widget(guide_input, editor_chunks[0]);

    // Replacement (Payload)
    let replace_border = if app_state.crispr_focus == 2 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::White)
    };
    let replace_input = Paragraph::new(app_state.crispr_replace.clone()).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Payload (e.g. 'Push Sub')")
            .border_style(replace_border),
    );
    f.render_widget(replace_input, editor_chunks[1]);

    // Status / Controls
    let mut status_text = vec![
        Line::from(Span::styled(
            &app_state.crispr_result,
            Style::default().fg(Color::Cyan),
        )),
        Line::from(" "),
    ];

    if let InputMode::Editing = app_state.input_mode {
        status_text.extend(vec![
            Line::from(Span::styled(
                "EDITING MODE",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from("  Tab: Cycle Field (Guide <-> Replace)"),
            Line::from("  Type: Edit Text"),
            Line::from("  Enter: Execute Replace"),
            Line::from("  Esc: Exit Editing"),
        ]);
    } else {
        status_text.extend(vec![
            Line::from(Span::styled(
                "NORMAL MODE",
                Style::default().fg(Color::Green),
            )),
            Line::from("  Enter: Start Editing"),
            Line::from("  Up/Down: Change Strand"),
            Line::from("  Tab: Switch View"),
        ]);
    }

    let status_widget = Paragraph::new(status_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("CRISPR Status"),
    );
    f.render_widget(status_widget, editor_chunks[2]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_virology(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Viral Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();
            let mut ch = "·".to_string();

            if let Some(state) = &vm.viral_grid[y][x] {
                // Color based on virus
                if state.virus_id < vm.virus_library.len() {
                    let virus = &vm.virus_library[state.virus_id];
                    style = style.fg(Color::Rgb(virus.color.0, virus.color.1, virus.color.2));
                } else {
                    style = style.fg(Color::Red);
                }

                // Intensity based on infection level
                if state.infection_level > 80 {
                    style = style.add_modifier(Modifier::BOLD);
                    ch = "☣".to_string();
                } else if state.infection_level > 50 {
                    ch = "x".to_string();
                } else {
                    ch = ".".to_string();
                }
            } else {
                style = style.fg(Color::DarkGray);
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
            .title("Viral Grid (Infection Map)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right Panel: Split into Library (Top) and Designer (Bottom)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Top: Virus Library
    let mut items = Vec::new();
    if vm.virus_library.is_empty() {
        items.push(ListItem::new("No known viruses."));
    } else {
        for (i, v) in vm.virus_library.iter().enumerate() {
            let payload_desc = if let Some(pidx) = v.payload {
                format!("Payload: Strand {}", pidx)
            } else {
                "No Payload".to_string()
            };

            let content = format!(
                "ID {}: {} (Mut: {}%) [{}]\n  Pattern: '{}'",
                i, v.name, v.mutation_rate, payload_desc, v.pattern
            );

            let style = Style::default().fg(Color::Rgb(v.color.0, v.color.1, v.color.2));
            items.push(ListItem::new(content).style(style));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Virology Lab (Known Strains)"),
    );
    f.render_widget(list, right_chunks[0]);

    // Bottom: Virus Designer
    let designer_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3), // Name
                Constraint::Length(3), // Pattern
                Constraint::Length(3), // Rate
                Constraint::Length(3), // Payload
                Constraint::Length(3), // Mode
                Constraint::Min(0),    // Help
            ]
            .as_ref(),
        )
        .split(right_chunks[1]);

    let focused_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let default_style = Style::default().fg(Color::White);

    let mut draw_field = |title: &str, value: &str, focus_idx: u8, chunk_idx: usize| {
        let style = if app_state.virus_design_focus == focus_idx {
            focused_style
        } else {
            default_style
        };
        let widget = Paragraph::new(value)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(style),
            )
            .style(style);
        f.render_widget(widget, designer_chunks[chunk_idx]);
    };

    draw_field("Name (0)", &app_state.virus_design_name, 0, 0);
    draw_field("Target Pattern (1)", &app_state.virus_design_pattern, 1, 1);
    draw_field(
        "Mutation Rate % (2)",
        &app_state.virus_design_rate.to_string(),
        2,
        2,
    );
    draw_field(
        "Payload Strand ID (3)",
        &app_state.virus_design_payload.to_string(),
        3,
        3,
    );

    let mode_str = match app_state.virus_design_mode {
        0 => "Overwrite (Replace cell)",
        1 => "RewriteGrid (Mutate Grammar)",
        2 => "RewriteDNA (Mutate Organelle)",
        _ => "Unknown",
    };
    draw_field("Mode (4)", mode_str, 4, 4);

    let help_text = vec![
        Line::from("Controls:"),
        Line::from("  Tab: Next Field"),
        Line::from("  Enter: Edit / Confirm"),
        Line::from("  S: Synthesize (Save to Library)"),
        Line::from("  I: Inject Selected (From Library)"),
        Line::from("  Space: Outbreak (Step Sim)"),
    ];
    let help_widget = Paragraph::new(help_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Viral Engineering"),
    );
    f.render_widget(help_widget, designer_chunks[5]);
}

pub(crate) fn render_evolution(f: &mut Frame, _vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Top: Stats & Graph
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(chunks[0]);

    if let Some(engine) = &app_state.evolution_state.engine {
        // Stats
        let stats = vec![
            Line::from(Span::styled(
                "GENETIC OPTIMIZER",
                Style::default().add_modifier(Modifier::BOLD),
            )),
            Line::from(" "),
            Line::from(format!("Generation: {}", engine.generation)),
            Line::from(format!("Best Fitness: {}", engine.best_fitness)),
            Line::from(format!(
                "Challenge: {}",
                app_state.evolution_state.challenge
            )),
            Line::from(format!("Population: {}", engine.population.len())),
            Line::from(format!("Auto-Run: {}", app_state.evolution_state.auto_run)),
            Line::from(" "),
            Line::from("Controls:"),
            Line::from("  Space: Step Generation"),
            Line::from("  A: Toggle Auto-Run"),
            Line::from("  Enter: Set Target (Int)"),
            Line::from("  Tab: Cycle Challenge"),
        ];

        let stats_widget =
            Paragraph::new(stats).block(Block::default().borders(Borders::ALL).title("Status"));
        f.render_widget(stats_widget, top_chunks[0]);

        // Sparkline
        // Fitness usually drops. Sparkline shows bars. High bars = High fitness (bad).
        // We want to see it go down.
        // Limit history size
        let history: Vec<u64> = engine
            .history
            .iter()
            .rev()
            .take(100)
            .rev()
            .map(|&x| x.min(1000) as u64)
            .collect();

        let sparkline = ratatui::widgets::Sparkline::default()
            .block(
                Block::default()
                    .title("Fitness History (Lower is Better)")
                    .borders(Borders::ALL),
            )
            .data(&history)
            .style(Style::default().fg(Color::Green));
        f.render_widget(sparkline, top_chunks[1]);

        // Bottom: Code
        if !engine.population.is_empty() {
            let best = &engine.population[0];
            let mut gene_items = Vec::new();
            for gene in &best.genes {
                let args: Vec<String> = gene.args.iter().map(|a| format!("{:?}", a)).collect();
                let s = if args.is_empty() {
                    format!("{}", gene.op)
                } else {
                    format!("{}({})", gene.op, args.join(", "))
                };
                gene_items.push(ListItem::new(s).style(Style::default().fg(Color::Cyan)));
            }
            let list = List::new(gene_items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(format!("Best Specimen (Fitness: {})", engine.best_fitness)),
            );
            f.render_widget(list, chunks[1]);
        }
    } else {
        let area = app_state.get_render_area(f.area());
        let btn_width = 40;
        let btn_height = 3;
        let x = area.x + (area.width.saturating_sub(btn_width)) / 2;
        let y = area.y + (area.height.saturating_sub(btn_height)) / 2;
        let btn_area = ratatui::layout::Rect {
            x,
            y,
            width: btn_width,
            height: btn_height,
        };

        #[cfg(feature = "nova")]
        f.render_widget(
            tui_shared::Button::new("Initialize Evolution Engine (E)")
                .style_variant(tui_shared::ButtonStyle::Warning),
            btn_area,
        );

        #[cfg(not(feature = "nova"))]
        f.render_widget(
            Paragraph::new("Evolution Engine Offline (Enable 'nova' feature)")
                .alignment(ratatui::layout::Alignment::Center)
                .block(Block::default().borders(Borders::ALL)),
            btn_area,
        );
    }
}

#[cfg(feature = "nova")]
pub(crate) fn render_ecology(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Grid Visualization
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let mut style = Style::default();
            let mut ch = "·".to_string();

            // Background for Food
            if let crate::vm::Value::Int(n) = val {
                if *n > 0 {
                    style = style.fg(Color::Green);
                    ch = "*".to_string();
                } else if *n < 0 {
                    style = style.fg(Color::Magenta);
                    ch = "☢".to_string();
                } else {
                    style = style.fg(Color::DarkGray);
                }
            } else {
                style = style.fg(Color::DarkGray);
            }

            // Overlay Organelles
            for org in &vm.organelles {
                if org.context_loc == (y, x) {
                    // Color based on genome hash
                    let hash = org.genome_id;
                    let r = (hash & 0xFF) as u8;
                    let g = ((hash >> 8) & 0xFF) as u8;
                    let b = ((hash >> 16) & 0xFF) as u8;

                    style = style.fg(Color::Rgb(r, g, b)).add_modifier(Modifier::BOLD);
                    ch = "@".to_string();
                    break;
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
            .title("Ecology Grid (Genetic Sandbox)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let mut info = Vec::new();
    info.push(Line::from("GENETIC ECOLOGY"));
    info.push(Line::from(" "));
    info.push(Line::from(format!("Organisms: {}", vm.organelles.len())));
    info.push(Line::from(format!("Shared Energy: {}", vm.energy)));
    info.push(Line::from(" "));

    // Organelle Details under cursor
    let (cx, cy) = app_state.grid_cursor;
    let mut found = false;
    for org in &vm.organelles {
        if org.context_loc == (cy, cx) {
            info.push(Line::from(format!("Name: {}", org.name)));
            info.push(Line::from(format!("ID: {}", org.id)));
            info.push(Line::from(format!("Type: {:?}", org.kind)));
            info.push(Line::from(format!("Energy: {}", org.energy)));
            info.push(Line::from(format!("Traits: {:?}", org.traits)));
            found = true;
            break;
        }
    }

    if !found {
        info.push(Line::from("No organism at cursor."));
        if let crate::vm::Value::Int(n) = vm.grid[cy][cx] {
            if n > 0 {
                info.push(Line::from(format!("Food Energy: {}", n)));
            }
        }
    }

    info.push(Line::from(" "));
    info.push(Line::from("Controls:"));
    info.push(Line::from("  S: Spawn 10 Random"));
    info.push(Line::from("  f: Spawn Food"));
    info.push(Line::from("  I: Inject Code"));
    info.push(Line::from("  K: Extinction Event"));

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Status"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_metazoa(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Metazoa (Multicellular Life)"),
        )
        .x_bounds([0.0, 16.0])
        .y_bounds([0.0, 16.0])
        .paint(|ctx| {
            // Draw Tissues (Connections)
            for tissue in vm.tissues.values() {
                // Collect positions of members
                let mut points = Vec::new();
                for member_id in &tissue.members {
                    if let Some(org) = vm.organelles.iter().find(|o| o.id == *member_id) {
                        points.push((org.context_loc.1 as f64, org.context_loc.0 as f64));
                        // x, y
                    }
                }

                // Draw lines between adjacent members
                for i in 0..points.len() {
                    for j in (i + 1)..points.len() {
                        let (x1, y1) = points[i];
                        let (x2, y2) = points[j];
                        let dist = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
                        if dist < 1.5 {
                            // Adjacent (including diagonals)
                            ctx.draw(&ratatui::widgets::canvas::Line {
                                x1: x1 + 0.5,
                                y1: 15.5 - y1,
                                x2: x2 + 0.5,
                                y2: 15.5 - y2,
                                color: Color::Green,
                            });
                        }
                    }
                }
            }

            // Draw Organelles
            for org in &vm.organelles {
                let (y, x) = org.context_loc;
                let color = match org.kind {
                    crate::vm::nova::OrganelleType::Metazoan => Color::Yellow,
                    _ => Color::Cyan,
                };

                // Draw cursor if selected
                if app_state.grid_cursor == (x, y) {
                    ctx.print(x as f64 + 0.5, 15.5 - y as f64, "@");
                } else {
                    ctx.draw(&Rectangle {
                        x: x as f64 + 0.2,
                        y: 15.5 - y as f64 - 0.2,
                        width: 0.6,
                        height: 0.6,
                        color,
                    });
                }
            }

            // Draw Grid Cursor
            let (cx, cy) = app_state.grid_cursor;
            ctx.print(cx as f64 + 0.5, 15.5 - cy as f64, "+");
        });

    f.render_widget(canvas, chunks[0]);

    // Info Panel
    let mut info = Vec::new();
    info.push(Line::from("METAZOA INSPECTOR"));
    info.push(Line::from(" "));

    // Find organelle at cursor
    let (cx, cy) = app_state.grid_cursor;
    if let Some(org) = vm.organelles.iter().find(|o| o.context_loc == (cy, cx)) {
        info.push(Line::from(format!("Name: {}", org.name)));
        info.push(Line::from(format!("ID: {}", org.id)));
        if let Some(tid) = org.tissue_id {
            info.push(Line::from(format!("Tissue ID: {}", tid)));
        } else {
            info.push(Line::from("Tissue: None"));
        }
    } else {
        info.push(Line::from("No Agent Selected"));
    }

    info.push(Line::from(" "));
    info.push(Line::from("Controls:"));
    info.push(Line::from("  Space: Step"));
    info.push(Line::from("  Arrows: Move Cursor"));
    info.push(Line::from("  i: Inject Metazoan"));

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Details"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_phylogeny(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(app_state.get_render_area(f.area()));

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

pub(crate) fn render_mutagen(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Helix Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Mutagen Chamber"),
        )
        .x_bounds([0.0, 20.0])
        .y_bounds([0.0, 40.0])
        .paint(|ctx| {
            // Draw Helix
            if let Some(strand) = vm.dna.helix.strands.get(app_state.selected_strand) {
                for (i, gene) in strand.genes.iter().enumerate() {
                    let y = 38.0 - (i as f64 * 2.0); // Start from top
                    if y < 0.0 {
                        break;
                    }

                    let phase = (i as f64) * 0.5;
                    let x1 = 10.0 + 5.0 * phase.sin();
                    let x2 = 10.0 + 5.0 * (phase + std::f64::consts::PI).sin();

                    // Draw Strands
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1,
                        y1: y,
                        x2: x1,
                        y2: y - 2.0,
                        color: Color::Cyan,
                    });
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: x2,
                        y1: y,
                        x2: x2,
                        y2: y - 2.0,
                        color: Color::Magenta,
                    });

                    // Draw Base Pair (Rung)
                    let color = if i == app_state.selected_gene {
                        Color::Yellow
                    } else {
                        Color::Green
                    };
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1,
                        y1: y,
                        x2: x2,
                        y2: y,
                        color,
                    });

                    // Draw Label
                    if i == app_state.selected_gene {
                        ctx.print(x2 + 2.0, y, format!("<- {}", gene.op));
                    }
                }
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Info Panel
    let mut info = Vec::new();
    info.push(Line::from("MUTAGEN CONTROLS"));
    info.push(Line::from(" "));
    info.push(Line::from("Arrows: Navigate"));
    info.push(Line::from("M: Mutate Gene (Randomize)"));
    info.push(Line::from("Tab: Switch View"));

    if let Some(strand) = vm.dna.helix.strands.get(app_state.selected_strand) {
        info.push(Line::from(format!("Strand: {}", app_state.selected_strand)));
        if let Some(gene) = strand.genes.get(app_state.selected_gene) {
            info.push(Line::from(" "));
            info.push(Line::from(format!("Selected Gene: {}", gene.op)));
            info.push(Line::from(format!("Args: {:?}", gene.args)));
        }
    }

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Genetic Sequencer"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_biolum(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Biolum Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let intensity = vm.light_grid[y][x];
            let (r, g, b) = vm.light_color_grid[y][x];
            let mut style = Style::default();

            // Background color based on light
            // Scale intensity?
            let scale = (intensity as f64).clamp(0.0, 255.0) / 255.0;
            if intensity > 0 {
                let fr = (r as f64 * scale) as u8;
                let fg = (g as f64 * scale) as u8;
                let fb = (b as f64 * scale) as u8;
                style = style.bg(Color::Rgb(fr, fg, fb));

                // Contrast text
                if intensity > 128 {
                    style = style.fg(Color::Black);
                } else {
                    style = style.fg(Color::White);
                }
            } else {
                style = style.fg(Color::DarkGray);
            }

            let mut ch = "·".to_string();
            if intensity > 50 {
                ch = "*".to_string();
            }
            if intensity > 150 {
                ch = "☼".to_string();
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
            .title("Bioluminescence (Luciferin/Photophore)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info
    let (cx, cy) = app_state.grid_cursor;
    let intensity = vm.light_grid[cy][cx];
    let (r, g, b) = vm.light_color_grid[cy][cx];

    let info = vec![
        Line::from("BIOLUM SENSOR"),
        Line::from(" "),
        Line::from(format!("Pos: {},{}", cx, cy)),
        Line::from(format!("Intensity: {}", intensity)),
        Line::from(format!("Color: ({}, {}, {})", r, g, b)),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  Luciferin(r,g,b,int)"),
        Line::from("  Photophore(radius)"),
        Line::from("  Lumine(int, radius)"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Spectrometer"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_cambrian(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let hormone = vm.hormone_grid[y][x]; // [i64; 3]
                                                 // Normalize 0-1000 -> 0-255
            let r = (hormone[0] as f64 / 1000.0 * 255.0).clamp(0.0, 255.0) as u8;
            let g = (hormone[1] as f64 / 1000.0 * 255.0).clamp(0.0, 255.0) as u8;
            let b = (hormone[2] as f64 / 1000.0 * 255.0).clamp(0.0, 255.0) as u8;

            let mut style = Style::default().bg(Color::Rgb(r, g, b));

            // Contrast text color
            let brightness = (r as u16 + g as u16 + b as u16) / 3;
            if brightness > 128 {
                style = style.fg(Color::Black);
            } else {
                style = style.fg(Color::White);
            }

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let val = &vm.grid[y][x];
            let s = match val {
                crate::vm::Value::Str(s) => s.chars().next().unwrap_or(' ').to_string(),
                crate::vm::Value::Int(n) => n.to_string(),
                _ => "?".to_string(),
            };
            let display = format!("{:^3.3}", s);
            line_spans.push(Span::styled(display, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Cambrian Morphogens (RGB)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info
    let (cx, cy) = app_state.grid_cursor;
    let h = vm.hormone_grid[cy][cx];

    let info = vec![
        Line::from("MORPHOGEN GRADIENTS"),
        Line::from(" "),
        Line::from(format!("Pos: {},{}", cx, cy)),
        Line::from(format!("Ch A (Red):   {}", h[0])),
        Line::from(format!("Ch B (Green): {}", h[1])),
        Line::from(format!("Ch C (Blue):  {}", h[2])),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  Morphogen(ch, amt)"),
        Line::from("  HoxSwitch(ch, thresh, strand)"),
        Line::from("  Adhere(dir)"),
    ];
    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Development"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_garden(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
pub(crate) fn render_biomesh(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Grid with connections
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();
            let mut ch = "·".to_string();

            if let Some(node) = vm.biomesh.nodes.get(&(y, x)) {
                ch = format!("N{}", node.id % 10);
                style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);

                // If buffer has data, show differently
                if !node.buffer.is_empty() {
                    style = style.fg(Color::Yellow).add_modifier(Modifier::RAPID_BLINK);
                }
            } else {
                style = style.fg(Color::DarkGray);
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
            .title("BioMesh Topology"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Right: Info
    let (cx, cy) = app_state.grid_cursor;
    let mut info = Vec::new();
    info.push(Line::from(format!("Cursor: {},{}", cx, cy)));

    if let Some(node) = vm.biomesh.nodes.get(&(cy, cx)) {
        info.push(Line::from(format!("Node ID: {}", node.id)));
        info.push(Line::from(format!("Buffer Size: {}", node.buffer.len())));
        info.push(Line::from("Connections:"));
        for (ny, nx) in &node.connections {
            info.push(Line::from(format!("  -> {},{}", nx, ny)));
        }
        if !node.buffer.is_empty() {
            info.push(Line::from("Buffer Head:"));
            if let Some(val) = node.buffer.front() {
                info.push(Line::from(format!("  {}", val)));
            }
        }
    } else {
        info.push(Line::from("No Node at this location."));
    }

    info.push(Line::from(" "));
    info.push(Line::from("Opcodes:"));
    info.push(Line::from("  MeshNet(id)"));
    info.push(Line::from("  MeshGrow()"));
    info.push(Line::from("  MeshPrune()"));
    info.push(Line::from("  MeshSend(id, val)"));
    info.push(Line::from("  MeshRecv()"));

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Node Inspector"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_genesis(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
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
        .split(app_state.get_render_area(f.area()));

    // 1. Editor (ChimeraScript)
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .title("Genesis Editor (Code)")
        .border_style(if app_state.genesis_focus == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let editor_text = if app_state.genesis_editor_buffer.is_empty() {
        "Type ChimeraScript here..."
    } else {
        app_state.genesis_editor_buffer.as_str()
    };
    f.render_widget(
        Paragraph::new(editor_text)
            .block(editor_block)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        chunks[0],
    );

    // 2. Grammar Editor (Babel)
    let grammar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(chunks[1]);

    let grammar_block = Block::default()
        .borders(Borders::ALL)
        .title("Active Grammar (Perception)")
        .border_style(if app_state.genesis_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    let grammar_text = if app_state.genesis_grammar_buffer.is_empty() {
        format!("Current: {:?}", vm.active_grammar)
    } else {
        app_state.genesis_grammar_buffer.clone()
    };
    f.render_widget(
        Paragraph::new(grammar_text)
            .block(grammar_block)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        grammar_chunks[0],
    );

    // Controls Help
    let help_text = vec![
        Line::from("GENESIS CONSOLE"),
        Line::from("Tab: Switch Pane"),
        Line::from("Enter: Edit Pane"),
        Line::from("Ctrl+Enter: Execute/Compile"),
        Line::from(" "),
        Line::from("Ops:"),
        Line::from("  SelfRewrite(grammar)"),
        Line::from("  Perceive(len)"),
    ];
    let help_widget =
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Manual"));
    f.render_widget(help_widget, grammar_chunks[1]);

    // 3. Grid Visualizer (Right)
    let grid_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(chunks[2]);

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let mut style = Style::default();

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let s = match val {
                crate::vm::Value::Str(s) => s.chars().next().unwrap_or(' ').to_string(),
                crate::vm::Value::Int(n) => n.to_string(),
                _ => "?".to_string(),
            };

            line_spans.push(Span::styled(format!("{:^3.3}", s), style));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_block = Block::default()
        .borders(Borders::ALL)
        .title("Reality (Grid)")
        .border_style(if app_state.genesis_focus == 2 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        });

    f.render_widget(Paragraph::new(grid_lines).block(grid_block), grid_chunks[0]);

    // Status
    f.render_widget(
        Paragraph::new(app_state.status_msg.as_str()).block(Block::default().borders(Borders::ALL)),
        grid_chunks[1],
    );
}

#[cfg(feature = "nova")]
pub(crate) fn render_lifecycle(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Organelle List Grouped by Stage
    let mut stages: Vec<Vec<&crate::vm::nova::Organelle>> = vec![Vec::new(); 4];
    for org in &vm.organelles {
        let s = org.stage as usize;
        if s < stages.len() {
            stages[s].push(org);
        } else {
            // Fallback for higher stages
            if let Some(last) = stages.last_mut() {
                last.push(org);
            }
        }
    }

    let mut items = Vec::new();
    let stage_names = ["Larva", "Pupa", "Imago", "Titan"];

    for (i, list) in stages.iter().enumerate() {
        if !list.is_empty() {
            items.push(ListItem::new(Span::styled(
                format!(
                    "--- Stage {}: {} ---",
                    i,
                    stage_names.get(i).unwrap_or(&"Unknown")
                ),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            for org in list {
                let threshold = 100 * (org.stage as i64 + 1);
                let progress = (org.experience as f64 / threshold as f64).clamp(0.0, 1.0);
                let bar_len = 10;
                let filled = (progress * bar_len as f64) as usize;
                let bar = "=".repeat(filled) + &"-".repeat(bar_len - filled);

                items.push(ListItem::new(format!(
                    "{} [XP: {}/{}] [{}]",
                    org.name, org.experience, threshold, bar
                )));
            }
        }
    }

    if items.is_empty() {
        items.push(ListItem::new("No life detected."));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Life Cycle Analysis"),
    );
    f.render_widget(list, chunks[0]);

    // Right: Info
    let info_text = vec![
        Line::from("METAMORPHOSIS"),
        Line::from(" "),
        Line::from("Stages:"),
        Line::from("  0. Larva (Base form)"),
        Line::from("  1. Pupa (Cocoon, tough)"),
        Line::from("  2. Imago (Wings, specialized)"),
        Line::from("  3. Titan (Colossal, massive energy)"),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  Organelles gain Experience (XP) by eating."),
        Line::from("  Evolution is automatic when threshold reached."),
        Line::from("  Threshold = 100 * (Stage + 1)."),
    ];

    let info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::ALL).title("Encyclopedia"));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_memetics(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(main_chunks[0]);

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
    f.render_widget(meme_list, top_chunks[0]);

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
    f.render_widget(dialect_list, top_chunks[1]);

    // Bottom: Infection Map (Viral Grid)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();
            let mut ch = "·".to_string();

            let mut state_opt = None;
            if y < vm.viral_grid.len() && x < vm.viral_grid[y].len() {
                state_opt = vm.viral_grid[y][x].as_ref();
            }

            if let Some(state) = state_opt {
                // Color based on virus
                if state.virus_id < vm.virus_library.len() {
                    let virus = &vm.virus_library[state.virus_id];
                    style = style.fg(Color::Rgb(virus.color.0, virus.color.1, virus.color.2));
                } else {
                    style = style.fg(Color::Red);
                }

                // Intensity based on infection level
                if state.infection_level > 80 {
                    style = style.add_modifier(Modifier::BOLD);
                    ch = "☣".to_string();
                } else if state.infection_level > 50 {
                    ch = "x".to_string();
                } else {
                    ch = ".".to_string();
                }
            } else {
                style = style.fg(Color::DarkGray);
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
            .title("Viral Infection Map"),
    );
    f.render_widget(grid_widget, main_chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_scent(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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

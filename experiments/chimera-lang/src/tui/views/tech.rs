use crate::tui::state::AppState;
use crate::vm::ChimeraVM;
use ratatui::widgets::canvas::Canvas;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "silicon")]
pub(crate) fn render_foundry(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                        match s.as_str() {
                            "♨" => (
                                "♨".to_string(),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ),
                            "Ϡ" => (
                                "Ϡ".to_string(),
                                Style::default()
                                    .fg(Color::Yellow)
                                    .add_modifier(Modifier::BOLD),
                            ),
                            _ => ("?".to_string(), Style::default().fg(Color::White)),
                        }
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

#[cfg(feature = "nova")]
pub(crate) fn render_signals(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                let intensity = trail;
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
pub(crate) fn render_sovereignty(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Sovereignty Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let owner = vm.sovereignty_grid[y][x];
            let mut style = Style::default();
            let ch;

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
            #[cfg(feature = "nova")]
            {
                let biome = vm.biome_grid[y][x];
                let (br, bg, bb) = biome.color();
                if (br, bg, bb) != (0, 0, 0) && style.bg.is_none() {
                    style = style.bg(Color::Rgb(br, bg, bb));
                }
            }

            if app_state.grid_cursor == (x, y) {
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
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

#[cfg(feature = "nova")]
pub(crate) fn render_spectrogram(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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

pub(crate) fn render_heatmap(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let mut max_count = 1;
    for strand_counts in &vm.gene_execution_counts {
        for count in strand_counts {
            if *count > max_count {
                max_count = *count;
            }
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
            let count = vm
                .gene_execution_counts
                .get(s_idx)
                .and_then(|s| s.get(g_idx))
                .unwrap_or(&0);
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

#[cfg(feature = "nova")]
pub(crate) fn render_laboratory(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
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
        3 => "Frankenstein",
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
            3 => {
                // Frankenstein (Preview)
                // Just show interleaved chunks with sparks
                preview_items.push(
                    ListItem::new("Frankenstein Stitching...")
                        .style(Style::default().fg(Color::Red)),
                );
                preview_items
                    .push(ListItem::new("[ A Chunk ]").style(Style::default().fg(Color::Cyan)));
                preview_items.push(
                    ListItem::new("⚡ SPARK ⚡").style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
                preview_items
                    .push(ListItem::new("[ B Chunk ]").style(Style::default().fg(Color::Magenta)));
                preview_items.push(
                    ListItem::new("⚡ SPARK ⚡").style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                );
                preview_items
                    .push(ListItem::new("...").style(Style::default().fg(Color::DarkGray)));
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

#[cfg(feature = "silicon")]
pub(crate) fn render_schematic(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
pub(crate) fn render_reactor(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let flash = vm.reactor_flash[y][x];
            let mut style = Style::default();

            if flash > 0 {
                let intensity = flash;
                // Yellow flash
                style = style
                    .bg(Color::Rgb(intensity, intensity, 0))
                    .fg(Color::Black);
            } else {
                style = style.fg(Color::Cyan);
            }

            if app_state.grid_cursor == (x, y) {
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
            }

            let s = match val {
                crate::vm::Value::Int(n) => n.to_string(),
                crate::vm::Value::Str(s) => s.chars().next().unwrap_or(' ').to_string(),
                _ => "?".to_string(),
            };

            // Fixed width
            let display = format!("{:^3.3}", s);
            line_spans.push(Span::styled(display, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let status = if vm.reactor_mode { "ON" } else { "OFF" };
    let title = format!("REACTOR CHAMBER (Active: {})", status);

    let grid_widget =
        Paragraph::new(grid_lines).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(grid_widget, chunks[0]);

    // Info
    let mut info = Vec::new();
    info.push(Line::from("LOGIC AUTOMATA"));
    info.push(Line::from(" "));
    info.push(Line::from("Rules (KB):"));

    // Scan KB for reaction rules
    let mut rules_count = 0;
    #[cfg(feature = "oracle")]
    for fact in &vm.knowledge_base {
        // Simple check for reaction fact
        if let crate::vm::Value::Junction(_, args) = fact {
            if let Some(crate::vm::Value::Str(name)) = args.first() {
                if name == "reaction" {
                    if rules_count < 20 {
                        info.push(Line::from(format!("  {}", fact)));
                    }
                    rules_count += 1;
                }
            }
        }
    }

    if rules_count > 20 {
        info.push(Line::from(format!("  ... and {} more", rules_count - 20)));
    }

    info.push(Line::from(" "));
    info.push(Line::from("Controls:"));
    info.push(Line::from("  Reactor (OpCode) to toggle"));
    info.push(Line::from("  Reaction(A,B,C) to add rule"));

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Schematics"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_market(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
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
        .split(app_state.get_render_area(f.area()));

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

#[cfg(feature = "elektra")]
pub(crate) fn render_elektra(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Pre-calculate visual effects overlay
    let mut overlay = std::collections::HashMap::new();
    for effect in &vm.visual_effects {
        match effect {
            crate::vm::VisualEffect::Spark { loc, color, .. } => {
                overlay.insert(*loc, ('*', Color::Rgb(color.0, color.1, color.2)));
            }
            crate::vm::VisualEffect::Lightning {
                from, to, color, ..
            } => {
                // Bresenham's Line Algorithm
                let (mut x0, mut y0) = (from.1 as i64, from.0 as i64);
                let (x1, y1) = (to.1 as i64, to.0 as i64);
                let dx = (x1 - x0).abs();
                let dy = -(y1 - y0).abs();
                let sx = if x0 < x1 { 1 } else { -1 };
                let sy = if y0 < y1 { 1 } else { -1 };
                let mut err = dx + dy;

                loop {
                    if (0..16).contains(&x0) && (0..16).contains(&y0) {
                        overlay.insert(
                            (y0 as usize, x0 as usize),
                            ('⚡', Color::Rgb(color.0, color.1, color.2)),
                        );
                    }
                    if x0 == x1 && y0 == y1 {
                        break;
                    }
                    let e2 = 2 * err;
                    if e2 >= dy {
                        err += dy;
                        x0 += sx;
                    }
                    if e2 <= dx {
                        err += dx;
                        y0 += sy;
                    }
                }
            }
        }
    }

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
            let ch = if let Some((c, col)) = overlay.get(&(y, x)) {
                style = style
                    .fg(*col)
                    .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK);
                c.to_string()
            } else if r == -1.0 {
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
            } else if let crate::vm::Value::Str(s) = &vm.grid[y][x] {
                if s.starts_with("C:") {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                    "C".to_string()
                } else if s.starts_with("R:") {
                    style = style.fg(Color::Magenta).add_modifier(Modifier::BOLD);
                    "R".to_string()
                } else if s.starts_with("D:") {
                    style = style.fg(Color::White).add_modifier(Modifier::BOLD);
                    let dir = s.trim_start_matches("D:").parse::<i64>().unwrap_or(0);
                    match dir {
                        0 => "△",
                        1 => "▷",
                        2 => "▽",
                        3 => "◁",
                        _ => "D",
                    }
                    .to_string()
                } else if s.starts_with("T:") {
                    style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
                    "Y".to_string()
                } else if s.starts_with("M:") {
                    style = style.fg(Color::Red).add_modifier(Modifier::BOLD);
                    "M".to_string()
                } else if s.starts_with("S:") {
                    style = style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                    "?".to_string()
                } else {
                    // Fallback to voltage visualization for other strings
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
                }
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
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
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
        Line::from("  diode(dir, y, x) - One-way"),
        Line::from("  transistor(dir, y, x) - Switch"),
        Line::from("  muscle(thresh, y, x) - Actuator"),
        Line::from("  sensor(mode, y, x) - Source"),
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
pub(crate) fn render_quipu(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                let val = *cord;
                let s = val.abs().to_string();

                for c in s.chars() {
                    ctx.print(x - 0.5, current_y, c.to_string());
                    current_y -= 2.0;
                }

                // Draw Value at bottom
                ctx.print(x - 1.0, 2.0, val.to_string());
            }
        });

    f.render_widget(canvas, chunks[0]);

    let info = Paragraph::new("Quipu Interface.\nActive Cord highlighted Yellow.\nOpcodes: Knot, Unknot, Cord, ReadCord, Tangle")
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_hydra(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                } else if wind.1 > 0 {
                    ch = "→".to_string();
                } else {
                    ch = "←".to_string();
                }
            }

            if app_state.grid_cursor == (x, y) {
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
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
pub(crate) fn render_logos(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
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

pub(crate) fn render_catalyst(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Catalyst List
    let mut items = Vec::new();
    if vm.catalysts.is_empty() {
        items.push(ListItem::new("No Catalysts synthesized."));
        items.push(ListItem::new(""));
        items.push(ListItem::new("Use 'Synthesize(strand_idx)' to create one."));
    } else {
        for (i, cat) in vm.catalysts.iter().enumerate() {
            let style = if i == app_state.catalyst_scroll {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Cyan)
            };

            let status = if cat.charge > 0 { "ACTIVE" } else { "DEPLETED" };

            let content = format!(
                "#{}: ID {:x} | Charge: {} | Stability: {:.2} | [{}]",
                i, cat.id, cat.charge, cat.stability, status
            );

            items.push(ListItem::new(Span::styled(content, style)));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Catalyst Storage"),
    );
    f.render_widget(list, chunks[0]);

    // Recipe Detail
    let mut recipe_lines = Vec::new();
    if !vm.catalysts.is_empty() && app_state.catalyst_scroll < vm.catalysts.len() {
        let cat = &vm.catalysts[app_state.catalyst_scroll];
        recipe_lines.push(Line::from(vec![Span::styled(
            format!("Catalyst #{} Analysis", cat.id),
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        recipe_lines.push(Line::from(""));
        recipe_lines.push(Line::from("Recipe (Gene Pattern):"));

        for (i, op) in cat.recipe.iter().enumerate() {
            recipe_lines.push(Line::from(format!("  {}: {}", i, op)));
        }

        if cat.recipe.is_empty() {
            recipe_lines.push(Line::from("  (Empty Recipe)"));
        }
    } else {
        recipe_lines.push(Line::from("Select a Catalyst to view details."));
    }

    let detail = Paragraph::new(recipe_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Molecular Structure"),
    );
    f.render_widget(detail, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_orca(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let signal = vm.signal_grid[y][x];
            let mut style = Style::default();

            // Check for Organelle Overlay
            let mut overlay_char = None;
            let mut overlay_color = None;

            #[cfg(feature = "nova")]
            for org in &vm.organelles {
                if org.context_loc == (y, x) {
                    match org.kind {
                        crate::vm::nova::OrganelleType::Phage => {
                            overlay_char = Some("P".to_string());
                            overlay_color = Some(Color::Red);
                        }
                        crate::vm::nova::OrganelleType::Void => {
                            overlay_char = Some("Ø".to_string());
                            overlay_color = Some(Color::DarkGray);
                        }
                        _ => {}
                    }
                }
            }

            let (ch, base_color) = if let Some(c) = overlay_char {
                (c, overlay_color.unwrap_or(Color::White))
            } else {
                match val {
                    crate::vm::Value::Str(s) => {
                        let c = s.chars().next().unwrap_or('.');
                        let color = match c {
                            'E' => Color::Yellow,
                            'V' => Color::Blue,
                            '*' | '!' => Color::Red,
                            ':' | ';' => Color::Magenta,
                            '☿' | '♀' => Color::Red,
                            '$' | '&' => Color::Blue,
                            '0'..='9' => Color::Cyan,
                            'a'..='z' => Color::Green,
                            'A'..='Z' => Color::Yellow,
                            _ => Color::DarkGray,
                        };
                        (c.to_string(), color)
                    }
                    crate::vm::Value::Int(n) => {
                        let v = (n).rem_euclid(36);
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
                }
            };

            style = style.fg(base_color);

            if signal > 0 {
                style = style
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD);
            }

            if app_state.grid_cursor == (x, y) {
                style = style.fg(Color::Black).bg(Color::Yellow).add_modifier(Modifier::BOLD);
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
        Line::from("  Γ (Gamma) - Randomize East"),
        Line::from("  Σ (Sigma) - Sum Neighbors"),
        Line::from("  ☿ (Mercury) - Transmute Self"),
        Line::from("  ♀ (Venus) - Transmute Neighbors"),
        Line::from("  { - Inject OpCode (W->N:S)"),
        Line::from("  } - Extract OpCode (W:N->E)"),
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

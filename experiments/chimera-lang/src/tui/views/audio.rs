#[cfg(feature = "resonance")]
use crate::constants::GOLDEN_FREQUENCIES;
use crate::tui::state::AppState;
use crate::vm::ChimeraVM;
use ratatui::widgets::canvas::{Canvas, Rectangle};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph},
    Frame,
};

#[cfg(feature = "resonance")]
pub(crate) fn render_resonance(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Wave Grid
    let mut lines = Vec::new();
    for y in 0..16 {
        let mut spans = Vec::new();
        for x in 0..16 {
            let idx = y * 16 + x;
            let val = if idx < vm.audio_snapshot.pressure.len() {
                vm.audio_snapshot.pressure[idx]
            } else {
                0.0
            };

            // Check Harmonic
            #[cfg(feature = "nova")]
            let (freq, amp_res) = vm.resonance_grid[y][x];
            #[cfg(not(feature = "nova"))]
            let (freq, amp_res) = (0.0_f32, 0.0_f32);

            let is_harmonic = if amp_res > 10.0 {
                GOLDEN_FREQUENCIES.iter().any(|&g| (freq - g).abs() < 5.0)
            } else {
                false
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

            let color = if abs_val > 0.8 {
                Color::Red // Mutation / Shockwave
            } else if is_harmonic {
                Color::Yellow // Harmonic
            } else if val > 0.0 {
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

            let mut style = Style::default().fg(color);
            if abs_val > 0.8 {
                style = style.add_modifier(Modifier::RAPID_BLINK | Modifier::BOLD);
            }
            if is_harmonic {
                style = style.add_modifier(Modifier::BOLD);
            }

            spans.push(Span::styled(ch, style));
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
pub(crate) fn render_choir(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Active Sirens
    let mut items = Vec::new();
    let mut siren_count = 0;

    for agent in &vm.prologue_state.agents {
        if let Some(crate::vm::Value::Str(s)) = vm.grid.get(agent.y).and_then(|r| r.get(agent.x)) {
            if s == "♬" {
                siren_count += 1;
                // Parse state format: "♬:BPM:Oct:Vel:Wave:Dir:Buffer"
                let state_str = if let crate::vm::Value::Str(st) = &agent.state {
                    st.clone()
                } else {
                    String::new()
                };

                let parts: Vec<&str> = state_str.split(':').collect();
                let bpm = parts.get(1).unwrap_or(&"?");
                let oct = parts.get(2).unwrap_or(&"?");
                let buffer_str = parts.get(6).unwrap_or(&"");

                let mut notes = String::new();
                if !buffer_str.is_empty() {
                    for n_str in buffer_str.split(',') {
                        if let Ok(n) = n_str.parse::<u8>() {
                            let note_names = [
                                "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
                            ];
                            let name = note_names[(n as usize) % 12];
                            let octave = (n / 12) as i32 - 1;
                            notes.push_str(&format!("{}{}, ", name, octave));
                        }
                    }
                }

                items.push(
                    ListItem::new(format!(
                        "Siren @ {},{} | BPM:{} Oct:{} | Buf: [{}]",
                        agent.x,
                        agent.y,
                        bpm,
                        oct,
                        notes.trim_end_matches(", ")
                    ))
                    .style(Style::default().fg(Color::Cyan)),
                );
            }
        }
    }

    if items.is_empty() {
        items.push(ListItem::new("No Active Sirens. Place '♬' to begin."));
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Genetic Choir ({} Voices)", siren_count)),
    );
    f.render_widget(list, chunks[0]);

    // Right: Composition Log (Last created strands)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[1]);

    let mut dna_items = Vec::new();
    let total_strands = vm.dna.helix.strands.len();
    let start = total_strands.saturating_sub(10);

    for i in start..total_strands {
        let strand = &vm.dna.helix.strands[i];
        let mut genes_str = String::new();
        for (j, gene) in strand.genes.iter().enumerate() {
            if j > 5 {
                genes_str.push_str("...");
                break;
            }
            genes_str.push_str(&format!("{} ", gene.op));
        }
        dna_items.push(ListItem::new(format!("Strand #{}: {}", i, genes_str)));
    }

    let dna_list =
        List::new(dna_items).block(Block::default().borders(Borders::ALL).title("Composed DNA"));
    f.render_widget(dna_list, right_chunks[0]);
}

pub(crate) fn render_sequencer(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Top: Tracks
    let strand_count = vm.dna.helix.strands.len();
    // Show up to 4 strands
    let display_count = if strand_count == 0 {
        1
    } else {
        strand_count.min(4)
    };

    let track_constraints: Vec<Constraint> = (0..display_count)
        .map(|_| Constraint::Ratio(1, display_count as u32))
        .collect();

    let track_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(track_constraints)
        .split(chunks[0]);

    // Calculate time window based on scroll
    let window_start = app_state.sequencer_state.scroll_x;
    let window_width = chunks[0].width as usize - 4; // approximate
    let window_end = window_start + window_width;

    for i in 0..display_count {
        let s_idx = i;
        if s_idx < strand_count {
            let strand = &vm.dna.helix.strands[s_idx];

            // Render Track
            let title = format!("Track {} ({} Genes)", s_idx, strand.genes.len());

            // Use Canvas to draw genes as blocks
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(title))
                .x_bounds([window_start as f64, window_end as f64])
                .y_bounds([0.0, 10.0])
                .paint(|ctx| {
                    // Draw Playhead
                    let tick = app_state.sequencer_state.tick as f64;
                    if tick >= window_start as f64 && tick <= window_end as f64 {
                        ctx.draw(&ratatui::widgets::canvas::Line {
                            x1: tick,
                            y1: 0.0,
                            x2: tick,
                            y2: 10.0,
                            color: Color::Red,
                        });
                    }

                    // Draw Genes
                    for (g_idx, gene) in strand.genes.iter().enumerate() {
                        if g_idx >= window_start && g_idx <= window_end {
                            // Map OpCode to Color/Height
                            // Simple hash mapping
                            let h = (gene.op.to_string().len() % 8) + 2;
                            let color = match gene.op {
                                crate::opcode::OpCode::Push => Color::Blue,
                                crate::opcode::OpCode::Add | crate::opcode::OpCode::Sub => {
                                    Color::Green
                                }
                                crate::opcode::OpCode::Jump | crate::opcode::OpCode::Brz => {
                                    Color::Yellow
                                }
                                crate::opcode::OpCode::GRead | crate::opcode::OpCode::GWrite => {
                                    Color::Cyan
                                }
                                _ => Color::DarkGray,
                            };

                            let x = g_idx as f64;
                            ctx.draw(&Rectangle {
                                x,
                                y: 1.0,
                                width: 0.8,
                                height: h as f64,
                                color,
                            });
                        }
                    }
                });

            f.render_widget(canvas, track_chunks[i]);
        }
    }

    // Bottom: Controls & Status
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Info
    let status = if app_state.sequencer_state.playing {
        "PLAYING"
    } else {
        "PAUSED"
    };
    let status_color = if app_state.sequencer_state.playing {
        Color::Green
    } else {
        Color::Yellow
    };

    let info_text = vec![
        Line::from(Span::styled(
            "CHIMERA SEQUENCER",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(" "),
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(
                status,
                Style::default()
                    .fg(status_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(format!("BPM: {}", app_state.sequencer_state.bpm)),
        Line::from(format!("Tick: {}", app_state.sequencer_state.tick)),
    ];

    f.render_widget(
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Transport")),
        bottom_chunks[0],
    );

    // Help
    let help_text = vec![
        Line::from("Controls:"),
        Line::from("  Space: Play/Pause"),
        Line::from("  +/-: Adjust BPM"),
        Line::from("  Left/Right: Scrub / Scroll"),
        Line::from("  M: Mutate at Playhead"),
        Line::from("  S: Scramble Track"),
    ];

    f.render_widget(
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL).title("Manual")),
        bottom_chunks[1],
    );
}

#[cfg(feature = "nova")]
pub(crate) fn render_narrative(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Library List
    let mut items = Vec::new();
    if vm.prologue_state.library.is_empty() {
        items.push(ListItem::new("Library is empty.").style(Style::default().fg(Color::DarkGray)));
    } else {
        for (key, val) in &vm.prologue_state.library {
            items.push(
                ListItem::new(format!("{}: {}", key, val)).style(Style::default().fg(Color::Cyan)),
            );
        }
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Prologue Library (The Book 📖)"),
    );
    f.render_widget(list, chunks[0]);

    // Info
    let info = vec![
        Line::from("NARRATIVE ENGINE"),
        Line::from(" "),
        Line::from("Runes:"),
        Line::from("  α (Alpha) - Incipit (Seed -> Theme)"),
        Line::from("  ω (Omega) - Terminus (Story -> Outcome)"),
        Line::from("  ✍ (Hand) - Revision (Edit)"),
        Line::from("  ? (Twist) - Plot Twist"),
        Line::from("  📖 (Book) - Library Read/Write"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Legend"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_lexicon(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let mut style = Style::default();
            let val = &vm.grid[y][x];
            let s = match val {
                crate::vm::Value::Str(s) => s.chars().next().unwrap_or(' ').to_string(),
                crate::vm::Value::Int(n) => n.to_string(),
                _ => ".".to_string(),
            };

            // Colorize letters
            if let crate::vm::Value::Str(_) = val {
                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            } else if let crate::vm::Value::Int(n) = val {
                if *n != 0 {
                    style = style.fg(Color::Cyan);
                } else {
                    style = style.fg(Color::DarkGray);
                }
            }

            if app_state.grid_cursor == (x, y) {
                style = style
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }

            line_spans.push(Span::styled(format!("{:^3.3}", s), style));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Lexicon (Spell Casting)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info
    let info = vec![
        Line::from("LEXICON GRIMOIRE"),
        Line::from(" "),
        Line::from("Cast Spells by writing words:"),
        Line::from("  FIRE, HEAL, VOID, LIFE"),
        Line::from("  DIE, WARP, TIME, SOUL"),
        Line::from("  CHAOS, ORDER, GROW, HUNT"),
        Line::from("  SEEK, SCAN, ECHO, BOMB"),
        Line::from("  NULL, LOVE, HATE"),
        Line::from(" "),
        Line::from("Mechanics:"),
        Line::from("  Spells consume letters (turn to 0)."),
        Line::from("  Can be Horizontal or Vertical."),
        Line::from("  Overlapping spells are possible."),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Known Spells"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_babel(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // 1. Grammar AST (Top Left)
    let ast_text = if let Some(ast) = &app_state.babel_ast {
        format!("{}", ast) // Value::fmt handles pretty printing somewhat
    } else {
        "No Grammar Loaded (Press 'R' to seed)".to_string()
    };

    let ast_widget = Paragraph::new(ast_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Grammar AST")
                .style(Style::default().fg(Color::Magenta)),
        )
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(ast_widget, top_chunks[0]);

    // 2. Grid Trace (Top Right)
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = &vm.grid[y][x];
            let mut style = Style::default();

            // Check trace
            if vm.babel_live_trace.contains(&(y, x)) {
                style = style
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD);
            } else if matches!(val, crate::vm::Value::Str(_)) {
                style = style.fg(Color::Cyan);
            } else {
                style = style.fg(Color::DarkGray);
            }

            if app_state.grid_cursor == (x, y) {
                style = style
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD);
            }

            let ch = match val {
                crate::vm::Value::Str(s) => s.chars().next().unwrap_or('.').to_string(),
                crate::vm::Value::Int(n) => {
                    if *n == 0 {
                        ".".to_string()
                    } else {
                        "#".to_string()
                    }
                }
                _ => ".".to_string(),
            };
            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Babel Grid (Live Trace)")
            .style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(grid_widget, top_chunks[1]);

    // 3. Parser Input (Bottom Left)
    let input_lines = vec![
        Line::from(vec![
            Span::raw("Pattern (Regex): "),
            Span::styled(
                &app_state.babel_pattern,
                if app_state.babel_focus == 0 {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            ),
        ]),
        Line::from(vec![
            Span::raw("Input String:    "),
            Span::styled(
                &app_state.babel_input,
                if app_state.babel_focus == 1 {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                },
            ),
        ]),
        Line::from(" "),
        Line::from(Span::styled(
            "Result: (Press Space to Parse)",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(Span::raw(format!("Tablet Size: {}", vm.tablet.len()))),
    ];
    let input_widget = Paragraph::new(input_lines)
        .block(Block::default().borders(Borders::ALL).title("Parser Test"));
    f.render_widget(input_widget, bottom_chunks[0]);

    // 4. Babel Chaos & Controls (Bottom Right)
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(bottom_chunks[1]);

    let integrity = vm.babel_state.integrity;
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Linguistic Integrity"),
        )
        .gauge_style(Style::default().fg(if integrity > 0.8 {
            Color::Green
        } else if integrity > 0.4 {
            Color::Yellow
        } else {
            Color::Red
        }))
        .ratio(integrity);
    f.render_widget(gauge, right_chunks[0]);

    let lower_right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(right_chunks[1]);

    let mut map_items = Vec::new();
    if vm.babel_state.chaos_map.is_empty() {
        map_items.push(ListItem::new("No active confusions."));
    } else {
        for (k, v) in &vm.babel_state.chaos_map {
            map_items.push(
                ListItem::new(format!("{} -> {}", k, v)).style(Style::default().fg(Color::Magenta)),
            );
        }
    }
    let map_list = List::new(map_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Chaos Mappings"),
    );
    f.render_widget(map_list, lower_right[0]);

    let controls = vec![
        Line::from("Controls:"),
        Line::from("  M: Mutate Grammar (Evolve)"),
        Line::from("  G: Generate Sample"),
        Line::from("  R: Reset/Seed Grammar"),
        Line::from("  Space: Run Parser (Regex)"),
        Line::from("  Tab: Switch Focus (Pattern/Input)"),
    ];
    let control_widget = Paragraph::new(controls)
        .block(Block::default().borders(Borders::ALL).title("Lab Controls"));
    f.render_widget(control_widget, lower_right[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_piano_roll(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(app_state.get_render_area(f.area()));

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

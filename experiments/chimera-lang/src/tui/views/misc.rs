use crate::tui::get_all_views;
use crate::tui::panel_block;
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
#[cfg(feature = "nova")]
use tui_shared::{Bobber, Button, TensionBar};

#[cfg(feature = "nova")]
pub(crate) fn render_fishing(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(0),
                Constraint::Length(3), // Controls
                Constraint::Length(3), // Info
            ]
            .as_ref(),
        )
        .split(app_state.get_render_area(f.area()));

    let mut block = panel_block("Fishing Minigame", app_state.fishing_hooked);

    // Flash background if tension is critical
    if app_state.fishing_tension > 0.9 && vm.tick_counter % 4 < 2 {
        block = block.style(Style::default().bg(Color::Red));
    }

    let scene_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(6)])
        .split(chunks[0]);

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
            // Deep Sky
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 80.0,
                width: 100.0,
                height: 20.0,
                color: Color::Blue,
            });

            // Water Gradient
            // Surface
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 40.0,
                width: 100.0,
                height: 10.0,
                color: Color::LightBlue,
            });
            // Mid
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 20.0,
                width: 100.0,
                height: 20.0,
                color: Color::Blue,
            });
            // Deep
            ctx.draw(&Rectangle {
                x: 0.0,
                y: 0.0,
                width: 100.0,
                height: 20.0,
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
                // Bobber X Animation (Shake when hooked)
                let mut bobber_x = 50.0;
                if app_state.fishing_hooked {
                    let shake_mag = if app_state.fishing_tension > 0.5 {
                        2.0
                    } else {
                        0.8
                    };
                    // Simple pseudo-random shake using tick
                    let offset = ((vm.tick_counter % 3) as f64 - 1.0) * shake_mag;
                    bobber_x += offset;
                }

                // Fishing Line (Rod Bending Animation)
                let line_color = if app_state.fishing_tension > 0.8 {
                    Color::Red
                } else if app_state.fishing_tension > 0.5 {
                    Color::Yellow
                } else {
                    Color::White
                };

                // Bob animation
                let bob_offset = if !app_state.fishing_hooked {
                    (vm.tick_counter as f64 * 0.2).sin() * 2.0
                } else {
                    0.0
                };

                // Rod tip dips when tension is high
                let rod_tip_y = 100.0 - (app_state.fishing_tension * 5.0);

                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: 50.0,
                    y1: rod_tip_y,
                    x2: bobber_x,
                    y2: app_state.fishing_bobber_y + bob_offset,
                    color: line_color,
                });

                // Draw Bobber using Component with updated signature
                let bobber = Bobber::new(
                    bobber_x,
                    app_state.fishing_bobber_y + bob_offset,
                    app_state.fishing_hooked,
                );
                bobber.draw(ctx, vm.tick_counter);

                // Fish (Icon)
                if app_state.fishing_fish_y > 0.0 && app_state.fishing_fish_y < 100.0 {
                    let fish_icon = if app_state.fishing_tension > 0.8 {
                        "🦈"
                    } else if app_state.fishing_tension > 0.5 {
                        "🐠"
                    } else {
                        "🐟"
                    };
                    // Fish tries to align with bobber X somewhat, or fights away?
                    let fish_x = 49.0
                        + (app_state.fishing_tension * 10.0 * ((vm.tick_counter % 5) as f64 - 2.0));
                    ctx.print(fish_x, app_state.fishing_fish_y, fish_icon);
                }

                // Catch Zone Indicator
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: 45.0,
                    y1: 90.0,
                    x2: 55.0,
                    y2: 90.0,
                    color: Color::Green,
                });
            }
        });

    f.render_widget(canvas, scene_chunks[0]);
    f.render_widget(TensionBar::new(app_state.fishing_tension), scene_chunks[1]);

    // Success Check: Overlay "FISH ON!" if hooked
    if app_state.fishing_hooked {
        let area = scene_chunks[0];
        let popup_area = ratatui::layout::Rect {
            x: area.x + (area.width.saturating_sub(20)) / 2,
            y: area.y + (area.height.saturating_sub(3)) / 2,
            width: 20,
            height: 3,
        };
        // Clear background for popup
        f.render_widget(ratatui::widgets::Clear, popup_area);

        let popup = Paragraph::new("FISH ON!")
            .style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Red)
                    .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
            )
            .alignment(ratatui::layout::Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            );

        f.render_widget(popup, popup_area);
    }

    // Controls Row (Buttons)
    let control_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    f.render_widget(
        Button::new("CAST (Enter)").active(!app_state.fishing_cast),
        control_chunks[0],
    );

    f.render_widget(
        Button::new("REEL (Space)")
            .active(app_state.fishing_cast && app_state.fishing_tension > 0.0), // Highlight when reeling
        control_chunks[1],
    );

    // Dashboard Info
    let depth = if app_state.fishing_bobber_y < 50.0 {
        format!("Depth: {:.1}m", 50.0 - app_state.fishing_bobber_y)
    } else {
        format!("Air: {:.1}m", app_state.fishing_bobber_y - 50.0)
    };

    let status = if app_state.fishing_hooked {
        Span::styled(
            " FISH ON! ",
            Style::default()
                .bg(Color::Red)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD | Modifier::RAPID_BLINK),
        )
    } else if app_state.fishing_cast {
        Span::styled("Waiting...", Style::default().fg(Color::Cyan))
    } else {
        Span::styled("Ready", Style::default().fg(Color::Green))
    };

    let info_text = vec![
        Line::from(vec![
            Span::raw("Status: "),
            status,
            Span::raw(" | "),
            Span::raw(depth),
        ]),
        Line::from("Controls: Space (Cast/Reel)"),
    ];

    let info =
        Paragraph::new(info_text).block(Block::default().borders(Borders::ALL).title("Tackle Box"));
    f.render_widget(info, chunks[2]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_fractal(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    // 1. Compute
    crate::vm::nova_fractal::compute_fractal(vm);

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // 2. Render Grid
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let cell = &vm.chroma_grid[y][x];
            let mut style = Style::default();

            if let Some((r, g, b)) = cell.fg {
                style = style.fg(Color::Rgb(r, g, b));
            }

            let ch = cell.char.unwrap_or(' ').to_string();

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            line_spans.push(Span::styled(ch, style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let mode_str = match vm.fractal.mode {
        crate::vm::nova_fractal::FractalMode::Mandelbrot => "Mandelbrot",
        crate::vm::nova_fractal::FractalMode::Julia => "Julia",
    };

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Fractal View ({})", mode_str)),
    );
    f.render_widget(grid_widget, chunks[0]);

    // 3. Render Info
    let info = vec![
        Line::from("FRACTAL ENGINE"),
        Line::from(" "),
        Line::from(format!("Mode: {}", mode_str)),
        Line::from(format!("Zoom: {:.2e}", vm.fractal.zoom)),
        Line::from(format!(
            "Center: {:.6} + {:.6}i",
            vm.fractal.center_re, vm.fractal.center_im
        )),
        Line::from(format!("Max Iter: {}", vm.fractal.max_iter)),
        Line::from(" "),
        Line::from("Julia Constant:"),
        Line::from(format!("{:.6} + {:.6}i", vm.fractal.c_re, vm.fractal.c_im)),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  Mandelbrot(iter), Julia(re, im)"),
        Line::from("  Zoom(factor), Pan(dx, dy)"),
        Line::from("  Iterate, Escape"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Parameters"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_semiotics(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Top: Context Info
    let context_hash = vm.semiotic_context;
    let info = vec![
        Line::from(vec![
            Span::styled(
                "SEMIOTIC ENGINE",
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(" [Context: {:x}]", context_hash)),
        ]),
        Line::from(" "),
        Line::from("Meaning is fluid. Symbols shift. Reality is negotiable."),
    ];
    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Semiotics"));
    f.render_widget(info_widget, chunks[0]);

    // Bottom: Meaning Map
    // Filter map for current context
    let mut items = Vec::new();
    let mut count = 0;

    // Sort keys for stability
    let mut keys: Vec<_> = vm.meaning_map.keys().collect();
    keys.sort();

    for (ctx, id) in keys {
        if *ctx == context_hash || *ctx == 0 {
            // Show current context + global (0)
            let val = &vm.meaning_map[&(*ctx, *id)];
            let prefix = if *ctx == 0 { "Global" } else { "Local" };
            let style = if *ctx == 0 {
                Style::default().fg(Color::Gray)
            } else {
                Style::default().fg(Color::Yellow)
            };

            items.push(ListItem::new(format!("[{}] §{:x} -> {}", prefix, id, val)).style(style));
            count += 1;
        }
    }

    if count == 0 {
        items.push(ListItem::new("No defined symbols in this context."));
    }

    let map_list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Active Meanings ({})", count)),
    );
    f.render_widget(map_list, chunks[1]);
}

pub(crate) fn render_palette(f: &mut Frame, app_state: &AppState) {
    let area = app_state.get_render_area(f.area());
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

pub(crate) fn render_view_selector(f: &mut Frame, app_state: &AppState) {
    let area = app_state.get_render_area(f.area());
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

#[cfg(feature = "nova")]
pub(crate) fn render_paradox(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Left: Editor
    let editor_block = Block::default()
        .borders(Borders::ALL)
        .title("Paradox Rule Editor (Enter to Compile, Esc to Exit)")
        .border_style(Style::default().fg(Color::Yellow));

    let editor_text = if app_state.paradox_editor_buffer.is_empty() {
        "Type rule here... e.g. 'rule Test triggers always do log Hello'"
    } else {
        app_state.paradox_editor_buffer.as_str()
    };

    f.render_widget(
        Paragraph::new(editor_text)
            .block(editor_block)
            .wrap(ratatui::widgets::Wrap { trim: false }),
        chunks[0],
    );

    // Right: Active Rules
    let mut items = Vec::new();
    if vm.paradox.rules.is_empty() {
        items.push(ListItem::new("No Paradox Rules active."));
    } else {
        for (i, rule) in vm.paradox.rules.iter().enumerate() {
            items.push(
                ListItem::new(format!("#{}: {}", i, rule.name))
                    .style(Style::default().fg(Color::Cyan)),
            );
            items.push(
                ListItem::new(format!("  Trigger: {:?}", rule.trigger))
                    .style(Style::default().fg(Color::DarkGray)),
            );
            for action in &rule.actions {
                items.push(
                    ListItem::new(format!("  Do: {:?}", action))
                        .style(Style::default().fg(Color::Green)),
                );
            }
            items.push(ListItem::new(""));
        }
    }

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Active Laws of Physics"),
    );
    f.render_widget(list, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_arena(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
        // Use Button for Call-to-Action
        let center = Button::new("Start Auto-Draft (S)")
            .style_variant(tui_shared::ButtonStyle::Primary)
            .active(true);
        // Button fills its area, so we center the area
        let area = chunks[0];
        let btn_width = 30;
        let btn_height = 3;
        let x = area.x + (area.width.saturating_sub(btn_width)) / 2;
        let y = area.y + (area.height.saturating_sub(btn_height)) / 2;
        let btn_area = ratatui::layout::Rect {
            x,
            y,
            width: btn_width,
            height: btn_height,
        };
        f.render_widget(center, btn_area);
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
pub(crate) fn render_egregore(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
pub(crate) fn render_weaver(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25), // Strand A
                Constraint::Percentage(25), // Strand B
                Constraint::Percentage(50), // Loom
            ]
            .as_ref(),
        )
        .split(app_state.get_render_area(f.area()));

    // Helper to render strand preview (reused concept from Laboratory)
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

    // Strand A
    f.render_widget(
        render_strand(
            app_state.lab_parent_a,
            "Warp A",
            app_state.selected_strand == 0,
        ),
        chunks[0],
    );

    // Strand B
    f.render_widget(
        render_strand(
            app_state.lab_parent_b,
            "Warp B",
            app_state.selected_strand == 1,
        ),
        chunks[1],
    );

    // Loom (Pattern & Result)
    let loom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(chunks[2]);

    let pattern_str = &app_state.input_buffer;
    let mut pattern_spans = Vec::new();
    let display_str = if pattern_str.is_empty() {
        "Type pattern (Enter to edit)... e.g. ABAB"
    } else {
        pattern_str
    };

    for c in display_str.chars() {
        let style = match c.to_ascii_uppercase() {
            'A' => Style::default().fg(Color::Green),
            'B' => Style::default().fg(Color::Blue),
            'X' => Style::default().fg(Color::Magenta),
            '0' => Style::default().fg(Color::DarkGray),
            _ => Style::default(),
        };
        pattern_spans.push(Span::styled(c.to_string(), style));
    }

    let pattern_widget = Paragraph::new(Line::from(pattern_spans)).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Weaving Pattern (A/B/X/0)")
            .border_style(Style::default().fg(Color::Cyan)),
    );
    f.render_widget(pattern_widget, loom_chunks[0]);

    // Preview Result
    let mut preview_items = Vec::new();
    let idx_a = app_state.lab_parent_a;
    let idx_b = app_state.lab_parent_b;

    if idx_a < vm.dna.helix.strands.len() && idx_b < vm.dna.helix.strands.len() {
        let strand_a = &vm.dna.helix.strands[idx_a];
        let strand_b = &vm.dna.helix.strands[idx_b];
        let mut ptr_a = 0;
        let mut ptr_b = 0;

        for (i, c) in pattern_str.chars().enumerate() {
            let prefix = format!("{:03}: ", i);
            match c.to_ascii_uppercase() {
                'A' => {
                    if ptr_a < strand_a.genes.len() {
                        let op_str = format!("{}", strand_a.genes[ptr_a].op);
                        preview_items.push(
                            ListItem::new(format!("{}A -> {}", prefix, op_str))
                                .style(Style::default().fg(Color::Green)),
                        );
                        ptr_a += 1;
                    } else {
                        preview_items.push(
                            ListItem::new(format!("{}A -> (End)", prefix))
                                .style(Style::default().fg(Color::DarkGray)),
                        );
                    }
                }
                'B' => {
                    if ptr_b < strand_b.genes.len() {
                        let op_str = format!("{}", strand_b.genes[ptr_b].op);
                        preview_items.push(
                            ListItem::new(format!("{}B -> {}", prefix, op_str))
                                .style(Style::default().fg(Color::Blue)),
                        );
                        ptr_b += 1;
                    } else {
                        preview_items.push(
                            ListItem::new(format!("{}B -> (End)", prefix))
                                .style(Style::default().fg(Color::DarkGray)),
                        );
                    }
                }
                'X' => {
                    preview_items.push(
                        ListItem::new(format!("{}X -> Random(A/B)", prefix))
                            .style(Style::default().fg(Color::Magenta)),
                    );
                }
                '0' => {
                    preview_items.push(
                        ListItem::new(format!("{}0 -> Nop (Skip)", prefix))
                            .style(Style::default().fg(Color::DarkGray)),
                    );
                }
                _ => {}
            }
        }
    }

    let preview_list = List::new(preview_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Weft (Result Preview)"),
    );
    f.render_widget(preview_list, loom_chunks[1]);
}

use rand::Rng;
use crate::tui::panel_block;
use crate::vm::ChimeraVM;
use crate::tui::state::AppState;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Row, Table},
    Frame,
};
use ratatui::widgets::canvas::{Canvas, Rectangle};
#[cfg(feature = "nova")]
use hyper_system::math::Vec4;
#[cfg(feature = "nova")]
use tui_shared::{Bobber, Button, TensionBar};

#[cfg(feature = "nova")]
pub(crate) fn render_topology(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
    let map = Paragraph::new(map_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Topology Map ({:?})", vm.topology)),
    );
    f.render_widget(map, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_retina(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
        "Retina Display Active.\nControl via `retina_draw`, `retina_clear`.\nGlitch: `scanline(y)`, `rasterize(y,x,j,m)`.",
    )
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_ballistics(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
pub(crate) fn render_attractor(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Strange Attractor (X/Z Plane)"),
        )
        .x_bounds([-50.0, 50.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            let mut prev_x = vm.attractor.x;
            let mut prev_z = vm.attractor.z;

            // Draw history
            for (hx, _hy, hz) in &vm.attractor.history {
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: prev_x,
                    y1: prev_z,
                    x2: *hx,
                    y2: *hz,
                    color: Color::Cyan,
                });
                prev_x = *hx;
                prev_z = *hz;
            }

            // Current pos
            ctx.print(vm.attractor.x, vm.attractor.z, "@");
        });
    f.render_widget(canvas, chunks[0]);

    // Info
    let info = vec![
        Line::from("ATTRACTOR STATE"),
        Line::from(format!(
            "Mode: {}",
            match vm.attractor.mode {
                0 => "Lorenz",
                1 => "Rossler",
                2 => "Thomas",
                _ => "Unknown",
            }
        )),
        Line::from(format!("X: {:.4}", vm.attractor.x)),
        Line::from(format!("Y: {:.4}", vm.attractor.y)),
        Line::from(format!("Z: {:.4}", vm.attractor.z)),
        Line::from(" "),
        Line::from("Params:"),
        Line::from(format!("Sigma: {:.4}", vm.attractor.sigma)),
        Line::from(format!("Rho:   {:.4}", vm.attractor.rho)),
        Line::from(format!("Beta:  {:.4}", vm.attractor.beta)),
        Line::from(format!("dt:    {:.4}", vm.attractor.dt)),
    ];
    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Dynamics"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_hyperspace(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hyperspace Tunnel (Recursion Depth)"),
        )
        .paint(|ctx| {
            let center_x = 50.0;
            let center_y = 50.0;
            // Draw frames from Root (Outer) to Current (Inner)
            // Limit to last 10 frames to avoid clutter
            let stack_len = vm.call_stack.len();
            let start_idx = stack_len.saturating_sub(10);

            for i in start_idx..=stack_len {
                let relative_depth = i - start_idx;
                // Scale decreases as we go deeper (inner)
                // Root (0) -> Large
                // Current -> Small
                // But we want perspective.
                // Outer ring = Caller. Inner ring = Callee.

                // Let's invert: i=0 (Root) is smallest? No, tunnel view.
                // You move INTO the tunnel.
                // So Current is far away (small). Root is behind you (large).
                // Or: You are AT Current. Callers are enclosing you.
                // So Current is Center/Large? No, call stack is "below" or "around".

                // Let's stick to: Root is Outer (Large), Current is Inner (Small).
                // It looks like a tunnel going forward.

                let max_steps = 10.0;
                let step = relative_depth as f64;
                let scale = 100.0 * (1.0 - (step / (max_steps + 2.0)));

                if scale <= 0.0 {
                    break;
                }

                let rect_w = scale;
                let rect_h = scale * 0.6;

                let color = if i == stack_len {
                    Color::Yellow // Current
                } else {
                    Color::Cyan // Caller
                };

                ctx.draw(&Rectangle {
                    x: center_x - rect_w / 2.0,
                    y: center_y - rect_h / 2.0,
                    width: rect_w,
                    height: rect_h,
                    color,
                });

                // Label
                if scale > 20.0 {
                    let label = if i < stack_len {
                        let (s, g) = vm.call_stack[i];
                        format!("Stack[{}]: Strand {}:{}", i, s, g)
                    } else {
                        format!("Current: Strand {}:{}", vm.ip.0, vm.ip.1)
                    };
                    ctx.print(
                        center_x - rect_w / 2.0 + 2.0,
                        center_y + rect_h / 2.0 - 5.0,
                        label,
                    );
                }
            }
        })
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0]);

    f.render_widget(canvas, chunks[0]);

    // Info
    let info = vec![
        Line::from(format!("Recursion Depth: {}", vm.recursion_depth)),
        Line::from(format!("Call Stack Size: {}", vm.call_stack.len())),
        Line::from(" "),
        Line::from("Controls:"),
        Line::from("  Compose, Curry, Quote: Functional Ops"),
        Line::from("  Call/Ret: Manual flow"),
    ];
    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Metrics"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_tesseract(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Tesseract (4D Hypercube)"),
        )
        .x_bounds([0.0, 16.0])
        .y_bounds([0.0, 16.0])
        .paint(|ctx| {
            // Draw 2D Grid Base
            for y in 0..16 {
                for x in 0..16 {
                    let val = &vm.grid[y][x];
                    if let crate::vm::Value::Int(n) = val {
                        if *n != 0 {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: 15.0 - y as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::DarkGray,
                            });
                        }
                    }
                }
            }

            // Draw Hyper-Agents
            for ((y, x), (z, w)) in &vm.prologue_state.hyper_state.extra_dims {
                // Projection: Use hyper-system's stereographic projection
                let v = Vec4::new(*x as f32, *y as f32, *z as f32, *w as f32);
                let p3 = v.project_to_3d(5.0); // Camera W distance

                let px = p3.x as f64;
                let py = 15.0 - p3.y as f64; // Invert Y for canvas

                // Color based on W (Hyper-depth)
                let color = if *w > 0 {
                    Color::Magenta
                } else if *w < 0 {
                    Color::Cyan
                } else {
                    Color::Yellow
                };

                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: *x as f64 + 0.5,
                    y1: 15.0 - *y as f64 + 0.5,
                    x2: px + 0.5,
                    y2: py + 0.5,
                    color: Color::Gray,
                });

                // Use color for the point
                ctx.draw(&ratatui::widgets::canvas::Line {
                    x1: px + 0.5 - 0.1,
                    y1: py + 0.5 - 0.1,
                    x2: px + 0.5 + 0.1,
                    y2: py + 0.5 + 0.1,
                    color,
                });

                ctx.print(px + 0.5, py + 0.5, "♦");
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Info Panel
    let mut info = Vec::new();
    info.push(Line::from("HYPERSPACE NAVIGATOR"));
    info.push(Line::from(" "));
    info.push(Line::from(format!(
        "Rotation: {:.1}",
        vm.prologue_state.hyper_state.rotation
    )));
    info.push(Line::from(format!(
        "Projection: {}",
        vm.prologue_state.hyper_state.projection_mode
    )));
    info.push(Line::from(" "));
    info.push(Line::from("Active Hyper-Agents:"));
    for ((y, x), (z, w)) in &vm.prologue_state.hyper_state.extra_dims {
        info.push(Line::from(format!("  ({}, {}) -> Z:{} W:{}", x, y, z, w)));
    }

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("4D Coordinates"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_hologram(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
        .split(app_state.get_render_area(f.area()));

    // Hologram Grid Visualization
    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let (re, im) = vm.hologram_grid[y][x];
            let magnitude = (re * re + im * im).sqrt();
            let phase = im.atan2(re); // -PI to PI

            // Visualizing Magnitude as Character density
            let ch = if magnitude < 0.1 {
                " ".to_string()
            } else if magnitude < 1.0 {
                "·".to_string()
            } else if magnitude < 2.0 {
                "~".to_string()
            } else if magnitude < 5.0 {
                "x".to_string()
            } else if magnitude < 10.0 {
                "%".to_string()
            } else {
                "#".to_string()
            };

            // Visualizing Phase as Color
            // Map -PI..PI to Hue spectrum
            let hue = (phase + std::f64::consts::PI) / (2.0 * std::f64::consts::PI); // 0.0 to 1.0

            let color = if magnitude < 0.1 {
                Color::DarkGray
            } else if hue < 0.16 {
                Color::Red
            } else if hue < 0.33 {
                Color::Yellow
            } else if hue < 0.5 {
                Color::Green
            } else if hue < 0.66 {
                Color::Cyan
            } else if hue < 0.83 {
                Color::Blue
            } else {
                Color::Magenta
            };

            let mut style = Style::default().fg(color);

            if magnitude > 10.0 {
                style = style.add_modifier(Modifier::BOLD);
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
            .title("Interference Pattern (Re/Im)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info Panel
    let (cx, cy) = app_state.grid_cursor;
    let (re, im) = vm.hologram_grid[cy][cx];
    let mag = (re * re + im * im).sqrt();
    let phase = im.atan2(re);

    let info = vec![
        Line::from("HOLOGRAPHIC PLATE"),
        Line::from(" "),
        Line::from(format!("Cursor: {},{}", cx, cy)),
        Line::from(format!("Real: {:.4}", re)),
        Line::from(format!("Imag: {:.4}", im)),
        Line::from(format!("Mag:  {:.4}", mag)),
        Line::from(format!("Phase:{:.4} rad", phase)),
        Line::from(" "),
        Line::from("Operations:"),
        Line::from("  Interfere(s) -> Encode Strand"),
        Line::from("  Refract()    -> Decode to Strand"),
        Line::from("  Project()    -> Manifest on Grid"),
    ];

    let info_widget = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Wave Analysis"),
    );
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_strings(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
                let _dx = (s.end.0 - s.start.0) / steps as f64;
                let _dy = (s.end.1 - s.start.1) / steps as f64;

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
pub(crate) fn render_chronos(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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
        Line::from(format!("Chronos Integrity: {:.1}%", vm.chronos_integrity)),
        Line::from(format!("Active Time Loops: {}", vm.paradox_loops.len())),
        Line::from(" "),
        Line::from(format!("Cursor: {},{}", cx, cy)),
        Line::from(" "),
        Line::from("Opcodes:"),
        Line::from("  TimeWarp(factor, radius)"),
        Line::from("  TimeLoop(id) / Paradox(id, val)"),
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

    // Add Active Loops
    if !vm.paradox_loops.is_empty() {
        echoes.push(ListItem::new(""));
        echoes
            .push(ListItem::new("--- Active Loops ---").style(Style::default().fg(Color::Yellow)));
        for (id, idx) in &vm.paradox_loops {
            echoes.push(ListItem::new(format!("ID {}: Spore #{}", id, idx)));
        }
    }

    let echo_list = List::new(echoes).block(
        Block::default()
            .borders(Borders::ALL)
            .title(format!("Echoes at {},{}", cx, cy)),
    );
    f.render_widget(echo_list, right_chunks[1]);
}

pub(crate) fn render_biotic_chaos(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let mut grid_lines = Vec::new();
    for y in 0..16 {
        let mut line_spans = Vec::new();
        for x in 0..16 {
            let val = vm.chaos_struct.grid[y][x];
            let r = vm.chaos_struct.r_grid[y][x];

            // Value 0.0 - 1.0 mapped to color intensity
            let intensity = (val * 255.0).clamp(0.0, 255.0) as u8;

            let mut style = Style::default();
            // Color based on r (Growth rate)
            // 3.0 (Blue) -> 3.5 (Green) -> 3.8 (Yellow) -> 4.0 (Red)
            let (cr, cg, cb) = if r < 3.5 {
                (0, intensity, 255 - intensity) // Blue-Cyan
            } else if r < 3.7 {
                (0, 255, intensity) // Green
            } else if r < 3.9 {
                (255, 255, 0) // Yellow
            } else {
                (255, 0, 0) // Red (Chaos)
            };

            style = style.fg(Color::Rgb(cr, cg, cb));

            if app_state.grid_cursor == (x, y) {
                style = style.add_modifier(Modifier::REVERSED);
            }

            let ch = if val < 0.2 {
                "·"
            } else if val < 0.4 {
                "░"
            } else if val < 0.6 {
                "▒"
            } else if val < 0.8 {
                "▓"
            } else {
                "█"
            };

            line_spans.push(Span::styled(ch.to_string(), style));
            line_spans.push(Span::raw(" "));
        }
        grid_lines.push(Line::from(line_spans));
    }

    let grid_widget = Paragraph::new(grid_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Biotic Chaos (CML)"),
    );
    f.render_widget(grid_widget, chunks[0]);

    // Info
    let (cx, cy) = app_state.grid_cursor;
    let val = vm.chaos_struct.grid[cy][cx];
    let r = vm.chaos_struct.r_grid[cy][cx];

    let info = vec![
        Line::from("COUPLED MAP LATTICE"),
        Line::from(" "),
        Line::from(format!("Pos: {},{}", cx, cy)),
        Line::from(format!("Value: {:.4}", val)),
        Line::from(format!("Growth (r): {:.4}", r)),
        Line::from(format!("Coupling (e): {:.4}", vm.chaos_struct.coupling)),
        Line::from(" "),
        Line::from("Behavior:"),
        Line::from("  r < 3.5: Stable/Periodic"),
        Line::from("  r > 3.57: Chaos"),
        Line::from(" "),
        Line::from("Interactions:"),
        Line::from("  chaos(0) -> Read Value"),
        Line::from("  chaos(1, v) -> Set r (3.0 + v/100)"),
        Line::from("  chaos(2, v) -> Set coupling (v/100)"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Parameters"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_pandemonium(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
        .split(app_state.get_render_area(f.area()));

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Pandemonium Reactor"),
        )
        .paint(|ctx| {
            let mut linear_idx = 0;
            for (s_idx, strand) in vm.dna.helix.strands.iter().enumerate() {
                for (g_idx, _gene) in strand.genes.iter().enumerate() {
                    let theta = (linear_idx as f64) * 0.1;
                    let r = theta * 0.5;
                    let x = r * theta.cos();
                    let y = r * theta.sin();

                    let color = if s_idx == vm.ip.0 && g_idx == vm.ip.1 {
                        Color::Yellow
                    } else if s_idx % 2 == 0 {
                        Color::Cyan
                    } else {
                        Color::Magenta
                    };

                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: x,
                        y1: y,
                        x2: x + 0.2,
                        y2: y + 0.2,
                        color,
                    });

                    linear_idx += 1;
                }
            }

            // Draw Reticle
            let (cx, cy) = app_state.pandemonium_cursor;
            let radius = app_state.pandemonium_radius;

            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: cx,
                y: cy,
                radius,
                color: Color::Red,
            });

            ctx.print(cx, cy, "+");
        })
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0]);

    f.render_widget(canvas, chunks[0]);

    // Info Panel
    let tool_name = match app_state.pandemonium_selected_tool {
        0 => "Mutate (Single Gene)",
        1 => "Scramble (Radius)",
        2 => "Purge (Radius)",
        3 => "Duplicate (Single Gene)",
        4 => "Storm (Chaos & Lightning)",
        _ => "Unknown",
    };

    let info = vec![
        Line::from("PANDEMONIUM REACTOR"),
        Line::from(" "),
        Line::from(format!("Tool: {} (1-4)", tool_name)),
        Line::from(format!(
            "Radius: {:.1} ([ / ])",
            app_state.pandemonium_radius
        )),
        Line::from(format!(
            "Cursor: {:.1}, {:.1}",
            app_state.pandemonium_cursor.0, app_state.pandemonium_cursor.1
        )),
        Line::from(" "),
        Line::from("Controls:"),
        Line::from("  Arrows: Move Cursor"),
        Line::from("  Space: Apply Tool"),
    ];

    let info_widget =
        Paragraph::new(info).block(Block::default().borders(Borders::ALL).title("Controls"));
    f.render_widget(info_widget, chunks[1]);
}

#[cfg(feature = "nova")]
pub(crate) fn render_quantum(f: &mut Frame, vm: &mut ChimeraVM, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(app_state.get_render_area(f.area()));

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

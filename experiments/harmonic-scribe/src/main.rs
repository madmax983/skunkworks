use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use harmonic_engine::physics::PhysicsWorld;
use nalgebra::Vector2;
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph, Wrap},
};
use spqr_rsa::roman::Roman;
use std::{io, str::FromStr, time::Duration};

struct ScribeApp {
    world: PhysicsWorld,
    input_buffer: String,
    history: Vec<String>,
    integrator_labels: Vec<String>,
    integrator_values: Vec<String>, // Cached Roman string representation
}

impl ScribeApp {
    fn new() -> Self {
        let mut world = PhysicsWorld::new();

        // Integrator 0: PRIMUS
        world.add_integrator(Vector2::new(-20.0, 0.0));
        // Integrator 1: SECUNDUS
        world.add_integrator(Vector2::new(20.0, 0.0));

        // Couple 0 -> 1
        world.couplings.push((0, 1, 0.05)); // Smaller gain for smoother animation

        Self {
            world,
            input_buffer: String::new(),
            history: vec![
                "HARMONIC SCRIBE: ONLINE".to_string(),
                "Commands: set <id> <roman>, push <id> <roman>".to_string(),
                "Example: push 0 V".to_string(),
            ],
            integrator_labels: vec!["PRIMUS".to_string(), "SECUNDUS".to_string()],
            integrator_values: vec!["NULLA".to_string(), "NULLA".to_string()],
        }
    }

    fn update(&mut self) {
        self.world.step();

        // Read physics state and convert to Roman
        for (i, integrator) in self.world.integrators.iter().enumerate() {
            if let Some(out) = self.world.rigid_body_set.get(integrator.output_handle) {
                let angle = out.rotation().angle();
                // 1 Radian = 1 Unit.
                // Roman numerals are positive integers.
                let val = angle.abs().floor() as u64;
                if val > 0 {
                    let roman = Roman::from_u64(val);
                    // Use format! to invoke Display trait
                    if i < self.integrator_values.len() {
                        self.integrator_values[i] = format!("{}", roman);
                    } else {
                        self.integrator_values.push(format!("{}", roman));
                    }
                } else if i < self.integrator_values.len() {
                    self.integrator_values[i] = "NULLA".to_string();
                }
            }
        }
    }

    fn handle_command(&mut self) {
        let cmd = self.input_buffer.trim().to_string();
        self.history.push(format!("> {}", cmd));

        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() {
            return;
        }

        if parts[0] == "help" {
            self.history.push("Commands:".to_string());
            self.history
                .push("  set <id> <val>: Set integrator value".to_string());
            self.history
                .push("  push <id> <val>: Set ball position (rate)".to_string());
            return;
        }

        if parts.len() < 3 {
            self.history
                .push("ERROR: Invalid syntax. Try 'push 0 V'".to_string());
            return;
        }

        let action = parts[0];
        let idx_res = parts[1].parse::<usize>();
        // Parse Roman. Since spqr-rsa Roman::from_str is implemented
        let roman_res = Roman::from_str(parts[2]);

        if let (Ok(idx), Ok(roman)) = (idx_res, roman_res) {
            if idx >= self.world.integrators.len() {
                self.history
                    .push(format!("ERROR: Integrator {} not found", idx));
                return;
            }

            // Convert Roman to f32
            // Roman::value() returns BigUint.
            let val_big = roman.value();
            let val_f32 = val_big.to_string().parse::<f32>().unwrap_or(0.0);

            match action {
                "set" => {
                    if let Some(body) = self
                        .world
                        .rigid_body_set
                        .get_mut(self.world.integrators[idx].output_handle)
                    {
                        body.set_rotation(nalgebra::UnitComplex::new(val_f32), true);
                        self.history
                            .push(format!("SCRIBE: Set INT {} to {}", idx, parts[2]));
                    }
                }
                "push" => {
                    let center = self.world.integrators[idx].disk_center;
                    if let Some(body) = self
                        .world
                        .rigid_body_set
                        .get_mut(self.world.integrators[idx].ball_handle)
                    {
                        // Limit position to disk radius (10.0)
                        let clamped_val = val_f32.clamp(-9.0, 9.0);
                        body.set_translation(center + Vector2::new(clamped_val, 0.0), true);
                        self.history
                            .push(format!("SCRIBE: Pushed INT {} ball to {}", idx, parts[2]));
                    }
                }
                _ => {
                    self.history
                        .push(format!("ERROR: Unknown action {}", action));
                }
            }
        } else {
            self.history
                .push("ERROR: Parse error (Invalid ID or Roman Numeral)".to_string());
        }

        // Keep history short
        if self.history.len() > 10 {
            self.history.remove(0);
        }
    }

    fn handle_input(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => return true,
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Enter => {
                if !self.input_buffer.is_empty() {
                    self.handle_command();
                    self.input_buffer.clear();
                }
            }
            _ => {}
        }
        false
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = ScribeApp::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && app.handle_input(key.code) {
                        break;
                    }
            }
        }

        app.update();
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &ScribeApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    enum DrawCmd {
        Circle {
            x: f64,
            y: f64,
            radius: f64,
            color: Color,
        },
        Line {
            x1: f64,
            y1: f64,
            x2: f64,
            y2: f64,
            color: Color,
        },
        Text {
            x: f64,
            y: f64,
            text: String,
        },
    }

    let mut cmds = Vec::new();

    for (i, integrator) in app.world.integrators.iter().enumerate() {
        // Draw Disk
        if let Some(disk) = app.world.rigid_body_set.get(integrator.disk_handle) {
            let pos = disk.translation();
            cmds.push(DrawCmd::Circle {
                x: pos.x as f64,
                y: pos.y as f64,
                radius: 10.0,
                color: Color::White,
            });

            // Draw Label
            let label = if i < app.integrator_labels.len() {
                app.integrator_labels[i].clone()
            } else {
                "?".to_string()
            };
            cmds.push(DrawCmd::Text {
                x: pos.x as f64 - 3.0,
                y: pos.y as f64 - 13.0,
                text: label,
            });

            // Draw Value
            let val = if i < app.integrator_values.len() {
                app.integrator_values[i].clone()
            } else {
                "NULLA".to_string()
            };
            cmds.push(DrawCmd::Text {
                x: pos.x as f64 - 3.0,
                y: pos.y as f64 + 13.0,
                text: val,
            });
        }

        // Draw Ball
        if let Some(ball) = app.world.rigid_body_set.get(integrator.ball_handle) {
            let pos = ball.translation();
            cmds.push(DrawCmd::Circle {
                x: pos.x as f64,
                y: pos.y as f64,
                radius: 1.5,
                color: Color::Red,
            });
        }

        // Draw Output Cylinder (simplified as rotating line)
        if let Some(out) = app.world.rigid_body_set.get(integrator.output_handle) {
            let pos = out.translation();
            let rot = out.rotation().angle();
            cmds.push(DrawCmd::Line {
                x1: pos.x as f64,
                y1: pos.y as f64,
                x2: (pos.x + 8.0 * rot.cos()) as f64,
                y2: (pos.y + 8.0 * rot.sin()) as f64,
                color: Color::Yellow,
            });
        }
    }

    // Draw Coupling (Line between output and input)
    for (src, dst, _) in &app.world.couplings {
        if let (Some(src_int), Some(dst_int)) = (
            app.world.integrators.get(*src),
            app.world.integrators.get(*dst),
        ) {
            // From Src Output to Dst Ball
            if let (Some(out), Some(ball)) = (
                app.world.rigid_body_set.get(src_int.output_handle),
                app.world.rigid_body_set.get(dst_int.ball_handle),
            ) {
                cmds.push(DrawCmd::Line {
                    x1: out.translation().x as f64,
                    y1: out.translation().y as f64,
                    x2: ball.translation().x as f64,
                    y2: ball.translation().y as f64,
                    color: Color::Green,
                });
            }
        }
    }

    // Physics View
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("MECHANICAL INTEGRATORS"),
        )
        .x_bounds([-40.0, 40.0])
        .y_bounds([-30.0, 30.0])
        .paint(move |ctx| {
            for cmd in &cmds {
                match cmd {
                    DrawCmd::Circle {
                        x,
                        y,
                        radius,
                        color,
                    } => {
                        ctx.draw(&ratatui::widgets::canvas::Circle {
                            x: *x,
                            y: *y,
                            radius: *radius,
                            color: *color,
                        });
                    }
                    DrawCmd::Line {
                        x1,
                        y1,
                        x2,
                        y2,
                        color,
                    } => {
                        ctx.draw(&ratatui::widgets::canvas::Line {
                            x1: *x1,
                            y1: *y1,
                            x2: *x2,
                            y2: *y2,
                            color: *color,
                        });
                    }
                    DrawCmd::Text { x, y, text } => {
                        ctx.print(*x, *y, text.clone());
                    }
                }
            }
        });
    f.render_widget(canvas, chunks[0]);

    // Tablet View
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(chunks[1]);

    let history_text = app.history.join("\n");
    let tablet = Paragraph::new(history_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("TABULA (Tablet)"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(tablet, right_chunks[0]);

    let input = Paragraph::new(format!("> {}", app.input_buffer))
        .block(Block::default().borders(Borders::ALL).title("SCRIBE INPUT"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(input, right_chunks[1]);
}

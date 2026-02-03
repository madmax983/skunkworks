use anyhow::Result;
use cargo_metadata::MetadataCommand;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use tui_shared::Tui;

struct Blip {
    name: String,
    version: String,
    kind: String,
    x: f64,
    y: f64,
    angle: f64,
    #[allow(dead_code)]
    radius: f64,
}

struct App {
    root_name: String,
    blips: Vec<Blip>,
    sweep_angle: f64,
    exit: bool,
}

impl App {
    fn new() -> Result<Self> {
        // Run cargo metadata to get dependencies
        let metadata = MetadataCommand::new().exec()?;

        // Find the root package (or the one in current dir)
        let root = metadata.root_package().ok_or_else(|| anyhow::anyhow!("No root package found in current directory"))?;

        let mut blips = Vec::new();

        for dep in &root.dependencies {
            let mut hasher = DefaultHasher::new();
            dep.name.hash(&mut hasher);
            let h = hasher.finish();

            // Deterministic Angle: 0 to 360 degrees
            let angle = (h as f64 % 360.0).to_radians();

            // Radius: Deterministic, distributed between 20.0 and 90.0
            // We can use a different part of the hash
            let radius = 20.0 + (h.wrapping_shr(10) as f64 % 70.0);

            let x = radius * angle.cos();
            let y = radius * angle.sin();

            let kind = format!("{:?}", dep.kind); // e.g., Normal, Development, Build

            blips.push(Blip {
                name: dep.name.clone(),
                version: dep.req.to_string(),
                kind,
                x,
                y,
                angle,
                radius,
            });
        }

        Ok(Self {
            root_name: root.name.clone(),
            blips,
            sweep_angle: 0.0,
            exit: false,
        })
    }

    fn on_tick(&mut self) {
        self.sweep_angle += 0.05;
        if self.sweep_angle > std::f64::consts::PI * 2.0 {
            self.sweep_angle -= std::f64::consts::PI * 2.0;
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Initialize app. If it fails (e.g. no Cargo.toml), print error and exit cleanly.
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            tui.exit()?;
            eprintln!("Error initializing Crate Radar: {}", e);
            return Ok(());
        }
    };

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc | KeyCode::Char('q') => app.exit = true,
                        _ => {}
                    }
                }
            }
        }

        if app.exit {
            break;
        }
        app.on_tick();
    }

    tui.exit()?;
    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(main_layout[0]);

    // Calculate active blip (closest to sweep)
    let mut best_blip: Option<&Blip> = None;
    let mut min_diff = 0.2; // Threshold in radians

    for blip in &app.blips {
        let diff = (app.sweep_angle - blip.angle).abs();
        let diff = if diff > std::f64::consts::PI {
            2.0 * std::f64::consts::PI - diff
        } else {
            diff
        };
        if diff < min_diff {
            min_diff = diff;
            best_blip = Some(blip);
        }
    }

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Crate Radar "),
        )
        .x_bounds([-100.0, 100.0])
        .y_bounds([-100.0, 100.0])
        .paint(|ctx| {
            // Radar Circles
            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: 0.0,
                y: 0.0,
                radius: 90.0,
                color: Color::DarkGray,
            });
            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: 0.0,
                y: 0.0,
                radius: 50.0,
                color: Color::DarkGray,
            });
             ctx.draw(&ratatui::widgets::canvas::Circle {
                x: 0.0,
                y: 0.0,
                radius: 20.0,
                color: Color::DarkGray,
            });

            // Center Point (Root Crate)
            ctx.draw(&ratatui::widgets::canvas::Circle {
                x: 0.0,
                y: 0.0,
                radius: 2.0,
                color: Color::Cyan,
            });

            // Sweep Line
            let sweep_x = 95.0 * app.sweep_angle.cos();
            let sweep_y = 95.0 * app.sweep_angle.sin();
            ctx.draw(&CanvasLine {
                x1: 0.0,
                y1: 0.0,
                x2: sweep_x,
                y2: sweep_y,
                color: Color::Green,
            });

            // Blips
            let mut bright_points = Vec::new();
            let mut dim_points = Vec::new();

            for blip in &app.blips {
                let diff = (app.sweep_angle - blip.angle).abs();
                let diff = if diff > std::f64::consts::PI {
                    2.0 * std::f64::consts::PI - diff
                } else {
                    diff
                };

                // If sweep is passing over, it's bright
                if diff < 0.3 {
                    bright_points.push((blip.x, blip.y));
                } else {
                    // Always show points dimly so we can see the constellation
                    dim_points.push((blip.x, blip.y));
                }
            }

            ctx.draw(&Points {
                coords: &dim_points,
                color: Color::Gray,
            });
            ctx.draw(&Points {
                coords: &bright_points,
                color: Color::Green,
            });

            // Highlight best blip
            if let Some(blip) = best_blip {
                ctx.draw(&ratatui::widgets::canvas::Circle {
                    x: blip.x,
                    y: blip.y,
                    radius: 3.0,
                    color: Color::Yellow,
                });
            }
        });

    f.render_widget(canvas, top_layout[0]);

    // Details Panel
    let details_text = if let Some(blip) = best_blip {
        vec![
            Line::from(vec![
                Span::styled("Dependency: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    &blip.name,
                    Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("Version: ", Style::default().fg(Color::Gray)),
                Span::raw(&blip.version),
            ]),
             Line::from(vec![
                Span::styled("Kind: ", Style::default().fg(Color::Gray)),
                Span::raw(&blip.kind),
            ]),
        ]
    } else {
        vec![
            Line::from("Scanning sector..."),
            Line::from(""),
            Line::from(vec![
                Span::styled("Center: ", Style::default().fg(Color::Gray)),
                Span::styled(&app.root_name, Style::default().fg(Color::Cyan)),
            ]),
        ]
    };

    let details = Paragraph::new(details_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Target Info "),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(details, top_layout[1]);

    let help_text = "ESC/q: Quit";
    let status_bar = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status_bar, main_layout[1]);
}

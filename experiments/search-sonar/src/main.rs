use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{canvas::{Canvas, Line as CanvasLine, Points}, Block, Borders, Paragraph},
    Frame,
};
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::{Duration};
use tui_shared::Tui;

use crate::search::{spawn_search_thread, SearchResult};

mod search;

struct Blip {
    x: f64,
    y: f64,
    angle: f64, // 0 to 2PI
    result: SearchResult,
}

struct App {
    query: String,
    results: Vec<Blip>,
    rx: Option<Receiver<SearchResult>>,
    sweep_angle: f64,
    exit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            query: String::new(),
            results: Vec::new(),
            rx: None,
            sweep_angle: 0.0,
            exit: false,
        }
    }

    fn on_tick(&mut self) {
        self.sweep_angle += 0.05;
        if self.sweep_angle > std::f64::consts::PI * 2.0 {
            self.sweep_angle -= std::f64::consts::PI * 2.0;
        }

        // Poll results
        if let Some(rx) = &self.rx {
            // Try to read all available messages
            loop {
                 match rx.try_recv() {
                    Ok(res) => {
                        let (x, y, angle) = Self::map_to_coords(&res.path);
                        self.results.push(Blip { x, y, angle, result: res });
                    },
                    Err(_) => break, // Empty or disconnected
                 }
            }
        }
    }

    fn map_to_coords(path: &PathBuf) -> (f64, f64, f64) {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        path.hash(&mut hasher);
        let h = hasher.finish();

        // Angle: deterministic based on path
        let angle = (h as f64 % 360.0).to_radians();

        // Radius: randomized but deterministic
        // We want radius between 10.0 and 90.0 (assuming canvas is 200x200 centered at 0)
        let radius = 20.0 + (h.wrapping_shr(10) as f64 % 70.0);

        let x = radius * angle.cos();
        let y = radius * angle.sin();

        (x, y, angle)
    }

    fn start_search(&mut self) {
        if self.query.trim().is_empty() {
            return;
        }
        self.results.clear();
        let rx = spawn_search_thread(self.query.clone(), PathBuf::from("."));
        self.rx = Some(rx);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.exit = true,
                        KeyCode::Enter => app.start_search(),
                        KeyCode::Char(c) => app.query.push(c),
                        KeyCode::Backspace => { app.query.pop(); },
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

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(70),
            Constraint::Percentage(30),
        ])
        .split(main_layout[0]);

    // Calculate active blip (closest to sweep)
    let mut best_blip: Option<&Blip> = None;
    let mut min_diff = 0.2; // Threshold

    for blip in &app.results {
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
        .block(Block::default().borders(Borders::ALL).title(" Search Sonar "))
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

            for blip in &app.results {
                let diff = (app.sweep_angle - blip.angle).abs();
                let diff = if diff > std::f64::consts::PI {
                    2.0 * std::f64::consts::PI - diff
                } else {
                    diff
                };

                if diff < 0.3 {
                    bright_points.push((blip.x, blip.y));
                } else if diff < 1.0 {
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
            Line::from(vec![Span::raw("File: "), Span::styled(blip.result.path.to_string_lossy(), Style::default().fg(Color::Cyan))]),
            Line::from(vec![Span::raw(format!("Line: {}", blip.result.line_num))]),
            Line::from(Span::raw("")),
            Line::from(vec![Span::styled(format!("> {}", blip.result.content.trim()), Style::default().fg(Color::Green))]),
        ]
    } else {
        vec![Line::from("Scanning...")]
    };

    let details = Paragraph::new(details_text)
        .block(Block::default().borders(Borders::ALL).title(" Target Info "))
        .wrap(ratatui::widgets::Wrap { trim: true });

    f.render_widget(details, top_layout[1]);

    let input = Paragraph::new(format!("Query: {}", app.query))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(input, main_layout[1]);
}

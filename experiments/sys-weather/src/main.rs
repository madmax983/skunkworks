pub mod model;
pub mod monitor;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::collections::VecDeque;
use std::time::Duration;
use tui_shared::Tui;

use crate::model::{integrate, LorenzParams, LorenzState};
use crate::monitor::SystemMonitor;

const HISTORY_SIZE: usize = 2000;
const STEPS_PER_FRAME: usize = 10;
const DT: f64 = 0.005;

struct App {
    state: LorenzState,
    params: LorenzParams,
    history: VecDeque<LorenzState>,
    monitor: SystemMonitor,

    // UI State
    paused_monitor: bool,
    cpu_usage: f32,
    mem_used: u64,
    mem_total: u64,
}

impl App {
    fn new() -> Self {
        let state = LorenzState::new(0.1, 0.0, 0.0);
        let params = LorenzParams {
            sigma: 10.0,
            rho: 28.0,
            beta: 8.0 / 3.0,
        };

        Self {
            state,
            params,
            history: VecDeque::with_capacity(HISTORY_SIZE),
            monitor: SystemMonitor::new(),
            paused_monitor: false,
            cpu_usage: 0.0,
            mem_used: 0,
            mem_total: 0,
        }
    }

    fn update(&mut self) {
        // Update Monitor
        if !self.paused_monitor {
            let (params, cpu, used, total) = self.monitor.poll();
            self.params = params;
            self.cpu_usage = cpu;
            self.mem_used = used;
            self.mem_total = total;
        }

        // Integrate
        for _ in 0..STEPS_PER_FRAME {
            self.state = integrate(&self.state, &self.params, DT);

            if self.history.len() >= HISTORY_SIZE {
                self.history.pop_front();
            }
            self.history.push_back(self.state);
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Fill initial history
    for _ in 0..100 {
        app.update();
    }

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.paused_monitor = !app.paused_monitor,
                    KeyCode::Char('r') => {
                        app.state = LorenzState::new(0.1, 0.0, 0.0);
                        app.history.clear();
                    }
                    _ => {}
                }
            }
        }

        app.update();
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    draw_attractor(f, chunks[0], app);
    draw_stats(f, chunks[1], app);
}

fn draw_attractor(f: &mut Frame, area: Rect, app: &App) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Lorenz Attractor (X-Z Projection)"),
        )
        .x_bounds([-30.0, 30.0])
        .y_bounds([0.0, 60.0])
        .paint(|ctx| {
            // Draw axes
            ctx.draw(&CanvasLine {
                x1: -30.0,
                y1: 0.0,
                x2: 30.0,
                y2: 0.0,
                color: Color::DarkGray,
            });
            ctx.draw(&CanvasLine {
                x1: 0.0,
                y1: 0.0,
                x2: 0.0,
                y2: 60.0,
                color: Color::DarkGray,
            });

            // Draw history
            let mut iter = app.history.iter();
            if let Some(mut prev) = iter.next() {
                for (i, p) in iter.enumerate() {
                    // Color gradient based on index (age)
                    let color = if i < app.history.len() / 3 {
                        Color::DarkGray
                    } else if i < app.history.len() * 2 / 3 {
                        Color::Blue
                    } else {
                        Color::Cyan
                    };

                    ctx.draw(&CanvasLine {
                        x1: prev.x,
                        y1: prev.z,
                        x2: p.x,
                        y2: p.z,
                        color,
                    });
                    prev = p;
                }
            }

            // Draw head
            ctx.print(app.state.x, app.state.z, "O");
        });

    f.render_widget(canvas, area);
}

fn draw_stats(f: &mut Frame, area: Rect, app: &App) {
    let mem_gb = app.mem_used as f64 / 1024.0 / 1024.0 / 1024.0;
    let total_gb = app.mem_total as f64 / 1024.0 / 1024.0 / 1024.0;

    let text = vec![
        Line::from(Span::styled(
            "System Weather",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(vec![
            Span::raw("CPU Usage: "),
            Span::styled(
                format!("{:.1}%", app.cpu_usage),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            Span::raw("Memory: "),
            Span::styled(
                format!("{:.1} GB / {:.1} GB", mem_gb, total_gb),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Lorenz Parameters:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(vec![
            Span::raw("Sigma (CPU): "),
            Span::styled(
                format!("{:.2}", app.params.sigma),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Rho (Mem):   "),
            Span::styled(
                format!("{:.2}", app.params.rho),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::raw("Beta:        "),
            Span::styled(
                format!("{:.2}", app.params.beta),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Controls:",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from("Space: Toggle Live Data"),
        Line::from("R:     Reset Simulation"),
        Line::from("Q:     Quit"),
        Line::from(""),
        Line::from(if app.paused_monitor {
            Span::styled(
                "STATUS: PAUSED (Static Params)",
                Style::default().fg(Color::Red),
            )
        } else {
            Span::styled(
                "STATUS: LIVE (System Driven)",
                Style::default().fg(Color::Green),
            )
        }),
    ];

    let p = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Stats"));

    f.render_widget(p, area);
}

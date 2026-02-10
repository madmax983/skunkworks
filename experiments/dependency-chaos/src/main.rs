use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Points},
        Block, Borders, List, ListItem, ListState, Paragraph, Wrap,
    },
    Terminal, Frame,
};
use std::{io, time::{Duration, Instant}};

mod physics;
mod analysis;
mod simulation;

use physics::{State, DoublePendulumParams};
use analysis::{analyze_workspace, CrateAnalysis};
use simulation::Cloud;

const PARTICLE_COUNT: usize = 10_000;

struct App {
    crates: Vec<CrateAnalysis>,
    selected_idx: usize,
    cloud: Cloud,
    params: DoublePendulumParams,

    // View
    zoom: f64,
    offset_x: f64,
    offset_y: f64,

    // Simulation
    paused: bool,
    time_scale: f32,
    spread: f32,
}

impl App {
    fn new() -> Result<Self> {
        let crates = analyze_workspace().unwrap_or_else(|e| {
            vec![CrateAnalysis {
                name: format!("Error: {}", e),
                mass1: 1000.0,
                mass2: 1000.0,
                len1: 5.0,
                len2: 5.0,
                color: (255, 0, 0),
            }]
        });

        let selected_idx = 0;
        let params = map_crate_to_params(&crates[selected_idx]);
        let center = State::new(std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0, 0.0, 0.0);
        let cloud = Cloud::new(PARTICLE_COUNT, center, 0.01, params);

        Ok(Self {
            crates,
            selected_idx,
            cloud,
            params,
            zoom: 10.0,
            offset_x: 0.0,
            offset_y: 0.0,
            paused: false,
            time_scale: 1.0,
            spread: 0.01,
        })
    }

    fn update(&mut self, dt: f32) {
        if !self.paused {
            self.cloud.update(dt * self.time_scale);
        }
    }

    fn select_crate(&mut self, idx: usize) {
        if idx < self.crates.len() {
            self.selected_idx = idx;
            self.params = map_crate_to_params(&self.crates[idx]);
            self.cloud.params = self.params;
            self.reset();
        }
    }

    fn reset(&mut self) {
        let center = State::new(std::f32::consts::PI / 2.0, std::f32::consts::PI / 2.0, 0.0, 0.0);
        self.cloud.reset(center, self.spread);
    }

    fn kick(&mut self) {
        self.cloud.kick(5.0);
    }
}

fn map_crate_to_params(c: &CrateAnalysis) -> DoublePendulumParams {
    let m1 = (c.mass1 as f32 / 10_000.0).max(0.1).min(10.0);
    let m2 = (c.mass2 as f32 / 10_000.0).max(0.1).min(10.0);
    let l1 = (c.len1 / 5.0).max(0.5).min(5.0);
    let l2 = (c.len2 / 5.0).max(0.5).min(5.0);

    DoublePendulumParams {
        m1,
        m2,
        l1,
        l2,
        g: 9.81,
        damping: 0.0,
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => app.paused = !app.paused,
                    KeyCode::Char('r') => app.reset(),
                    KeyCode::Char('k') => app.kick(),
                    KeyCode::Up => {
                        if app.selected_idx > 0 {
                            app.select_crate(app.selected_idx - 1);
                        }
                    }
                    KeyCode::Down => {
                        if app.selected_idx < app.crates.len() - 1 {
                            app.select_crate(app.selected_idx + 1);
                        }
                    }
                    KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 0.9,
                    KeyCode::Char('-') => app.zoom *= 1.1,
                    KeyCode::Left => app.offset_x -= app.zoom * 0.1,
                    KeyCode::Right => app.offset_x += app.zoom * 0.1,
                    // WASD for offset
                    KeyCode::Char('w') => app.offset_y += app.zoom * 0.1,
                    KeyCode::Char('s') => app.offset_y -= app.zoom * 0.1,
                    KeyCode::Char('a') => app.offset_x -= app.zoom * 0.1,
                    KeyCode::Char('d') => app.offset_x += app.zoom * 0.1,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = last_tick.elapsed().as_secs_f32();
            app.update(dt.min(0.05));
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    draw_sidebar(f, app, chunks[0]);
    draw_canvas(f, app, chunks[1]);
}

fn draw_sidebar(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(0),    // List
            Constraint::Length(10), // Stats
        ])
        .split(area);

    let title = Paragraph::new("DEPENDENCY CHAOS ⚛️")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app.crates.iter().enumerate().map(|(i, c)| {
        let style = if i == app.selected_idx {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };
        ListItem::new(c.name.clone()).style(style)
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Crates"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    // We need state to scroll, but for now just center?
    // Usually ListState is persistent.
    // For simplicity, we just render. If list is long, we won't see selected if out of view.
    // Ideally we should maintain ListState in App.
    // But let's skip scrolling logic for now or implement basic.
    // Actually, List doesn't auto-scroll to selected index unless we provide state.
    // But `List::new` renders all? No, it clips.

    // Let's create a temporary state
    let mut state = ListState::default();
    state.select(Some(app.selected_idx));
    f.render_stateful_widget(list, chunks[1], &mut state);

    let c = &app.crates[app.selected_idx];
    let stats = vec![
        Line::from(vec![Span::raw("Mass 1 (Src): "), Span::styled(format!("{:.1} KB", c.mass1/1024.0), Style::default().fg(Color::Green))]),
        Line::from(vec![Span::raw("Mass 2 (Tst): "), Span::styled(format!("{:.1} KB", c.mass2/1024.0), Style::default().fg(Color::Green))]),
        Line::from(vec![Span::raw("Deps: "), Span::styled(format!("{}", c.len1), Style::default().fg(Color::Blue))]),
        Line::from(vec![Span::raw("Dev-Deps: "), Span::styled(format!("{}", c.len2), Style::default().fg(Color::Blue))]),
        Line::from(vec![Span::raw("----- Physics -----")]),
        Line::from(vec![Span::raw(format!("m1: {:.2}, m2: {:.2}", app.params.m1, app.params.m2))]),
        Line::from(vec![Span::raw(format!("l1: {:.2}, l2: {:.2}", app.params.l1, app.params.l2))]),
    ];

    let stats_p = Paragraph::new(stats)
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .wrap(Wrap { trim: true });
    f.render_widget(stats_p, chunks[2]);
}

fn draw_canvas(f: &mut Frame, app: &App, area: Rect) {
    let zoom = app.zoom;
    let ox = app.offset_x;
    let oy = app.offset_y;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Phase Space Trajectory"))
        .x_bounds([ox - zoom, ox + zoom])
        .y_bounds([oy - zoom, oy + zoom])
        .marker(Marker::Braille)
        .paint(|ctx| {
            // Draw origin
            ctx.print(0.0, 0.0, Span::styled("+", Style::default().fg(Color::White)));

            // Draw Pendulum Arm (Mean)
            if let Some(s) = app.cloud.states.first() {
                let x1 = app.params.l1 * s.theta1.sin();
                let y1 = -app.params.l1 * s.theta1.cos();
                let x2 = x1 + app.params.l2 * s.theta2.sin();
                let y2 = y1 - app.params.l2 * s.theta2.cos();

                ctx.draw(&CanvasLine {
                    x1: 0.0, y1: 0.0, x2: x1.into(), y2: y1.into(), color: Color::Gray
                });
                ctx.draw(&CanvasLine {
                    x1: x1.into(), y1: y1.into(), x2: x2.into(), y2: y2.into(), color: Color::White
                });
            }

            // Draw Points
            // Convert states to world pos
            // We need a Vec<(f64, f64)>
            // This allocation every frame might be slow for 100k points?
            // But we have 10k points now.
            let points: Vec<(f64, f64)> = app.cloud.states.iter().map(|s| {
                 let x1 = app.params.l1 * s.theta1.sin();
                 let y1 = -app.params.l1 * s.theta1.cos();
                 let x2 = x1 + app.params.l2 * s.theta2.sin();
                 let y2 = y1 - app.params.l2 * s.theta2.cos();
                 (x2 as f64, y2 as f64)
            }).collect();

            let color = &app.crates[app.selected_idx].color;
            let c = Color::Rgb(color.0, color.1, color.2);

            ctx.draw(&Points {
                coords: &points,
                color: c,
            });
        });

    f.render_widget(canvas, area);
}

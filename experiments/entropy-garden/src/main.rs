use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    symbols::Marker,
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Context, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod monitor;
mod simulation;

use monitor::SystemMonitor;
use simulation::GrayScott;

const SIM_WIDTH: usize = 160;
const SIM_HEIGHT: usize = 100;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    run_app(&mut tui, &mut app)?;

    Ok(())
}

struct App {
    sim: GrayScott,
    monitor: SystemMonitor,
    f: f64,
    k: f64,
    cpu: f32,
    mem: f64,
    last_monitor_update: Instant,
}

impl App {
    fn new() -> Self {
        let mut sim = GrayScott::new(SIM_WIDTH, SIM_HEIGHT);
        sim.seed();
        Self {
            sim,
            monitor: SystemMonitor::new(),
            f: 0.055,
            k: 0.062,
            cpu: 0.0,
            mem: 0.0,
            // Initialize with a past time so it updates immediately on first frame
            last_monitor_update: Instant::now() - Duration::from_secs(2),
        }
    }

    fn update(&mut self) {
        if self.last_monitor_update.elapsed() >= Duration::from_secs(1) {
            let (f, k, cpu, mem) = self.monitor.get_parameters();
            self.f = f;
            self.k = k;
            self.cpu = cpu;
            self.mem = mem;
            self.last_monitor_update = Instant::now();
        }

        // Run simulation multiple times per frame for speed?
        // 1 step might be too slow to see evolution.
        // 4 steps?
        for _ in 0..4 {
            self.sim.update(self.f, self.k);
        }
    }
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 fps

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Entropy Garden"),
        )
        .marker(Marker::Braille)
        .x_bounds([0.0, SIM_WIDTH as f64])
        .y_bounds([0.0, SIM_HEIGHT as f64])
        .paint(|ctx| {
            paint_canvas(ctx, &app.sim);
        });
    f.render_widget(canvas, chunks[0]);

    // Info Panel
    let info_block = Block::default()
        .borders(Borders::ALL)
        .title("System Status");
    let info_text = vec![
        Line::from(Span::styled(
            "Reaction-Diffusion Monitor",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(format!("CPU Load: {:.1}%", app.cpu)),
        Line::from(format!("Memory:   {:.1}%", app.mem)),
        Line::from(""),
        Line::from(Span::styled(
            "Parameters:",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(format!("Feed (F): {:.5}", app.f)),
        Line::from(format!("Kill (k): {:.5}", app.k)),
        Line::from(""),
        Line::from(Span::styled("Controls:", Style::default().fg(Color::Green))),
        Line::from("Press 'q' to exit"),
    ];
    let paragraph = Paragraph::new(info_text).block(info_block);
    f.render_widget(paragraph, chunks[1]);
}

fn paint_canvas(ctx: &mut Context, sim: &GrayScott) {
    let mut layer1 = Vec::new(); // Low concentration
    let mut layer2 = Vec::new(); // Med
    let mut layer3 = Vec::new(); // High

    for y in 0..SIM_HEIGHT {
        for x in 0..SIM_WIDTH {
            let (_, v) = sim.get_concentration(x, y);
            // Flip y for canvas logic (0,0 is bottom left usually, but screen is top left)
            // Ratatui canvas: 0,0 is bottom-left.
            // Simulation: 0 is top.
            // Let's invert Y.
            let draw_y = (SIM_HEIGHT - 1 - y) as f64;
            let draw_x = x as f64;

            if v > 0.6 {
                layer3.push((draw_x, draw_y));
            } else if v > 0.4 {
                layer2.push((draw_x, draw_y));
            } else if v > 0.2 {
                layer1.push((draw_x, draw_y));
            }
        }
    }

    ctx.draw(&Points {
        coords: &layer1,
        color: Color::DarkGray,
    });
    ctx.draw(&Points {
        coords: &layer2,
        color: Color::Gray,
    });
    ctx.draw(&Points {
        coords: &layer3,
        color: Color::White,
    });
}

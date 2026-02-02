use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub mod model;
use model::Leaf;

struct App {
    leaf: Leaf,
    running: bool,
    last_tick: Instant,
    tick_rate: Duration,
    paused: bool,
}

impl App {
    fn new() -> Self {
        let mut leaf = Leaf::new();
        // Leaf coordinates: 0..100, 0..100.
        // We'll set detection radius and kill radius appropriate for this scale.
        leaf.detection_radius = 10.0;
        leaf.kill_radius = 2.0;
        leaf.growth_distance = 1.0;

        leaf.seed_random(100.0, 100.0, 800);
        leaf.init_vein(50.0, 5.0); // Start at bottom middle

        Self {
            leaf,
            running: true,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(50), // 20 TPS growth
            paused: false,
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.leaf.grow();
        }
    }

    fn reset(&mut self) {
        let mut leaf = Leaf::new();
        leaf.detection_radius = 10.0;
        leaf.kill_radius = 2.0;
        leaf.growth_distance = 1.0;

        leaf.seed_random(100.0, 100.0, 800);
        leaf.init_vein(50.0, 5.0);

        self.leaf = leaf;
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    while app.running {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = app.tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.running = false,
                        KeyCode::Char('r') => app.reset(),
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        _ => {}
                    }
                }
            }
        }

        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            app.last_tick = Instant::now();
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Venation Vines 🌿"))
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Draw Attractors
            for attractor in &app.leaf.attractors {
                if attractor.active {
                    ctx.print(attractor.pos.x as f64, attractor.pos.y as f64, ".");
                }
            }

            // Draw Veins
            for vein in &app.leaf.veins {
                if let Some(parent_idx) = vein.parent_idx {
                    let parent = &app.leaf.veins[parent_idx];

                    let color = if vein.thickness < 1.0 {
                        Color::Rgb(50, 100, 50)
                    } else if vein.thickness < 2.0 {
                        Color::Rgb(100, 150, 100)
                    } else if vein.thickness < 3.5 {
                        Color::Rgb(150, 200, 150)
                    } else {
                        Color::Rgb(220, 255, 220)
                    };

                    ctx.draw(&CanvasLine {
                        x1: parent.pos.x as f64,
                        y1: parent.pos.y as f64,
                        x2: vein.pos.x as f64,
                        y2: vein.pos.y as f64,
                        color,
                    });
                }
            }
        });

    frame.render_widget(canvas, main_layout[0]);

    // Status Bar
    let status_text = format!(
        "Nodes: {} | Attractors: {} | [R] Reset | [Space] Pause/Resume | [Q] Quit",
        app.leaf.veins.len(),
        app.leaf.attractors.iter().filter(|a| a.active).count()
    );
    frame.render_widget(
        Paragraph::new(status_text).style(Style::default().fg(Color::Black).bg(Color::White)),
        main_layout[1]
    );
}

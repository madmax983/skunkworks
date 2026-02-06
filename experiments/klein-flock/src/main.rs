mod topology;
mod renderer;
mod boid;

use std::time::{Duration, Instant};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use glam::Vec3;
use ratatui::{
    prelude::*,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};

use renderer::{draw_wireframe, draw_boids, Camera};
use boid::Boid;

struct App {
    boids: Vec<Boid>,
    camera_angle: f32,
    camera_height: f32,
    camera_radius: f32,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        let mut boids = Vec::new();
        for _ in 0..100 {
            boids.push(Boid::new());
        }

        Self {
            boids,
            camera_angle: 0.0,
            camera_height: 1.0,
            camera_radius: 7.0,
            should_quit: false,
        }
    }

    fn on_tick(&mut self) {
        // Clone for reading neighbors state
        let current_state = self.boids.clone();

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.update(&current_state, i);
        }

        // Auto-rotate camera slowly
        self.camera_angle += 0.01;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    std::io::stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
    let tick_rate = Duration::from_millis(30);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => app.should_quit = true,
                    KeyCode::Left => app.camera_angle -= 0.1,
                    KeyCode::Right => app.camera_angle += 0.1,
                    KeyCode::Char('w') => app.camera_height += 0.5,
                    KeyCode::Char('s') => app.camera_height -= 0.5,
                    KeyCode::Char('a') => app.camera_radius += 0.5,
                    KeyCode::Char('d') => app.camera_radius -= 0.5,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    std::io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // 3D Canvas
    let camera_pos = Vec3::new(
        app.camera_radius * app.camera_angle.cos(),
        app.camera_height,
        app.camera_radius * app.camera_angle.sin(),
    );
    let camera = Camera::new(camera_pos, Vec3::ZERO);

    let canvas_area = chunks[0];
    let width = canvas_area.width as f32;
    let height = canvas_area.height as f32;
    // Aspect ratio correction (2:1 char ratio approx)
    let aspect = (width * 0.5) / height;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Klein Flock - Boids on a Non-Orientable Manifold"),
        )
        .x_bounds([-2.0, 2.0])
        .y_bounds([-1.5, 1.5])
        .paint(move |ctx| {
            draw_wireframe(ctx, &camera, aspect);
            draw_boids(ctx, &camera, aspect, &app.boids);
        });

    f.render_widget(canvas, chunks[0]);

    // Info Bar
    let info_text = vec![
        Line::from(vec![
            Span::styled("Klein Flock", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" | Arrows/WASD: Move Camera | Q: Quit"),
        ]),
        Line::from(vec![
             Span::raw("Observations: Boids wrap X axis with a Twist (Y flip)."),
        ]),
    ];

    let info = Paragraph::new(info_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Status"),
    );
    f.render_widget(info, chunks[1]);
}

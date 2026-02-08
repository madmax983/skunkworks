pub mod audio;
pub mod physics;
pub mod render;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use glam::Vec3;
use physics::CosmicString;
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{
        canvas::Canvas,
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io::{self, Stdout},
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app
    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    // Initialize Simulation
    let start = Vec3::new(-20.0, 0.0, 0.0);
    let end = Vec3::new(20.0, 0.0, 0.0);
    let mut string = CosmicString::new(start, end, 40, 50.0, 0.5); // 40 segments, Tension 50, Damping 0.5

    let mut camera = render::Camera::new();
    camera.zoom = 2.0;
    camera.azimuth = 0.5;
    camera.elevation = 0.5;

    let mut audio_sys = audio::AudioSystem::new()?;

    let mut last_tick = Instant::now();
    loop {
        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f32();
        if dt >= 0.016 {
            string.update(dt.min(0.05)); // Cap dt to avoid explosion
            audio_sys.update(&string);
            last_tick = now;
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Render Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Cosmic String Simulation"))
                .x_bounds([-40.0, 40.0])
                .y_bounds([-20.0, 20.0])
                .paint(|ctx| {
                    render::draw_cosmic_string(ctx, &string, &camera);
                });
            f.render_widget(canvas, chunks[0]);

            // Render Info
            let info_text = format!(
                "Tension: {:.1} | Damping: {:.2} | KE: {:.2} | Cam: Az {:.2} El {:.2}\nControls: Arrows (Cam), +/- (Tension), [Space] Pluck, [R] Reset, [Q] Quit",
                string.tension,
                string.damping,
                string.total_kinetic_energy(),
                camera.azimuth,
                camera.elevation
            );
            let info = Paragraph::new(info_text)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        // Handle Input
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Left => camera.azimuth -= 0.1,
                        KeyCode::Right => camera.azimuth += 0.1,
                        KeyCode::Up => camera.elevation += 0.1,
                        KeyCode::Down => camera.elevation -= 0.1,
                        KeyCode::Char('+') | KeyCode::Char('=') => string.tension += 5.0,
                        KeyCode::Char('-') | KeyCode::Char('_') => string.tension = (string.tension - 5.0).max(1.0),
                        KeyCode::Char(' ') => {
                            // Pluck random node
                            let mut rng = rand::thread_rng();
                            let idx = rng.gen_range(1..string.nodes.len() - 1);
                            let force = Vec3::new(0.0, rng.gen_range(-10.0..10.0), rng.gen_range(-10.0..10.0));
                            string.pluck(idx, force);
                        }
                        KeyCode::Char('r') => {
                             // Reset
                             let start = Vec3::new(-20.0, 0.0, 0.0);
                             let end = Vec3::new(20.0, 0.0, 0.0);
                             let tension = string.tension;
                             let damping = string.damping;
                             string = CosmicString::new(start, end, 40, tension, damping);
                        }
                        KeyCode::Char('1') => {
                            // Fundamental mode pluck
                             for i in 1..string.nodes.len()-1 {
                                 let x = i as f32 / string.nodes.len() as f32;
                                 let disp = (x * std::f32::consts::PI).sin() * 5.0;
                                 string.pluck(i, Vec3::new(0.0, disp, 0.0));
                             }
                        }
                        KeyCode::Char('2') => {
                            // 2nd Harmonic
                             for i in 1..string.nodes.len()-1 {
                                 let x = i as f32 / string.nodes.len() as f32;
                                 let disp = (x * 2.0 * std::f32::consts::PI).sin() * 5.0;
                                 string.pluck(i, Vec3::new(0.0, disp, 0.0));
                             }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

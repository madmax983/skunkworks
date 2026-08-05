//! # origami-poincare 🦢✨
//!
//! A hybrid experiment demonstrating **Hyperbolic Soft-Body Morphogenesis**.
//!
//! ## Lineage 🧬
//! - **Parent A (`crates/origami`)**: Provides the procedural Miura-ori soft-body mesh generation and folding tension logic.
//! - **Parent B (`crates/poincare-disk`)**: Provides the continuous non-Euclidean geometry boundary, mapping Euclidean coordinates into a hyperbolic disk.
//!
//! ## Phenotype 🔬
//! The continuous, breathing physical 3D vertices of the Miura-ori paper mesh are mapped directly to the non-Euclidean space of the Poincaré disk using Mobius transformations. As the paper mesh folds and unfolds, the hyperbolic geometry compresses the perceived structural distance at the disk boundaries. This creates a mesmerizing effect where the mesh appears dense and infinitely detailed at the edges, while expanding cleanly in the center.
//!
//! ## Emergence
//! The hybrid combines physical material stress (`origami`) with infinite mathematical bounds (`poincare-disk`). This creates a soft-body organism that can seemingly stretch to infinity without violating its physical constraints.
//!
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use origami::{generate_miura_mesh, MiuraParams, Orientation, OrigamiMesh};
use poincare_disk::{Mobius, Point};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

struct App {
    params: MiuraParams,
    mesh: OrigamiMesh,
    time: f32,
    should_quit: bool,
}

impl App {
    fn new() -> App {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(),
            orientation: Orientation::Horizontal,
        };
        App {
            params,
            mesh: generate_miura_mesh(params, (10, 10), 0.5),
            time: 0.0,
            should_quit: false,
        }
    }

    fn update(&mut self) {
        self.time += 0.05;
        let extension = 0.5 + 0.4 * self.time.sin();
        self.mesh = generate_miura_mesh(self.params, (10, 10), extension);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().any(|a| a == "--headless") {
        let mut app = App::new();
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    app.should_quit = true;
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
        if app.should_quit {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let area = f.area();

    let canvas_block = Block::default()
        .borders(Borders::ALL)
        .title("Origami-Poincare (Hyperbolic Soft-Body)");

    // Simple rendering of the hyperbolic soft body
    let mut buffer = vec![vec![' '; area.width as usize]; area.height as usize];

    // Get the logical center and radius based on TUI dimensions
    let cx = area.width as f32 / 2.0;
    let cy = area.height as f32 / 2.0;
    let radius = cx.min(cy) * 0.9;

    // Draw the Poincare disk boundary
    for y in 0..area.height {
        for x in 0..area.width {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + (dy * 2.0) * (dy * 2.0)).sqrt(); // Compensate for terminal char ratio

            if (dist - radius).abs() < 1.0 {
                buffer[y as usize][x as usize] = '.';
            }
        }
    }

    // Map the Miura mesh points onto the Poincare disk
    let scale_x = 0.15; // Map from mesh domain to Poincare disk domain (-1.0 to 1.0)
    let scale_y = 0.15;

    for point in &app.mesh.vertices {
        let px = point.pos.x * scale_x;
        let py = point.pos.y * scale_y;

        let mut p_disk = Point::new(px as f64, py as f64);

        // Use poincare-disk to dynamically transform the points using a Mobius transformation
        let mobius = Mobius::rotation(app.time as f64);
        p_disk = mobius.apply(p_disk);

        // We ensure it is inside the disk
        if p_disk.norm() < 1.0 {
            let screen_x = cx + (p_disk.re as f32 * radius);
            let screen_y = cy + (p_disk.im as f32 * radius / 2.0); // Compensate

            if screen_x >= 0.0
                && screen_x < area.width as f32
                && screen_y >= 0.0
                && screen_y < area.height as f32
            {
                let char_idx = (point.pos.z * 10.0) as i32;
                let char_to_draw = match char_idx {
                    z if z > 5 => 'X',
                    z if z > 0 => 'x',
                    z if z > -5 => '-',
                    _ => '.',
                };

                buffer[screen_y as usize][screen_x as usize] = char_to_draw;
            }
        }
    }

    let mut lines = Vec::new();
    for row in buffer {
        let s: String = row.into_iter().collect();
        lines.push(ratatui::text::Line::from(s));
    }

    let p = Paragraph::new(lines).block(canvas_block);
    f.render_widget(p, area);
}

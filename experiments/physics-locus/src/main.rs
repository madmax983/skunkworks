use locus::Topology;
use physics_pbd::PbdSystem;
use glam::Vec3;
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::env;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

fn map_to_topology(p: Vec3, width: f64, height: f64, topo: Topology) -> Vec3 {
    let x_idx = p.x.round() as i64;
    let y_idx = p.y.round() as i64;

    if let Some((ny, nx)) = topo.normalize(y_idx, x_idx, width as usize, height as usize) {
        Vec3::new(nx as f32, ny as f32, p.z)
    } else {
        p // Outside bounds on a bounded topology
    }
}

struct App {
    system: PbdSystem,
    topology: Topology,
    width: f64,
    height: f64,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        let mut system = PbdSystem::new();

        // Add a few particles
        for i in 0..10 {
            system.add_particle(Vec3::new(width as f32 / 2.0, height as f32 / 4.0 + (i as f32 * 2.0), 0.0), 1.0);

            // push them down-right to trigger continuous wrapping
            system.particles[i].vel = Vec3::new(20.0, 5.0, 0.0);
        }

        // Connect them as a chain
        for i in 0..9 {
            let _ = system.add_distance_constraint(i, i+1, 2.0);
        }

        Self {
            system,
            topology: Topology::Klein,
            width,
            height,
        }
    }

    fn update(&mut self) {
        // Enforce topological wrap on particles *before* solver
        for p in self.system.particles.iter_mut() {
            p.pos = map_to_topology(p.pos, self.width, self.height, self.topology);
        }

        // Step physics (PbdSystem::step doesn't have config struct in lib.rs, it takes dt and iterations)
        self.system.step(1.0/60.0, 10);

        // Enforce topological wrap on particles *after* solver
        for p in self.system.particles.iter_mut() {
            p.pos = map_to_topology(p.pos, self.width, self.height, self.topology);
        }
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(" Topological Soft-Body Physics 🧬 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            for y in 0..inner_area.height {
                let mut spans = Vec::new();
                for x in 0..inner_area.width {
                    let mut ch = " ";
                    let mut color = Color::Reset;

                    let p_x = x as f32;
                    let p_y = y as f32;

                    for p in &self.system.particles {
                        if (p.pos.x - p_x).abs() < 1.0 && (p.pos.y - p_y).abs() < 1.0 {
                            ch = "●";
                            color = Color::Green;
                        }
                    }

                    spans.push(Span::styled(ch, Style::default().fg(color)));
                }
                lines.push(Line::from(spans));
            }

            f.render_widget(Paragraph::new(lines), inner_area);
        })?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--headless".to_string()) {
        let mut app = App::new(60.0, 30.0);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    let mut app = App::new(60.0, 30.0);
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);
    let mut running = true;

    while running {
        if event::poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    running = false;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            app.render(&mut terminal)?;
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

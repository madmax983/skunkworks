//! # Knotted Morphogenesis (`quipu-gray`)
//!
//! A TUI visualization crossing the discrete structural knots of `quipu` with the continuous chemical reaction-diffusion substrate of `gray-scott`.
//!
//! ## Concept
//! The discrete knots of a Quipu cord act as continuous chemical catalysts within a 2D Gray-Scott reaction-diffusion grid. The knotted data values inject the "V" (kill) chemical, sparking localized Turing patterns exactly where data exists.
//!
//! ## Emergent Phenotype
//! An emergent biological organism seeded entirely by data integers. The distinct knots form glowing chemical "spores" that blossom into intricate, spreading Turing patterns, bridging discrete physical data storage with continuous organic growth.
//!
//! ## Commands
//! * `cargo run -p quipu-gray`: Runs the TUI visualization.
//! * `cargo run -p quipu-gray -- --headless`: Runs headless tests for CI environments.
//!
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use quipu::{Cord, Quipu};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::env;
use std::io::{self, stdout};
use std::time::{Duration, Instant};

struct App {
    sim: GrayScott,
    sim_width: usize,
    sim_height: usize,
    quipu: Quipu,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let sim = GrayScott::new(width, height);

        let mut quipu = Quipu::new();
        quipu.add_cord(Cord::from(420));
        quipu.add_cord(Cord::from(77));
        quipu.add_cord(Cord::from(108));
        quipu.add_cord(Cord::from(999));

        Self {
            sim,
            sim_width: width,
            sim_height: height,
            quipu,
        }
    }

    fn inject_knots(&mut self) {
        // Map knots to chemical injection points
        let cord_count = self.quipu.cords.len();
        if cord_count == 0 {
            return;
        }

        let spacing_x = self.sim_width / (cord_count + 1);

        for (i, cord) in self.quipu.cords.iter().enumerate() {
            let x = spacing_x * (i + 1);

            // Map clusters down the cord
            let cluster_count = cord.clusters.len();
            if cluster_count == 0 {
                continue;
            }
            let spacing_y = self.sim_height / (cluster_count + 1);

            for (j, cluster) in cord.clusters.iter().enumerate() {
                let y = spacing_y * (j + 1);

                // Inject kill chemical based on knot count
                let knot_count = cluster.len();
                if knot_count > 0 {
                    let injection_amount = (knot_count as f32) * 0.2;

                    // Inject chemical in a small area
                    for dy in 0..3 {
                        for dx in 0..3 {
                            let nx = x as i32 + dx - 1;
                            let ny = y as i32 + dy - 1;
                            if nx >= 0
                                && nx < self.sim_width as i32
                                && ny >= 0
                                && ny < self.sim_height as i32
                            {
                                self.sim
                                    .add_chemical(nx as usize, ny as usize, injection_amount);
                            }
                        }
                    }
                }
            }
        }
    }

    fn update(&mut self) {
        // Standard "Spots" parameters for Gray-Scott
        self.sim.update(0.055, 0.062, 1.0);
        // Continuously inject a little bit of the knots to keep them alive
        self.inject_knots();
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Knotted Morphogenesis 🧬 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();

            for y in 0..inner_area.height {
                let mut spans = Vec::new();
                for x in 0..inner_area.width {
                    // Map screen coordinates to simulation grid
                    let sim_x =
                        (x as f32 / inner_area.width as f32 * self.sim_width as f32) as usize;
                    let sim_y =
                        (y as f32 / inner_area.height as f32 * self.sim_height as f32) as usize;

                    let sim_x = sim_x.min(self.sim_width - 1);
                    let sim_y = sim_y.min(self.sim_height - 1);

                    let idx = sim_y * self.sim_width + sim_x;
                    let v = self.sim.v()[idx];

                    let mut ch = " ";
                    let mut color = Color::Reset;

                    if v > 0.3 {
                        ch = "█";
                        color = Color::Magenta;
                    } else if v > 0.1 {
                        ch = "▓";
                        color = Color::Blue;
                    } else if v > 0.05 {
                        ch = "▒";
                        color = Color::DarkGray;
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
        let mut app = App::new(100, 100);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    let mut app = App::new(100, 100);
    // Initial injection
    app.inject_knots();

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

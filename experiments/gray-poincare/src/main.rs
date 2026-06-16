use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use gray_scott::GrayScott;
use poincare_disk::Point;
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
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        let mut sim = GrayScott::new(width, height);
        // Seed the center
        for dy in 0..5 {
            for dx in 0..5 {
                sim.add_chemical(width / 2 - 2 + dx, height / 2 - 2 + dy, 1.0);
            }
        }
        Self {
            sim,
            sim_width: width,
            sim_height: height,
        }
    }

    fn update(&mut self) {
        // Standard "Spots" parameters
        self.sim.update(0.055, 0.062, 1.0);
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.area();
            let block = Block::default()
                .title(" Hyperbolic Reaction-Diffusion 🧬 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();
            let screen_w = inner_area.width as f64;
            let screen_h = inner_area.height as f64;
            let aspect = screen_w / screen_h; // Screen aspect ratio
            let aspect_char = 2.0; // Terminal chars are approx twice as tall as they are wide

            // We iterate over the screen space and sample from the Poincare disk
            for y in 0..inner_area.height {
                let mut spans = Vec::new();
                for x in 0..inner_area.width {
                    // Normalize screen coordinates to [-1, 1]
                    let nx = (x as f64 / screen_w) * 2.0 - 1.0;
                    let ny = (y as f64 / screen_h) * 2.0 - 1.0;

                    // Adjust for terminal character aspect ratio
                    let px = nx;
                    let py = ny * aspect_char / aspect;

                    let p = Point::new(px, py);

                    if p.norm() >= 1.0 {
                        // Outside the disk
                        spans.push(Span::styled(" ", Style::default()));
                        continue;
                    }

                    // For each point on the screen (in the Poincare disk), we want to map it to our Gray-Scott grid.
                    // We'll map the disk's complex plane to the Gray-Scott grid.
                    // The origin (0,0) maps to the center of the grid.
                    // To make it interesting, we apply a Mobius transformation just for the visual mapping.

                    // Let's create a visual transformation that breathes based on some time?
                    // Or we just map the coordinates directly to show how it distorts the grid.
                    let center_x = self.sim_width as f64 / 2.0;
                    let center_y = self.sim_height as f64 / 2.0;
                    let scale = center_x.min(center_y);

                    let gx = (p.re * scale + center_x).round() as i64;
                    let gy = (p.im * scale + center_y).round() as i64;

                    let mut ch = " ";
                    let mut color = Color::Reset;

                    if gx >= 0
                        && gx < self.sim_width as i64
                        && gy >= 0
                        && gy < self.sim_height as i64
                    {
                        let idx = (gy as usize) * self.sim_width + (gx as usize);
                        let v = self.sim.v()[idx];
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
        let mut app = App::new(40, 40);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    let mut app = App::new(40, 40);
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

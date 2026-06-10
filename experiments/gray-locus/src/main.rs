use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use locus::Topology;
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

struct GrayLocusSystem {
    width: usize,
    height: usize,
    u: Vec<f32>,
    v: Vec<f32>,
    next_u: Vec<f32>,
    next_v: Vec<f32>,
    pub diff_u: f32,
    pub diff_v: f32,
    pub feed: f32,
    pub kill: f32,
    topology: Topology,
}

impl GrayLocusSystem {
    fn new(width: usize, height: usize, topology: Topology) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![1.0; size],
            v: vec![0.0; size],
            next_u: vec![1.0; size],
            next_v: vec![0.0; size],
            diff_u: 1.0,
            diff_v: 0.5,
            feed: 0.055,
            kill: 0.062,
            topology,
        }
    }

    fn add_chemical(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.v[idx] = (self.v[idx] + amount).clamp(0.0, 1.0);
        }
    }

    fn update(&mut self, dt: f32) {
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let cur_u = self.u[idx];
                let cur_v = self.v[idx];

                let mut sum_u = -cur_u;
                let mut sum_v = -cur_v;

                // 3x3 Convolution using Topology
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = x as i64 + dx;
                        let ny = y as i64 + dy;

                        if let Some((ty, tx)) = self.topology.normalize(ny, nx, w, h) {
                            let tidx = ty * w + tx;
                            let weight = if dx == 0 && dy == 0 {
                                -1.0
                            } else if dx == 0 || dy == 0 {
                                0.2
                            } else {
                                0.05
                            };

                            // Do not double count center
                            if dx != 0 || dy != 0 {
                                sum_u += self.u[tidx] * weight;
                                sum_v += self.v[tidx] * weight;
                            }
                        }
                    }
                }

                let reaction = cur_u * cur_v * cur_v;

                let du = self.diff_u * sum_u - reaction + self.feed * (1.0 - cur_u);
                let dv = self.diff_v * sum_v + reaction - (self.feed + self.kill) * cur_v;

                self.next_u[idx] = (cur_u + du * dt).clamp(0.0, 1.0);
                self.next_v[idx] = (cur_v + dv * dt).clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }
}

struct App {
    system: GrayLocusSystem,
}

impl App {
    fn new(width: usize, height: usize, topology: Topology) -> Self {
        let mut system = GrayLocusSystem::new(width, height, topology);
        // Seed center
        for x in (width / 2 - 5)..(width / 2 + 5) {
            for y in (height / 2 - 5)..(height / 2 + 5) {
                system.add_chemical(x, y, 1.0);
            }
        }
        // Seed an edge to test wrapping
        for y in 0..5 {
            system.add_chemical(0, y, 1.0);
        }

        Self { system }
    }

    fn update(&mut self) {
        for _ in 0..5 {
            self.system.update(1.0);
        }
    }

    fn render(&self, terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default()
                .title(" Topological Reaction-Diffusion 🍄📍 ")
                .borders(Borders::ALL);

            let inner_area = block.inner(size);
            f.render_widget(block, size);

            let mut lines = Vec::new();
            let w = self.system.width;
            let h = self.system.height;

            for y in 0..inner_area.height.min(h as u16) {
                let mut spans = Vec::new();
                for x in 0..inner_area.width.min(w as u16) {
                    let idx = (y as usize) * w + (x as usize);
                    let v = self.system.v[idx];

                    let (ch, color) = if v > 0.8 {
                        ("█", Color::White)
                    } else if v > 0.5 {
                        ("▓", Color::Yellow)
                    } else if v > 0.2 {
                        ("▒", Color::DarkGray)
                    } else if v > 0.05 {
                        ("░", Color::Gray)
                    } else {
                        (" ", Color::Reset)
                    };

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
        let mut app = App::new(60, 30, Topology::Klein);
        for _ in 0..10 {
            app.update();
        }
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    execute!(terminal.backend_mut(), EnterAlternateScreen)?;

    let mut app = App::new(60, 30, Topology::Klein);
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

use std::env;
use std::path::Path;
use std::time::Duration;

use anyhow::Result;
use tui_shared::crossterm::event::{self, Event, KeyCode};
use tui_shared::ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};

use market_sim::{Grid, Particle};
use miller_lattice::Crystal;
use tui_shared::Tui;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    // Fallback path
    let target_path = if args.len() > 1 && !args[1].starts_with("--") {
        args[1].clone()
    } else {
        ".".to_string()
    };

    let mut app = App::new(&target_path)?;

    if headless {
        // Run a few frames headless
        for _ in 0..10 {
            app.tick();
        }
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal, &mut app);
    tui.exit()?;

    res
}

struct App {
    crystal: Crystal,
    market: Grid,
    width: usize,
    height: usize,
    tick_count: usize,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let crystal = Crystal::build_from_path(Path::new(path))?;

        let width = 80;
        let height = 40;
        let mut market = Grid::new(width, height);

        // Seed market based on crystal properties
        let mut idx = 1;
        for atom in &crystal.atoms {
            // Map 3D coordinate to 2D
            let x = (atom.position.x.unsigned_abs() as usize % (width - 4)) + 2;
            let y = (atom.position.y.unsigned_abs() as usize % (height / 2)) + 2;

            // Directories inject bids, files inject asks
            if atom.is_dir {
                market.set(x, height - y - 1, Particle::Bid(idx));
            } else {
                market.set(x, y, Particle::Ask(idx));
            }
            idx += 1;
        }

        Ok(Self {
            crystal,
            market,
            width,
            height,
            tick_count: 0,
        })
    }

    fn tick(&mut self) {
        self.market.update();
        self.tick_count += 1;

        // Pseudo-random injection to keep market alive without external rand dependency
        if self.tick_count % 10 == 0 {
            let mut idx = 1;
            for atom in &self.crystal.atoms {
                // Use pseudo-random logic based on tick and atom position
                if (self.tick_count + idx) % 20 == 0 {
                    let x = (atom.position.x.unsigned_abs() as usize % (self.width - 4)) + 2;
                    let y = (atom.position.y.unsigned_abs() as usize % (self.height / 2)) + 2;

                    if atom.is_dir {
                        if matches!(self.market.get(x, self.height - y - 1), Particle::Empty) {
                            self.market.set(x, self.height - y - 1, Particle::Bid(idx));
                        }
                    } else {
                        if matches!(self.market.get(x, y), Particle::Empty) {
                            self.market.set(x, y, Particle::Ask(idx));
                        }
                    }
                }
                idx += 1;
            }
        }
    }
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Render Market
            let mut text = Vec::new();
            for y in 0..app.height {
                let mut line = Vec::new();
                for x in 0..app.width {
                    let cell = app.market.get(x, y);
                    let (symbol, color) = match cell {
                        Particle::Empty => (" ", Color::Reset),
                        Particle::Wall => ("█", Color::DarkGray),
                        Particle::Bid(_) => ("▲", Color::Green),
                        Particle::Ask(_) => ("▼", Color::Red),
                        Particle::Trade { .. } => ("*", Color::Yellow),
                    };
                    line.push(Span::styled(symbol, Style::default().fg(color)));
                }
                text.push(Line::from(line));
            }

            let market_block = Paragraph::new(text).block(
                Block::default()
                    .title("Miller Market (Dir = Bid, File = Ask)")
                    .borders(Borders::ALL),
            );
            f.render_widget(market_block, chunks[0]);

            // For stats, we don't have direct access to total_bids() etc.
            // But we can approximate or read them if we implemented them.
            // Since they are private/missing, let's just show ticks and nodes.
            let stats = Paragraph::new(format!(
                "Nodes: {} | Ticks: {}",
                app.crystal.atoms.len(),
                app.tick_count
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    return Ok(());
                }
            }
        }

        app.tick();
    }
}

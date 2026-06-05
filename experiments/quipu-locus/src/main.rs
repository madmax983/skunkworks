//! Topological Data Knots
//!
//! This experiment is a hybrid crossing `quipu` and `locus`.
//! Knots on a quipu cord are mapped onto a 2D topological boundary.
//! As data is added to the cord, the knots spatially distribute and wrap seamlessly
//! across the chosen geometry, demonstrating non-Euclidean distribution of encoded integer states.

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use locus::Topology;
use quipu::{Cord, Quipu};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::env;
use std::{io, time::Duration};

pub struct QuipuLocusHybrid {
    quipu: Quipu,
    topology: Topology,
    grid_width: usize,
    grid_height: usize,
}

impl QuipuLocusHybrid {
    pub fn new(width: usize, height: usize, topology: Topology) -> Self {
        Self {
            quipu: Quipu::new(),
            topology,
            grid_width: width,
            grid_height: height,
        }
    }

    pub fn add_data(&mut self, value: u64) {
        self.quipu.add_cord(Cord::from(value));
    }

    pub fn render_grid(&self) -> Vec<Vec<Color>> {
        let mut grid = vec![vec![Color::Black; self.grid_width]; self.grid_height];

        // Map knots to space
        let mut offset_y = 0;
        for cord in &self.quipu.cords {
            let val = cord.value();
            let x_pos = (val % self.grid_width as u64) as f32;
            let y_pos = (offset_y as f32) + (val / self.grid_width as u64) as f32;

            // Animate or wrap using locus topology
            let y_idx = y_pos.round() as i64;
            let x_idx = x_pos.round() as i64;

            if let Some((ny, nx)) =
                self.topology
                    .normalize(y_idx, x_idx, self.grid_width, self.grid_height)
            {
                let heat_color = match val {
                    v if v > 1000 => Color::Red,
                    v if v > 100 => Color::Yellow,
                    v if v > 10 => Color::Green,
                    _ => Color::Cyan,
                };
                grid[ny][nx] = heat_color;
            }

            offset_y += 2;
        }

        grid
    }
}

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let headless = args.contains(&"--headless".to_string());

    if headless {
        let mut hybrid = QuipuLocusHybrid::new(50, 20, Topology::Torus);
        hybrid.add_data(42);
        hybrid.add_data(1024);
        hybrid.add_data(9999);
        let _grid = hybrid.render_grid();
        println!("Headless execution completed successfully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut hybrid = QuipuLocusHybrid::new(50, 20, Topology::Klein);

    let mut value_to_add = 0;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(f.area());

            let instructions = Paragraph::new("Topological Data Knots: Press Space to Add Data, Esc to quit.")
                .block(Block::default().borders(Borders::ALL).title("Quipu-Locus"));
            f.render_widget(instructions, chunks[0]);

            let grid = hybrid.render_grid();
            let mut lines = Vec::new();
            for row in grid {
                let mut spans = Vec::new();
                for cell in row {
                    let s = match cell {
                        Color::Black => Span::styled(" . ", Style::default().fg(Color::DarkGray)),
                        c => Span::styled(" O ", Style::default().fg(c)),
                    };
                    spans.push(s);
                }
                lines.push(ratatui::text::Line::from(spans));
            }
            let p = Paragraph::new(lines).block(Block::default().borders(Borders::ALL));
            f.render_widget(p, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        value_to_add += rand::random::<u64>() % 2000;
                        hybrid.add_data(value_to_add);
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

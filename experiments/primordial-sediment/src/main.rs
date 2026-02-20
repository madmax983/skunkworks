mod biology;
mod git;
mod physics;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use git::RepoHandler;
use physics::{FluidSolver, Species};

struct App {
    text_buffer: Vec<Vec<char>>,
    fluid: FluidSolver,
}

impl App {
    fn new() -> Result<Self> {
        // Load initial content
        let repo = RepoHandler::new(".")?;
        // Try to load own source code or README
        let commits = repo.list_commits(1)?;
        let content = if !commits.is_empty() {
            repo.get_file_content(
                &commits[0].id,
                "experiments/primordial-sediment/src/main.rs",
            )
            .unwrap_or_else(|_| include_str!("main.rs").to_string())
        } else {
            include_str!("main.rs").to_string()
        };

        let text_buffer: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();

        let width = 100.0; // Simulation width
        let height = text_buffer.len() as f32; // Simulation height based on lines

        let mut fluid = FluidSolver::new(width, height);

        // Seed Life
        let mut rng = rand::thread_rng();

        // Algae (Detritivores)
        for _ in 0..50 {
            fluid.add_particle(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                Species::Algae,
            );
        }

        // Grazers
        for _ in 0..10 {
            fluid.add_particle(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                Species::Grazer,
            );
        }

        // Predators
        for _ in 0..2 {
            fluid.add_particle(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                Species::Predator,
            );
        }

        Ok(Self { text_buffer, fluid })
    }

    fn update(&mut self) {
        let mut rng = rand::thread_rng();

        // 1. Natural Entropy (Bit Rot)
        // Occasional random bit flip or decay
        if rng.gen_bool(0.1) {
            // 10% chance per tick
            if !self.text_buffer.is_empty() {
                let row = rng.gen_range(0..self.text_buffer.len());
                if !self.text_buffer[row].is_empty() {
                    let col = rng.gen_range(0..self.text_buffer[row].len());
                    let c = self.text_buffer[row][col];
                    if !c.is_whitespace() {
                        // Decay to block or glitch
                        self.text_buffer[row][col] = if rng.gen_bool(0.5) { '░' } else { '▒' };
                    }
                }
            }
        }

        // 2. Physics
        self.fluid.update(0.5);

        // 3. Biology (Interaction)
        self.fluid.update_biology(&mut self.text_buffer);
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let app_result = App::new();

    if let Err(e) = app_result {
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        eprintln!("Error: {}", e);
        return Err(e);
    }

    let mut app = app_result.unwrap();

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
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)].as_ref())
        .split(f.size());

    // Create Display Buffer (Clone text and overlay particles)
    // We only render what fits on screen to avoid huge allocations if file is big
    // But Render is per frame.
    // Let's just create lines for the visible area or just the whole buffer if small.

    // Convert text_buffer to Lines
    // We iterate the buffer. If a particle is at that location, we use its char/color.

    // 1. Build a map of particle positions for O(1) lookup
    // (row, col) -> Particle
    // Since multiple particles might be at same spot, we prioritize Predator > Grazer > Algae
    use std::collections::HashMap;
    let mut particle_map = HashMap::new();

    for p in &app.fluid.particles {
        let r = p.y.round() as usize;
        let c = p.x.round() as usize;
        // Simple overwrite priority
        particle_map
            .entry((r, c))
            .and_modify(|e: &mut Species| {
                if *e == Species::Algae && p.species != Species::Algae {
                    *e = p.species;
                } else if *e == Species::Grazer && p.species == Species::Predator {
                    *e = p.species;
                }
            })
            .or_insert(p.species);
    }

    let mut lines = Vec::new();
    for (r, line_chars) in app.text_buffer.iter().enumerate() {
        let mut spans = Vec::new();
        // We iterate up to width of screen? Or just the line length.
        // We should pad lines if particles are beyond line length?
        // For simplicity, only render particles within line length (or just extends slightly).

        let len = line_chars.len().max(100); // minimal width

        for c_idx in 0..len {
            let char_at = if c_idx < line_chars.len() {
                line_chars[c_idx]
            } else {
                ' '
            };

            if let Some(species) = particle_map.get(&(r, c_idx)) {
                let (symbol, color) = match species {
                    Species::Algae => ('♣', Color::Green),
                    Species::Grazer => ('●', Color::Cyan),
                    Species::Predator => ('▲', Color::Red),
                };
                spans.push(Span::styled(
                    symbol.to_string(),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                ));
            } else {
                // Decay color?
                let color = if char_at == '░' || char_at == '▒' {
                    Color::DarkGray
                } else {
                    Color::Gray
                };
                spans.push(Span::styled(
                    char_at.to_string(),
                    Style::default().fg(color),
                ));
            }
        }
        lines.push(Line::from(spans));
    }

    let p = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Primordial Sediment: Life in the Code "),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(p, chunks[0]);

    // Status Bar
    let stats = format!(
        "Particles: {} | Algae: {} | Grazer: {} | Predator: {} | q: Quit",
        app.fluid.particles.len(),
        app.fluid
            .particles
            .iter()
            .filter(|p| p.species == Species::Algae)
            .count(),
        app.fluid
            .particles
            .iter()
            .filter(|p| p.species == Species::Grazer)
            .count(),
        app.fluid
            .particles
            .iter()
            .filter(|p| p.species == Species::Predator)
            .count(),
    );
    f.render_widget(
        Paragraph::new(stats).style(Style::default().bg(Color::Blue).fg(Color::White)),
        chunks[1],
    );
}

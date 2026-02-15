use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, Borders, Paragraph},
    Terminal,
    Frame,
};

mod lexer;
mod phonology;
mod evolution;

use phonology::{GrimmsLaw, HighGermanShift, GreatVowelShift, SoundChange};
use evolution::Evolver;

const SAMPLE_CODE: &str = r#"
fn calculate_entropy(data: &[u8]) -> f64 {
    let mut counts = [0usize; 256];
    let mut total = 0;

    for &byte in data {
        counts[byte as usize] += 1;
        total += 1;
    }

    let mut entropy = 0.0;
    for &count in counts.iter() {
        if count == 0 { continue; }
        let p = count as f64 / total as f64;
        entropy -= p * p.log2();
    }

    entropy
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}
"#;

struct App {
    original_code: String,
    evolved_code: String,
    era_index: usize,
    eras: Vec<Box<dyn SoundChange>>,
}

impl App {
    fn new(code: &str) -> Self {
        let mut app = Self {
            original_code: code.to_string(),
            evolved_code: code.to_string(),
            era_index: 0,
            eras: vec![
                Box::new(GrimmsLaw),
                Box::new(HighGermanShift),
                Box::new(GreatVowelShift),
            ],
        };
        // Initial evolution (identity)
        app.update_evolution();
        app
    }

    fn update_evolution(&mut self) {
        let mut current = self.original_code.clone();
        for i in 0..self.era_index {
            if i < self.eras.len() {
                // Apply changes sequentially
                current = Evolver::evolve(&current, self.eras[i].as_ref());
            }
        }
        self.evolved_code = current;
    }

    fn next_era(&mut self) {
        if self.era_index < self.eras.len() {
            self.era_index += 1;
            self.update_evolution();
        }
    }

    fn prev_era(&mut self) {
        if self.era_index > 0 {
            self.era_index -= 1;
            self.update_evolution();
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(SAMPLE_CODE);

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend<Error = io::Error>>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Right => app.next_era(),
                KeyCode::Left => app.prev_era(),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.area());

    // Title
    let title = Paragraph::new(Line::from(vec![
        Span::styled("Grimm's Code", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw(" - The Evolution of Source Code"),
    ]))
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(title, chunks[0]);

    // Body (Split)
    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let original = Paragraph::new(app.original_code.as_str())
        .block(Block::default().borders(Borders::ALL).title("Original (Proto-Code)"));

    f.render_widget(original, body_chunks[0]);

    let era_name = if app.era_index == 0 {
        "Proto-Code".to_string()
    } else {
        app.eras[app.era_index - 1].name().to_string()
    };

    let evolved_title = format!("Era {}: {}", app.era_index, era_name);

    let evolved = Paragraph::new(app.evolved_code.as_str())
        .block(Block::default().borders(Borders::ALL).title(evolved_title.as_str()));

    f.render_widget(evolved, body_chunks[1]);

    // Footer
    let help_text = Line::from(vec![
        Span::raw("Press "),
        Span::styled("Left/Right", Style::default().fg(Color::Yellow)),
        Span::raw(" to evolve/devolve. "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(" to quit."),
    ]);

    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(footer, chunks[2]);
}

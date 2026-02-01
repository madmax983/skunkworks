use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    DefaultTerminal, Frame,
};

use crate::mutator::evolve_code;
use crate::phonology::{Evolver, SoundLaw};

pub struct App {
    original_code: String,
    current_code: String,
    history: Vec<String>,
    year: usize,
    running: bool,
}

impl App {
    pub fn new(code: String) -> Self {
        Self {
            original_code: code.clone(),
            current_code: code,
            history: vec!["Origin: Proto-Code-Germanic".to_string()],
            year: 0,
            running: true,
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let tick_rate = Duration::from_millis(250);
        let mut last_tick = Instant::now();

        while self.running {
            terminal.draw(|frame| self.draw(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                            KeyCode::Char(' ') => self.advance_epoch(),
                            KeyCode::Char('r') => self.reset(),
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn advance_epoch(&mut self) {
        self.year += 100;

        // Select a law based on "era" or random
        let law = if self.year < 500 {
            SoundLaw::GrimmsLaw
        } else if self.year < 1000 {
            SoundLaw::Lenition
        } else if self.year < 1500 {
            SoundLaw::GreatVowelShift
        } else {
            // Entropy takes over
            if self.year % 200 == 0 {
                SoundLaw::LossOfEndings
            } else {
                SoundLaw::Lenition
            }
        };

        self.history
            .push(format!("Year {}: Applied {}", self.year, law.description()));

        let mut step_evolver = Evolver::new();
        step_evolver.add_law(law.clone());

        match evolve_code(&self.current_code, step_evolver) {
            Ok(new_code) => self.current_code = new_code,
            Err(e) => {
                self.history.push(format!(
                    "Year {}: The language collapsed! (Parse Error: {})",
                    self.year, e
                ));
            }
        }
    }

    fn reset(&mut self) {
        self.current_code = self.original_code.clone();
        self.year = 0;
        self.history.clear();
        self.history.push("Origin: Proto-Code-Germanic".to_string());
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Content
                Constraint::Length(3), // Footer / Log
            ])
            .split(area);

        // Header
        let title = Paragraph::new(format!("PHONETIC DECAY SIMULATOR | Year: {}", self.year))
            .block(Block::default().borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            );
        frame.render_widget(title, chunks[0]);

        // Content
        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        let original = Paragraph::new(self.original_code.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Proto-Language (Original)"),
            )
            .wrap(Wrap { trim: false });

        let evolved = Paragraph::new(self.current_code.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Modern Dialect (Evolved)"),
            )
            .style(Style::default().fg(Color::Green))
            .wrap(Wrap { trim: false });

        frame.render_widget(original, content_chunks[0]);
        frame.render_widget(evolved, content_chunks[1]);

        // Footer / Log
        let last_log = self.history.last().map(|s| s.as_str()).unwrap_or("");
        let log = Paragraph::new(last_log)
            .block(Block::default().borders(Borders::ALL).title("History Log"))
            .style(Style::default().fg(Color::Yellow));
        frame.render_widget(log, chunks[2]);
    }
}

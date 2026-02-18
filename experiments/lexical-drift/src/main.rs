use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
    widgets::{Block, Borders, Paragraph},
    layout::{Constraint, Direction, Layout},
    text::{Line, Span as TuiSpan, Text},
    style::{Style, Color, Modifier},
};
use std::{collections::HashMap, io};
use proc_macro2::Span;
use rand::{rngs::StdRng, SeedableRng};

mod phonology;
mod mutator;

use phonology::PhonologyEngine;
use mutator::{collect_identifiers, rewrite_source_segments};

struct DriftApp {
    original_source: String,
    // We store segments for rendering
    display_segments: Vec<Vec<(String, bool)>>,
    original_identifiers: HashMap<String, Vec<Span>>,
    current_mapping: HashMap<String, String>,
    phonology: PhonologyEngine,
    generation: u32,
    scroll: u16,
    rng: StdRng,
    exit: bool,
}

impl DriftApp {
    fn new(initial_source: String) -> Result<Self> {
        let locations = collect_identifiers(&initial_source)?;

        let mut mapping = HashMap::new();
        for key in locations.keys() {
            mapping.insert(key.clone(), key.clone());
        }

        // Initial segments (no changes)
        // Just treat everything as not identifier for simplicity, or we can highlight identifiers immediately.
        // Let's use rewrite_source_segments with identity mapping.
        let segments = rewrite_source_segments(&initial_source, &mapping, &locations);

        Ok(Self {
            original_source: initial_source.clone(),
            display_segments: segments,
            original_identifiers: locations,
            current_mapping: mapping,
            phonology: PhonologyEngine::new(),
            generation: 0,
            scroll: 0,
            rng: StdRng::from_entropy(),
            exit: false,
        })
    }

    fn run(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(frame.area());

        let code_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" Generation: {} ", self.generation));

        // Convert segments to Ratatui Text
        let lines: Vec<Line> = self.display_segments.iter().map(|segment_line| {
            let spans: Vec<TuiSpan> = segment_line.iter().map(|(text, is_ident)| {
                if *is_ident {
                    TuiSpan::styled(text.clone(), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else {
                    TuiSpan::raw(text.clone())
                }
            }).collect();
            Line::from(spans)
        }).collect();

        let paragraph = Paragraph::new(Text::from(lines))
            .block(code_block)
            .scroll((self.scroll, 0));

        frame.render_widget(paragraph, chunks[0]);

        let help_text = "Press 'n' for next generation, 'q' to quit, Up/Down to scroll.";
        let help_block = Block::default().borders(Borders::ALL).title(" Controls ");
        let help = Paragraph::new(help_text).block(help_block);
        frame.render_widget(help, chunks[1]);
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => self.exit = true,
                        KeyCode::Char('n') => self.evolve(),
                        KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                        KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn evolve(&mut self) {
        self.generation += 1;

        // Evolve mapping
        let mut new_mapping = HashMap::new();
        for (original, current) in &self.current_mapping {
            let evolved = self.phonology.evolve(current, &mut self.rng);
            new_mapping.insert(original.clone(), evolved);
        }
        self.current_mapping = new_mapping;

        // Rewrite source
        self.display_segments = rewrite_source_segments(
            &self.original_source,
            &self.current_mapping,
            &self.original_identifiers
        );
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load source code (self source code for fun)
    let path = "experiments/lexical-drift/src/phonology.rs";
    let source = std::fs::read_to_string(path)
        .unwrap_or_else(|_| format!("fn main() {{ println!(\"Error loading file: {}\"); }}", path));

    let app = DriftApp::new(source);

    // Handle potential error in app creation (parsing error)
    if let Err(e) = app {
        // Cleanup terminal before panicking
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;
        return Err(e);
    }

    let mut app = app.unwrap();
    let res = app.run(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

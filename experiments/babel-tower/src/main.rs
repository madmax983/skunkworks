use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use std::{io, time::Duration};
use syn::visit_mut::VisitMut;

mod phonology;
mod obfuscator;
use phonology::PhoneticEngine;
use obfuscator::Obfuscator;

struct App {
    original_code: String,
    current_code: String,
    history: Vec<String>,
    history_index: usize,
    status: String,
}

impl App {
    fn new() -> Self {
        let code = r#"
fn calculate_total(prices: &[f32]) -> f32 {
    let mut total = 0.0;
    for price in prices {
        total += price;
    }
    total
}

struct User {
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String) -> Self {
        Self {
            username,
            email,
            active: true,
        }
    }
}
"#;
        Self {
            original_code: code.to_string(),
            current_code: code.to_string(),
            history: vec![code.to_string()],
            history_index: 0,
            status: "Ready. Press 'g' (Grimm), 'v' (Vowel Shift), 'r' (Rhotacism), 'u' (Undo). 'q' to quit.".to_string(),
        }
    }

    fn apply_law(&mut self, law_name: &str) {
        let engine = match law_name {
            "grimm" => PhoneticEngine::grimm(),
            "vowel" => PhoneticEngine::great_vowel_shift(),
            "rhotacism" => PhoneticEngine::rhotacism(),
            _ => return,
        };

        // Parse current code
        let ast_result = syn::parse_file(&self.current_code);
        match ast_result {
            Ok(mut ast) => {
                let mut obfuscator = Obfuscator::new(engine);
                obfuscator.visit_file_mut(&mut ast);

                let new_code = quote::quote!(#ast).to_string();

                // Add new state to history
                self.history.truncate(self.history_index + 1);
                self.history.push(new_code.clone());
                self.history_index += 1;
                self.current_code = new_code;
                self.status = format!("Applied {}. Era: {}", law_name, self.history_index);
            },
            Err(e) => {
                self.status = format!("Failed to parse code: {}", e);
            }
        }
    }

    fn undo(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
            self.current_code = self.history[self.history_index].clone();
            self.status = format!("Undo. Era: {}", self.history_index);
        }
    }
}

fn main() -> Result<(), io::Error> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char('g') => app.apply_law("grimm"),
                    KeyCode::Char('v') => app.apply_law("vowel"),
                    KeyCode::Char('r') => app.apply_law("rhotacism"),
                    KeyCode::Char('u') | KeyCode::Left => app.undo(),
                    _ => {}
                }
            }
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Min(3),
                Constraint::Percentage(80),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("⚛️ Babel Tower: Linguistic Code Evolution")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    let original_block = Block::default()
        .borders(Borders::ALL)
        .title("Original / Previous Era")
        .style(Style::default().fg(Color::Gray));

    // Show previous era if > 0, else original
    let prev_code = if app.history_index > 0 {
        &app.history[app.history_index - 1]
    } else {
        &app.original_code
    };

    let original_text = Paragraph::new(prev_code.as_str())
        .block(original_block)
        .wrap(Wrap { trim: false });
    f.render_widget(original_text, content_chunks[0]);

    let current_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Current Era ({})", app.history_index))
        .style(Style::default().fg(Color::Green));
    let current_text = Paragraph::new(app.current_code.as_str())
        .block(current_block)
        .wrap(Wrap { trim: false });
    f.render_widget(current_text, content_chunks[1]);

    let status_block = Block::default()
        .borders(Borders::ALL)
        .title("Status")
        .style(Style::default().fg(Color::Yellow));
    let status_text = Paragraph::new(app.status.as_str()).block(status_block);
    f.render_widget(status_text, chunks[2]);
}

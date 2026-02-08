use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quipu_serializer::quipu::{Cord, Quipu};
use quipu_serializer::ser::to_quipu;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};
use serde_json::Value;
use std::io;

enum Mode {
    Calculator,
    Serializer,
}

struct App {
    mode: Mode,
    // Calculator State
    calc_input_a: String,
    calc_input_b: String,
    calc_focus: usize, // 0=A, 1=B

    // Serializer State
    ser_input: String,
    ser_output: Option<Quipu>,
    ser_scroll: u16,
}

impl App {
    fn new() -> Self {
        Self {
            mode: Mode::Calculator,
            calc_input_a: String::new(),
            calc_input_b: String::new(),
            calc_focus: 0,
            ser_input: String::new(),
            ser_output: None,
            ser_scroll: 0,
        }
    }

    fn update_serializer(&mut self) {
        if let Ok(v) = serde_json::from_str::<Value>(&self.ser_input) {
            if let Ok(q) = to_quipu(&v) {
                self.ser_output = Some(q);
            } else {
                self.ser_output = None;
            }
        } else {
            self.ser_output = None;
        }
    }
}

fn main() -> Result<()> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match app.mode {
                Mode::Calculator => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Tab => {
                        app.mode = Mode::Serializer;
                    }
                    KeyCode::Char(c) if c.is_digit(10) => {
                        if app.calc_focus == 0 {
                            app.calc_input_a.push(c);
                        } else {
                            app.calc_input_b.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        if app.calc_focus == 0 {
                            app.calc_input_a.pop();
                        } else {
                            app.calc_input_b.pop();
                        }
                    }
                    KeyCode::Enter | KeyCode::Down | KeyCode::Up => {
                        app.calc_focus = 1 - app.calc_focus;
                    }
                    _ => {}
                },
                Mode::Serializer => match key.code {
                    KeyCode::Esc => break,
                    KeyCode::Tab => {
                        app.mode = Mode::Calculator;
                    }
                    KeyCode::Char(c) => {
                        app.ser_input.push(c);
                        app.update_serializer();
                    }
                    KeyCode::Backspace => {
                        app.ser_input.pop();
                        app.update_serializer();
                    }
                    KeyCode::Up => {
                        if app.ser_scroll > 0 {
                            app.ser_scroll -= 1;
                        }
                    }
                    KeyCode::Down => {
                        app.ser_scroll += 1;
                    }
                    _ => {}
                },
            }
        }
    }

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    let title_text = match app.mode {
        Mode::Calculator => "🧮 QUIPU CALCULATOR (Tab to Switch)",
        Mode::Serializer => "📜 QUIPU SERIALIZER (Tab to Switch)",
    };

    let header = Paragraph::new(title_text)
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    match app.mode {
        Mode::Calculator => render_calculator(f, chunks[1], app),
        Mode::Serializer => render_serializer(f, chunks[1], app),
    }

    let footer = Paragraph::new("Q/Esc: Quit | Tab: Switch Mode")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(footer, chunks[2]);
}

fn render_calculator(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);

    // Input A
    let val_a = app.calc_input_a.parse::<u64>().unwrap_or(0);
    let cord_a = Cord::from(val_a);
    let style_a = if app.calc_focus == 0 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let block_a = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Input A: {} ", app.calc_input_a));
    f.render_widget(
        Paragraph::new(format!("{}", cord_a))
            .block(block_a)
            .style(style_a),
        chunks[0],
    );

    // Input B
    let val_b = app.calc_input_b.parse::<u64>().unwrap_or(0);
    let cord_b = Cord::from(val_b);
    let style_b = if app.calc_focus == 1 {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let block_b = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Input B: {} ", app.calc_input_b));
    f.render_widget(
        Paragraph::new(format!("{}", cord_b))
            .block(block_b)
            .style(style_b),
        chunks[1],
    );

    // Result
    let cord_sum = cord_a.clone() + cord_b.clone();
    let block_sum = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Sum: {} ", cord_sum.value()));
    f.render_widget(
        Paragraph::new(format!("{}", cord_sum))
            .block(block_sum)
            .style(Style::default().fg(Color::Green)),
        chunks[2],
    );
}

fn render_serializer(f: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // JSON Input
    let input_block = Block::default().borders(Borders::ALL).title(" JSON Input ");
    f.render_widget(
        Paragraph::new(app.ser_input.clone())
            .block(input_block)
            .wrap(Wrap { trim: true }),
        chunks[0],
    );

    // Quipu Output
    let output_block = Block::default()
        .borders(Borders::ALL)
        .title(" Quipu Output ");
    let text = if let Some(q) = &app.ser_output {
        format!("{}", q)
    } else {
        if app.ser_input.is_empty() {
            "Type JSON to see Quipu...".to_string()
        } else {
            "Invalid JSON".to_string()
        }
    };

    f.render_widget(
        Paragraph::new(text)
            .block(output_block)
            .scroll((app.ser_scroll, 0)),
        chunks[1],
    );
}

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use std::time::Duration;
use tui_shared::Tui;
use quipu_renderer::quipu::Cord;

struct App {
    cords: Vec<Cord>,
    input: String,
    message: String,
}

impl App {
    fn new() -> Self {
        Self {
            cords: Vec::new(),
            input: String::new(),
            message: String::from("Welcome to the Quipu Accountant"),
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Add some initial data for demo
    app.cords.push(Cord::from(123));
    app.cords.push(Cord::from(45));

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Min(0),    // Cords
                    Constraint::Length(3), // Input/Status
                ])
                .split(f.area());

            // Title
            let title = Paragraph::new("⚛️  QUIPU RENDERER 🏺")
                .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Cords Area
            let area = chunks[1];
            let block = Block::default().borders(Borders::ALL).title("Cords");
            let inner_area = block.inner(area);
            f.render_widget(block, area);

            if !app.cords.is_empty() {
                // Split horizontally for each cord
                let constraints: Vec<Constraint> = (0..app.cords.len())
                    .map(|_| Constraint::Ratio(1, app.cords.len() as u32))
                    .collect();

                let cord_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(inner_area);

                for (i, cord) in app.cords.iter().enumerate() {
                    let ascii = cord.to_string();
                    let p = Paragraph::new(ascii)
                        .style(Style::default().fg(Color::Cyan))
                        .block(Block::default().borders(Borders::RIGHT)); // Separator
                    f.render_widget(p, cord_chunks[i]);
                }
            } else {
                 let p = Paragraph::new("No cords. Enter a number.")
                    .style(Style::default().fg(Color::DarkGray));
                 f.render_widget(p, inner_area);
            }

            // Footer / Input
            let input_text = format!("Input: {} | Status: {} | [0-9] Type, [Enter] Push, [a] Add Last 2, [d] Drop, [q] Quit", app.input, app.message);
            let footer = Paragraph::new(input_text)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(c) if c.is_digit(10) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        if let Ok(val) = app.input.parse::<u32>() {
                            app.cords.push(Cord::from(val));
                            app.input.clear();
                            app.message = format!("Added cord value: {}", val);
                        } else if !app.input.is_empty() {
                            app.message = "Invalid number".to_string();
                        }
                    }
                    KeyCode::Char('a') => {
                        if app.cords.len() >= 2 {
                            let c2 = app.cords.pop().unwrap();
                            let c1 = app.cords.pop().unwrap();
                            let sum = c1.add(&c2);
                            let val = sum.value();
                            app.cords.push(sum);
                            app.message = format!("Added {} + {} = {}", c1.value(), c2.value(), val);
                        } else {
                            app.message = "Need at least 2 cords to add".to_string();
                        }
                    }
                    KeyCode::Char('d') => {
                        if app.cords.pop().is_some() {
                             app.message = "Dropped last cord".to_string();
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

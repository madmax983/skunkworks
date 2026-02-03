use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use quipu_renderer::quipu::Cord;
use quipu_renderer::serializer::to_khipu;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use serde::Serialize;
use std::time::Duration;
use tui_shared::Tui;

#[derive(Serialize)]
struct AncientTrade {
    corn: u32,
    gold: u32,
    llamas: u32,
    is_valid: bool,
}

enum Mode {
    Calculator,
    Serializer,
}

struct App {
    cords: Vec<Cord>,
    input: String,
    message: String,
    mode: Mode,
}

impl App {
    fn new() -> Self {
        Self {
            cords: Vec::new(),
            input: String::new(),
            message: String::from("Welcome to the Quipu Accountant. Press 'm' to toggle mode."),
            mode: Mode::Calculator,
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    // Initial demo data
    app.cords.push(Cord::from(10));
    app.cords.push(Cord::from(2));

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Title
                    Constraint::Min(0),    // Main Cord Area
                    Constraint::Length(4), // Footer
                ])
                .split(f.area());

            let title_text = match app.mode {
                Mode::Calculator => "⚛️  QUIPU CALCULATOR 🏺 (Add Cords)",
                Mode::Serializer => "⚛️  QUIPU SERIALIZER 🏺 (Rust Struct -> Knots)",
            };

            let title = Paragraph::new(title_text)
                .style(
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            // Quipu Rendering Area
            let area = chunks[1];
            let block = Block::default().borders(Borders::ALL).title("Main Cord");
            let inner_area = block.inner(area);
            f.render_widget(block, area);

            // Draw the main horizontal cord
            let main_cord_str = "=".repeat(inner_area.width as usize);
            let main_cord_widget =
                Paragraph::new(main_cord_str).style(Style::default().fg(Color::Red));
            // We want this at the top of inner_area
            let cord_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(1), Constraint::Min(0)])
                .split(inner_area);
            f.render_widget(main_cord_widget, cord_layout[0]);

            // Draw Pendant Cords
            if !app.cords.is_empty() {
                // Split horizontally for each cord
                let constraints: Vec<Constraint> = (0..app.cords.len())
                    .map(|_| Constraint::Ratio(1, app.cords.len() as u32))
                    .collect();

                let pendant_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints(constraints)
                    .split(cord_layout[1]);

                for (i, cord) in app.cords.iter().enumerate() {
                    // Render the vertical cord string
                    let cord_str = cord.to_string();
                    // Need to add the top connection line `|` to connect to main cord
                    let display_str = format!("|\n{}", cord_str);

                    let p = Paragraph::new(display_str).style(Style::default().fg(Color::Cyan));
                    // .alignment(Alignment::Center) // Maybe center if possible
                    f.render_widget(p, pendant_chunks[i]);
                }
            } else {
                let p =
                    Paragraph::new("No cords hanging.").style(Style::default().fg(Color::DarkGray));
                f.render_widget(p, cord_layout[1]);
            }

            // Footer
            let help = match app.mode {
                Mode::Calculator => "[0-9] Input, [Enter] Push, [a] Add Last 2, [d] Drop",
                Mode::Serializer => "[s] Serialize Demo Struct (Corn=100, Gold=50, Llamas=12)",
            };
            let status = format!(
                "Input: {} | {} | [m] Mode, [q] Quit",
                app.input, app.message
            );

            let footer_text = format!("{}\n{}", status, help);
            let footer = Paragraph::new(footer_text)
                .style(Style::default().fg(Color::White))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(footer, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('m') => {
                        app.mode = match app.mode {
                            Mode::Calculator => Mode::Serializer,
                            Mode::Serializer => Mode::Calculator,
                        };
                        app.message = "Mode switched".to_string();
                        app.input.clear();
                    }
                    KeyCode::Char(c) if c.is_digit(10) => {
                        if let Mode::Calculator = app.mode {
                            app.input.push(c);
                        }
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Enter => {
                        if let Mode::Calculator = app.mode {
                            if let Ok(val) = app.input.parse::<u32>() {
                                app.cords.push(Cord::from(val));
                                app.input.clear();
                                app.message = "Cord added".to_string();
                            }
                        }
                    }
                    KeyCode::Char('a') => {
                        if let Mode::Calculator = app.mode {
                            if app.cords.len() >= 2 {
                                let c2 = app.cords.pop().unwrap();
                                let c1 = app.cords.pop().unwrap();
                                let sum = c1.add(&c2);
                                app.cords.push(sum);
                                app.message = "Cords combined".to_string();
                            }
                        }
                    }
                    KeyCode::Char('d') => {
                        app.cords.pop();
                        app.message = "Cord dropped".to_string();
                    }
                    KeyCode::Char('s') => {
                        if let Mode::Serializer = app.mode {
                            let demo = AncientTrade {
                                corn: 100,
                                gold: 50,
                                llamas: 12,
                                is_valid: true,
                            };
                            if let Ok(khipu) = to_khipu(&demo) {
                                app.cords = khipu.cords;
                                app.message = "Serialized AncientTrade struct".to_string();
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

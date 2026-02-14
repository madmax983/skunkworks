use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use tui_shared::Tui;

mod lexer;
mod obfuscator;
mod phonology;

use obfuscator::Obfuscator;

const SAMPLE_CODE: &str = r#"
fn main() {
    let mut x = 0;
    for i in 0..10 {
        x += i;
        println!("Value: {}", x);
    }
    match x {
        0 => println!("Zero"),
        _ => println!("Non-zero"),
    }
}

pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
"#;

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::init()?;
    let mut obfuscator = Obfuscator::new(42);
    let mut generation = 0;

    // Initialize dictionary
    obfuscator.transmute(SAMPLE_CODE);

    loop {
        tui.terminal.draw(|f| {
            let size = f.area();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Length(3)])
                .split(size);

            let main_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            // Left Pane: Original
            let original_block = Block::default()
                .title(" Proto-Code (Original) ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));
            let original_text = Paragraph::new(SAMPLE_CODE)
                .block(original_block)
                .wrap(Wrap { trim: false });
            f.render_widget(original_text, main_chunks[0]);

            // Right Pane: Evolved
            let evolved_code = obfuscator.transmute(SAMPLE_CODE);
            let evolved_block = Block::default()
                .title(format!(" Modern Dialect (Gen {}) ", generation))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta));
            let evolved_text = Paragraph::new(evolved_code)
                .block(evolved_block)
                .wrap(Wrap { trim: false });
            f.render_widget(evolved_text, main_chunks[1]);

            // Bottom Pane: Controls
            let controls_block = Block::default().title(" Controls ").borders(Borders::ALL);
            let controls_text = Paragraph::new("SPACE: Evolve | R: Reset | Q: Quit")
                .block(controls_block)
                .style(Style::default().fg(Color::Yellow))
                .alignment(ratatui::layout::Alignment::Center);
            f.render_widget(controls_text, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char(' ') => {
                            obfuscator.advance_generation();
                            generation += 1;
                        }
                        KeyCode::Char('r') => {
                            obfuscator = Obfuscator::new(42);
                            obfuscator.transmute(SAMPLE_CODE); // Reset dictionary
                            generation = 0;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

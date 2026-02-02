mod cards;
mod oracle;

use anyhow::Result;
use cards::Card;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use oracle::Oracle;
use rand::Rng;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::time::Duration;
use tui_shared::Tui;

struct App {
    reading: oracle::Reading,
    revealed_count: usize, // 0 = All Hidden, 1 = Past Revealed, 2 = Present Revealed, 3 = Future Revealed
}

impl App {
    fn new() -> Self {
        Self {
            reading: Oracle::consult(),
            revealed_count: 0,
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if app.revealed_count < 3 {
                                app.revealed_count += 1;
                            } else {
                                // Reset for a new reading? Or just exit?
                                // Let's reset
                                app.reading = Oracle::consult();
                                app.revealed_count = 0;
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(20),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🔮 QUANTUM TAROT 🔮")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Cards Area
    let card_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
        ])
        .split(chunks[1]);

    render_card(
        f,
        card_chunks[0],
        "PAST (Git History)",
        &app.reading.past,
        app.revealed_count >= 1,
    );
    render_card(
        f,
        card_chunks[1],
        "PRESENT (Filesystem)",
        &app.reading.present,
        app.revealed_count >= 2,
    );
    render_card(
        f,
        card_chunks[2],
        "FUTURE (Entropy)",
        &app.reading.future,
        app.revealed_count >= 3,
    );

    // Footer
    let footer_text = if app.revealed_count < 3 {
        "Press [SPACE] to observe the quantum state..."
    } else {
        "Press [SPACE] to consult the oracle again, or [q] to quit."
    };

    let footer = Paragraph::new(footer_text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}

fn render_card(f: &mut Frame, area: Rect, label: &str, card: &Card, revealed: bool) {
    let block = Block::default()
        .title(label)
        .borders(Borders::ALL)
        .style(if revealed {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if revealed {
        // Render Art
        let art_lines: Vec<&str> = card.art.trim().lines().collect();
        let art_height = art_lines.len() as u16;

        let content_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(art_height + 2), Constraint::Min(1)])
            .split(inner_area);

        let art_paragraph = Paragraph::new(card.art)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
        f.render_widget(art_paragraph, content_layout[0]);

        let text = vec![
            Line::from(Span::styled(
                card.name,
                Style::default().fg(Color::Magenta).bold().underlined(),
            )),
            Line::from(""),
            Line::from(card.description),
        ];

        let desc_paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(desc_paragraph, content_layout[1]);
    } else {
        // Render Quantum Noise
        let mut rng = rand::thread_rng();
        let noise: String = (0..inner_area.height)
            .map(|_| {
                (0..inner_area.width)
                    .map(|_| rng.gen_range(33..126) as u8 as char)
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        let noise_p = Paragraph::new(noise)
            .style(Style::default().fg(Color::DarkGray).dim())
            .wrap(Wrap { trim: true }); // Ensure it wraps if needed, though we manually sized
        f.render_widget(noise_p, inner_area);
    }
}

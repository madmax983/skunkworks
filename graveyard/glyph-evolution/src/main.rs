mod model;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use model::Script;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

const SAMPLE_TEXT: &str = "In the beginning was the Word, and the Word was with God, and the Word was God. \
The same was in the beginning with God. All things were made by him; and without him was not any thing made that was made. \
In him was life; and the life was the light of men. And the light shineth in darkness; and the darkness comprehended it not. \
There was a man sent from God, whose name was John. The same came for a witness, to bear witness of the Light, that all men through him might believe. \
He was not that Light, but was sent to bear witness of that Light. That was the true Light, which lighteth every man that cometh into the world. \
He was in the world, and the world was made by him, and the world knew him not. He came unto his own, and his own received him not. \
But as many as received him, to them gave he power to become the sons of God, even to them that believe on his name: \
Which were born, not of blood, nor of the will of the flesh, nor of the will of man, but of God. \
And the Word was made flesh, and dwelt among us, (and we beheld his glory, the glory as of the only begotten of the Father,) full of grace and truth.";

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut script = Script::new(SAMPLE_TEXT);
    let mut epoch = 0;
    let mut last_evolution = Instant::now();
    let mut auto_play = false;

    loop {
        tui.terminal.draw(|f| draw(f, &script, epoch, auto_play))?;

        // Event polling
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char(' ') => {
                        if let Some(_) = script.evolve() {
                            epoch += 1;
                        }
                    }
                    KeyCode::Char('p') => {
                        auto_play = !auto_play;
                    }
                    KeyCode::Char('r') => {
                        script = Script::new(SAMPLE_TEXT);
                        epoch = 0;
                        auto_play = false;
                    }
                    _ => {}
                }
            }
        }

        if auto_play && last_evolution.elapsed() > Duration::from_millis(100) {
            if let Some(_) = script.evolve() {
                epoch += 1;
            }
            last_evolution = Instant::now();
        }
    }

    Ok(())
}

fn draw(f: &mut Frame, script: &Script, epoch: usize, auto_play: bool) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header/Stats
            Constraint::Min(0),     // Content
            Constraint::Length(10), // Dictionary
            Constraint::Length(3),  // Help
        ])
        .split(f.area());

    // Header
    let original_len = SAMPLE_TEXT.len();
    let current_len = script.corpus.len();
    let ratio = if current_len > 0 {
        original_len as f64 / current_len as f64
    } else {
        0.0
    };

    let title = format!(
        "Glyph Evolution | Epoch: {} | Compression: {:.2}x ({} -> {} tokens) | Mode: {}",
        epoch,
        ratio,
        original_len,
        current_len,
        if auto_play { "PLAYING" } else { "PAUSED" }
    );

    f.render_widget(
        Block::default().borders(Borders::ALL).title(title),
        chunks[0],
    );

    // Content (The Tablet)
    // Render the corpus as braille
    // We can just create a long string.
    let s: String = script
        .corpus
        .iter()
        .map(|&id| script.tokens[id].to_braille())
        .collect();

    let paragraph = Paragraph::new(s)
        .block(Block::default().borders(Borders::ALL).title("The Tablet"))
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[1]);

    // Dictionary (Newest tokens)
    // Show the last N tokens added
    let mut dict_spans = Vec::new();
    let count = script.tokens.len();
    let start = count.saturating_sub(30); // Show last 30

    for i in start..count {
        let g = script.tokens[i];
        dict_spans.push(Span::raw(format!("{} ", g.to_braille())));
    }

    let dict_p = Paragraph::new(Line::from(dict_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Recent Glyphs (Evolutionary Edge)"),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(dict_p, chunks[2]);

    // Help
    let help = Paragraph::new("SPACE: Evolve | P: Play/Pause | R: Reset | Q: Quit")
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[3]);
}

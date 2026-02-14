use crate::audio::Synthesizer;
use crate::music::FugueEvent;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::time::{Duration, Instant};

pub fn run_tui(synthesizer: Synthesizer, events: Vec<FugueEvent>) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, synthesizer, events);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    synthesizer: Synthesizer,
    events: Vec<FugueEvent>,
) -> Result<()> {
    let mut current_event_idx = 0;
    let mut messages: Vec<String> = Vec::new();
    let mut wait_until: Option<Instant> = None;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.size());

            // Render played events (scrolling log)
            let log_text = messages
                .iter()
                .rev()
                .take(f.size().height as usize - 5)
                .rev()
                .cloned()
                .collect::<Vec<String>>()
                .join("\n");
            let events_widget = Paragraph::new(log_text)
                .block(Block::default().title("Fugue Log").borders(Borders::ALL));

            f.render_widget(events_widget, chunks[0]);

            // Status bar
            let status = format!(
                "Events: {}/{} | Press 'q' to quit",
                current_event_idx,
                events.len()
            );
            let status_widget = Paragraph::new(status)
                .block(Block::default().title("Status").borders(Borders::ALL));

            f.render_widget(status_widget, chunks[1]);
        })?;

        // Handle Input
        // Poll for input with a short timeout to allow the loop to run frequently
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    synthesizer.stop_all();
                    return Ok(());
                }
            }
        }

        // Logic
        let now = Instant::now();

        if let Some(deadline) = wait_until {
            if now < deadline {
                continue; // Still waiting
            }
            wait_until = None; // Wait finished
        }

        if current_event_idx < events.len() {
            let event = &events[current_event_idx];
            synthesizer.play_event(event); // Queues notes to sink
            messages.push(format!("{}", event));

            match event {
                FugueEvent::Silence { duration_ms } => {
                    wait_until = Some(now + Duration::from_millis(*duration_ms));
                }
                FugueEvent::SubjectEntry { notes, .. }
                | FugueEvent::Ostinato { pattern: notes, .. } => {
                    // Wait for duration of one note to create staggered entry (canon effect)
                    if !notes.is_empty() {
                        wait_until = Some(now + Duration::from_millis(notes[0].duration_ms));
                    }
                }
                _ => {}
            }

            current_event_idx += 1;
        }
    }
}

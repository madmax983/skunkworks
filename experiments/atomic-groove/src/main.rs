pub mod audio;
pub mod groove;

use std::{error::Error, io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*};
use std::sync::atomic::Ordering;

use crate::audio::AudioEngine;
use crate::groove::Groove;

fn main() -> Result<(), Box<dyn Error>> {
    // Audio Setup
    let audio_engine = AudioEngine::new()?;
    let event_queue = audio_engine.event_queue.clone();

    // Groove Setup
    let groove = Groove::start(event_queue);

    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &groove);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, groove: &Groove) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, groove))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
    }
}

fn ui(f: &mut Frame, groove: &Groove) {
    let area = f.area();

    let block = Block::default()
        .title("Atomic Groove ⚛️🥁")
        .borders(Borders::ALL);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3), // Title/Instructions
            Constraint::Min(0),    // Grid
        ])
        .split(area);

    let instructions = Paragraph::new("Press 'q' to quit. Audio requires ALSA/cpal feature.")
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(instructions, chunks[0]);

    let grid_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ])
        .split(chunks[1]);

    let flags = &groove.active_flags;

    // Kick
    let kick_active = flags.kick.load(Ordering::Relaxed);
    draw_instrument(f, grid_chunks[0], "Kick (Metronome)", kick_active, Color::Red, "Sleep 500ms -> Barrier");

    // Snare
    let snare_active = flags.snare.load(Ordering::Relaxed);
    draw_instrument(f, grid_chunks[1], "Snare (Follower)", snare_active, Color::Blue, "Sleep 700ms -> Barrier");

    // Hat
    let hat_active = flags.hat.load(Ordering::Relaxed);
    draw_instrument(f, grid_chunks[2], "Hi-Hat (Contender)", hat_active, Color::Yellow, "RwLock (TryRead)");

    // Clap
    let clap_active = flags.clap.load(Ordering::Relaxed);
    draw_instrument(f, grid_chunks[3], "Clap (Consumer)", clap_active, Color::Green, "Channel Recv");
}

fn draw_instrument(f: &mut Frame, area: Rect, title: &str, active: bool, color: Color, desc: &str) {
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if active { Style::default().fg(color) } else { Style::default() });

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    if active {
        let flash = Block::default()
            .bg(color);
        f.render_widget(flash, inner_area);
    }

    let text = Paragraph::new(desc)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    // Center text vertically
    let text_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Length(2), // Allow 2 lines
            Constraint::Min(0),
        ])
        .split(inner_area)[1];

    f.render_widget(text, text_area);
}

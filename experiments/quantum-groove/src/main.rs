use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Gauge},
    Terminal,
};

mod audio;
mod quantum;
mod state;

use audio::AudioWriter;
use state::GameState;

fn main() -> Result<()> {
    // Setup Audio
    let (audio_tx, audio_rx) = mpsc::channel();

    // Spawn Audio Thread
    thread::spawn(move || {
        let mut writer = AudioWriter::new("groove_output.wav", 44100, audio_rx).unwrap();
        // Generate audio loop
        loop {
            // Process any events
            if let Err(_) = writer.process_events() {
                break;
            }
            // Generate small chunk
            if let Err(_) = writer.generate_chunk(0.01) {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Game Loop
    let mut state = GameState::new(40, 20, audio_tx);
    let beat_interval = Duration::from_secs_f64(60.0 / state.bpm as f64);
    let mut last_beat = Instant::now();
    let tolerance = Duration::from_millis(150); // Generous window

    let res = run_app(&mut terminal, &mut state, beat_interval, &mut last_beat, tolerance);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    } else {
        println!("Game Over! WAV file written to groove_output.wav");
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    state: &mut GameState,
    beat_interval: Duration,
    last_beat: &mut Instant,
    tolerance: Duration,
) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: Send + Sync + 'static,
{
    loop {
        let now = Instant::now();
        let time_since_beat = now.duration_since(*last_beat);

        // Check for next beat
        if time_since_beat >= beat_interval {
            *last_beat = now;
            state.trigger_beat();
        }

        // Calculate beat phase (0.0 to 1.0)
        let phase = time_since_beat.as_secs_f64() / beat_interval.as_secs_f64();

        // Determine if we are "on beat" (close to 0.0 or 1.0)
        let dist_to_beat = if phase < 0.5 {
            time_since_beat
        } else {
            beat_interval - time_since_beat
        };
        let on_beat = dist_to_beat < tolerance;

        // Draw
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(3), // Header / Score
                        Constraint::Length(1), // Metronome Bar
                        Constraint::Min(0),    // Game Grid
                        Constraint::Length(1), // Status
                    ]
                    .as_ref(),
                )
                .split(f.size());

            // Header
            let header = Paragraph::new(format!("Quantum Groove | Score: {} | BPM: {}", state.player.score, state.bpm))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(header, chunks[0]);

            // Metronome
            let gauge_val = if phase < 0.5 { phase * 2.0 } else { (1.0 - phase) * 2.0 };
            let color = if on_beat { Color::Green } else { Color::Red };
            let gauge = Gauge::default()
                .block(Block::default().borders(Borders::NONE))
                .gauge_style(Style::default().fg(color))
                .ratio(gauge_val.clamp(0.0, 1.0));
            f.render_widget(gauge, chunks[1]);

            // Grid
            let grid_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(state.width as u16 + 2), Constraint::Min(0)].as_ref())
                .split(chunks[2]);

            // We need to construct the grid text
            let mut rows = Vec::new();
            for y in 0..state.height {
                let mut line_spans = Vec::new();
                for x in 0..state.width {
                    // Check entities
                    let entity = state.entities.iter().find(|e| e.x == x && e.y == y);
                    let (char, color) = if state.player.x == x && state.player.y == y {
                        ('@', Color::Yellow)
                    } else if let Some(e) = entity {
                        (e.glyph, e.color)
                    } else {
                        match state.grid[y][x] {
                            state::Tile::Wall => ('#', Color::DarkGray),
                            state::Tile::Exit => ('>', Color::Green),
                            state::Tile::Empty => ('.', Color::Black),
                        }
                    };
                    line_spans.push(Span::styled(char.to_string(), Style::default().fg(color)));
                }
                rows.push(Line::from(line_spans));
            }

            let grid_paragraph = Paragraph::new(rows)
                .block(Block::default().borders(Borders::ALL).title("Dungeon"));
            f.render_widget(grid_paragraph, grid_chunks[0]);

            // Inventory / Status
            let status = Paragraph::new(state.message.clone())
                .style(Style::default().fg(if on_beat { Color::Green } else { Color::Red }));
            f.render_widget(status, chunks[3]);
        })?;

        // Input
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up | KeyCode::Char('k') => state.try_move_player(0, -1, on_beat),
                    KeyCode::Down | KeyCode::Char('j') => state.try_move_player(0, 1, on_beat),
                    KeyCode::Left | KeyCode::Char('h') => state.try_move_player(-1, 0, on_beat),
                    KeyCode::Right | KeyCode::Char('l') => state.try_move_player(1, 0, on_beat),
                    _ => {}
                }
            }
        }

        // Logic Update
        state.update_entities();
    }
}

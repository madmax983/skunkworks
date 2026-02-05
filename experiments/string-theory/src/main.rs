pub mod dsp;
pub mod harp;
pub mod audio;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use std::{io, time::{Duration, Instant}};
use audio::{run_audio, AudioCommand};
use harp::{scan_directory, StringEntity};
use crossbeam_channel::bounded;

struct App {
    strings: Vec<StringEntity>,
    list_state: ListState,
    audio_tx: crossbeam_channel::Sender<AudioCommand>,
}

impl App {
    fn new(audio_tx: crossbeam_channel::Sender<AudioCommand>) -> Self {
        let strings = scan_directory(".");
        let mut list_state = ListState::default();
        if !strings.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            strings,
            list_state,
            audio_tx,
        }
    }

    fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.strings.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.strings.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    fn pluck_current(&mut self) {
        if let Some(i) = self.list_state.selected() {
            self.pluck(i);
        }
    }

    fn pluck(&mut self, index: usize) {
        if let Some(s) = self.strings.get_mut(index) {
            s.energy = 1.0;
            // Send to audio
            // Decay: Higher freq decays faster naturally in KS, but we can parameterize.
            // Let's use 0.99 for all for now, or 0.995 for low freq.
            // Map size to decay?
            // Large size = low freq = long sustain.
            let decay = 0.99 + (s.size as f32 % 100.0) * 0.00005;
            let decay = decay.clamp(0.90, 0.999);

            let _ = self.audio_tx.send(AudioCommand::Pluck {
                freq: s.freq,
                decay,
                amp: 0.8,
            });
        }
    }

    fn strum(&mut self) {
        // Strum visible or all?
        // Strumming all might be chaos.
        // Let's strum 10 random strings.
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let idx = rng.gen_range(0..self.strings.len());
            self.pluck(idx);
        }
    }

    fn tick(&mut self) {
        // Decay visual energy
        for s in self.strings.iter_mut() {
            s.energy *= 0.90; // Visual decay
            if s.energy < 0.01 {
                s.energy = 0.0;
            }
        }
    }
}

fn main() -> Result<()> {
    // Setup Audio
    let (cmd_tx, cmd_rx) = bounded(100);
    // run_audio returns a stream that must be kept alive
    let _audio_stream = run_audio(cmd_rx)?;

    // Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(cmd_tx);
    let tick_rate = Duration::from_millis(30); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Char(' ') | KeyCode::Enter => app.pluck_current(),
                        KeyCode::Char('s') => app.strum(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }

    // Restore
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area()); // Changed from f.size() to f.area() for ratatui > 0.26

    let title = Paragraph::new("String Theory: The Codebase Harp")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .strings
        .iter()
        .map(|s| {
            // Visualizer string
            // energy 0 -> "-"
            // energy > 0.5 -> "~"
            // energy > 0.8 -> "="
            let string_char = if s.energy > 0.8 {
                "≣"
            } else if s.energy > 0.4 {
                "≈"
            } else if s.energy > 0.1 {
                "~"
            } else {
                "-"
            };

            // Color based on size/freq?
            // High freq (small size) = Yellow
            // Low freq (large size) = Blue
            let color = if s.freq > 1000.0 {
                Color::Yellow
            } else if s.freq > 200.0 {
                Color::Green
            } else {
                Color::Blue
            };

            let content = Line::from(vec![
                Span::styled(format!("[{}] ", string_char.repeat(5)), Style::default().fg(if s.energy > 0.0 { Color::Red } else { Color::DarkGray })),
                Span::styled(format!("{} ", s.path.display()), Style::default().fg(color)),
                Span::raw(format!("({} Hz)", s.freq.round())),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD | Modifier::REVERSED))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[1], &mut app.list_state);

    let info = Paragraph::new("UP/DOWN: Navigate | SPACE: Pluck | S: Strum | Q: Quit")
        .style(Style::default().fg(Color::Gray))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(info, chunks[2]);
}

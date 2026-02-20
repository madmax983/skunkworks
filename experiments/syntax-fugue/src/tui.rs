use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io::{self, Stdout};
use std::time::Duration;

use crate::parser::{Voice, VoiceType};

pub struct VoiceState {
    pub voice: Voice,
    pub current_token_idx: usize,
    pub last_play_time: std::time::Instant,
    pub started: bool,
    pub start_delay: Duration,
}

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub should_quit: bool,
    pub voices: Vec<VoiceState>,
}

impl TuiApp {
    pub fn new(mut voices: Vec<Voice>) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        // Sort voices: Soprano -> Alto -> Tenor -> Bass
        voices.sort_by_key(|v| match v.voice_type {
            VoiceType::Soprano => 0,
            VoiceType::Alto => 1,
            VoiceType::Tenor => 2,
            VoiceType::Bass => 3,
        });

        let voice_states = voices
            .into_iter()
            .map(|v| VoiceState {
                voice: v,
                current_token_idx: 0,
                last_play_time: std::time::Instant::now(),
                started: false,
                start_delay: Duration::ZERO,
            })
            .collect();

        Ok(Self {
            terminal,
            should_quit: false,
            voices: voice_states,
        })
    }

    pub fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|f| {
            let size = f.size();

            if self.voices.is_empty() {
                let block = Block::default()
                    .title("No voices found")
                    .borders(Borders::ALL);
                f.render_widget(block, size);
                return;
            }

            let constraints: Vec<Constraint> = (0..self.voices.len())
                .map(|_| Constraint::Ratio(1, self.voices.len() as u32))
                .collect();

            let chunks = Layout::default()
                .direction(Direction::Vertical) // Stack tracks vertically
                .constraints(constraints)
                .split(size);

            for (i, state) in self.voices.iter().enumerate() {
                let type_label = match state.voice.voice_type {
                    VoiceType::Soprano => "SOPRANO",
                    VoiceType::Alto => "ALTO",
                    VoiceType::Tenor => "TENOR",
                    VoiceType::Bass => "BASS",
                };

                let title = format!("[ {} | {} ]", type_label, state.voice.name);

                let mut spans = Vec::new();

                // Only render a window of tokens around the current one to save performance/space?
                // Or just render all and let Paragraph handle it?
                // Paragraph with Wrap can be heavy if text is huge.
                // Let's render all for now.

                for (j, token) in state.voice.tokens.iter().enumerate() {
                    let mut style = Style::default().fg(token.color);

                    if j == state.current_token_idx {
                        style = style
                            .bg(Color::White)
                            .fg(Color::Black)
                            .add_modifier(Modifier::BOLD);
                    } else if j < state.current_token_idx {
                        // Dim past tokens
                        style = style.add_modifier(Modifier::DIM);
                    }

                    spans.push(Span::styled(format!("{} ", token.text), style));
                }

                let line = Line::from(spans);

                // Auto-scroll logic needs to know where the active token is.
                // Since we wrap, it's hard to know exactly.
                // But since we use Vertical layout, each track has full width.
                // So wrapping is less frequent.

                let paragraph = Paragraph::new(vec![line])
                    .block(Block::default().title(title).borders(Borders::ALL))
                    .wrap(Wrap { trim: true });
                // .scroll() // We'd need to calculate scroll.
                // For now, let's rely on the fact that tracks are short enough or we just see the start.
                // Ideally we should scroll.

                f.render_widget(paragraph, chunks[i]);
            }
        })?;
        Ok(())
    }

    pub fn handle_events(&mut self) -> Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                }
            }
        }
        Ok(())
    }
}

impl Drop for TuiApp {
    fn drop(&mut self) {
        disable_raw_mode().unwrap_or(());
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap_or(());
        self.terminal.show_cursor().unwrap_or(());
    }
}

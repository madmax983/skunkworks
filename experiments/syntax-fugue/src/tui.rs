use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io::{self, Stdout};
use std::time::Duration;

use crate::parser::Voice;

pub struct VoiceState {
    pub voice: Voice,
    pub current_token_idx: usize,
    pub last_play_time: std::time::Instant,
    pub started: bool,
}

pub struct TuiApp {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    pub should_quit: bool,
    pub voices: Vec<VoiceState>,
}

impl TuiApp {
    pub fn new(voices: Vec<Voice>) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let voice_states = voices
            .into_iter()
            .map(|v| VoiceState {
                voice: v,
                current_token_idx: 0,
                last_play_time: std::time::Instant::now(),
                started: false,
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
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(size);

            for (i, state) in self.voices.iter().enumerate() {
                let title = format!("Voice {}: {}", i + 1, state.voice.name);

                let mut current_line_spans = Vec::new();
                let mut char_count = 0;
                let mut current_line_chars = 0;
                let mut current_token_line = 0;

                let panel_width = chunks[i].width.saturating_sub(2) as usize; // remove borders
                if panel_width == 0 {
                    continue;
                } // Too small

                for (j, token) in state.voice.tokens.iter().enumerate() {
                    let mut style = Style::default().fg(token.color);
                    let token_len = token.text.len() + 1; // +1 for space

                    // Simple wrap logic simulation to find line of current token
                    if current_line_chars + token_len > panel_width {
                        current_line_chars = 0;
                        // Logic for wrapping: if token fits on next line, good.
                        // ratatui wrap trims? assuming yes.
                        // But actually if token is longer than width, it splits.
                        // Let's assume standard word wrap.
                        char_count += token_len; // rough estimate
                                                 // Every time we wrap, we increment line count?
                                                 // Actually, we just need to know which line the token STARTS on.
                    }

                    // Update current line char count for wrapping logic
                    if current_line_chars + token_len > panel_width {
                        current_line_chars = 0;
                    }
                    current_line_chars += token_len;

                    if j == state.current_token_idx {
                        style = style.bg(Color::White).fg(Color::Black);
                        // Use char_count for rough vertical position if wrapping fails?
                        // Actually, let's trust char_count / panel_width as primary heuristic
                        // since we don't track exact line breaks of Paragraph widget easily.
                        current_token_line = char_count / panel_width;
                    }

                    char_count += token_len;

                    current_line_spans.push(Span::styled(format!("{} ", token.text), style));
                }

                let lines = vec![Line::from(current_line_spans)];

                // Ensure scroll keeps current line in middle
                let height = chunks[i].height.saturating_sub(2) as usize;
                let scroll_y = if current_token_line > height / 2 {
                    (current_token_line - height / 2) as u16
                } else {
                    0
                };

                let paragraph = Paragraph::new(lines)
                    .block(Block::default().title(title).borders(Borders::ALL))
                    .wrap(Wrap { trim: true })
                    .scroll((scroll_y, 0));

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

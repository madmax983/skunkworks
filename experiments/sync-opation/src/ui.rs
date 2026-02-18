use crate::conductor::{MusicianState, UiEvent};
use anyhow::Result;
use crossbeam::channel::Receiver;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use std::collections::HashMap;
use std::time::Duration;

pub struct App {
    states: HashMap<usize, MusicianState>,
    rx: Receiver<UiEvent>,
}

impl App {
    pub fn new(rx: Receiver<UiEvent>) -> Self {
        Self {
            states: HashMap::new(),
            rx,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let res = self.run_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        res
    }

    fn run_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> Result<()> {
        loop {
            // Process all pending events to avoid lag
            while let Ok(event) = self.rx.try_recv() {
                match event {
                    UiEvent::StateChange(id, state) => {
                        self.states.insert(id, state);
                    }
                }
            }

            terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                    .split(f.size());

                let title =
                    Paragraph::new("Sync-Opation: Thread Contention Rhythms (Press 'q' to quit)")
                        .style(Style::default().fg(Color::Cyan))
                        .block(Block::default().borders(Borders::ALL));
                f.render_widget(title, chunks[0]);

                let mut sorted_states: Vec<_> = self.states.iter().collect();
                sorted_states.sort_by_key(|k| k.0);

                let rows: Vec<Row> = sorted_states
                    .iter()
                    .map(|(id, state)| {
                        let (color, status_text) = match state {
                            MusicianState::Resting => (Color::Blue, "RESTING (Sleep)"),
                            MusicianState::Waiting => (Color::Yellow, "WAITING (Blocked)"),
                            MusicianState::Playing => (Color::Green, "PLAYING (Critical Section)"),
                        };
                        Row::new(vec![format!("Musician {}", id), status_text.to_string()])
                            .style(Style::default().fg(color))
                    })
                    .collect();

                let widths = [Constraint::Percentage(20), Constraint::Percentage(80)];
                let table = Table::new(rows, widths)
                    .header(
                        Row::new(vec!["ID", "State"])
                            .style(Style::default().fg(Color::White).bg(Color::Black)),
                    )
                    .block(Block::default().borders(Borders::ALL).title("Musicians"));
                f.render_widget(table, chunks[1]);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
}

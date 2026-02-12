use crate::conductor::Instrument;
use crossbeam_channel::Receiver;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use std::io;

pub enum TuiEvent {
    StateChange { id: usize, state: MusicianState },
    Beat { id: usize, instrument: Instrument },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicianState {
    Sleeping,
    Waiting,
    Playing,
}

pub struct App {
    musicians: Vec<MusicianState>,
    logs: Vec<String>,
    receiver: Receiver<TuiEvent>,
}

impl App {
    pub fn new(num_musicians: usize, receiver: Receiver<TuiEvent>) -> Self {
        Self {
            musicians: vec![MusicianState::Sleeping; num_musicians],
            logs: Vec::new(),
            receiver,
        }
    }

    pub fn update(&mut self) {
        // Process all pending events
        while let Ok(event) = self.receiver.try_recv() {
            match event {
                TuiEvent::StateChange { id, state } => {
                    if id < self.musicians.len() {
                        self.musicians[id] = state;
                    }
                }
                TuiEvent::Beat { id, instrument } => {
                    self.logs
                        .push(format!("Musician {} played {:?}", id, instrument));
                    if self.logs.len() > 20 {
                        self.logs.remove(0);
                    }
                }
            }
        }
    }

    pub fn draw(&self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(f.area());

            // Musicians
            let constraints: Vec<Constraint> = (0..self.musicians.len())
                .map(|_| Constraint::Ratio(1, self.musicians.len() as u32))
                .collect();

            let musician_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(constraints)
                .split(chunks[0]);

            for (i, state) in self.musicians.iter().enumerate() {
                let color = match state {
                    MusicianState::Sleeping => Color::DarkGray,
                    MusicianState::Waiting => Color::Red,
                    MusicianState::Playing => Color::White,
                };

                let block = Block::default()
                    .borders(Borders::ALL)
                    .title(format!("M{}", i))
                    .style(Style::default().bg(color).fg(Color::Black));

                f.render_widget(block, musician_chunks[i]);
            }

            // Logs
            let items: Vec<ListItem> = self
                .logs
                .iter()
                .rev() // Show newest at top? Or bottom?
                .map(|log| ListItem::new(Line::from(log.as_str())))
                .collect();

            let list =
                List::new(items).block(Block::default().borders(Borders::ALL).title("Event Log"));

            f.render_widget(list, chunks[1]);
        })?;

        Ok(())
    }
}

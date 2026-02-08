use crate::sim::{Musician, MusicianState};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, musicians: &[Musician]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("Thread Frequency - Polyrhythmic Processor Monitor")
        .block(Block::default().borders(Borders::ALL))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    // Musician List
    let items: Vec<ListItem> = musicians
        .iter()
        .map(|m| {
            let state = *m.state.lock().unwrap();
            let (status, color) = match state {
                MusicianState::Sleeping => ("SLEEPING", Color::DarkGray),
                MusicianState::Waiting => ("WAITING ", Color::Yellow),
                MusicianState::Playing => ("PLAYING ", Color::Green),
                MusicianState::Contending => ("CONTENTION", Color::Red),
            };

            let content = Line::from(vec![
                Span::raw(format!("Thread #{:<2} {}: ", m.id, m.name)),
                Span::styled(status, Style::default().fg(color)),
            ]);

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Threads").borders(Borders::ALL));
    f.render_widget(list, chunks[1]);

    // Footer
    let footer = Paragraph::new("Press 'q' to quit | Output written to 'thread_frequency_output.wav'")
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, chunks[2]);
}

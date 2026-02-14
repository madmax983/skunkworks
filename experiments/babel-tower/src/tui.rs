use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::time::Duration;

#[derive(Clone)]
pub struct SpeakerStat {
    pub id: usize,
    pub last_wait: Duration,
    pub last_word: String,
    pub total_words: usize,
    pub avg_wait_ms: f32,
}

pub struct TuiState {
    pub transcript: Vec<(String, f32)>,
    pub speaker_stats: Vec<SpeakerStat>,
}

pub fn draw_ui(f: &mut Frame, state: &TuiState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    // Transcript Pane
    let transcript_block = Block::default().title(" The Tower of Babel ").borders(Borders::ALL);

    // Take last 1000 words to prevent memory issues in rendering
    let display_transcript = state.transcript.iter().rev().take(1000).rev();

    let mut spans = Vec::new();
    for (word, intensity) in display_transcript {
        let color = if *intensity < 0.5 {
            Color::White
        } else if *intensity < 2.0 {
            Color::Yellow
        } else {
            Color::Red
        };
        spans.push(Span::styled(format!("{} ", word), Style::default().fg(color)));
    }

    // If we just put all spans in one Line, it will wrap if `Wrap` is set? Yes.
    let text = Line::from(spans);
    let transcript_paragraph = Paragraph::new(text)
        .block(transcript_block)
        .wrap(Wrap { trim: true });

    f.render_widget(transcript_paragraph, chunks[0]);

    // Stats Pane
    let stats_block = Block::default().title(" Speakers (Threads) ").borders(Borders::ALL);
    let items: Vec<ListItem> = state.speaker_stats.iter().map(|s| {
        let color = if s.avg_wait_ms > 20.0 { Color::Red } else { Color::Green };
        ListItem::new(format!(
            "Speaker #{}: Last Word: {:<15} Wait: {:<4}ms (Avg: {:.1}ms)",
            s.id, s.last_word, s.last_wait.as_millis(), s.avg_wait_ms
        )).style(Style::default().fg(color))
    }).collect();
    let stats_list = List::new(items).block(stats_block);
    f.render_widget(stats_list, chunks[1]);
}

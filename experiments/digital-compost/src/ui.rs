use crate::compost::CompostBin;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};

pub fn draw(
    f: &mut Frame,
    bin: &CompostBin,
    list_state: &mut ListState,
    content: &Option<String>,
    scroll: u16,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    // File List
    let items: Vec<ListItem> = bin
        .files
        .iter()
        .map(|file| {
            let age_days = (chrono::Utc::now().timestamp() - file.timestamp) / 86400;
            let label = format!(
                "{} [{}d] [{}]",
                file.relative_path,
                age_days,
                file.decay_level.label()
            );
            ListItem::new(label).style(Style::default().fg(file.decay_level.color()))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Digital Compost Bin"),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::REVERSED)
                .fg(Color::White),
        )
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], list_state);

    // Content View
    let block = Block::default().borders(Borders::ALL).title("Viewer");

    if let Some(text) = content {
        // Find selected file to get color
        let color = if let Some(idx) = list_state.selected() {
            bin.files[idx].decay_level.color()
        } else {
            Color::White
        };

        let p = Paragraph::new(text.as_str())
            .block(block)
            .style(Style::default().fg(color))
            .scroll((scroll, 0));
        f.render_widget(p, chunks[1]);
    } else {
        let p = Paragraph::new("Select a file to observe its entropy.").block(block);
        f.render_widget(p, chunks[1]);
    }
}

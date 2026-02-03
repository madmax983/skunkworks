use crate::analysis::FileStats;
use chrono::{DateTime, Utc};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Gauge},
    Frame,
};

pub fn draw(f: &mut Frame, files: &[FileStats], list_state: &mut ListState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
        .split(f.area());

    draw_file_list(f, chunks[0], files, list_state);
    draw_details(f, chunks[1], files, list_state.selected());
}

fn draw_file_list(f: &mut Frame, area: Rect, files: &[FileStats], state: &mut ListState) {
    let items: Vec<ListItem> = files
        .iter()
        .map(|file| {
            let score = file.haunt_score;
            let color = if score > 50.0 {
                Color::Red
            } else if score > 20.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            let style = if score > 50.0 {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            let name = file.path.to_string_lossy();
            // Show icon based on score?
            let icon = if score > 50.0 {
                "👻"
            } else if score > 20.0 {
                "⚠️ "
            } else {
                "✅ "
            };

            ListItem::new(Line::from(vec![
                Span::raw(icon),
                Span::styled(format!("{} ({:.0})", name, score), style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Haunted Files "),
        )
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, area, state);
}

fn draw_details(f: &mut Frame, area: Rect, files: &[FileStats], selected_index: Option<usize>) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Séance Room ");

    if let Some(index) = selected_index {
        if index < files.len() {
            let file = &files[index];

            let inner_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3), // Stats
                    Constraint::Length(3), // Gauge
                    Constraint::Min(0),    // Messages
                ])
                .margin(1)
                .split(block.inner(area));

            f.render_widget(block, area);

            // 1. Stats Text
            let date = DateTime::from_timestamp(file.last_modified, 0)
                .map(|d: DateTime<Utc>| d.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            let stats_text = vec![
                Line::from(vec![
                    Span::raw("Path: "),
                    Span::styled(
                        file.path.to_string_lossy(),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
                Line::from(vec![Span::raw(format!(
                    "Churn: {} | Fixes: {} | Last Modified: {}",
                    file.churn, file.fixes, date
                ))]),
            ];

            let p = Paragraph::new(stats_text);
            f.render_widget(p, inner_layout[0]);

            // 2. Haunt Gauge
            let gauge_color = if file.haunt_score > 50.0 {
                Color::Red
            } else if file.haunt_score > 20.0 {
                Color::Yellow
            } else {
                Color::Green
            };

            // Normalize for gauge (assuming max score 100 for visualization, cap at 100)
            let ratio = (file.haunt_score / 100.0).min(1.0);

            let gauge = Gauge::default()
                .block(Block::default().title("Haunt Level"))
                .gauge_style(Style::default().fg(gauge_color))
                .ratio(ratio)
                .label(format!("{:.1}", file.haunt_score));

            f.render_widget(gauge, inner_layout[1]);

            // 3. Recent Messages
            let messages: Vec<ListItem> = file
                .recent_messages
                .iter()
                .map(|msg| ListItem::new(format!("- {}", msg)))
                .collect();

            let msg_list = List::new(messages).block(
                Block::default()
                    .borders(Borders::TOP)
                    .title(" Recent Incantations (Commits) "),
            );

            f.render_widget(msg_list, inner_layout[2]);

            return;
        }
    }

    // Default empty state
    let p = Paragraph::new("Select a file to summon its details...")
        .block(block)
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(p, area);
}

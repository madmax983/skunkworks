use crate::game::BattleState;
use crate::parser::Fighter;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw(f: &mut Frame, state: &BattleState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(20),    // Arena
            Constraint::Length(12), // Log
        ])
        .split(f.area());

    // Title
    let title = Paragraph::new("🏟️  FUNC ARENA 🥊")
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Arena (Split A / B)
    let arena_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    draw_fighter(
        f,
        &state.fighter_a,
        arena_chunks[0],
        Color::Cyan,
        "Player 1",
    );
    draw_fighter(
        f,
        &state.fighter_b,
        arena_chunks[1],
        Color::Magenta,
        "Player 2",
    );

    // Combat Log
    let log_items: Vec<ListItem> = state
        .log
        .messages
        .iter()
        .rev()
        .take(10)
        .map(|m| ListItem::new(Line::from(vec![Span::raw(m)])))
        .collect();

    let log = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title(" Combat Log "))
        .style(Style::default().fg(Color::White));
    f.render_widget(log, chunks[2]);

    // Winner Overlay?
    if let Some(_winner) = &state.winner {
        // ... maybe a popup, but text in log is fine for now.
    }
}

fn draw_fighter(
    f: &mut Frame,
    fighter: &Fighter,
    area: ratatui::layout::Rect,
    color: Color,
    label: &str,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ", label))
        .style(Style::default().fg(color));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Name
            Constraint::Length(3), // HP Bar
            Constraint::Length(6), // Stats
            Constraint::Min(0),    // Signature
        ])
        .margin(1)
        .split(inner_area);

    // Name
    f.render_widget(
        Paragraph::new(fighter.name.clone())
            .style(Style::default().add_modifier(Modifier::BOLD).fg(color))
            .alignment(Alignment::Center),
        chunks[0],
    );

    // HP Bar
    let hp_percent = (fighter.hp as f64 / fighter.max_hp as f64).clamp(0.0, 1.0);
    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::NONE))
        .gauge_style(Style::default().fg(if hp_percent < 0.3 {
            Color::Red
        } else {
            Color::Green
        }))
        .ratio(hp_percent)
        .label(format!("{}/{}", fighter.hp, fighter.max_hp));
    f.render_widget(gauge, chunks[1]);

    // Stats
    let stats = vec![
        Line::from(vec![
            Span::raw("⚔️  Attack: "),
            Span::styled(fighter.attack.to_string(), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("🛡️  Defense: "),
            Span::styled(
                fighter.defense.to_string(),
                Style::default().fg(Color::Blue),
            ),
        ]),
        Line::from(vec![
            Span::raw("💨 Speed: "),
            Span::styled(
                fighter.speed.to_string(),
                Style::default().fg(Color::Yellow),
            ),
        ]),
        Line::from(vec![
            Span::raw("📍 File: "),
            Span::styled(
                fighter.file_path.clone(),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
    ];
    f.render_widget(Paragraph::new(stats), chunks[2]);

    // Signature
    let sig_text = Paragraph::new(fighter.signature.clone())
        .block(Block::default().borders(Borders::TOP).title(" Signature "))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::Gray));
    f.render_widget(sig_text, chunks[3]);
}

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::game::Game;

pub fn draw(f: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header/Stats
            Constraint::Min(0),     // Main Game Area
            Constraint::Length(10), // Log
        ])
        .split(f.area());

    draw_stats(f, game, chunks[0]);
    draw_room(f, game, chunks[1]);
    draw_log(f, game, chunks[2]);
}

fn draw_stats(f: &mut Frame, game: &Game, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(30), // HP Bar
        ])
        .split(area);

    // Left: Stats Text
    let text = vec![Line::from(vec![
        Span::styled("GIT ROGUE", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" | "),
        Span::raw("XP: "),
        Span::styled(format!("{}", game.xp), Style::default().fg(Color::Yellow)),
        Span::raw(" | "),
        Span::raw("Loc: "),
        Span::styled(
            game.current_node().short_hash.clone(),
            Style::default().fg(Color::Cyan),
        ),
        if game.game_over {
            Span::styled(
                " | GAME OVER",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )
        } else {
            Span::raw("")
        },
    ])];

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    let p = Paragraph::new(text).block(block);
    f.render_widget(p, chunks[0]);

    // Right: HP Gauge
    let hp_ratio = (game.hp as f64 / game.max_hp as f64).clamp(0.0, 1.0);
    let hp_color = if hp_ratio < 0.3 {
        Color::Red
    } else if hp_ratio < 0.6 {
        Color::Yellow
    } else {
        Color::Green
    };

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" ❤️ HP "))
        .gauge_style(Style::default().fg(hp_color))
        .ratio(hp_ratio)
        .label(format!("{}/{}", game.hp, game.max_hp));

    f.render_widget(gauge, chunks[1]);
}

fn draw_room(f: &mut Frame, game: &Game, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    // Left: Room Description
    let node = game.current_node();
    let desc_text = vec![
        Line::from(vec![Span::styled(
            "Commit Message:",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::raw(&node.message)]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Author:",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::raw(&node.author)]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Hash:",
            Style::default().fg(Color::Yellow),
        )]),
        Line::from(vec![Span::raw(&node.hash)]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Controls:",
            Style::default().fg(Color::Magenta),
        )]),
        Line::from("1-9: ⏬ Go to Parent (Back in time)"),
        Line::from("Shift + 1-9: ⏫ Go to Child (Forward in time)"),
        Line::from("Q: Quit"),
    ];

    let desc_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(" 📍 Current Commit ");
    let p = Paragraph::new(desc_text)
        .block(desc_block)
        .wrap(Wrap { trim: true });
    f.render_widget(p, chunks[0]);

    // Right: Exits
    let exits_block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" 🚪 Exits ");

    let mut items = Vec::new();

    // Parents (Down)
    if !node.parents.is_empty() {
        items.push(ListItem::new(Span::styled(
            "PARENTS (Back in time):",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )));
        for (i, p_hash) in node.parents.iter().enumerate() {
            let label = if let Some(p_node) = game.nodes.get(p_hash) {
                format!(
                    "{} - {}",
                    p_node.short_hash,
                    p_node.message.lines().next().unwrap_or("").trim()
                )
            } else {
                format!("{} (Unknown - outside crawled range)", &p_hash[..7])
            };
            items.push(ListItem::new(format!("[{}] ⏬ {}", i + 1, label)));
        }
    } else {
        items.push(ListItem::new(Span::styled(
            "No Parents (Initial Commit?)",
            Style::default().fg(Color::DarkGray),
        )));
    }

    items.push(ListItem::new(""));

    // Children (Up)
    if !node.children.is_empty() {
        items.push(ListItem::new(Span::styled(
            "CHILDREN (Forward in time):",
            Style::default().add_modifier(Modifier::UNDERLINED),
        )));
        for (i, c_hash) in node.children.iter().enumerate() {
            let label = if let Some(c_node) = game.nodes.get(c_hash) {
                format!(
                    "{} - {}",
                    c_node.short_hash,
                    c_node.message.lines().next().unwrap_or("").trim()
                )
            } else {
                format!("{} (Unknown)", &c_hash[..7])
            };
            // Use Shift+N logic display
            items.push(ListItem::new(format!("[Shift+{}] ⏫ {}", i + 1, label)));
        }
    } else {
        items.push(ListItem::new(Span::styled(
            "No Children (HEAD?)",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let list = List::new(items).block(exits_block);
    f.render_widget(list, chunks[1]);
}

fn draw_log(f: &mut Frame, game: &Game, area: Rect) {
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded).title(" 📜 Log ");
    let mut lines = Vec::new();
    for msg in game.log.iter().rev() {
        let style = if msg.contains("damage") || msg.contains("FAILURE") {
            Style::default().fg(Color::Red)
        } else if msg.contains("XP") || msg.contains("refreshed") {
            Style::default().fg(Color::Green)
        } else if msg.contains("Entered Commit") {
             Style::default().fg(Color::Blue)
        } else {
            Style::default()
        };
        lines.push(Line::from(Span::styled(msg, style)));
    }
    let p = Paragraph::new(lines).block(block);
    f.render_widget(p, area);
}

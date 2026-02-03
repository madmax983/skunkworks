use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

use crate::pet::{Mood, Pet};

pub fn render(f: &mut Frame, pet: &Pet) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Title
            Constraint::Min(10),   // Pet View
            Constraint::Length(3), // Stats
            Constraint::Length(3), // Help
        ])
        .split(f.area());

    render_title(f, chunks[0]);
    render_pet(f, chunks[1], pet);
    render_stats(f, chunks[2], pet);
    render_help(f, chunks[3]);
}

fn render_title(f: &mut Frame, area: Rect) {
    let title = Paragraph::new("👾 Git-Gotchi 👾")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, area);
}

fn render_pet(f: &mut Frame, area: Rect, pet: &Pet) {
    let (art, color) = match pet.mood {
        Mood::Happy => (
            vec!["   ( ^_^)   ", "   (>  <)   ", "   ======   "],
            Color::Green,
        ),
        Mood::Neutral => (
            vec!["   ( o_o)   ", "   (>  <)   ", "   ======   "],
            Color::Yellow,
        ),
        Mood::Sad => (
            vec!["   ( ;_;)   ", "   (>  <)   ", "   ======   "],
            Color::Blue,
        ),
        Mood::Dead => (
            vec!["   ( x_x)   ", "   (>  <)   ", "   ======   "],
            Color::Red,
        ),
    };

    let art_text = Text::from(
        art.iter()
            .map(|line| Line::from(Span::styled(*line, Style::default().fg(color))))
            .collect::<Vec<_>>(),
    );

    let p = Paragraph::new(art_text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" The Pet "));

    // Center vertically manually if needed, but Alignment::Center handles horizontal.
    // To handle vertical centering in Paragraph, we might need a Flex layout or just padding.
    // For now, simple is fine.

    f.render_widget(p, area);
}

fn render_stats(f: &mut Frame, area: Rect, pet: &Pet) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let hunger_percent = (pet.hunger / 100.0).clamp(0.0, 1.0);
    let hunger_label = format!("{:.1}%", pet.hunger);
    let hunger_gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Hunger"))
        .gauge_style(Style::default().fg(if pet.hunger > 80.0 {
            Color::Red
        } else {
            Color::Green
        }))
        .ratio(hunger_percent)
        .label(hunger_label);

    f.render_widget(hunger_gauge, layout[0]);

    let xp_text = format!(
        "XP: {} (Lvl {}) | Today: {} commits",
        pet.xp,
        pet.get_level(),
        pet.activity_today
    );
    let xp_widget = Paragraph::new(xp_text)
        .block(Block::default().borders(Borders::ALL).title("Stats"))
        .alignment(Alignment::Center);

    f.render_widget(xp_widget, layout[1]);
}

fn render_help(f: &mut Frame, area: Rect) {
    let help = Paragraph::new("Quit: q | Force Feed (Cheat): f | Refresh: r")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(help, area);
}

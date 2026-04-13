use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};

// Heatmap colors from chrontext
const COLD: Color = Color::Rgb(30, 30, 80);
const COOL: Color = Color::Rgb(40, 80, 150);
const WARM: Color = Color::Rgb(150, 100, 40);
const HOT: Color = Color::Rgb(220, 60, 40);

fn get_color(score: f64) -> Color {
    if score < 0.25 {
        COLD
    } else if score < 0.5 {
        COOL
    } else if score < 0.75 {
        WARM
    } else {
        HOT
    }
}

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.area());

    draw_text_pane(f, app, chunks[0]);
    draw_swarm_pane(f, app, chunks[1]);
}

fn draw_text_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let mut lines = Vec::new();

    let height = area.height.saturating_sub(2) as usize;
    app.update_scroll(height);

    let start = app.scroll;
    let end = (start + height).min(app.content.len());

    for i in start..end {
        let content = &app.content[i];
        let info = app.blame_info.get(i);
        let is_selected = i == app.selected_line;

        lines.push(format_line(content, info, is_selected));
    }

    let block = Block::default()
        .title(format!(" 📜 {} ", app.path))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, area);
}

fn format_line<'a>(
    content: &'a str,
    info: Option<&'a crate::blame::LineInfo>,
    is_selected: bool,
) -> Line<'a> {
    let bg_color = if is_selected {
        Color::Rgb(60, 60, 60)
    } else {
        Color::Reset
    };

    let Some(info) = info else {
        return Line::from(Span::styled(
            content.replace('\t', "    "),
            Style::default().fg(Color::DarkGray).bg(bg_color),
        ));
    };

    let fg_color = get_color(info.age_score);
    let age_indicator = if info.age_score > 0.8 {
        "🔥"
    } else if info.age_score < 0.2 {
        "❄️ "
    } else {
        "  "
    };

    Line::from(vec![
        Span::styled(
            format!("{} {:8} ", age_indicator, info.commit_hash),
            Style::default().fg(fg_color).bg(bg_color),
        ),
        Span::styled(
            format!("{:15} ", info.author.chars().take(15).collect::<String>()),
            Style::default().fg(Color::DarkGray).bg(bg_color),
        ),
        Span::styled(
            format!("| {} ", content.replace('\t', "    ")),
            Style::default().fg(fg_color).bg(bg_color),
        ),
    ])
}

fn draw_swarm_pane(f: &mut Frame, app: &mut App, area: Rect) {
    let block = Block::default()
        .title(" 🦋 Swarm ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::DarkGray));

    let inner_area = block.inner(area);
    app.bounds = (
        inner_area.width as f64 * 2.0,
        inner_area.height as f64 * 2.0,
    );

    let positions: Vec<(f64, f64)> = app.positions.iter().map(|p| (p.x, p.y)).collect();

    let canvas = Canvas::default()
        .block(block)
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, app.bounds.0])
        .y_bounds([0.0, app.bounds.1])
        .paint(|ctx| {
            ctx.draw(&Points {
                coords: &positions,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, area);
}

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(f.area());

    draw_content(f, app, chunks[0]);
    draw_status(f, app, chunks[1]);
}

fn draw_content(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Chrontext: {} ", app.path));

    let inner_area = block.inner(area);
    f.render_widget(block, area);

    let height = inner_area.height as usize;
    let start_line = app.scroll;
    let end_line = (start_line + height).min(app.content.len());

    for (i, line_idx) in (start_line..end_line).enumerate() {
        let line_num = line_idx + 1;
        let content = &app.content[line_idx];

        // Find blame info
        let info = app.blame_info.iter().find(|l| l.line_number == line_num);

        let color = if let Some(info) = info {
            let s = info.age_score;
            // Cold: (0, 0, 255)
            // Hot: (255, 255, 255)
            // Linear interpolation
            let val = (s * 255.0) as u8;
            // Blue channel is always high for cold, but for white it stays high.
            // R, G increase with s. B stays high.
            // s=0 -> 0,0,255 (Blue)
            // s=1 -> 255,255,255 (White)
            Color::Rgb(val, val, 255)
        } else {
            Color::Gray
        };

        let style = if line_idx == app.selected_line {
            Style::default().fg(Color::Black).bg(color).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(color)
        };

        let line_widget = Line::from(vec![
            Span::styled(format!("{:4} ", line_num), Style::default().fg(Color::DarkGray)),
            Span::styled(content, style),
        ]);

        let y = inner_area.y + i as u16;
        if y < inner_area.bottom() {
             f.render_widget(Paragraph::new(line_widget), Rect::new(inner_area.x, y, inner_area.width, 1));
        }
    }
}

fn draw_status(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Info ");
    let inner = block.inner(area);
    f.render_widget(block, area);

    let line_num = app.selected_line + 1;
    let info = app.blame_info.iter().find(|l| l.line_number == line_num);

    let text = if let Some(info) = info {
        format!(
            "Hash: {} | Author: {} | Date: {} | Age: {:.2} | Msg: {}",
            info.commit_hash,
            info.author,
            info.date.format("%Y-%m-%d %H:%M"),
            info.age_score,
            info.message
        )
    } else {
        "No blame info".to_string()
    };

    f.render_widget(Paragraph::new(text).style(Style::default().fg(Color::White)), inner);
}

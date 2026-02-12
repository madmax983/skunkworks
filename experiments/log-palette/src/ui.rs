use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    widgets::canvas::{Canvas, Context, Rectangle},
    Frame,
};
use crate::{App, ViewMode};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    match app.mode {
        ViewMode::Text => draw_text_view(f, app, chunks[0]),
        ViewMode::Spectrum => draw_spectrum_view(f, app, chunks[0]),
    }

    draw_status_bar(f, app, chunks[1]);
}

fn draw_text_view(f: &mut Frame, app: &mut App, area: Rect) {
    // Viewport height is `area.height`.
    // We adjust for borders (height - 2) roughly.
    let height = area.height.saturating_sub(2) as usize;
    let start = app.scroll;
    let end = (start + height).min(app.logs.len());

    let logs_slice = if start < app.logs.len() {
        &app.logs[start..end]
    } else {
        &[]
    };

    let items: Vec<ListItem> = logs_slice
        .iter()
        .map(|log| {
            // Use raw line content
            let content = log.entry.raw.clone();
            let style = Style::default().fg(log.color);

            ListItem::new(Line::from(vec![
                Span::styled(content, style)
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Logs (Text Mode) "));

    f.render_widget(list, area);
}

fn draw_spectrum_view(f: &mut Frame, app: &mut App, area: Rect) {
    // Barcode view.
    // Each log is a vertical line.
    // We fit as many as possible, or scroll.

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Palette (Spectrum Mode) "))
        .x_bounds([0.0, area.width as f64])
        .y_bounds([0.0, area.height as f64])
        .paint(|ctx: &mut Context| {
            let width = area.width as usize;
            let start = app.scroll;
            // Draw one bar per column?
            // Or fill the screen?
            // Let's say each log is 1 unit wide.

            for i in 0..width {
                let idx = start + i;
                if idx >= app.logs.len() {
                    break;
                }

                let log = &app.logs[idx];

                // Height based on sentiment or length?
                // Full height for barcode look.
                let h = area.height as f64;

                // Draw rectangle
                // ctx.draw(&Rectangle { ... })
                // Ratatui Canvas Rectangle takes x, y, width, height, color

                ctx.draw(&Rectangle {
                    x: i as f64,
                    y: 0.0,
                    width: 1.0,
                    height: h,
                    color: log.color,
                });
            }
        });

    f.render_widget(canvas, area);
}

fn draw_status_bar(f: &mut Frame, app: &mut App, area: Rect) {
    let mode_str = match app.mode {
        ViewMode::Text => "TEXT",
        ViewMode::Spectrum => "SPECTRUM",
    };

    let text = format!(
        " Mode: {} | Scroll: {}/{} | 'Tab': Switch | 'q': Quit | 'j'/'k': Scroll ",
        mode_str,
        app.scroll,
        app.logs.len()
    );

    let p = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(p, area);
}

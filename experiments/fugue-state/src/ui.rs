use crate::app::App;
use crate::parser::Timbre;
#[cfg(feature = "nova")]
use ratatui::text::{Line, Span};
use ratatui::widgets::canvas::{Canvas, Line as CanvasLine, Rectangle};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn ui(f: &mut Frame, app: &App) {
    #[cfg(feature = "nova")]
    let (left_area, right_area) = {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(f.area());
        (chunks[0], Some(chunks[1]))
    };

    #[cfg(not(feature = "nova"))]
    let (left_area, _right_area) = (f.area(), Option::<Rect>::None);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(left_area);

    let title_text = format!("Fugue State: Playing {} events", app.events.len());
    let title = Paragraph::new(title_text)
        .block(Block::default().borders(Borders::ALL).title("Information"))
        .style(Style::default().fg(Color::Cyan));
    f.render_widget(title, chunks[0]);

    #[cfg(feature = "nova")]
    if let Some(area) = right_area {
        draw_code_view(f, app, area);
    }

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Score"))
        .x_bounds([0.0, app.total_duration as f64])
        .y_bounds([0.0, 1000.0])
        .paint(|ctx| {
            for (i, event) in app.events.iter().enumerate() {
                let start_time = app.event_start_times[i] as f64;
                let duration = event.duration as f64;
                let freq = event.frequency as f64;

                let color = if i == app.current_event_index {
                    Color::Yellow
                } else {
                    match event.timbre {
                        Timbre::Sine => Color::Green,
                        Timbre::Triangle => Color::Magenta,
                        Timbre::Square => Color::Red,
                        Timbre::Sawtooth => Color::Blue,
                    }
                };

                ctx.draw(&Rectangle {
                    x: start_time,
                    y: freq,
                    width: duration,
                    height: 10.0,
                    color,
                });
            }

            ctx.draw(&CanvasLine {
                x1: app.elapsed as f64,
                y1: 0.0,
                x2: app.elapsed as f64,
                y2: 1000.0,
                color: Color::White,
            });
        });

    f.render_widget(canvas, chunks[1]);

    let current_desc = if app.current_event_index < app.events.len() {
        &app.events[app.current_event_index].description
    } else {
        "Finished"
    };

    let status = Paragraph::new(format!(
        "Event: {} | Time: {:.2}s / {:.2}s",
        current_desc, app.elapsed, app.total_duration
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

#[cfg(feature = "nova")]
fn draw_code_view(f: &mut Frame, app: &App, area: Rect) {
    if app.source_code.is_empty() {
        return;
    }

    let current_event = if app.current_event_index < app.events.len() {
        Some(&app.events[app.current_event_index])
    } else {
        None
    };

    let mut lines = Vec::new();
    let mut scroll_offset = 0;
    // Highlight Style
    let highlight_style = Style::default().bg(Color::Yellow).fg(Color::Black);

    for (i, line_content) in app.source_code.iter().enumerate() {
        let line_num = i + 1;
        let mut spans = Vec::new();

        // Line number
        spans.push(Span::styled(
            format!("{:4} ", line_num),
            Style::default().fg(Color::DarkGray),
        ));

        let mut highlighted = false;
        if let Some(event) = current_event {
            if let Some(span) = event.span {
                if line_num >= span.start_line && line_num <= span.end_line {
                    highlighted = true;
                    // Set scroll offset to center the active line
                    if line_num == span.start_line && area.height > 0 {
                        scroll_offset = i.saturating_sub(area.height as usize / 2);
                    }

                    if line_num == span.start_line && line_num == span.end_line {
                        // Single line span logic
                        let start_col = span.start_col;
                        let end_col = span.end_col;

                        // Convert char indices to byte indices safely
                        let char_indices: Vec<(usize, char)> =
                            line_content.char_indices().collect();
                        let len_bytes = line_content.len();

                        let s_byte = char_indices
                            .get(start_col)
                            .map(|(i, _)| *i)
                            .unwrap_or(len_bytes);
                        let e_byte = char_indices
                            .get(end_col)
                            .map(|(i, _)| *i)
                            .unwrap_or(len_bytes);

                        let s = s_byte.min(len_bytes);
                        let e = e_byte.max(s).min(len_bytes);

                        spans.push(Span::raw(&line_content[..s]));
                        spans.push(Span::styled(&line_content[s..e], highlight_style));
                        spans.push(Span::raw(&line_content[e..]));
                    } else {
                        // Multi-line span: just highlight the whole line for simplicity
                        spans.push(Span::styled(line_content, highlight_style));
                    }
                }
            }
        }

        if !highlighted {
            spans.push(Span::raw(line_content));
        }

        lines.push(Line::from(spans));
    }

    let code_paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Source Code"))
        .scroll((scroll_offset as u16, 0));

    f.render_widget(code_paragraph, area);
}

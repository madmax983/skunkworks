use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color},
    widgets::{Block, Borders, List, ListItem, canvas::{Canvas, Line, Context}},
    Frame,
};
use crate::app::App;

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.area());

    draw_canvas(f, app, chunks[0]);
    draw_stack(f, app, chunks[1]);
}

fn draw_canvas(f: &mut Frame, app: &App, area: Rect) {
    let width = app.turtle.bounds.1 - app.turtle.bounds.0;
    let height = app.turtle.bounds.3 - app.turtle.bounds.2;
    // Ensure minimal bounds to avoid crash on 0 size
    let width = if width < 1.0 { 10.0 } else { width };
    let height = if height < 1.0 { 10.0 } else { height };

    let pad_x = width * 0.1 + 5.0;
    let pad_y = height * 0.1 + 5.0;

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(format!("Growth (Gen {}, Step {}/{})", app.generation, app.pc, app.expanded_string.len())))
        .x_bounds([app.turtle.bounds.0 - pad_x, app.turtle.bounds.1 + pad_x])
        .y_bounds([app.turtle.bounds.2 - pad_y, app.turtle.bounds.3 + pad_y])
        .paint(|ctx: &mut Context| {
            for (x1, y1, x2, y2) in &app.turtle.lines {
                ctx.draw(&Line {
                    x1: *x1,
                    y1: *y1,
                    x2: *x2,
                    y2: *y2,
                    color: Color::Green,
                });
            }

            ctx.draw(&ratatui::widgets::canvas::Points {
                coords: &[(app.turtle.current.x, app.turtle.current.y)],
                color: Color::Red,
            });
        });
    f.render_widget(canvas, area);
}

fn draw_stack(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.turtle.stack.iter().rev().enumerate().map(|(i, state)| {
        let depth = app.turtle.stack.len() - i;
        ListItem::new(format!("[{:02}] X:{:.1}, Y:{:.1}", depth, state.x, state.y))
    }).collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(format!("Call Stack (Depth: {})", app.turtle.stack.len())));

    f.render_widget(list, area);
}

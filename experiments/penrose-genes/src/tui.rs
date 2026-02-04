use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use crate::penrose::{PenroseTiling, Point, TriangleType};
use crate::vm::Value;

pub fn ui(f: &mut Frame, tiling: &PenroseTiling<Value>, stack: &[Value], output: &[String], energy: i64) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(10)])
        .split(f.area());

    let canvas_area = chunks[0];
    let bottom_area = chunks[1];

    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(bottom_area);

    let stack_area = bottom_chunks[0];
    let output_area = bottom_chunks[1];

    // Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Penrose Memory"))
        .x_bounds([-120.0, 120.0])
        .y_bounds([-120.0, 120.0])
        .paint(move |ctx| {
            for (i, t) in tiling.triangles.iter().enumerate() {
                let data = &tiling.data[i];
                let color = match data {
                    Value::Int(n) => {
                        if *n == 0 {
                             match t.t_type {
                                TriangleType::Acute => Color::DarkGray,
                                TriangleType::Obtuse => Color::Gray,
                            }
                        } else {
                            match n.abs() % 6 {
                                0 => Color::Red,
                                1 => Color::Green,
                                2 => Color::Yellow,
                                3 => Color::Blue,
                                4 => Color::Magenta,
                                5 => Color::Cyan,
                                _ => Color::White,
                            }
                        }
                    },
                    Value::Str(_) => Color::White,
                };

                // Draw edges
                // A-B
                ctx.draw(&CanvasLine {
                    x1: t.a.x, y1: t.a.y,
                    x2: t.b.x, y2: t.b.y,
                    color,
                });
                // B-C
                ctx.draw(&CanvasLine {
                    x1: t.b.x, y1: t.b.y,
                    x2: t.c.x, y2: t.c.y,
                    color,
                });
                // C-A
                ctx.draw(&CanvasLine {
                    x1: t.c.x, y1: t.c.y,
                    x2: t.a.x, y2: t.a.y,
                    color,
                });
            }
        });

    f.render_widget(canvas, canvas_area);

    // Stack
    let stack_text: Vec<Line> = stack.iter().rev().take(8).map(|v| Line::from(format!("{}", v))).collect();
    let stack_widget = Paragraph::new(stack_text)
        .block(Block::default().borders(Borders::ALL).title(format!("Stack (Energy: {})", energy)));
    f.render_widget(stack_widget, stack_area);

    // Output
    let output_text: Vec<Line> = output.iter().rev().take(8).map(|s| Line::from(s.as_str())).collect();
    let output_widget = Paragraph::new(output_text)
        .block(Block::default().borders(Borders::ALL).title("Output"));
    f.render_widget(output_widget, output_area);
}

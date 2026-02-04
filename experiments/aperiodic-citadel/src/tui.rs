use crate::penrose::{PenroseTiling, Point, TriangleType};
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

pub fn ui(f: &mut Frame, tiling: &PenroseTiling, player_idx: usize, offset: Point, zoom: f64) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    // Info Bar
    let player_tile = &tiling.triangles[player_idx];
    let center = player_tile.center();
    let info_text = vec![
        Line::from(vec![
            Span::styled("Aperiodic Citadel", Style::default().fg(Color::Cyan)),
            Span::raw(" | "),
            Span::styled(
                format!("Pos: {:.2}, {:.2}", center.x, center.y),
                Style::default().fg(Color::Yellow),
            ),
            Span::raw(format!(" | Zoom: {:.2}", zoom)),
        ]),
        Line::from(vec![Span::raw("WASD: Move | +/-: Zoom | Q: Quit")]),
    ];

    let info = Paragraph::new(info_text).block(Block::default().borders(Borders::ALL));

    f.render_widget(info, info_area);

    // Canvas
    // Calculate bounds based on zoom and offset
    // Viewport width/height ratio?
    // Canvas coordinate system is abstract.
    // Let's assume window is -100 to 100 scaled by zoom.

    let width = 100.0 / zoom;
    let height = 100.0 / zoom;

    let x_bounds = [offset.x - width, offset.x + width];
    let y_bounds = [offset.y - height, offset.y + height];

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Penrose Projection"),
        )
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(move |ctx| {
            // Draw Edges
            for (i, t) in tiling.triangles.iter().enumerate() {
                // Optimization: Only draw if visible?
                // Simple bounding box check
                if t.a.x < x_bounds[0] - 10.0
                    || t.a.x > x_bounds[1] + 10.0
                    || t.a.y < y_bounds[0] - 10.0
                    || t.a.y > y_bounds[1] + 10.0
                {
                    continue;
                }

                let color = match t.t_type {
                    TriangleType::Acute => Color::Magenta,
                    TriangleType::Obtuse => Color::Blue,
                };

                // Draw edges
                // A-B
                ctx.draw(&CanvasLine {
                    x1: t.a.x,
                    y1: t.a.y,
                    x2: t.b.x,
                    y2: t.b.y,
                    color,
                });
                // B-C
                ctx.draw(&CanvasLine {
                    x1: t.b.x,
                    y1: t.b.y,
                    x2: t.c.x,
                    y2: t.c.y,
                    color,
                });
                // C-A
                ctx.draw(&CanvasLine {
                    x1: t.c.x,
                    y1: t.c.y,
                    x2: t.a.x,
                    y2: t.a.y,
                    color,
                });

                // Draw Player
                if i == player_idx {
                    let c = t.center();
                    // Draw a small cross or box
                    let size = 1.0;
                    ctx.draw(&CanvasLine {
                        x1: c.x - size,
                        y1: c.y - size,
                        x2: c.x + size,
                        y2: c.y + size,
                        color: Color::White,
                    });
                    ctx.draw(&CanvasLine {
                        x1: c.x - size,
                        y1: c.y + size,
                        x2: c.x + size,
                        y2: c.y - size,
                        color: Color::White,
                    });
                }
            }
        });

    f.render_widget(canvas, canvas_area);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::penrose::PenroseTiling;
    use ratatui::backend::TestBackend;
    use ratatui::Terminal;

    #[test]
    fn test_ui_render() {
        let backend = TestBackend::new(100, 50);
        let mut terminal = Terminal::new(backend).unwrap();

        let tiling = PenroseTiling::generate_sun(100.0);
        // No subdivision for speed

        terminal
            .draw(|f| {
                ui(f, &tiling, 0, Point::new(0.0, 0.0), 1.0);
            })
            .unwrap();
    }
}

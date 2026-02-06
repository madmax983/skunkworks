use crate::graph::CommitGraph;
use git2::Oid;
use poincare_disk::Point;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line as TextLine, Span},
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, Borders, Paragraph, Wrap,
    },
    Frame,
};

pub fn draw_ui(
    f: &mut Frame,
    graph: &CommitGraph,
    layout: &[(Oid, Point)],
    focus_oid: Oid,
    selected_oid: Option<Oid>,
) {
    let area = f.area();
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(area);

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Hyperbolic Git (Poincaré Disk)"),
        )
        .x_bounds([-1.05, 1.05])
        .y_bounds([-1.05, 1.05])
        .paint(|ctx| {
            // Draw unit circle boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            // Draw edges
            for (oid, pos) in layout {
                if let Some(data) = graph.commits.get(oid) {
                    for parent in &data.parents {
                        // Find parent position in layout
                        if let Some((_, parent_pos)) = layout.iter().find(|(id, _)| id == parent) {
                            ctx.draw(&Line {
                                x1: pos.re,
                                y1: pos.im,
                                x2: parent_pos.re,
                                y2: parent_pos.im,
                                color: Color::DarkGray,
                            });
                        }
                    }
                    // We assume checking parents is enough to draw all edges between visible nodes
                    // because the edge exists in the parent's list if it's a child, etc.
                    // Wait, graph stores parents. So if both child and parent are in layout,
                    // iterating child will draw the edge.
                }
            }

            // Draw nodes
            for (oid, pos) in layout {
                let is_focused = *oid == focus_oid;
                let is_selected = Some(*oid) == selected_oid;

                let color = if is_focused {
                    Color::Yellow
                } else if is_selected {
                    Color::Cyan
                } else {
                    Color::Blue
                };

                let radius = if is_focused { 0.03 } else { 0.02 };

                ctx.draw(&Circle {
                    x: pos.re,
                    y: pos.im,
                    radius,
                    color,
                });

                // Label for focused/selected or nearby
                if is_focused || is_selected || pos.norm() < 0.5 {
                    let label = format!("{}", &oid.to_string()[0..6]);
                    ctx.print(pos.re + radius, pos.im, label);
                }
            }
        });

    f.render_widget(canvas, canvas_area);

    // Info Panel
    let info_block = Block::default().borders(Borders::ALL).title("Commit Info");

    let display_oid = selected_oid.unwrap_or(focus_oid);

    let info_text = if let Some(data) = graph.commits.get(&display_oid) {
        vec![
            TextLine::from(vec![
                Span::styled("Commit: ", Style::default().fg(Color::Cyan)),
                Span::raw(display_oid.to_string()),
            ]),
            TextLine::from(vec![
                Span::styled("Author: ", Style::default().fg(Color::Yellow)),
                Span::raw(&data.author),
            ]),
            TextLine::from(Span::raw("")),
            TextLine::from(Span::styled("Message:", Style::default().fg(Color::Green))),
            TextLine::from(Span::raw(&data.message)),
            TextLine::from(Span::raw("")),
            TextLine::from(Span::styled(
                "Controls:",
                Style::default().fg(Color::Magenta),
            )),
            TextLine::from("Arrows: Navigate selection"),
            TextLine::from("Enter: Center on selection"),
            TextLine::from("q/Esc: Quit"),
        ]
    } else {
        vec![TextLine::from("Loading...")]
    };

    let paragraph = Paragraph::new(info_text)
        .block(info_block)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, info_area);
}

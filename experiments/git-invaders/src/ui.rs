use crate::diff::LineType;
use crate::game::Game;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Span,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, game: &Game) {
    let area = f.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1), // Footer/Status
        ])
        .split(area);

    let main_area = chunks[0];
    let footer_area = chunks[1];

    if game.game_over {
        let p = Paragraph::new(format!(
            "GAME OVER\nScore: {}\nPress 'q' to quit.",
            game.score
        ))
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));

        let center = centered_rect(60, 20, main_area);
        f.render_widget(p, center);
        return;
    }

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Git Invaders "),
        )
        .marker(ratatui::symbols::Marker::Braille) // Braille allows finer resolution if used for shapes, but we use print
        .x_bounds([0.0, game.width])
        .y_bounds([game.height, 0.0]) // Inverted Y: 0 is Top, Height is Bottom
        .paint(|ctx| {
            // Draw Player
            let px = game.player_x;
            let py = game.height - 3.0; // Slightly above bottom
            ctx.print(
                px,
                py,
                Span::styled("/^\\", Style::default().fg(Color::Cyan)),
            );

            // Draw Bullets
            for b in &game.bullets {
                ctx.print(
                    b.x,
                    b.y,
                    Span::styled("|", Style::default().fg(Color::Yellow)),
                );
            }

            // Draw Enemies
            for e in &game.enemies {
                let color = match e.line.line_type {
                    LineType::Addition => Color::Green,
                    LineType::Deletion => Color::Red,
                    LineType::Context => Color::DarkGray,
                };

                // Truncate content if needed
                let content = if e.line.content.len() > e.max_width {
                    &e.line.content[..e.max_width]
                } else {
                    &e.line.content
                };

                ctx.print(
                    e.x,
                    e.y,
                    Span::styled(content.to_string(), Style::default().fg(color)),
                );
            }

            // Draw Score
            ctx.print(
                1.0,
                1.0,
                Span::styled(
                    format!("Score: {}", game.score),
                    Style::default().fg(Color::White),
                ),
            );
            ctx.print(
                1.0,
                2.0,
                Span::styled(
                    format!("Wave: {}", game.pending_lines.len()),
                    Style::default().fg(Color::Gray),
                ),
            );
        });

    f.render_widget(canvas, main_area);

    // Footer
    let footer = Paragraph::new("Left/Right: Move | Space: Shoot | Q: Quit")
        .style(Style::default().fg(Color::Gray));
    f.render_widget(footer, footer_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    let horiz_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1]);

    horiz_layout[1]
}

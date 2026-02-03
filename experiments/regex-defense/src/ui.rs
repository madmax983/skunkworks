use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame,
};

use crate::game::Game;

pub fn ui(f: &mut Frame, game: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(3), // Input box
            Constraint::Length(1), // Help/Status
        ])
        .split(f.area());

    draw_game_area(f, chunks[0], game);
    draw_input_area(f, chunks[1], game);
    draw_status_bar(f, chunks[2], game);
}

fn draw_game_area(f: &mut Frame, area: Rect, game: &Game) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Regex Defense "),
        )
        .x_bounds([0.0, game.width])
        .y_bounds([0.0, game.height])
        .paint(|ctx| {
            for enemy in &game.enemies {
                // Invert Y because Canvas is Y-up, but Game is Y-down
                let draw_y = game.height - enemy.pos.y;

                // Don't draw if out of bounds (though Canvas handles this)
                if draw_y >= 0.0 && draw_y <= game.height {
                    ctx.print(
                        enemy.pos.x,
                        draw_y,
                        Span::styled(enemy.text.clone(), Style::default().fg(Color::Red)),
                    );
                }
            }
        });

    f.render_widget(canvas, area);
}

fn draw_input_area(f: &mut Frame, area: Rect, game: &Game) {
    let style = if game.last_regex_error.is_some() {
        Style::default().fg(Color::Red)
    } else {
        Style::default().fg(Color::Yellow)
    };

    let input_text = vec![Line::from(vec![
        Span::raw("Regex > "),
        Span::styled(&game.input, style),
        Span::raw("_"), // Cursor
    ])];

    let block = Block::default().borders(Borders::ALL).title(" Input ");

    let p = Paragraph::new(input_text).block(block);
    f.render_widget(p, area);
}

fn draw_status_bar(f: &mut Frame, area: Rect, game: &Game) {
    let status = if game.game_over {
        Span::styled(
            " GAME OVER - Press 'R' to Restart ",
            Style::default().bg(Color::Red).fg(Color::White),
        )
    } else if let Some(err) = &game.last_regex_error {
        Span::styled(
            format!(" Error: {} ", err),
            Style::default().bg(Color::Red).fg(Color::White),
        )
    } else {
        Span::styled(
            format!(
                " Score: {} | Enemies: {} | 'ESC' to Quit ",
                game.score,
                game.enemies.len()
            ),
            Style::default().bg(Color::Blue).fg(Color::White),
        )
    };

    f.render_widget(Paragraph::new(status), area);
}

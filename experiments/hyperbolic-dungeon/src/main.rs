pub mod dungeon;
pub mod render;
pub mod game;
pub mod entity;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use game::Game;
use poincare_disk::{Mobius};
use render::draw_dungeon;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut game = Game::new();
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut game))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Left | KeyCode::Char('a') => game.move_player(-0.05, 0.0),
                    KeyCode::Right | KeyCode::Char('d') => game.move_player(0.05, 0.0),
                    KeyCode::Up | KeyCode::Char('w') => game.move_player(0.0, 0.05),
                    KeyCode::Down | KeyCode::Char('s') => game.move_player(0.0, -0.05),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= Duration::from_millis(50) {
            game.update();
            last_tick = Instant::now();
        }
    }

    tui.exit()?;
    Ok(())
}

fn ui(f: &mut Frame, game: &mut Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];
    let info_area = chunks[1];

    // Info
    let player = game.get_player();
    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::styled("Hyperbolic Dungeon", Style::default().fg(Color::Cyan)),
            Span::raw(format!(
                " | Path: {:?} | Pos: {:.2}, {:.2}",
                player.path, player.offset.re, player.offset.im
            )),
        ]),
        Line::from(vec![Span::raw(format!("Message: {}", game.message))]),
    ])
    .block(Block::default().borders(Borders::ALL));

    f.render_widget(info, info_area);

    // Canvas
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Poincaré Disk"),
        )
        .x_bounds([-1.05, 1.05])
        .y_bounds([-1.05, 1.05])
        .paint(|ctx| {
            // Draw boundary
            ctx.draw(&Circle {
                x: 0.0,
                y: 0.0,
                radius: 1.0,
                color: Color::White,
            });

            let player = game.get_player();
            let view_transform = Mobius::inverse_translation(player.offset);

            draw_dungeon(
                ctx,
                game,
                &view_transform,
            );
        });

    f.render_widget(canvas, canvas_area);
}

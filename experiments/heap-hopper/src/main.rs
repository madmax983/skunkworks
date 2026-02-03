pub mod allocator;
pub mod game;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::Game;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Width of the heap should match terminal width roughly
    // But we don't know it yet. Let's start with 100.
    let width = tui.terminal.size()?.width as usize;
    let mut app = Game::new(width.max(50));

    // Seed the heap a bit more
    app.heap.allocate(30);
    app.heap.allocate(20);

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut Game) -> Result<()> {
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('r') => {
                            // Restart
                            let width = app.heap.total_size;
                            *app = Game::new(width);
                            app.heap.allocate(30);
                            app.heap.allocate(20);
                        }
                        KeyCode::Char(' ') | KeyCode::Up | KeyCode::Char('w') => app.jump(),
                        KeyCode::Left | KeyCode::Char('a') => app.player.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => app.player.move_right(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn ui(f: &mut Frame, app: &Game) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Main Game Area
    let game_area = chunks[0];
    let ground_y = game_area.height as f64 - 2.0;

    // 1. Draw Heap (Ground)
    let mut spans = Vec::new();

    for block in &app.heap.blocks {
        let width = block.size;
        let style = if block.is_allocated {
            Style::default().bg(Color::Green).fg(Color::Black)
        } else {
            Style::default().bg(Color::Reset).fg(Color::DarkGray)
        };

        let content = " ".repeat(width);

        // If the gap is huge, we might not render it all if it goes off screen.
        // But for now, simple render.
        spans.push(Span::styled(content, style));
    }

    let ground_line = Line::from(spans);

    // Render Ground
    f.render_widget(
        Paragraph::new(ground_line),
        ratatui::layout::Rect::new(
            game_area.x,
            game_area.y + ground_y as u16 + 1,
            game_area.width,
            1,
        ),
    );

    // 2. Draw Player
    if !app.player.is_dead {
        let px = app.player.x as u16;
        let py_offset = app.player.y; // Positive is up
        let py = (ground_y - py_offset).max(0.0) as u16;

        if px < game_area.width && py < game_area.height {
            f.render_widget(
                Paragraph::new("P").style(Style::default().fg(Color::Cyan).bold()),
                ratatui::layout::Rect::new(game_area.x + px, game_area.y + py, 1, 1),
            );
        }
    } else {
        // Game Over Text
        let center_y = game_area.height / 2;
        f.render_widget(
            Paragraph::new("SEGMENTATION FAULT (CORE DUMPED)\nPress 'r' to restart")
                .style(Style::default().fg(Color::Red).bold())
                .alignment(ratatui::layout::Alignment::Center),
            ratatui::layout::Rect::new(game_area.x, game_area.y + center_y, game_area.width, 2),
        );
    }

    // 3. Status Bar
    let status = format!(
        "Score: {} | Time: {} | Fragmentation: {:.2}% | Pos: ({:.1}, {:.1})",
        app.score,
        app.time,
        app.heap.fragmentation() * 100.0,
        app.player.x,
        app.player.y
    );
    let instructions = " [WASD/Arrows] Move/Jump | [q] Quit";

    let status_paragraph = Paragraph::new(vec![
        Line::from(status),
        Line::from(instructions).style(Style::default().dim()),
    ])
    .block(Block::default().borders(Borders::TOP));

    f.render_widget(status_paragraph, chunks[1]);
}

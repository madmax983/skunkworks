mod game;
mod git;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use game::{Direction, World};
use git::GitHistory;
use ratatui::{
    layout::{Constraint, Direction as LayoutDirection, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{canvas::Canvas, Block, Borders, List, ListItem, Paragraph},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

fn main() -> Result<()> {
    // 1. Load Git History
    let history = match GitHistory::load() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("Error loading git history: {}", e);
            eprintln!("Make sure you are running this inside a git repository.");
            return Ok(());
        }
    };

    if history.commits.is_empty() {
        eprintln!("No commits found in git history.");
        return Ok(());
    }

    // 2. Initialize TUI
    let mut tui = Tui::init()?;

    // 3. Initialize Game World
    // We'll set dimensions based on terminal size later, or fixed?
    // Let's start with fixed and scale or center.
    let game_width = 40;
    let game_height = 20;
    let mut world = World::new(game_width, game_height, history);

    // Game loop
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &world))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => break,
                        KeyCode::Char('h') | KeyCode::Left => {
                            world.change_direction(Direction::Left)
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            world.change_direction(Direction::Right)
                        }
                        KeyCode::Char('k') | KeyCode::Up => world.change_direction(Direction::Up),
                        KeyCode::Char('j') | KeyCode::Down => {
                            world.change_direction(Direction::Down)
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            world.update();
            last_tick = Instant::now();
        }

        if world.game_over {
            // Wait for Q
            // We could implement a "Game Over" screen state, but for now just freeze and wait.
            // Actually, let's just let the loop continue but not update, so they can see the score.
            // But we need to allow Q to exit.
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, world: &World) {
    let chunks = Layout::default()
        .direction(LayoutDirection::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Game Board
            Constraint::Percentage(40), // Info Panel
        ])
        .split(f.area());

    // --- Game Board ---
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Git-Ouroboros "),
        )
        .x_bounds([0.0, world.width as f64])
        .y_bounds([0.0, world.height as f64])
        .paint(|ctx| {
            // Draw Snake
            for (i, segment) in world.snake.body.iter().enumerate() {
                let color = if i == 0 {
                    Color::Green
                } else {
                    Color::LightGreen
                };
                // Invert Y: draw_y = height - 1 - y
                let draw_y = (world.height - 1 - segment.y) as f64;

                ctx.print(
                    segment.x as f64,
                    draw_y,
                    Span::styled("█", Style::default().fg(color)),
                );
            }

            // Draw Food
            if let Some(ref food) = world.food {
                let draw_y = (world.height - 1 - food.position.y) as f64;
                // Use the first char of hash as the symbol? Or just a special char.
                let symbol = &food.commit.hash[0..1];
                ctx.print(
                    food.position.x as f64,
                    draw_y,
                    Span::styled(
                        symbol.to_string(),
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ),
                );
            }
        });

    f.render_widget(canvas, chunks[0]);

    // --- Info Panel ---
    let info_chunks = Layout::default()
        .direction(LayoutDirection::Vertical)
        .constraints([
            Constraint::Length(3), // Score
            Constraint::Min(10),   // Log
            Constraint::Length(3), // Controls
        ])
        .split(chunks[1]);

    // Score
    let score_text = format!("Score (Commits Consumed): {}", world.score);
    let score_block = Paragraph::new(score_text)
        .block(Block::default().borders(Borders::ALL).title(" Stats "))
        .style(Style::default().fg(Color::Yellow));
    f.render_widget(score_block, info_chunks[0]);

    // Log
    let mut log_items = Vec::new();
    if let Some(ref food) = world.food {
        log_items.push(ListItem::new(Line::from(vec![
            Span::styled(
                "TARGET: ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!(
                "{} - {}",
                &food.commit.hash[0..7],
                food.commit.author
            )),
        ])));
        log_items.push(ListItem::new(Line::from(Span::styled(
            format!("\"{}\"", food.commit.message),
            Style::default().fg(Color::White),
        ))));
        log_items.push(ListItem::new(Line::from(" ")));
    }

    log_items.push(ListItem::new(Line::from(Span::styled(
        "--- DIGESTED HISTORY ---",
        Style::default().fg(Color::Gray),
    ))));

    for commit in &world.last_eaten_commits {
        log_items.push(ListItem::new(Line::from(vec![
            Span::styled(
                format!("{} ", &commit.hash[0..7]),
                Style::default().fg(Color::Green),
            ),
            Span::raw(format!("by {}", commit.author)),
        ])));
        log_items.push(ListItem::new(Line::from(Span::styled(
            format!("   {}", commit.message),
            Style::default().fg(Color::DarkGray),
        ))));
    }

    let log_list = List::new(log_items)
        .block(Block::default().borders(Borders::ALL).title(" Git Log "))
        .style(Style::default().fg(Color::White));
    f.render_widget(log_list, info_chunks[1]);

    // Controls or Game Over
    let status_text = if world.game_over {
        "GAME OVER! Press 'q' to quit."
    } else {
        "WASD/Arrows to move. Q to quit."
    };

    let status_style = if world.game_over {
        Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let status_block = Paragraph::new(status_text)
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL))
        .style(status_style);
    f.render_widget(status_block, info_chunks[2]);
}

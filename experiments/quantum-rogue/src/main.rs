mod quantum;
mod state;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use quantum::Gate;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use state::{GameState, Tile};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut game = GameState::new(60, 30);
    game.update_entities(); // Initial color update

    let res = run_app(&mut terminal, &mut game);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, game: &mut GameState) -> Result<()>
where
    <B as Backend>::Error: Send + Sync + 'static,
{
    let tick_rate = Duration::from_millis(100);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, game))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Up => game.try_move_player(0, -1),
                        KeyCode::Down => game.try_move_player(0, 1),
                        KeyCode::Left => game.try_move_player(-1, 0),
                        KeyCode::Right => game.try_move_player(1, 0),
                        KeyCode::Char('h') => {
                            // Apply Hadamard to nearest qubit
                            let targets = game.find_nearest_qubits(1);
                            if let Some(&id) = targets.first() {
                                if let Err(e) = game.quantum.apply_gate(Gate::H, id) {
                                    game.message = format!("Hadamard Failed: {}", e);
                                } else {
                                    game.message = "Applied Hadamard Gate.".to_string();
                                    game.update_entities();
                                }
                            } else {
                                game.message = "No Qubits nearby.".to_string();
                            }
                        }
                        KeyCode::Char('x') => {
                            // Apply Pauli-X
                            let targets = game.find_nearest_qubits(1);
                            if let Some(&id) = targets.first() {
                                if let Err(e) = game.quantum.apply_gate(Gate::X, id) {
                                    game.message = format!("Pauli-X Failed: {}", e);
                                } else {
                                    game.message = "Applied Pauli-X Gate.".to_string();
                                    game.update_entities();
                                }
                            }
                        }
                        KeyCode::Char('c') => {
                            // Entangle 2 nearest
                            let targets = game.find_nearest_qubits(2);
                            if targets.len() >= 2 {
                                let id1 = targets[0];
                                let id2 = targets[1];
                                if let Err(e) = game.quantum.entangle(id1, id2) {
                                    game.message = format!("Entanglement Failed: {}", e);
                                } else {
                                    game.message = "Entangled 2 Qubits!".to_string();
                                    game.update_entities();
                                }
                            } else {
                                game.message = "Need 2 Qubits to Entangle.".to_string();
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Game update logic if any
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, game: &GameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(f.area());

    // Title / Score
    let title = Paragraph::new(format!(
        " QUANTUM ROGUE | Score: {} | Inventory: {:?}",
        game.player.score, game.player.inventory
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Grid
    // We render grid as text lines for simplicity
    let mut lines = Vec::new();
    for y in 0..game.height {
        let mut spans = Vec::new();
        for x in 0..game.width {
            // Check entities first (on top)
            if x == game.player.x && y == game.player.y {
                spans.push(Span::styled("@", Style::default().fg(Color::Yellow)));
            } else if let Some(e) = game.entities.iter().find(|e| e.x == x && e.y == y) {
                spans.push(Span::styled(
                    e.glyph.to_string(),
                    Style::default().fg(e.color),
                ));
            } else {
                match game.grid[y][x] {
                    Tile::Wall => {
                        spans.push(Span::styled("#", Style::default().fg(Color::DarkGray)))
                    }
                    Tile::Exit => spans.push(Span::styled("E", Style::default().fg(Color::Green))),
                    Tile::Empty => spans.push(Span::styled(".", Style::default().fg(Color::Gray))),
                }
            }
        }
        lines.push(Line::from(spans));
    }

    // We need to render lines into a Paragraph? Or Canvas?
    // Paragraph is easier for grid logic if lines match height.
    let grid_widget =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(" Map "));
    f.render_widget(grid_widget, chunks[1]);

    // Status
    let status = Paragraph::new(game.message.as_str())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[2]);
}

mod git;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git::get_commit_history;
use gray_scott::GrayScott;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<()> {
    // 1. Fetch Git history
    println!("Fetching git history...");
    let commits = get_commit_history()?;
    if commits.is_empty() {
        eprintln!("No commits found. Are you in a git repository?");
        return Ok(());
    }

    // 2. Setup simulation grid
    let grid_w = 120;
    let grid_h = 60;
    let mut grid = GrayScott::new(grid_w, grid_h);

    // 3. Setup TUI
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut grid, commits, grid_w, grid_h);

    // 4. Teardown
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn hash_to_coords(hash: &str, max_x: usize, max_y: usize) -> (usize, usize) {
    if hash.len() < 4 {
        return (max_x / 2, max_y / 2);
    }

    // Simple hex string to coords mapping
    let x_hex = &hash[0..2];
    let y_hex = &hash[2..4];

    let x_val = usize::from_str_radix(x_hex, 16).unwrap_or(0);
    let y_val = usize::from_str_radix(y_hex, 16).unwrap_or(0);

    let x = (x_val as f64 / 255.0 * max_x as f64) as usize;
    let y = (y_val as f64 / 255.0 * max_y as f64) as usize;

    (x.min(max_x - 1), y.min(max_y - 1))
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    grid: &mut GrayScott,
    commits: Vec<git::Commit>,
    width: usize,
    height: usize,
) -> Result<()> {
    let tick_rate = Duration::from_millis(50);
    let mut last_tick = Instant::now();
    let mut commit_idx = commits.len().saturating_sub(1);

    let mut ticks_since_commit = 0;
    let commits_delay = 10; // Ticks between drops

    // Arrays for drawing
    let mut pattern_points = Vec::with_capacity(width * height);
    let mut drop_points = Vec::new();

    loop {
        // Prepare rendering arrays
        pattern_points.clear();
        drop_points.clear();

        let _u_vals = grid.u();
        let v_vals = grid.v();

        for y in 0..height {
            for x in 0..width {
                let idx = grid.get_index(x, y);
                if idx < v_vals.len() {
                    let v = v_vals[idx];
                    if v > 0.2 {
                        pattern_points.push((x as f64, y as f64));
                    }
                }
            }
        }

        // Current commit drop
        let current_commit = &commits[commit_idx];
        let (cx, cy) = hash_to_coords(&current_commit.hash, width, height);
        drop_points.push((cx as f64, cy as f64));

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" 🧬 Git-Diffusion "))
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64])
                .paint(|ctx| {
                    ctx.draw(&Points {
                        coords: &pattern_points,
                        color: Color::Cyan,
                    });

                    // Draw the drop as a distinct point
                    ctx.draw(&Points {
                        coords: &drop_points,
                        color: Color::Yellow,
                    });
                });

            f.render_widget(canvas, chunks[0]);

            let status_text = vec![
                Line::from(vec![
                    Span::raw(" Commit: "),
                    Span::styled(&current_commit.hash[..7], Style::default().fg(Color::Yellow)),
                    Span::raw(format!(" - {} ({})", current_commit.message, current_commit.author)),
                ]),
                Line::from(vec![
                    Span::raw(" Press "),
                    Span::styled("q", Style::default().fg(Color::Yellow)),
                    Span::raw(" to quit. Press "),
                    Span::styled("SPACE", Style::default().fg(Color::Yellow)),
                    Span::raw(" to seed manually."),
                ]),
            ];

            f.render_widget(Block::default(), chunks[1]);
            f.render_widget(ratatui::widgets::Paragraph::new(status_text), chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                } else if key.code == KeyCode::Char(' ') {
                    // Manual drop at center
                    grid.add_chemical(width / 2, height / 2, 0.9);
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Drop chemical periodically from commits
            ticks_since_commit += 1;
            if ticks_since_commit > commits_delay {
                ticks_since_commit = 0;

                // Add chemical drop
                let (cx, cy) = hash_to_coords(&current_commit.hash, width, height);

                // Add a small 3x3 splat
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let px = (cx as isize + dx).max(0).min(width as isize - 1) as usize;
                        let py = (cy as isize + dy).max(0).min(height as isize - 1) as usize;
                        grid.add_chemical(px, py, 0.9);
                    }
                }

                // Move back in history
                if commit_idx > 0 {
                    commit_idx -= 1;
                }
            }

            // Update simulation (standard spot parameters)
            let f = 0.055;
            let k = 0.062;
            let dt = 1.0;
            grid.update(f, k, dt);

            last_tick = Instant::now();
        }
    }

    Ok(())
}

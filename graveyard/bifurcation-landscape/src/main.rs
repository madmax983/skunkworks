//! # Bifurcation Landscape 🦋
//!
//! A visualization of the **Logistic Map** bifurcation diagram, rendered as a scrolling 3D terrain.
//!
//! This experiment demonstrates how complex, chaotic behavior arises from the simple non-linear equation:
//!
//! $$x_{n+1} = r x_n (1 - x_n)$$
//!
//! ## Key Concepts
//!
//! - **Growth Rate ($r$)**: The parameter controlling the system's behavior.
//! - **Chaos**: As $r$ increases, the system transitions from stable to periodic to chaotic.
//! - **Painter's Algorithm**: A rendering technique where distant objects are drawn first to handle occlusion.

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};
use tui_shared::Tui;

/// The width of the terrain grid (number of X buckets).
const WIDTH: usize = 120;
/// The depth of the visible horizon (number of Z rows).
const DEPTH: usize = 60;

/// The main application state.
struct App {
    /// The circular buffer of terrain rows.
    /// Each row is a vector of heights (counts of hits in that bucket).
    terrain: VecDeque<Vec<u8>>,
    /// The current growth rate parameter ($r$) of the logistic map.
    /// This value drives the simulation.
    r: f64,
    /// The speed of flight, representing how much `r` increases per second.
    speed: f64,
    /// Whether the flight is paused.
    paused: bool,
    /// Internal accumulator for smooth simulation steps independent of frame rate.
    accumulation: f64,
}

impl App {
    /// Creates a new `App` instance with a flat initial terrain.
    fn new() -> Result<Self> {
        // Initialize flat terrain
        let mut terrain = VecDeque::new();
        for _ in 0..DEPTH {
            terrain.push_back(vec![0; WIDTH]);
        }

        Ok(Self {
            terrain,
            r: 2.8,
            speed: 0.1, // r increases by 0.1 per second
            paused: false,
            accumulation: 0.0,
        })
    }

    /// Updates the simulation state based on the time elapsed (`dt`).
    fn update(&mut self, dt: f64) {
        if self.paused {
            return;
        }

        // speed is "r units per second"
        // We want fixed r resolution for smooth terrain continuity
        let r_step_resolution = 0.002;

        // Calculate how much r we should cover
        let r_delta_needed = dt * self.speed;

        self.accumulation += r_delta_needed;

        while self.accumulation >= r_step_resolution {
            self.accumulation -= r_step_resolution;
            self.step(r_step_resolution);
        }
    }

    /// Advances the simulation by a small delta in `r`.
    fn step(&mut self, r_delta: f64) {
        self.r += r_delta;

        // Loop or Bounce
        if self.r > 4.0 {
            self.r = 2.8;
        }

        // Generate Row from Logistic Map
        let row = logistic_map_row(self.r, WIDTH);

        // Move Terrain
        self.terrain.pop_front(); // Remove closest
        self.terrain.push_back(row); // Add new at horizon
    }
}

/// Generates a single row of the bifurcation diagram for a given growth rate `r`.
///
/// This function simulates the logistic map $x_{n+1} = r x_n (1 - x_n)$ for a specific $r$.
/// It runs a transient phase to let the system settle, then samples the attractor
/// to build a histogram of visited values (buckets).
///
/// # Arguments
///
/// * `r` - The growth rate parameter (typically between 2.0 and 4.0).
/// * `width` - The number of buckets (resolution) for the histogram.
///
/// # Returns
///
/// A vector of size `width` where each value represents the "height" (frequency) of visits.
///
/// # Examples
///
/// ```text
/// // For low r, the system stabilizes to a single value.
/// // Note: We need to make sure we can access the function in a doctest if it's not public.
/// // Since this is a binary crate, doctests on private items usually fail or need special handling.
/// // However, for the sake of this example in the plan, I'll assume standard usage.
/// // In a real binary, we might put this in a lib.rs or make it public for tests.
/// // For now, I'll document it clearly.
/// ```
fn logistic_map_row(r: f64, width: usize) -> Vec<u8> {
    let mut row = vec![0u8; width];
    let mut x = 0.5;

    // Transient: iterate to let the system settle into its attractor
    for _ in 0..100 {
        x = r * x * (1.0 - x);
    }

    // Stable / Sampling: record where the system visits
    // Iterate more to catch the period doubling and chaos
    let samples = 200;
    for _ in 0..samples {
        x = r * x * (1.0 - x);

        // Map x [0, 1] to bucket [0, WIDTH]
        let bucket = (x * width as f64) as usize;
        if bucket < width {
            // Increment height, cap at 40 to prevent visual clamping issues
            row[bucket] = row[bucket].saturating_add(1).min(40);
        }
    }

    row
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new()?;

    run_app(&mut tui, &mut app)?;

    Ok(())
}

fn run_app(tui: &mut Tui, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
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
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Up => app.speed = (app.speed + 0.05).min(2.0),
                        KeyCode::Down => app.speed = (app.speed - 0.05).max(0.01),
                        _ => {}
                    }
                }
            }
        }

        let now = Instant::now();
        let dt = now.duration_since(last_tick).as_secs_f64();
        last_tick = now;

        app.update(dt);
    }
    Ok(())
}

/// Renders the TUI interface.
fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Bifurcation Landscape: Flight over Chaos "),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, 100.0]) // Z + Height
        .paint(|ctx| {
            // Draw lines
            // Z=0 is CLOSE (Bottom). Z=DEPTH is HORIZON (Top).
            // Painter's Algorithm: Draw Far (High Z) first, then Close (Low Z).

            for (z, row) in app.terrain.iter().enumerate().rev() {
                let z_factor = z as f64;

                for (x, &h) in row.iter().enumerate() {
                    let h = h as f64;
                    if h == 0.0 {
                        continue;
                    }

                    let x_pos = x as f64;
                    // Simple perspective: lines just go up
                    let y_base = z_factor * 1.0;
                    let y_top = y_base + h;

                    let color = if h > 20.0 {
                        Color::Red
                    } else if h > 10.0 {
                        Color::Yellow
                    } else if h > 5.0 {
                        Color::Green
                    } else {
                        Color::Blue
                    };

                    // Draw vertical pillar
                    ctx.draw(&Line {
                        x1: x_pos,
                        y1: y_base,
                        x2: x_pos,
                        y2: y_top,
                        color,
                    });
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let status = format!(
        "R: {:.5} | Speed: {:.3} | Space: Pause | Up/Down: Speed",
        app.r, app.speed
    );
    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logistic_map_convergence() {
        // For r = 2.5, the map converges to x = 0.6.
        // 0.6 * 120 = 72. So bucket 72 should have hits.
        let width = 120;
        let row = logistic_map_row(2.5, width);

        let total_hits: u8 = row.iter().sum();
        assert!(total_hits > 0, "Should have hits");

        // Find the bucket with the most hits
        let (max_idx, &max_val) = row.iter().enumerate().max_by_key(|&(_, val)| val).unwrap();

        // It should be around index 72
        assert!(
            max_idx >= 70 && max_idx <= 74,
            "Peak should be around 0.6 (index 72), found {}",
            max_idx
        );
        assert!(max_val > 10, "Peak should be significant");
    }

    #[test]
    fn test_logistic_map_chaos() {
        // For r = 3.9, the map is chaotic and visits many buckets.
        let width = 120;
        let row = logistic_map_row(3.9, width);

        let nonzero_buckets = row.iter().filter(|&&x| x > 0).count();
        assert!(
            nonzero_buckets > 20,
            "Chaotic map should visit many buckets, visited {}",
            nonzero_buckets
        );
    }
}

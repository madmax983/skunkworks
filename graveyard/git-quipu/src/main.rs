mod git_loader;
mod quipu;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    widgets::{
        canvas::{Canvas, Line, Rectangle},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use crate::git_loader::load_recent_commits;
use crate::quipu::CommitCord;

struct App {
    commits: Vec<CommitCord>,
    scroll_offset: usize, // Which commit is leftmost
    time: f64,
    last_tick: Instant,
}

impl App {
    fn new() -> Result<Self> {
        let repo_path = "."; // Current directory
                             // Try to load up to 100 commits
        let commit_data = match load_recent_commits(repo_path, 100) {
            Ok(data) => data,
            Err(_) => {
                // If failed (maybe not a git repo or no commits), return empty or error
                // For robustness, let's try to return empty and show "No Data"
                Vec::new()
            }
        };

        let mut commits = Vec::new();
        for (i, data) in commit_data.into_iter().enumerate() {
            let mut cord = CommitCord::from_commit(data);
            cord.apply_wind(0.0, i);
            commits.push(cord);
        }

        Ok(Self {
            commits,
            scroll_offset: 0,
            time: 0.0,
            last_tick: Instant::now(),
        })
    }

    fn on_tick(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f64();
        self.last_tick = now;
        self.time += dt;

        for (i, cord) in self.commits.iter_mut().enumerate() {
            // Apply wind to all cords based on global time + their index
            cord.apply_wind(self.time, i);
        }
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new()?;

    loop {
        terminal.draw(|f| ui(f, &app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Right => {
                        if app.scroll_offset < app.commits.len().saturating_sub(1) {
                            app.scroll_offset += 1;
                        }
                    }
                    KeyCode::Left => {
                        if app.scroll_offset > 0 {
                            app.scroll_offset -= 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        app.on_tick();
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(f.size());

    let title = Paragraph::new("🧬 GIT QUIPU: Physical History")
        .style(Style::default().fg(Color::Cyan).bold())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    // Canvas Logic
    let canvas_width = chunks[1].width as f64;
    let cord_spacing = 8.0; // Space between cords
    let visible_count = (canvas_width / cord_spacing) as usize;

    let start_idx = app.scroll_offset;
    let end_idx = (start_idx + visible_count).min(app.commits.len());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Timeline"))
        .x_bounds([0.0, canvas_width])
        .y_bounds([0.0, 100.0]) // 0 bottom, 100 top
        .paint(|ctx| {
            // Draw Main Cord (Top horizontal line)
            ctx.draw(&Line {
                x1: 0.0,
                y1: 90.0,
                x2: canvas_width,
                y2: 90.0,
                color: Color::White,
            });

            if app.commits.is_empty() {
                return;
            }

            for i in start_idx..end_idx {
                if i >= app.commits.len() {
                    break;
                }
                let cord = &app.commits[i];

                // Visual index in current window
                let visual_idx = (i - start_idx) as f64;
                let x_base = visual_idx * cord_spacing + 4.0;

                // Add wind effect to x (x_offset from physics)
                // let x_pos = x_base + (cord.x_offset * 2.0); // Scale effect

                // Draw Pendant Cord (Vertical line)
                // Bottom point sways more
                // We draw a line from Top (fixed X) to Bottom (swaying X)?
                // Or whole cord sways?
                // Let's make Top fixed at `x_base`, Bottom at `x_pos + sway`.

                // For simplicity, let's assume the whole cord shifts slightly at top,
                // but mostly at bottom.
                let top_x = x_base;
                let bottom_x = x_base + (cord.x_offset * 5.0); // More sway at bottom

                ctx.draw(&Line {
                    x1: top_x,
                    y1: 90.0,
                    x2: bottom_x,
                    y2: 10.0,
                    color: Color::DarkGray,
                });

                // Draw Knots
                let num_clusters = cord.cord.clusters.len();
                // Clusters distributed from Y=80 down to Y=20
                let top_y = 80.0;
                let bottom_y = 20.0;
                let total_height = top_y - bottom_y;
                let cluster_spacing = if num_clusters > 1 {
                    total_height / (num_clusters as f64 - 1.0)
                } else {
                    total_height
                };

                for (c_idx, cluster) in cord.cord.clusters.iter().enumerate() {
                    let y_pos = top_y - (c_idx as f64 * cluster_spacing);

                    // Interpolate X based on Y (linear interpolation between top_x and bottom_x)
                    // t = 0 at top (90), 1 at bottom (10)
                    // y_pos maps to t: t = (90 - y) / 80
                    let t = (90.0 - y_pos) / 80.0;
                    let knot_x_center = top_x + (bottom_x - top_x) * t;

                    // Draw Cluster
                    let mut current_y = y_pos;

                    for knot in cluster {
                        let color = match knot {
                            crate::quipu::Knot::Simple => Color::Green,
                            crate::quipu::Knot::Long(_) => Color::Blue,
                            crate::quipu::Knot::FigureEight => Color::Red,
                        };

                        let width = match knot {
                            crate::quipu::Knot::Simple => 1.5,
                            crate::quipu::Knot::Long(v) => 1.5 + (*v as f64 * 0.3),
                            crate::quipu::Knot::FigureEight => 2.0,
                        };

                        ctx.draw(&Rectangle {
                            x: knot_x_center - (width / 2.0),
                            y: current_y - 1.0,
                            width,
                            height: 2.0,
                            color,
                        });

                        current_y -= 2.5; // Stack downwards
                    }
                }
            }
        });
    f.render_widget(canvas, chunks[1]);

    // Info panel
    let info_text = if !app.commits.is_empty() {
        if let Some(cord) = app.commits.get(app.scroll_offset) {
            format!(
                "Commit: {} | Author: {} | Date: {}\n+{} -{} ({} files)",
                cord.data.hash.chars().take(7).collect::<String>(),
                cord.data.author,
                cord.data.time.format("%Y-%m-%d %H:%M"),
                cord.data.insertions,
                cord.data.deletions,
                cord.data.files_changed
            )
        } else {
            "End of list".to_string()
        }
    } else {
        "No commits loaded".to_string()
    };

    let instructions = Paragraph::new(format!(
        "Use Left/Right to scroll history. Q to Quit.\n{}",
        info_text
    ))
    .style(Style::default().fg(Color::Gray))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[2]);
}

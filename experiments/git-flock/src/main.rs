use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};

mod boid;
mod git;
mod world;

use git::{get_commit_history, Commit};
use tui_shared::Tui;
use world::World;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal);

    drop(tui);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    commits: Vec<Commit>,
    world: World,
    running: bool,
    auto_play: bool,
    tick_count: usize,
    ticks_per_commit: usize,
}

impl App {
    fn new(width: f64, height: f64) -> Result<Self> {
        let commits = match get_commit_history() {
            Ok(c) if !c.is_empty() => c,
            _ => vec![Commit {
                hash: "0000000000000000000000000000000000000000".to_string(),
                author: "Unknown".to_string(),
                date: chrono::Utc::now(),
                message: "No git history found".to_string(),
            }],
        };

        let world = World::new(width, height, &commits);

        Ok(Self {
            commits,
            world,
            running: true,
            auto_play: true,
            tick_count: 0,
            ticks_per_commit: 60, // frames before moving to next commit
        })
    }

    fn on_tick(&mut self) {
        if self.auto_play {
            self.tick_count += 1;
            if self.tick_count >= self.ticks_per_commit {
                self.tick_count = 0;
                self.world.current_commit = (self.world.current_commit + 1) % self.commits.len();
            }
        }
        self.world.update();
    }
}

fn run_app(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    // Give it a canvas space of roughly 100x100
    let mut app = App::new(200.0, 100.0)?;

    while app.running {
        terminal.draw(|f| draw(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Char(' ') => app.auto_play = !app.auto_play,
                        KeyCode::Right => {
                            app.world.current_commit =
                                (app.world.current_commit + 1) % app.commits.len();
                            app.tick_count = 0;
                        }
                        KeyCode::Left => {
                            if app.world.current_commit > 0 {
                                app.world.current_commit -= 1;
                            } else {
                                app.world.current_commit = app.commits.len() - 1;
                            }
                            app.tick_count = 0;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}

fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(4)])
        .split(f.area());

    let boids = &app.world.boids;
    let width = app.world.width;
    let height = app.world.height;

    let target = if !app.world.commit_targets.is_empty() {
        app.world.commit_targets[app.world.current_commit]
    } else {
        locus::Vec2::zero()
    };

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Git Swarming "),
        )
        .paint(|ctx| {
            // Draw current commit target
            ctx.draw(&Points {
                coords: &[(target.x, target.y)],
                color: Color::Red,
            });

            // Draw boids
            for b in boids {
                ctx.draw(&Points {
                    coords: &[(b.position.x, b.position.y)],
                    color: b.dna.color,
                });
            }
        })
        .x_bounds([0.0, width])
        .y_bounds([0.0, height]);
    f.render_widget(canvas, chunks[0]);

    // Bottom info panel
    let commit = &app.commits[app.world.current_commit];
    let info = vec![
        Line::from(vec![
            Span::styled("Commit: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(format!("{} - {} ", &commit.hash[0..7], commit.author)),
            Span::styled(
                format!("({})", commit.date.format("%Y-%m-%d")),
                Style::default().fg(Color::DarkGray),
            ),
        ]),
        Line::from(vec![
            Span::styled("Message: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(&commit.message),
        ]),
        Line::from(vec![
            Span::styled("Controls: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("[q/Esc] Quit  [Space] Play/Pause  [<- ->] Prev/Next Commit"),
        ]),
    ];

    let info_panel = Paragraph::new(info).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Codebase Metadata "),
    );
    f.render_widget(info_panel, chunks[1]);
}

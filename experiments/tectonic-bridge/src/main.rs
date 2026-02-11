mod git;
mod world;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::*,
    widgets::{
        canvas::{Canvas, Rectangle},
        Block, Borders, Paragraph,
    },
};
use std::{
    io,
    time::{Duration, Instant},
};

use git::GitScanner;
use world::{World, Terrain, AntState};

struct App {
    world: World,
    running: bool,
    zoom: f64,
}

impl App {
    fn new(commits: Vec<git::CommitData>) -> Self {
        Self {
            world: World::new(200, 100, commits),
            running: true,
            zoom: 1.0,
        }
    }

    fn on_tick(&mut self) {
        self.world.update();
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Load Git History
    terminal.draw(|f| {
        let p = Paragraph::new("Scanning Tectonic History... (git log -p)")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(p, f.area());
    })?;

    let commits = match GitScanner::scan() {
        Ok(c) => c,
        Err(e) => {
            disable_raw_mode()?;
            execute!(io::stdout(), LeaveAlternateScreen)?;
            eprintln!("Error scanning git: {}", e);
            return Err(e);
        }
    };

    let mut app = App::new(commits);
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.running = false,
                        KeyCode::Char('r') => {
                            // Reload world? Or just reset ants?
                            // Re-scanning is slow. Just re-gen terrain?
                            // app.world = World::new(200, 100, app.world.commits.clone());
                        }
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        KeyCode::Up => app.world.scroll_y += 5.0,
                        KeyCode::Down => app.world.scroll_y -= 5.0,
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas_area = chunks[0];

    // Viewport calculation
    let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
    let view_height = 100.0 / app.zoom;
    let view_width = view_height * aspect * 2.0;

    let center_y = app.world.scroll_y;
    let center_x = app.world.width as f64 / 2.0;

    let x_min = center_x - view_width / 2.0;
    let x_max = center_x + view_width / 2.0;
    let y_min = center_y - view_height / 2.0;
    let y_max = center_y + view_height / 2.0;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Tectonic Bridge "),
        )
        .x_bounds([x_min, x_max])
        .y_bounds([y_min, y_max])
        .paint(|ctx| {
            // Draw Terrain
            // Iterate visible range for performance
            // But terrain is a flat vector.
            // Let's iterate all world terrain? 200*100 = 20k points.
            // Canvas handles culling? Or should we cull?
            // Canvas.draw calls are cheap if we use shapes.
            // Drawing 20k points individually might be slow.
            // Rectangle is better.

            // Optimization: Iterate Y from y_min to y_max
            let start_y = y_min.max(0.0) as usize;
            let end_y = y_max.min(app.world.height as f64) as usize;
            let start_x = x_min.max(0.0) as usize;
            let end_x = x_max.min(app.world.width as f64) as usize;

            for y in start_y..end_y {
                for x in start_x..end_x {
                    let terrain = app.world.get_terrain(x as i32, y as i32);
                    match terrain {
                        Terrain::Solid { commit_idx } => {
                            // Color based on commit index (hash)?
                            let color = match commit_idx % 6 {
                                0 => Color::DarkGray,
                                1 => Color::Gray,
                                2 => Color::White,
                                3 => Color::Yellow,
                                4 => Color::Red,
                                _ => Color::Blue,
                            };
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: y as f64,
                                width: 1.0,
                                height: 1.0,
                                color,
                            });
                        },
                        Terrain::Gap => {
                            // Draw nothing (Empty) or explicit Gap color?
                            // Empty is fine.
                        },
                        Terrain::Bridge => {
                            ctx.draw(&Rectangle {
                                x: x as f64,
                                y: y as f64,
                                width: 1.0,
                                height: 1.0,
                                color: Color::Cyan, // Bridge color
                            });
                        },
                        Terrain::Empty => {}
                    }
                }
            }

            // Draw Ants
            for ant in &app.world.ants {
                // Cull off-screen
                if (ant.x as f64) < x_min || (ant.x as f64) > x_max ||
                   (ant.y as f64) < y_min || (ant.y as f64) > y_max {
                    continue;
                }

                let color = match ant.state {
                    AntState::Foraging => Color::Green,
                    AntState::Bridging => Color::Cyan, // Should match bridge terrain
                };

                // Draw as small point
                ctx.print(ant.x as f64, ant.y as f64, Span::styled("•", Style::default().fg(color)));
            }
        });

    f.render_widget(canvas, canvas_area);

    let info = format!(
        "Ants: {} | Scroll Y: {:.1} | Zoom: {:.1}x | [Arrows] Scroll [+/-] Zoom",
        app.world.ants.len(),
        app.world.scroll_y,
        app.zoom
    );
    f.render_widget(
        Paragraph::new(info).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

mod penrose;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::{Canvas, Line}, Block, Borders, Paragraph},
    Terminal,
};
use std::io;

use penrose::{Pentagrid, Rhombus, RhombusType, Point};

struct App {
    grid: Pentagrid,
    rhombi: Vec<Rhombus>,
    player_rhomb_idx: usize,
    view_x: f64,
    view_y: f64,
    zoom: f64,
}

impl App {
    fn new() -> Self {
        let grid = Pentagrid::new(0);
        let rhombi = grid.generate_tiling(15);
        // Start player at center-most rhombus
        let mut min_dist = f64::MAX;
        let mut idx = 0;
        let center = Point::new(0.0, 0.0);

        for (i, r) in rhombi.iter().enumerate() {
            let d = r.center().dist(&center);
            if d < min_dist {
                min_dist = d;
                idx = i;
            }
        }

        Self {
            grid,
            rhombi,
            player_rhomb_idx: idx,
            view_x: 0.0,
            view_y: 0.0,
            zoom: 20.0,
        }
    }

    fn move_player(&mut self, dx: f64, dy: f64) {
        if self.rhombi.is_empty() { return; }

        let current = self.rhombi[self.player_rhomb_idx].center();
        let target = Point::new(current.x + dx, current.y + dy);

        // Find nearest rhombus to target
        let mut min_dist = f64::MAX;
        let mut nearest_idx = self.player_rhomb_idx;

        for (i, r) in self.rhombi.iter().enumerate() {
            let d = r.center().dist(&target);
            if d < min_dist {
                min_dist = d;
                nearest_idx = i;
            }
        }
        self.player_rhomb_idx = nearest_idx;

        // Update view to follow player
        let new_pos = self.rhombi[self.player_rhomb_idx].center();
        self.view_x = new_pos.x;
        self.view_y = new_pos.y;
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App state
    let mut app = App::new();

    // Run app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Penrose Rogue"))
                .x_bounds([app.view_x - app.zoom, app.view_x + app.zoom])
                .y_bounds([app.view_y - app.zoom, app.view_y + app.zoom])
                .paint(|ctx| {
                    for (i, rhomb) in app.rhombi.iter().enumerate() {
                        let v = &rhomb.vertices;
                        // Color based on type, highlight player
                        let mut color = match rhomb.kind {
                            RhombusType::Thick => Color::Cyan,
                            RhombusType::Thin => Color::Magenta,
                        };

                        if i == app.player_rhomb_idx {
                            color = Color::Yellow;
                        }

                        // Draw 4 edges
                        ctx.draw(&Line {
                            x1: v[0].x, y1: v[0].y,
                            x2: v[1].x, y2: v[1].y,
                            color,
                        });
                        ctx.draw(&Line {
                            x1: v[1].x, y1: v[1].y,
                            x2: v[2].x, y2: v[2].y,
                            color,
                        });
                        ctx.draw(&Line {
                            x1: v[2].x, y1: v[2].y,
                            x2: v[3].x, y2: v[3].y,
                            color,
                        });
                        ctx.draw(&Line {
                            x1: v[3].x, y1: v[3].y,
                            x2: v[0].x, y2: v[0].y,
                            color,
                        });

                        // If player, draw 'X' inside
                        if i == app.player_rhomb_idx {
                            ctx.draw(&Line {
                                x1: v[0].x, y1: v[0].y,
                                x2: v[2].x, y2: v[2].y,
                                color: Color::Red,
                            });
                            ctx.draw(&Line {
                                x1: v[1].x, y1: v[1].y,
                                x2: v[3].x, y2: v[3].y,
                                color: Color::Red,
                            });
                        }
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let status = Paragraph::new("WASD/Arrows: Move | +/-: Zoom | q: Quit")
                .style(Style::default().bg(Color::White).fg(Color::Black));
            f.render_widget(status, chunks[1]);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        // Movement
                        KeyCode::Left | KeyCode::Char('a') => app.move_player(-1.0, 0.0),
                        KeyCode::Right | KeyCode::Char('d') => app.move_player(1.0, 0.0),
                        KeyCode::Up | KeyCode::Char('w') => app.move_player(0.0, 1.0),
                        KeyCode::Down | KeyCode::Char('s') => app.move_player(0.0, -1.0),
                        // Zoom
                        KeyCode::Char('=') | KeyCode::Char('+') => app.zoom *= 0.9,
                        KeyCode::Char('-') => app.zoom *= 1.1,
                        _ => {}
                    }
                }
            }
        }
    }
}

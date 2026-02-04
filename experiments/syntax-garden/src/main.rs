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
    text::Span,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{env, io, time::Duration};

mod parser;
mod turtle;

use parser::GardenParser;
use turtle::Turtle;

struct App {
    turtle: Turtle,
    genome: String,
    camera: (f64, f64),
    zoom: f64,
    parsing_path: String,
    status: String,
}

impl App {
    fn new(path: &str) -> Result<Self> {
        let mut parser = GardenParser::new();
        let genome = parser
            .parse_directory(path)
            .unwrap_or_else(|e| format!("Error: {}", e));

        // Initialize turtle
        // Start facing UP (90 degrees in Cartesian coords)
        let mut turtle = Turtle::new(0.0, 0.0, 90.0, 25.0, 5.0);
        turtle.process_str(&genome);

        let genome_len = genome.len();
        Ok(Self {
            turtle,
            genome,
            camera: (0.0, 0.0),
            zoom: 1.0,
            parsing_path: path.to_string(),
            status: format!("Parsed {} chars from {}", genome_len, path),
        })
    }

    fn on_resize(&mut self) {
        // Maybe recenter?
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 { &args[1] } else { "." };

    // Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run App
    let app = App::new(path)?;
    let res = run_app(&mut terminal, app);

    // Restore Terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, mut app: App) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let area = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(area);

            let canvas_area = chunks[0];

            // Calculate Viewport
            let aspect = canvas_area.width as f64 / canvas_area.height.max(1) as f64;
            // Base view size
            let view_height = 200.0 / app.zoom;
            let view_width = view_height * aspect * 2.0; // x2 because terminal cells are ~1:2

            let x_min = app.camera.0 - view_width / 2.0;
            let x_max = app.camera.0 + view_width / 2.0;
            let y_min = app.camera.1 - view_height / 2.0;
            let y_max = app.camera.1 + view_height / 2.0;

            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title(" Syntax Garden 🌿 "))
                .x_bounds([x_min, x_max])
                .y_bounds([y_min, y_max])
                .paint(|ctx| {
                    for (x1, y1, x2, y2, depth) in &app.turtle.lines {
                        let color = match depth % 4 {
                            0 => Color::Green,
                            1 => Color::LightGreen,
                            2 => Color::Yellow,
                            _ => Color::Cyan,
                        };

                        // Cull lines outside view for performance?
                        // Canvas does this? Not sure, but good to be safe if list is huge.
                        // Simple AABB check
                        let min_lx = x1.min(*x2);
                        let max_lx = x1.max(*x2);
                        let min_ly = y1.min(*y2);
                        let max_ly = y1.max(*y2);

                        if max_lx < x_min || min_lx > x_max || max_ly < y_min || min_ly > y_max {
                            continue;
                        }

                        ctx.draw(&Line {
                            x1: *x1,
                            y1: *y1,
                            x2: *x2,
                            y2: *y2,
                            color,
                        });
                    }
                });

            f.render_widget(canvas, canvas_area);

            let status_text = format!(
                "Path: {} | Nodes: {} | Pos: ({:.1}, {:.1}) | Zoom: {:.2}x | [WASD] Move [+/-] Zoom [Q] Quit",
                app.parsing_path,
                app.turtle.lines.len(),
                app.camera.0,
                app.camera.1,
                app.zoom
            );

            f.render_widget(
                Paragraph::new(status_text).block(Block::default().borders(Borders::ALL)),
                chunks[1]
            );
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('w') | KeyCode::Up => app.camera.1 -= 10.0 / app.zoom, // Move camera UP (so decrease Y? No, Y increases down in Canvas? No, Canvas is Cartesian usually, Y up. Wait. Ratatui Canvas: Y is up.)
                        // Let's assume Standard Cartesian.
                        // If I press Up, I want to see things above. So camera Y moves UP.
                        KeyCode::Char('s') | KeyCode::Down => app.camera.1 += 10.0 / app.zoom, // Camera Y Down.
                        KeyCode::Char('a') | KeyCode::Left => app.camera.0 -= 10.0 / app.zoom,
                        KeyCode::Char('d') | KeyCode::Right => app.camera.0 += 10.0 / app.zoom,
                        KeyCode::Char('+') | KeyCode::Char('=') => app.zoom *= 1.1,
                        KeyCode::Char('-') | KeyCode::Char('_') => app.zoom /= 1.1,
                        _ => {}
                    }
                }
            }
        }
    }
}

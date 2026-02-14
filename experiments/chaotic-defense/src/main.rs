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
    symbols,
    text::Span,
    widgets::{
        canvas::{Canvas, Context, Painter, Shape},
        Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph,
    },
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

mod model;
use model::{World, HEIGHT, WIDTH};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> Result<()> {
    let mut world = World::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50);

    // Simple cursor state
    let mut cursor_x = (WIDTH / 2) as f64;
    let mut cursor_y = (HEIGHT / 2) as f64;

    loop {
        terminal.draw(|f| ui(f, &world, cursor_x, cursor_y))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => world.global_r = (world.global_r - 0.01).max(0.0),
                        KeyCode::Right => world.global_r = (world.global_r + 0.01).min(4.0),
                        KeyCode::Char('w') => cursor_y = (cursor_y - 1.0).max(0.0),
                        KeyCode::Char('s') => cursor_y = (cursor_y + 1.0).min(HEIGHT as f64),
                        KeyCode::Char('a') => cursor_x = (cursor_x - 1.0).max(0.0),
                        KeyCode::Char('d') => cursor_x = (cursor_x + 1.0).min(WIDTH as f64),
                        KeyCode::Char(' ') => {
                            world.add_tower(cursor_x, cursor_y);
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
    }
}

fn ui(f: &mut Frame, world: &World, cursor_x: f64, cursor_y: f64) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10),
            Constraint::Length(3), // Status bar
        ])
        .split(f.size());

    // Main Game View
    // We can use Canvas for drawing points
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chaotic Defense"),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .paint(|ctx: &mut Context| {
            // Draw Nests
            ctx.draw(&Points {
                coords: &world
                    .nests
                    .iter()
                    .map(|n| (n.pos.x, HEIGHT as f64 - n.pos.y))
                    .collect::<Vec<_>>(),
                color: Color::Magenta,
            });
            // Draw Towers
            ctx.draw(&Points {
                coords: &world
                    .towers
                    .iter()
                    .map(|t| (t.pos.x, HEIGHT as f64 - t.pos.y))
                    .collect::<Vec<_>>(),
                color: Color::Cyan,
            });
            // Draw Enemies
            ctx.draw(&Points {
                coords: &world
                    .enemies
                    .iter()
                    .map(|e| (e.pos.x, HEIGHT as f64 - e.pos.y))
                    .collect::<Vec<_>>(),
                color: Color::Red,
            });
            // Draw Cursor
            ctx.print(
                cursor_x,
                HEIGHT as f64 - cursor_y,
                Span::styled("X", Style::default().fg(Color::Yellow)),
            );
        });
    f.render_widget(canvas, chunks[0]);

    // Bifurcation / History Plot
    let history_data: Vec<(f64, f64)> = world
        .history
        .iter()
        .enumerate()
        .map(|(i, &val)| (i as f64, val))
        .collect();

    let datasets = vec![Dataset::default()
        .name("Population (x)")
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Green))
        .data(&history_data)];

    let chart = Chart::new(datasets)
        .block(
            Block::default()
                .title("Chaos Monitor (Nest 0)")
                .borders(Borders::ALL),
        )
        .x_axis(Axis::default().title("Time").bounds([0.0, 200.0])) // Fixed window size matching history capacity
        .y_axis(Axis::default().title("x").bounds([0.0, 1.0]));

    f.render_widget(chart, chunks[1]);

    // Status Bar
    let status_text = format!(
        "Global R: {:.4} | Resources: {:.1} | Enemies: {} | Ticks: {}",
        world.global_r,
        world.resources,
        world.enemies.len(),
        world.ticks
    );

    let status_p = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(status_p, chunks[2]);
}

struct Points<'a> {
    coords: &'a [(f64, f64)],
    color: Color,
}

impl<'a> Shape for Points<'a> {
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords {
            if let Some((x, y)) = painter.get_point(*x, *y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

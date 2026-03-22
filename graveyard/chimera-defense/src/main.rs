use anyhow::Result;
use chimera_lang::vm::{Value, GRID_SIZE};
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
        canvas::{Canvas, Context, Line, Painter, Shape},
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
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(f.size());

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(10),
            Constraint::Length(3), // Status bar
        ])
        .split(main_layout[0]);

    // Main Game View
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Chimera Defense"),
        )
        .x_bounds([0.0, WIDTH as f64])
        .y_bounds([0.0, HEIGHT as f64])
        .paint(|ctx: &mut Context| {
            // Draw Nests
            ctx.draw(&Points {
                coords: world
                    .nests
                    .iter()
                    .map(|n| (n.pos.x, HEIGHT as f64 - n.pos.y)),
                color: Color::Magenta,
            });
            // Draw Towers
            ctx.draw(&Points {
                coords: world
                    .towers
                    .iter()
                    .map(|t| (t.pos.x, HEIGHT as f64 - t.pos.y)),
                color: Color::Cyan,
            });
            // Draw Enemies
            ctx.draw(&Points {
                coords: world
                    .enemies
                    .iter()
                    .map(|e| (e.pos.x, HEIGHT as f64 - e.pos.y)),
                color: Color::Red,
            });

            // Draw Projectiles
            for proj in &world.projectiles {
                ctx.draw(&Line {
                    x1: proj.start.x,
                    y1: HEIGHT as f64 - proj.start.y,
                    x2: proj.end.x,
                    y2: HEIGHT as f64 - proj.end.y,
                    color: Color::Yellow,
                });
            }

            // Draw Cursor
            ctx.print(
                cursor_x,
                HEIGHT as f64 - cursor_y,
                Span::styled("X", Style::default().fg(Color::Yellow)),
            );
        });
    f.render_widget(canvas, left_chunks[0]);

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
        .x_axis(Axis::default().title("Time").bounds([0.0, 200.0]))
        .y_axis(Axis::default().title("x").bounds([0.0, 1.0]));

    f.render_widget(chart, left_chunks[1]);

    // Status Bar
    let status_text = format!(
        "Global R: {:.4} | Resources: {:.1} | Enemies: {} | Ticks: {}",
        world.global_r,
        world.resources,
        world.enemies.len(),
        world.ticks
    );

    let status_p = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(status_p, left_chunks[2]);

    // Sidebar: VM Inspector
    let mut selected_tower = None;
    let mut min_dist = 5.0;
    for tower in &world.towers {
        let dx = tower.pos.x - cursor_x;
        let dy = tower.pos.y - cursor_y;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < min_dist {
            min_dist = dist;
            selected_tower = Some(tower);
        }
    }

    if let Some(tower) = selected_tower {
        let mut grid_text = String::new();
        grid_text.push_str(&format!(
            "Tower @ ({:.1}, {:.1})\n",
            tower.pos.x, tower.pos.y
        ));
        grid_text.push_str(&format!("Kills: {}\n", tower.kills));
        grid_text.push_str("VM Grid:\n");
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                match &tower.vm.grid[y][x] {
                    Value::Int(0) => grid_text.push_str(". "),
                    Value::Int(n) => {
                        if *n > 0 {
                            grid_text.push_str("# ");
                        } else {
                            grid_text.push_str(". ");
                        }
                    }
                    Value::Str(_) => grid_text.push_str("S "),
                    _ => grid_text.push_str("? "),
                }
            }
            grid_text.push('\n');
        }

        // Show Inputs/Outputs specifically
        if let Value::Int(dist) = tower.vm.grid[0][0] {
            grid_text.push_str(&format!("\nSensors:\nDist: {}\n", dist));
        }
        if let Value::Int(angle) = tower.vm.grid[0][1] {
            grid_text.push_str(&format!("Angle: {:.2}\n", angle as f64 / 100.0));
        }

        let p = Paragraph::new(grid_text)
            .block(Block::default().borders(Borders::ALL).title("Tower VM"));
        f.render_widget(p, main_layout[1]);
    } else {
        let p = Paragraph::new("No Tower Selected\nHover over a tower to inspect.")
            .block(Block::default().borders(Borders::ALL).title("Tower VM"));
        f.render_widget(p, main_layout[1]);
    }
}

struct Points<I> {
    coords: I,
    color: Color,
}

impl<I> Shape for Points<I>
where
    I: Iterator<Item = (f64, f64)> + Clone,
{
    fn draw(&self, painter: &mut Painter) {
        for (x, y) in self.coords.clone() {
            if let Some((x, y)) = painter.get_point(x, y) {
                painter.paint(x, y, self.color);
            }
        }
    }
}

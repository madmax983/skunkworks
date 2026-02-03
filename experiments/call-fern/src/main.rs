use std::io;
use std::sync::mpsc::{self, Receiver};
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};

mod algorithms;
mod model;

use algorithms::run_fib;
use model::{Event, Node, Plant};
use tui_shared::math::Vec2;

struct App {
    plant: Plant,
    rx: Receiver<Event>,
    // Store the last received event for debug/status
    last_event: String,
}

impl App {
    fn new(rx: Receiver<Event>) -> Self {
        Self {
            plant: Plant::new(),
            rx,
            last_event: String::new(),
        }
    }

    fn update(&mut self) {
        while let Ok(event) = self.rx.try_recv() {
            match event {
                Event::Call { name, args } => {
                    self.last_event = format!("Call {}({})", name, args);
                    self.handle_call(name, args);
                }
                Event::Return { value } => {
                    self.last_event = format!("Return {}", value);
                    self.plant.pop_return(value);
                }
                Event::Sleep(_) => {
                    // Just a pause signal
                }
            }
        }
    }

    fn handle_call(&mut self, name: String, args: String) {
        let (pos, angle, length, depth) = if let Some(parent) = self.plant.get_cursor_node() {
            let num_children = parent.children.len();

            // Branching logic:
            // 1st child: -0.5 rad (~28 deg)
            // 2nd child: +0.5 rad
            let angle_offset = match num_children {
                0 => 0.5,
                1 => -0.5,
                _ => 0.0,
            };

            let new_angle = parent.angle + angle_offset;
            let new_length = parent.length * 0.8;
            let new_depth = parent.depth + 1;

            (parent.end_pos(), new_angle, new_length, new_depth)
        } else {
            // Root
            (Vec2::new(0.0, -100.0), std::f64::consts::PI / 2.0, 40.0, 0)
        };

        self.plant.push_call(pos, angle, length, depth, name, args);
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup App
    let (tx, rx) = mpsc::channel();

    // Start Algorithm
    run_fib(8, tx);

    let mut app = App::new(rx);

    let res = run_loop(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(33); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Draw Canvas
            let canvas = Canvas::default()
                .block(Block::default().borders(Borders::ALL).title("Garden"))
                .paint(|ctx| {
                    if let Some(root) = &app.plant.root {
                        draw_node(ctx, root, &app.plant.cursor_path, 0);
                    }
                })
                .x_bounds([-150.0, 150.0])
                .y_bounds([-150.0, 150.0]);

            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let status = Paragraph::new(format!("Status: {}", app.last_event))
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        #[allow(clippy::collapsible_if)]
        if crossterm::event::poll(timeout)? {
            if let CEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn draw_node(ctx: &mut ratatui::widgets::canvas::Context, node: &Node, _cursor_path: &[usize], _current_depth: usize) {
    let start = node.pos;
    let end = node.end_pos();

    let color = if node.return_value.is_some() {
        Color::Yellow
    } else {
        Color::Green
    };

    ctx.draw(&Line {
        x1: start.x,
        y1: start.y,
        x2: end.x,
        y2: end.y,
        color,
    });

    // Draw Leaf/Value if returned
    if let Some(val) = &node.return_value {
        ctx.print(end.x, end.y, val.clone());
    }

    for child in node.children.iter() {
        draw_node(ctx, child, _cursor_path, _current_depth + 1);
    }
}

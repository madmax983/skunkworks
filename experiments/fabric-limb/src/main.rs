use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Circle, Line as CanvasLine},
        Block, Borders, List, ListItem, ListState,
    },
    Frame,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod navigator;
mod physics;

use navigator::FileExplorer;
use physics::Arm;

struct App {
    explorer: FileExplorer,
    arm: Arm,
    state: ListState,
    target: (f64, f64),
    current_target: (f64, f64), // For smoothing (next step, but pre-empting)
}

impl App {
    fn new(root: PathBuf) -> Result<Self> {
        let mut explorer = FileExplorer::new(root)?;
        explorer.scan()?;

        // Arm setup
        // Base at bottom center of the Right Pane
        // We will coordinate-space later. Let's assume Canvas is 100x100.
        // Base at (50, 0).
        // 4 segments of length 20. Total length 80.
        let arm = Arm::new((50.0, 0.0), vec![20.0, 20.0, 20.0, 20.0]);

        Ok(Self {
            explorer,
            arm,
            state: ListState::default().with_selected(Some(0)),
            target: (50.0, 50.0),
            current_target: (50.0, 50.0),
        })
    }

    fn on_tick(&mut self) {
        // Calculate target based on selection
        // We need to map the selected item index to a Y coordinate.
        // This is tricky because the list scrolls.
        // Ideally, we map the VISIBLE position of the item.
        // For now, let's map the absolute index to a height, wrapping or clamping?
        // Let's assume the arm reaches for the file *conceptually*.
        // If there are many files, we might need to scroll the arm target?
        // Let's try to map the relative position in the list if we knew the visual offset.
        // But `ListState` in Ratatui holds the offset.

        // Simplified approach: Target Y is based on (index % 20) to simulate a page.
        // Or better: The arm points to a "Screen" representation.
        // Let's assume the screen height is mapped to Canvas Y 0..100.
        // If we select item 0, Y=90. Item 10, Y=10.

        if let Some(selected) = self.state.selected() {
            // Map 0..20 to 90..10
            let page_size = 20;
            let relative_idx = selected % page_size;
            let t = relative_idx as f64 / (page_size as f64);
            let target_y = 90.0 - (t * 80.0);

            // X coordinate: Let's make it reach to the left side of the canvas (where the list is)
            // Canvas X 0..100. List is to the left.
            // So target X should be near 0.
            let target_x = 10.0;

            self.target = (target_x, target_y);
        }

        // Smoothly interpolate current_target towards target
        let smooth_factor = 0.1; // Adjust for speed
        self.current_target.0 += (self.target.0 - self.current_target.0) * smooth_factor;
        self.current_target.1 += (self.target.1 - self.current_target.1) * smooth_factor;

        self.arm.solve(self.current_target);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    // Start in current directory
    let root = std::env::current_dir()?;
    let mut app = App::new(root)?;

    let tick_rate = Duration::from_millis(30); // ~30 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        app.explorer.move_cursor(1);
                        app.state.select(Some(app.explorer.selected_idx));
                    }
                    KeyCode::Up => {
                        app.explorer.move_cursor(-1);
                        app.state.select(Some(app.explorer.selected_idx));
                    }
                    _ => {}
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

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(f.area());

    // Left: File List
    let items: Vec<ListItem> = app
        .explorer
        .files
        .iter()
        .map(|file| {
            let name = file.path.file_name().unwrap_or_default().to_string_lossy();
            let indent = "  ".repeat(file.depth.saturating_sub(1)); // indent based on depth
            let content = format!("{}{}", indent, name);
            let style = if file.is_dir {
                Style::default().fg(Color::Blue)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Files"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    // Right: Canvas
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Arm"))
        .x_bounds([0.0, 100.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Draw Target
            ctx.draw(&Circle {
                x: app.target.0,
                y: app.target.1,
                radius: 2.0,
                color: Color::Red,
            });

            // Draw Arm
            // Draw bones
            for i in 0..app.arm.joints.len() - 1 {
                let p1 = app.arm.joints[i];
                let p2 = app.arm.joints[i + 1];
                ctx.draw(&CanvasLine {
                    x1: p1.0,
                    y1: p1.1,
                    x2: p2.0,
                    y2: p2.1,
                    color: Color::Green,
                });

                // Draw Joint
                ctx.draw(&Circle {
                    x: p1.0,
                    y: p1.1,
                    radius: 1.0,
                    color: Color::Yellow,
                });
            }
            // End effector
            let end = app.arm.end_effector();
            ctx.draw(&Circle {
                x: end.0,
                y: end.1,
                radius: 1.0,
                color: Color::Cyan,
            });
        });

    f.render_widget(canvas, chunks[1]);
}

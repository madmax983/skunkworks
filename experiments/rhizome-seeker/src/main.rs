#![allow(clippy::collapsible_if)]
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::Widget};
use std::time::{Duration, Instant};
use tui_shared::Tui;

pub mod model;
pub mod root;

use model::{Grid, SoilType};
use root::{Algorithm, RootSystem};

struct App {
    grid: Grid,
    roots: Vec<RootSystem>,
    should_quit: bool,
    last_tick: Instant,
    tick_rate: Duration,
}

impl App {
    fn new() -> Self {
        let width = 100;
        let height = 60;
        let mut grid = Grid::new(width, height);
        grid.generate();

        let mut roots = Vec::new();
        // Seed 1: Dijkstra from Top Left
        let mut root1 = RootSystem::new(Algorithm::Dijkstra, 5, 5, width);
        root1.set_target(width - 5, height - 5, width);
        roots.push(root1);

        // Seed 2: A* from Bottom Left
        let mut root2 = RootSystem::new(Algorithm::AStar, 5, height - 5, width);
        root2.set_target(width - 5, 5, width);
        roots.push(root2);

        // Seed 3: Greedy from Bottom Center
        let mut root3 = RootSystem::new(Algorithm::Greedy, width / 2, height - 5, width);
        root3.set_target(width / 2, 5, width);
        roots.push(root3);

        Self {
            grid,
            roots,
            should_quit: false,
            last_tick: Instant::now(),
            tick_rate: Duration::from_millis(50),
        }
    }

    fn on_tick(&mut self) {
        // We must split borrows carefully.
        // Roots need mutable access to themselves, AND mutable access to grid.
        // Rust won't like us iterating over self.roots while passing &mut self.grid.
        // So we just iterate indices.

        for i in 0..self.roots.len() {
            // Grow faster by doing multiple steps per frame
            for _ in 0..5 {
                self.roots[i].step(&mut self.grid, i);
            }
        }
    }

    fn reset(&mut self) {
        self.grid.generate();
        let width = self.grid.width;
        let height = self.grid.height;

        self.roots.clear();

        let mut root1 = RootSystem::new(Algorithm::Dijkstra, 5, 5, width);
        root1.set_target(width - 5, height - 5, width);
        self.roots.push(root1);

        let mut root2 = RootSystem::new(Algorithm::AStar, 5, height - 5, width);
        root2.set_target(width - 5, 5, width);
        self.roots.push(root2);

        let mut root3 = RootSystem::new(Algorithm::Greedy, width / 2, height - 5, width);
        root3.set_target(width / 2, 5, width);
        self.roots.push(root3);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &app))?;

        let timeout = app
            .tick_rate
            .checked_sub(app.last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char('r') => app.reset(),
                        _ => {}
                    }
                }
            }
        }

        if app.last_tick.elapsed() >= app.tick_rate {
            app.on_tick();
            app.last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn ui(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Render the grid and roots
    let soil_widget = SoilWidget {
        grid: &app.grid,
        roots: &app.roots,
    };

    frame.render_widget(soil_widget, size);
}

struct SoilWidget<'a> {
    grid: &'a Grid,
    roots: &'a Vec<RootSystem>,
}

impl Widget for SoilWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We iterate over the grid and map to the buffer area
        // Note: The grid might be larger or smaller than the area.
        // For simplicity, we clamp to the smaller dimensions.

        let width = area.width.min(self.grid.width as u16);
        let height = area.height.min(self.grid.height as u16);

        for y in 0..height {
            for x in 0..width {
                let idx = (y as usize) * self.grid.width + (x as usize);
                let cell = &self.grid.cells[idx];

                let (symbol, fg, bg) = match cell.kind {
                    SoilType::Rock => ("#", Color::Black, Color::DarkGray),
                    SoilType::Water => ("~", Color::White, Color::Blue),
                    SoilType::Clay => ("·", Color::DarkGray, Color::Rgb(60, 40, 20)), // Brownish
                    SoilType::Sand => ("░", Color::Yellow, Color::Rgb(100, 80, 50)),
                };

                if let Some(cell) = buf.cell_mut((area.left() + x, area.top() + y)) {
                    cell.set_symbol(symbol).set_fg(fg).set_bg(bg);
                }
            }
        }

        // Render Roots on top
        for root in self.roots.iter() {
            let color = match root.algorithm {
                Algorithm::Dijkstra => Color::Green, // Classic biology
                Algorithm::AStar => Color::Magenta,  // Smart logic
                Algorithm::Greedy => Color::Red,     // Aggressive
            };

            for &idx in root.visited.keys() {
                let rx = idx % self.grid.width;
                let ry = idx / self.grid.width;

                if rx < width as usize && ry < height as usize {
                    if let Some(cell) =
                        buf.cell_mut((area.left() + rx as u16, area.top() + ry as u16))
                    {
                        cell.set_symbol("√").set_fg(color);
                    }
                }
            }

            // Highlight frontier tips
            for state in &root.frontier {
                let idx = state.position;
                let rx = idx % self.grid.width;
                let ry = idx / self.grid.width;
                if rx < width as usize && ry < height as usize {
                    if let Some(cell) =
                        buf.cell_mut((area.left() + rx as u16, area.top() + ry as u16))
                    {
                        cell.set_symbol("o").set_fg(Color::White).set_bg(color);
                    }
                }
            }
        }

        // Overlay Help Text
        let help_text = "Q: Quit | R: Reset";
        buf.set_string(
            area.left(),
            area.top(),
            help_text,
            Style::default().fg(Color::White).bg(Color::Black),
        );
    }
}

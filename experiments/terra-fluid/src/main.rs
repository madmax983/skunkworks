use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

mod physics;
mod game;

use game::Game;

#[derive(PartialEq)]
enum Tool {
    Rain,
    RaiseLand,
    LowerLand,
    PlaceUnit,
}

struct App {
    game: Game,
    tool: Tool,
    running: bool,
    mouse_pressed: bool,
    mouse_pos: (u16, u16),
}

impl App {
    fn new() -> Self {
        Self {
            game: Game::new(120, 80),
            tool: Tool::Rain,
            running: true,
            mouse_pressed: false,
            mouse_pos: (0, 0),
        }
    }

    fn on_tick(&mut self) {
        // Run physics multiple times per frame for stability/speed
        for _ in 0..2 {
            self.game.update();
        }

        if self.mouse_pressed {
            self.apply_tool();
        }
    }

    fn apply_tool(&mut self) {
        // Map mouse pos to grid
        // This is approximate as we don't know the exact Rect of the Canvas here easily
        // But we can assume the canvas fills the top chunk.
        // Screen coords (col, row).
        // We need to map to (0..width, 0..height).
        // Let's assume 1:1 mapping with some offset for borders.

        let x = self.mouse_pos.0.saturating_sub(1) as usize;
        let y = self.mouse_pos.1.saturating_sub(1) as usize;

        // Invert Y for simulation? TUI is (0,0) top-left.
        // Canvas is (0,0) bottom-left usually unless inverted.
        // We'll map TUI y to Sim y.
        // Sim (0,0) is top-left in our grid logic usually.
        // But Canvas expects Y up.
        // Let's stick to TUI coordinates (Y down) for logic if possible,
        // OR map TUI (Y down) to Sim (Y down) and flip for Canvas.

        // Canvas Logic:
        // x_bounds: [0, w]
        // y_bounds: [0, h]
        // Paint: (x, h - y) to flip?

        // Let's treat Sim Y as "Down".
        // When painting on Canvas (Y Up), we draw at (x, Height - y).
        // So Mouse (Top-Left) corresponds to Sim (0,0) corresponds to Canvas (0, Height).
        // So Mouse Y maps directly to Sim Y.

        // Canvas uses 2x resolution vertically usually (Braille/Block)?
        // Ratatui Canvas with Block marker is 1 cell = 1 pixel usually?
        // No, Block marker is low res. Braille is high res.
        // Let's use Block marker.

        let sim_x = x.clamp(0, self.game.sim.width - 1);
        let sim_y = (y * 2).clamp(0, self.game.sim.height - 1);

        match self.tool {
            Tool::Rain => self.game.rain(sim_x, sim_y),
            Tool::RaiseLand => self.game.terraform(sim_x, sim_y, 0.5),
            Tool::LowerLand => self.game.terraform(sim_x, sim_y, -0.5),
            Tool::PlaceUnit => self.game.spawn_unit(sim_x as f32, sim_y as f32),
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut app = App::new();

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => app.running = false,
                            KeyCode::Char('1') => app.tool = Tool::Rain,
                            KeyCode::Char('2') => app.tool = Tool::RaiseLand,
                            KeyCode::Char('3') => app.tool = Tool::LowerLand,
                            KeyCode::Char('4') => app.tool = Tool::PlaceUnit,
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                     app.mouse_pos = (mouse.column, mouse.row);
                     match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.mouse_pressed = true;
                            app.apply_tool();
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.mouse_pressed = false;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            app.mouse_pressed = true; // Ensure pressed state
                            app.apply_tool();
                        }
                        _ => {}
                    }
                }
                _ => {}
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

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Terra Fluid: Shallow Water Strategy"),
        )
        .x_bounds([0.0, app.game.sim.width as f64])
        .y_bounds([0.0, app.game.sim.height as f64])
        .marker(ratatui::symbols::Marker::Block)
        .paint(|ctx| {
            // Grouping for performance
            let mut water_colors: Vec<(Color, Vec<(f64, f64)>)> = vec![
                (Color::Cyan, vec![]),
                (Color::Blue, vec![]),
                (Color::DarkGray, vec![]), // Deep water
            ];

            let mut terrain_colors: Vec<(Color, Vec<(f64, f64)>)> = vec![
                (Color::Green, vec![]),
                (Color::Yellow, vec![]),
                (Color::White, vec![]), // High peaks
                (Color::Red, vec![]), // Negative/Low
            ];

            let height = app.game.sim.height;
            let width = app.game.sim.width;

            for y in 0..height {
                for x in 0..width {
                    let h = app.game.sim.h.get(x, y);
                    let b = app.game.sim.b.get(x, y);

                    // Coordinates for Canvas (Y Up)
                    let draw_x = x as f64;
                    let draw_y = (height - 1 - y) as f64;

                    if h > 0.1 {
                        // Water
                        if h < 1.0 {
                            water_colors[0].1.push((draw_x, draw_y));
                        } else if h < 3.0 {
                            water_colors[1].1.push((draw_x, draw_y));
                        } else {
                            water_colors[2].1.push((draw_x, draw_y));
                        }
                    } else {
                        // Terrain
                        if b < 0.0 {
                            terrain_colors[3].1.push((draw_x, draw_y));
                        } else if b < 2.0 {
                             terrain_colors[0].1.push((draw_x, draw_y));
                        } else if b < 5.0 {
                             terrain_colors[1].1.push((draw_x, draw_y));
                        } else {
                             terrain_colors[2].1.push((draw_x, draw_y));
                        }
                    }
                }
            }

            // Draw Terrain first
            for (color, points) in terrain_colors {
                ctx.draw(&Points { coords: &points, color });
            }

            // Draw Water
            for (color, points) in water_colors {
                ctx.draw(&Points { coords: &points, color });
            }

            // Draw Units
            for unit in &app.game.units {
                if unit.alive {
                    ctx.print(unit.x as f64, (height as f32 - 1.0 - unit.y) as f64, unit.symbol.to_string());
                } else {
                    ctx.print(unit.x as f64, (height as f32 - 1.0 - unit.y) as f64, String::from("†"));
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let tool_str = match app.tool {
        Tool::Rain => "RAIN",
        Tool::RaiseLand => "RAISE",
        Tool::LowerLand => "LOWER",
        Tool::PlaceUnit => "UNIT",
    };

    let status = Paragraph::new(Line::from(vec![
        Span::raw("Tool: "),
        Span::styled(tool_str, Style::default().fg(Color::Yellow).bold()),
        Span::raw(" | [1] Rain [2] Raise [3] Lower [4] Unit | [Q]uit"),
    ])).block(Block::default().borders(Borders::ALL));

    f.render_widget(status, chunks[1]);
}

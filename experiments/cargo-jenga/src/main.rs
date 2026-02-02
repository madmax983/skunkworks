mod deps;
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use physics::{PhysicsWorld, RenderBody};
use rapier2d::prelude::*;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    prelude::*,
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Line},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let mut app = App::new();
    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }

    Ok(())
}

struct App {
    world: PhysicsWorld,
    camera_y: f64,
    paused: bool,
    mouse_pos: (u16, u16),
}

impl App {
    fn new() -> Self {
        let mut world = PhysicsWorld::new();
        world.spawn_ground();

        let crates = deps::fetch_workspace_crates().unwrap_or_else(|_| vec![]);
        world.spawn_tower(&crates);

        Self {
            world,
            camera_y: 10.0, // Start looking a bit up
            paused: false,
            mouse_pos: (0, 0),
        }
    }

    fn update(&mut self) {
        if !self.paused {
            self.world.step();
        }
    }

    fn handle_click(&mut self, area: Rect) {
        let canvas_width = 40.0;
        let canvas_height = 40.0;

        let left = -20.0;
        let bottom = self.camera_y - 20.0;

        // Screen area
        let rect_x = area.x;
        let rect_y = area.y;
        let rect_w = area.width;
        let rect_h = area.height;

        let mx = self.mouse_pos.0;
        let my = self.mouse_pos.1;

        if mx >= rect_x && mx < rect_x + rect_w && my >= rect_y && my < rect_y + rect_h {
            // Normalize
            let nx = (mx - rect_x) as f64 / rect_w as f64;
            let ny = (my - rect_y) as f64 / rect_h as f64;

            // Map to physics
            // Ratatui Y is 0 at top, increasing down.
            // Physics Y is increasing up.
            // So ny=0 (top) -> top of canvas (bottom + height)
            // ny=1 (bottom) -> bottom of canvas

            let phys_x = left + nx * canvas_width;
            let phys_y = bottom + (1.0 - ny) * canvas_height;

            self.world.remove_body_at(phys_x as f32, phys_y as f32);
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Up => app.camera_y += 1.0,
                        KeyCode::Down => app.camera_y -= 1.0,
                        KeyCode::Char('r') => {
                            // Reset
                            *app = App::new();
                        }
                        _ => {}
                    }
                }
                Event::Mouse(mouse) => {
                    app.mouse_pos = (mouse.column, mouse.row);
                    if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
                        let size = terminal.size()?;
                        let rect = Rect::new(0, 0, size.width, size.height);
                        let chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([Constraint::Min(0), Constraint::Length(3)])
                            .split(rect);

                        app.handle_click(chunks[0]);
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let x_bounds = [-20.0, 20.0];
    let y_bounds = [app.camera_y - 20.0, app.camera_y + 20.0];

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Cargo Jenga "),
        )
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            let bodies = app.world.get_render_bodies();
            for body in bodies {
                draw_body(ctx, &body);
            }

            // Draw Ground
            ctx.draw(&Line {
                x1: -20.0,
                y1: 0.0,
                x2: 20.0,
                y2: 0.0,
                color: Color::White,
            });
        });

    f.render_widget(canvas, chunks[0]);

    let help_text = format!(
        "Controls: [Click] Remove Block | [Arrows] Move Camera | [Space] Pause | [R] Reset | [Q] Quit | Bodies: {}",
        app.world.rigid_body_set.len()
    );
    let help = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL));
    f.render_widget(help, chunks[1]);
}

fn draw_body(ctx: &mut ratatui::widgets::canvas::Context, body: &RenderBody) {
    let pos = body.position;
    let shape = &body.shape;

    // Default color if info missing (e.g. ground)
    let (r, g, b) = if let Some(info) = &body.info {
        info.color
    } else {
        (100, 100, 100)
    };
    let color = Color::Rgb(r, g, b);

    if let TypedShape::Cuboid(c) = shape.as_typed_shape() {
        // Calculate 4 corners
        let hx = c.half_extents.x;
        let hy = c.half_extents.y;
        // nalgebra points
        let corners = [
            point![-hx, -hy],
            point![hx, -hy],
            point![hx, hy],
            point![-hx, hy],
        ];

        let world_corners: Vec<(f64, f64)> = corners
            .iter()
            .map(|p| {
                let wp = pos * p;
                (wp.x as f64, wp.y as f64)
            })
            .collect();

        // Draw lines
        for i in 0..4 {
            let (x1, y1) = world_corners[i];
            let (x2, y2) = world_corners[(i + 1) % 4];
            ctx.draw(&Line {
                x1,
                y1,
                x2,
                y2,
                color,
            });
        }

        // Draw Label if large enough
        if let Some(info) = &body.info {
            if hx > 1.0 {
                // arbitrary size threshold
                let tx = pos.translation.x as f64;
                let ty = pos.translation.y as f64;
                // Clone the name to avoid lifetime issue
                ctx.print(
                    tx - (hx as f64 * 0.8),
                    ty,
                    Span::styled(info.name.clone(), Style::default().fg(color)),
                );
            }
        }
    }
}

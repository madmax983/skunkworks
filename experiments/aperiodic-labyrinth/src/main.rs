pub mod tiling;

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use nalgebra::Point2;
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Frame,
};
use tiling::{Triangle, TriangleType};
use tui_shared::Tui;

struct App {
    triangles: Vec<Triangle>,
    depth: usize,
    player_pos: Point2<f64>,
    zoom: f64,
    exit: bool,
    current_room: Option<TriangleType>,
    artifacts: Vec<Point2<f64>>,
    score: usize,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            triangles: Vec::new(),
            depth: 0,
            player_pos: Point2::new(0.0, 0.0),
            zoom: 1.0,
            exit: false,
            current_room: None,
            artifacts: Vec::new(),
            score: 0,
        };
        app.regenerate();
        app
    }

    fn regenerate(&mut self) {
        let mut tris = tiling::generate_initial_sun();
        for _ in 0..self.depth {
            tris = tiling::subdivide(&tris);
        }
        self.triangles = tris;

        // Spawn artifacts on random triangle centers
        let mut rng = rand::thread_rng();
        self.artifacts.clear();
        for tri in &self.triangles {
            if rng.gen_bool(0.1) { // 10% chance
                self.artifacts.push(tri.center());
            }
        }
    }

    fn update_depth(&mut self, delta: i32) {
        if delta > 0 {
            if self.depth < 6 {
                self.depth += 1;
                self.regenerate();
            }
        } else if self.depth > 0 {
            self.depth -= 1;
            self.regenerate();
        }
    }

    fn move_player(&mut self, dx: f64, dy: f64) {
        self.player_pos.x += dx * 5.0 / self.zoom;
        self.player_pos.y += dy * 5.0 / self.zoom;

        // Update current room
        self.current_room = None;
        for tri in &self.triangles {
            if tri.contains(self.player_pos) {
                self.current_room = Some(tri.ttype);
                break;
            }
        }

        // Collect artifacts
        self.artifacts.retain(|art| {
            let dist = (art.x - self.player_pos.x).hypot(art.y - self.player_pos.y);
            if dist < 2.0 {
                self.score += 1;
                false // Remove
            } else {
                true
            }
        });
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        if app.exit {
            break;
        }

        tui.terminal.draw(|frame| render(frame, &mut app))?;

        if crossterm::event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = crossterm::event::read()? {
                if key.kind == KeyEventKind::Press {
                    handle_input(&mut app, key);
                }
            }
        }
    }
    Ok(())
}

fn handle_input(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.exit = true,
        KeyCode::Char('+') | KeyCode::Char('=') => app.update_depth(1),
        KeyCode::Char('-') | KeyCode::Char('_') => app.update_depth(-1),
        KeyCode::Char('z') => app.zoom *= 1.1,
        KeyCode::Char('x') => app.zoom /= 1.1,
        KeyCode::Left => app.move_player(-1.0, 0.0),
        KeyCode::Right => app.move_player(1.0, 0.0),
        KeyCode::Up => app.move_player(0.0, 1.0),
        KeyCode::Down => app.move_player(0.0, -1.0),
        _ => {}
    }
}

fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let canvas_area = main_layout[0];
    let status_area = main_layout[1];

    let view_width = 150.0 / app.zoom;
    let view_height = 100.0 / app.zoom;

    let x_bounds = [
        app.player_pos.x - view_width / 2.0,
        app.player_pos.x + view_width / 2.0,
    ];
    let y_bounds = [
        app.player_pos.y - view_height / 2.0,
        app.player_pos.y + view_height / 2.0,
    ];

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title(" Aperiodic Labyrinth "))
        .x_bounds(x_bounds)
        .y_bounds(y_bounds)
        .paint(|ctx| {
            let fog_radius = 50.0;

            // Draw triangles
            for tri in &app.triangles {
                let center = tri.center();

                // Fog of War
                let dist = (center.x - app.player_pos.x).hypot(center.y - app.player_pos.y);
                if dist > fog_radius {
                    continue;
                }

                // Culling for viewport (optimization)
                if center.x < x_bounds[0] - 20.0 || center.x > x_bounds[1] + 20.0 ||
                   center.y < y_bounds[0] - 20.0 || center.y > y_bounds[1] + 20.0 {
                    continue;
                }

                let color = match tri.ttype {
                    TriangleType::Acute => Color::Cyan,
                    TriangleType::Obtuse => Color::Magenta,
                };

                let [a, b, c] = tri.vertices;

                ctx.draw(&CanvasLine { x1: a.x, y1: a.y, x2: b.x, y2: b.y, color });
                ctx.draw(&CanvasLine { x1: b.x, y1: b.y, x2: c.x, y2: c.y, color });
                ctx.draw(&CanvasLine { x1: c.x, y1: c.y, x2: a.x, y2: a.y, color });
            }

            // Draw Artifacts
            for art in &app.artifacts {
                let dist = (art.x - app.player_pos.x).hypot(art.y - app.player_pos.y);
                if dist <= fog_radius {
                     ctx.print(art.x, art.y, Span::styled("*", Style::default().fg(Color::Yellow)));
                }
            }

            // Draw Player
            ctx.print(
                app.player_pos.x,
                app.player_pos.y,
                Span::styled("@", Style::default().fg(Color::White).bold()),
            );
        });

    frame.render_widget(canvas, canvas_area);

    // Status Bar
    let room_text = match app.current_room {
        Some(TriangleType::Acute) => "Acute Chamber",
        Some(TriangleType::Obtuse) => "Obtuse Hall",
        None => "Void",
    };

    let status_text = vec![
        Span::raw("Pos: "),
        Span::styled(format!("{:.1}, {:.1}", app.player_pos.x, app.player_pos.y), Style::default().fg(Color::Yellow)),
        Span::raw(" | Depth: "),
        Span::styled(format!("{}", app.depth), Style::default().fg(Color::Green)),
        Span::raw(" | Room: "),
        Span::styled(room_text, Style::default().fg(Color::Cyan)),
        Span::raw(" | Score: "),
        Span::styled(format!("{}", app.score), Style::default().fg(Color::Red)),
        Span::raw(" | [WASD] Move [+/-] Depth [z/x] Zoom [q] Quit"),
    ];

    let status_block = Paragraph::new(Line::from(status_text))
        .style(Style::default().bg(Color::Black).fg(Color::White));

    frame.render_widget(status_block, status_area);
}

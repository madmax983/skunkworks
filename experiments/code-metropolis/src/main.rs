use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::Color,
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::{self};
use std::time::{Duration, Instant};

use code_metropolis::iso::{Camera, Cube, Point3D};
use code_metropolis::layout::generate_layout;
use code_metropolis::scanner::scan;

const LAYOUT_X: f64 = -50.0;
const LAYOUT_Y: f64 = -50.0;
const LAYOUT_WIDTH: f64 = 100.0;
const LAYOUT_HEIGHT: f64 = 100.0;

const HEIGHT_THRESHOLD_RED: f64 = 10.0;
const HEIGHT_THRESHOLD_MAGENTA: f64 = 6.0;
const HEIGHT_THRESHOLD_YELLOW: f64 = 4.0;
const HEIGHT_THRESHOLD_CYAN: f64 = 2.0;

const CANVAS_X_BOUNDS: [f64; 2] = [-150.0, 150.0];
const CANVAS_Y_BOUNDS: [f64; 2] = [-100.0, 100.0];

const INITIAL_CAMERA_OFFSET_Y: f64 = 10.0;
const CAMERA_MOVE_SPEED: f64 = 5.0;
const CAMERA_ZOOM_FACTOR: f64 = 1.1;
const CAMERA_ROTATE_SPEED: f64 = 0.05;
const TICK_RATE_MS: u64 = 16;

struct App {
    cubes: Vec<Cube>,
    camera: Camera,
}

impl App {
    fn new(path: String) -> Result<Self> {
        println!("Scanning {}... (This might take a moment)", path);
        let root = scan(&path)?;

        let blocks = generate_layout(&root, LAYOUT_X, LAYOUT_Y, LAYOUT_WIDTH, LAYOUT_HEIGHT);
        println!("Generated {} blocks.", blocks.len());

        let cubes: Vec<Cube> = blocks
            .iter()
            .map(|b| {
                let color = if b.is_dir {
                    Color::DarkGray
                } else if b.height > HEIGHT_THRESHOLD_RED {
                    Color::Red
                } else if b.height > HEIGHT_THRESHOLD_MAGENTA {
                    Color::Magenta
                } else if b.height > HEIGHT_THRESHOLD_YELLOW {
                    Color::Yellow
                } else if b.height > HEIGHT_THRESHOLD_CYAN {
                    Color::Cyan
                } else {
                    Color::Green
                };

                Cube {
                    origin: Point3D {
                        x: b.x,
                        y: 0.0,
                        z: b.z,
                    },
                    width: b.width,
                    depth: b.depth,
                    height: b.height,
                    color,
                }
            })
            .collect();

        Ok(Self {
            cubes,
            camera: Camera {
                scale: 1.0,
                offset_y: INITIAL_CAMERA_OFFSET_Y,
                ..Camera::default()
            },
        })
    }

    fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        let tick_rate = Duration::from_millis(TICK_RATE_MS);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Char('w') => self.camera.offset_y -= CAMERA_MOVE_SPEED,
                            KeyCode::Char('s') => self.camera.offset_y += CAMERA_MOVE_SPEED,
                            KeyCode::Char('a') => self.camera.offset_x += CAMERA_MOVE_SPEED,
                            KeyCode::Char('d') => self.camera.offset_x -= CAMERA_MOVE_SPEED,
                            KeyCode::Char('z') => self.camera.scale *= CAMERA_ZOOM_FACTOR,
                            KeyCode::Char('x') => self.camera.scale /= CAMERA_ZOOM_FACTOR,
                            KeyCode::Char('r') => self.camera.angle += CAMERA_ROTATE_SPEED,
                            KeyCode::Char('f') => self.camera.angle -= CAMERA_ROTATE_SPEED,
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }
    }

    fn sort_cubes_by_depth(&self) -> Vec<usize> {
        // Sorting for Painter's Algorithm
        let mut sorted_indices: Vec<usize> = (0..self.cubes.len()).collect();

        // Sort by depth (furthest first).
        sorted_indices.sort_unstable_by(|&a_idx, &b_idx| {
            let a = &self.cubes[a_idx];
            let b = &self.cubes[b_idx];

            // Midpoint of cube for sorting
            let a_mx = a.origin.x + a.width / 2.0;
            let a_mz = a.origin.z + a.depth / 2.0;
            let b_mx = b.origin.x + b.width / 2.0;
            let b_mz = b.origin.z + b.depth / 2.0;

            let z_rot_a = a_mx * self.camera.angle.sin() + a_mz * self.camera.angle.cos();
            let z_rot_b = b_mx * self.camera.angle.sin() + b_mz * self.camera.angle.cos();

            // For a Painter's Algorithm, we need to draw objects from furthest to nearest.
            // Here, a larger rotated Z-coordinate (`z_rot`) is considered further away.
            // Therefore, we sort in descending order of `z_rot`.
            z_rot_b
                .partial_cmp(&z_rot_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted_indices
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(0), Constraint::Length(3)])
            .split(f.area());

        let sorted_indices = self.sort_cubes_by_depth();

        let canvas = Canvas::default()
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Code Metropolis"),
            )
            .x_bounds(CANVAS_X_BOUNDS)
            .y_bounds(CANVAS_Y_BOUNDS)
            .paint(|ctx| {
                for &i in &sorted_indices {
                    let cube = &self.cubes[i];
                    let lines = cube.get_lines(&self.camera);
                    for line in lines {
                        ctx.draw(&line);
                    }
                }
            });

        f.render_widget(canvas, chunks[0]);

        let help_text = format!(
            "WASD: Pan | ZX: Zoom | RF: Rotate | Q: Quit | Objects: {} | Angle: {:.2}",
            self.cubes.len(),
            self.camera.angle
        );
        f.render_widget(
            Paragraph::new(help_text).block(Block::default().borders(Borders::ALL)),
            chunks[1],
        );
    }
}

fn main() -> Result<()> {
    // Create and initialize the application
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let mut app = App::new(path)?;

    if app.cubes.is_empty() {
        println!("No files found to visualize.");
        return Ok(());
    }

    // 4. Terminal UI Loop
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = app.run(&mut terminal);

    // 5. Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

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
    widgets::{
        canvas::Canvas,
        Block, Borders, Paragraph,
    },
    Frame, Terminal,
};
use std::io::{self};
use std::time::{Duration, Instant};

use code_metropolis::scanner::scan;
use code_metropolis::layout::generate_layout;
use code_metropolis::iso::{Camera, Cube, Point3D};

fn main() -> Result<()> {
    // 1. Setup & Scan
    let path = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    println!("Scanning {}... (This might take a moment)", path);
    let root = scan(&path)?;

    // 2. Layout
    // Base layout size 100x100
    let blocks = generate_layout(&root, -50.0, -50.0, 100.0, 100.0);
    println!("Generated {} blocks.", blocks.len());

    // 3. Convert to Cubes
    let cubes: Vec<Cube> = blocks.iter().map(|b| {
        let color = if b.is_dir {
            Color::DarkGray
        } else {
            // Color map based on height (complexity/size)
            if b.height > 10.0 { Color::Red }
            else if b.height > 6.0 { Color::Magenta }
            else if b.height > 4.0 { Color::Yellow }
            else if b.height > 2.0 { Color::Cyan }
            else { Color::Green }
        };

        Cube {
            origin: Point3D { x: b.x, y: 0.0, z: b.z },
            width: b.width,
            depth: b.depth,
            height: b.height,
            color,
        }
    }).collect();

    if cubes.is_empty() {
        println!("No files found to visualize.");
        return Ok(());
    }

    // 4. Terminal UI Loop
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut camera = Camera {
        scale: 1.0,
        offset_y: 10.0,
        ..Camera::default()
    };

    let res = run_app(&mut terminal, &cubes, &mut camera);

    // 5. Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    cubes: &[Cube],
    camera: &mut Camera,
) -> Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, cubes, camera))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('w') => camera.offset_y -= 5.0, // Up moves map down? screen coords y+ is down
                        KeyCode::Char('s') => camera.offset_y += 5.0,
                        KeyCode::Char('a') => camera.offset_x += 5.0,
                        KeyCode::Char('d') => camera.offset_x -= 5.0,
                        KeyCode::Char('z') => camera.scale *= 1.1,
                        KeyCode::Char('x') => camera.scale /= 1.1,
                        KeyCode::Char('r') => camera.angle += 0.05,
                        KeyCode::Char('f') => camera.angle -= 0.05,
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

fn ui(f: &mut Frame, cubes: &[Cube], camera: &Camera) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Sorting for Painter's Algorithm
    let mut sorted_indices: Vec<usize> = (0..cubes.len()).collect();

    // Sort by depth (furthest first).
    // Depth = distance along camera view axis.
    // View axis vector (looking at origin):
    // V = (sin(angle), 0, cos(angle)) ? No, rotated.
    // The "depth" value we used in iso projection was roughly (x_rot + z_rot).
    // Let's use the rotated Z coordinate.
    // Z_rot = x * sin(a) + z * cos(a)
    // We want to draw from smallest Z_rot (far) to largest Z_rot (near) if +Z is towards cam.
    // Or if +Z is into screen, then largest to smallest.
    // Let's guess: Draw Smallest Z_rot first (background).

    sorted_indices.sort_by(|&a_idx, &b_idx| {
        let a = &cubes[a_idx];
        let b = &cubes[b_idx];

        // Midpoint of cube for sorting
        let a_mx = a.origin.x + a.width / 2.0;
        let a_mz = a.origin.z + a.depth / 2.0;
        let b_mx = b.origin.x + b.width / 2.0;
        let b_mz = b.origin.z + b.depth / 2.0;

        let z_rot_a = a_mx * camera.angle.sin() + a_mz * camera.angle.cos();
        let z_rot_b = b_mx * camera.angle.sin() + b_mz * camera.angle.cos();

        // Draw smallest first (assuming coordinate system where smaller is "further back")
        // In 45deg iso (x-z), x+z is depth?
        // If angle=0, z is depth. Small z is far?
        // Let's assume standard Z-buffer: draw furthest first.
        // Try ascending sort.
        z_rot_b.partial_cmp(&z_rot_a).unwrap_or(std::cmp::Ordering::Equal)
    });

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Code Metropolis"))
        .x_bounds([-150.0, 150.0])
        .y_bounds([-100.0, 100.0]) // Aspect correction handled by projection?
        .paint(|ctx| {
            for &i in &sorted_indices {
                let cube = &cubes[i];
                let lines = cube.get_lines(camera);
                for line in lines {
                    ctx.draw(&line);
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let help_text = format!(
        "WASD: Pan | ZX: Zoom | RF: Rotate | Q: Quit | Objects: {} | Angle: {:.2}",
        cubes.len(), camera.angle
    );
    f.render_widget(
        Paragraph::new(help_text).block(Block::default().borders(Borders::ALL)),
        chunks[1]
    );
}

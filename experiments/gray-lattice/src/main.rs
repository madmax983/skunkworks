use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use gray_scott::GrayScott;
use miller_lattice::Crystal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{canvas::Canvas, Block, Borders, Paragraph},
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

struct GrayLatticeApp {
    sim: GrayScott,
    width: usize,
    height: usize,
    crystal: Crystal,
    atom_positions: Vec<(usize, usize, bool)>, // (x, y, is_dir)
}

impl GrayLatticeApp {
    fn new(width: usize, height: usize) -> Self {
        // Run miller-lattice to get the codebase structure
        let root_path = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
        let crystal = Crystal::build_from_path(&root_path).unwrap_or_else(|_| Crystal::new());

        let mut atom_positions = Vec::new();

        let mut min_x = f32::MAX;
        let mut max_x = f32::MIN;
        let mut min_y = f32::MAX;
        let mut max_y = f32::MIN;

        for atom in &crystal.atoms {
            min_x = min_x.min(atom.position.x as f32);
            max_x = max_x.max(atom.position.x as f32);
            min_y = min_y.min(atom.position.y as f32);
            max_y = max_y.max(atom.position.y as f32);
        }

        let crys_w = (max_x - min_x).max(1.0);
        let crys_h = (max_y - min_y).max(1.0);

        for atom in &crystal.atoms {
            let normalized_x = (atom.position.x as f32 - min_x) / crys_w;
            let normalized_y = (atom.position.y as f32 - min_y) / crys_h;

            // Map to gray-scott simulation grid coordinates
            let grid_x =
                ((normalized_x * (width as f32 - 1.0)).round() as usize).clamp(0, width - 1);
            let grid_y =
                ((normalized_y * (height as f32 - 1.0)).round() as usize).clamp(0, height - 1);

            atom_positions.push((grid_x, grid_y, atom.is_dir));
        }

        Self {
            sim: GrayScott::new(width, height),
            width,
            height,
            crystal,
            atom_positions,
        }
    }

    fn tick(&mut self) {
        // First step the chemical reaction-diffusion (Standard feed/kill for Turing patterns)
        self.sim.update(0.055, 0.062, 1.0);

        // The rigid 3D codebase lattice acts as constraints/catalysts on the 2D Gray-Scott grid
        let width = self.width;
        for &(x, y, is_dir) in &self.atom_positions {
            let idx = y * width + x;
            if is_dir {
                // Directories act as strong sources of 'V' (kill) chemical -> decay/boundaries
                self.sim.v_mut()[idx] = (self.sim.v()[idx] + 0.1).min(1.0);
            } else {
                // Files act as strong sources of 'U' (feed) chemical -> growth
                self.sim.u_mut()[idx] = (self.sim.u()[idx] + 0.1).min(1.0);
            }
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--headless".to_string()) {
        println!("Running gray-lattice in headless mode...");
        let mut app = GrayLatticeApp::new(100, 50);
        for _ in 0..10 {
            app.tick();
        }
        println!("Finished headless run.");
        return Ok(());
    }

    let mut tui = Tui::init()?;
    let res = run_app(&mut tui);
    tui.exit()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app(tui: &mut Tui) -> Result<()> {
    let width = 100;
    let height = 50;
    let mut app = GrayLatticeApp::new(width, height);

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(30);

    // Initial chemical spark to start the reaction
    let center_idx = (height / 2) * width + (width / 2);
    app.sim.v_mut()[center_idx] = 1.0;

    loop {
        tui.terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" 🧬 Splice: gray-scott × miller-lattice (Morphogenesis) "),
                )
                .x_bounds([0.0, app.width as f64])
                .y_bounds([0.0, app.height as f64])
                .paint(|ctx| {
                    for y in 0..app.height {
                        for x in 0..app.width {
                            let idx = y * app.width + x;
                            let u = app.sim.u()[idx];
                            let v = app.sim.v()[idx];

                            if v > 0.1 {
                                let color = if v > 0.5 {
                                    Color::Red
                                } else if v > 0.3 {
                                    Color::Yellow
                                } else {
                                    Color::DarkGray
                                };

                                ctx.print(
                                    x as f64,
                                    (app.height - 1 - y) as f64,
                                    ratatui::text::Span::styled("█", Style::default().fg(color)),
                                );
                            } else if u < 0.9 {
                                ctx.print(
                                    x as f64,
                                    (app.height - 1 - y) as f64,
                                    ratatui::text::Span::styled(
                                        "░",
                                        Style::default().fg(Color::Blue),
                                    ),
                                );
                            }
                        }
                    }

                    // Draw crystal atoms overlay
                    for &(x, y, is_dir) in &app.atom_positions {
                        let color = if is_dir { Color::Green } else { Color::White };
                        ctx.print(
                            x as f64,
                            (app.height - 1 - y) as f64,
                            ratatui::text::Span::styled("●", Style::default().fg(color)),
                        );
                    }
                });

            f.render_widget(canvas, chunks[0]);

            let stats = Paragraph::new(format!(
                "Lattice Atoms: {} | Directories: Green, Files: White | [Q] Quit",
                app.atom_positions.len(),
            ))
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(stats, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = Instant::now();
        }
    }
}

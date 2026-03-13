use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use locus::Vec2;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::{io, time::Duration};

mod simulation;
use simulation::World;

mod hologram;
use hologram::Hologram;

struct App {
    world: World,
    hologram: Hologram,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> App {
        let width = 64;
        let height = 64;
        App {
            world: World::new(),
            hologram: Hologram::new(width, height),
            reconstruction_angle_x: 0,
            reconstruction_angle_y: 0,
            status_msg: "Locust Hologram: Press Space to Add Firewall".to_string(),
            reconstruction_data: vec![0.0; width * height],
        }
    }

    fn update(&mut self) {
        self.world.update();

        let width = self.hologram.width;
        let height = self.hologram.height;
        let mut density = vec![0.0; width * height];

        for agent in &self.world.agents {
            let px = (agent.pos.x / simulation::WORLD_SIZE * width as f64) as usize;
            let py = (agent.pos.y / simulation::WORLD_SIZE * height as f64) as usize;
            if px < width && py < height {
                density[py * width + px] += 1.0;
            }
        }

        self.hologram.set_density(&density);
        self.hologram.process_fft();
        self.reconstruction_data = self
            .hologram
            .reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    std::io::Error: From<<B as Backend>::Error>,
{
    loop {
        app.update();
        terminal.draw(|f| ui::<B>(f, app))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Left => app.reconstruction_angle_x -= 1,
                        KeyCode::Right => app.reconstruction_angle_x += 1,
                        KeyCode::Up => app.reconstruction_angle_y -= 1,
                        KeyCode::Down => app.reconstruction_angle_y += 1,
                        KeyCode::Char(' ') => {
                            let mut rng = rand::thread_rng();
                            use rand::Rng;
                            let fw_x = rng.gen_range(0.0..simulation::WORLD_SIZE);
                            let fw_y = rng.gen_range(0.0..simulation::WORLD_SIZE);
                            app.world.add_firewall(Vec2::new(fw_x, fw_y), 10.0);
                            app.status_msg =
                                format!("Firewall added at ({:.1}, {:.1})", fw_x, fw_y);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut ratatui::Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
        .split(f.area());

    let width = app.hologram.width;
    let height = app.hologram.height;

    // Normalize reconstruction data for display
    let mut max_val = 0.0;
    for &val in &app.reconstruction_data {
        if val > max_val {
            max_val = val;
        }
    }

    // Prepare points
    let mut points: Vec<(f64, f64, ratatui::style::Color)> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let val = app.reconstruction_data[y * width + x];
            if max_val > 0.0 {
                let norm = val / max_val;
                if norm > 0.1 {
                    let color = if norm > 0.8 {
                        Color::Red
                    } else if norm > 0.5 {
                        Color::Yellow
                    } else {
                        Color::Cyan
                    };
                    points.push((x as f64, y as f64, color));
                }
            }
        }
    }

    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("Holographic Spectral Fingerprint")
                .borders(Borders::ALL),
        )
        .paint(move |ctx| {
            for &(x, y, color) in &points {
                ctx.draw(&Points {
                    coords: &[(x, y)],
                    color,
                });
            }
        })
        .x_bounds([0.0, width as f64])
        .y_bounds([0.0, height as f64]);

    f.render_widget(canvas, chunks[0]);

    let controls = Paragraph::new(format!(
        "Controls: [q] Quit | [Arrows] Shift Angle ({}, {}) | [Space] Spawn Firewall\nStatus: {}",
        app.reconstruction_angle_x, app.reconstruction_angle_y, app.status_msg
    ))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(controls, chunks[1]);
}

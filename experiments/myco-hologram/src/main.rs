use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
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

mod hologram;
mod simulation;

use hologram::Hologram;
use simulation::{Agent, World};

struct App {
    world: World,
    agents: Vec<Agent>,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    reconstruction_data: Vec<f64>,
    hologram_width: usize,
    hologram_height: usize,
}

impl App {
    fn new() -> Self {
        let width = 128;
        let height = 64;
        let num_cities = 5;
        let (world, agents) = World::with_cities_and_agents(width, height, num_cities);

        Self {
            world,
            agents,
            reconstruction_angle_x: 20,
            reconstruction_angle_y: 10,
            reconstruction_data: vec![0.0; width * height],
            hologram_width: width,
            hologram_height: height,
        }
    }

    fn tick(&mut self) {
        // Update simulation
        self.world.diffuse_and_decay();

        let mut deposits = Vec::new();

        for agent in &mut self.agents {
            let res = agent.update(&self.world);
            deposits.push(res);
        }

        // Apply deposits
        for d in deposits {
            if d.deposit_amount > 0.0 {
                self.world
                    .set_trail(d.deposit_x, d.deposit_y, d.deposit_amount);
            }
        }

        // Generate Hologram from trails
        let hologram = Hologram::from_grid(
            self.hologram_width,
            self.hologram_height,
            &self.world.trails,
            20.0,
            10.0,
        );
        self.reconstruction_data =
            hologram.reconstruct(self.reconstruction_angle_x, self.reconstruction_angle_y);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> Result<()>
where
    anyhow::Error: From<<B as Backend>::Error>,
{
    let mut last_tick = std::time::Instant::now();
    let tick_rate = Duration::from_millis(50);

    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Left => app.reconstruction_angle_x -= 1,
                        KeyCode::Right => app.reconstruction_angle_x += 1,
                        KeyCode::Up => app.reconstruction_angle_y -= 1,
                        KeyCode::Down => app.reconstruction_angle_y += 1,
                        KeyCode::Char(' ') => {
                            app.reconstruction_angle_x = 20;
                            app.reconstruction_angle_y = 10;
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.tick();
            last_tick = std::time::Instant::now();
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &App) {
    let size = f.area();

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🍄 Myco-Hologram: Pheromone Spectral Interference ")
        .style(Style::default().fg(Color::Cyan).bg(Color::Black));
    let inner_area = block.inner(size);
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(inner_area);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Holographic Reconstruction "),
        )
        .paint(|ctx| {
            for y in 0..app.hologram_height {
                for x in 0..app.hologram_width {
                    let val = app.reconstruction_data[y * app.hologram_width + x];
                    if val > 10.0 {
                        // Threshold for visibility
                        let color = if val > 30.0 {
                            Color::Cyan
                        } else if val > 20.0 {
                            Color::Blue
                        } else {
                            Color::DarkGray
                        };
                        ctx.draw(&Points {
                            coords: &[(x as f64, (app.hologram_height - 1 - y) as f64)],
                            color,
                        });
                    }
                }
            }
        })
        .x_bounds([0.0, app.hologram_width as f64])
        .y_bounds([0.0, app.hologram_height as f64]);

    f.render_widget(canvas, chunks[0]);

    let instructions = Paragraph::new(format!(
        "Use Arrow Keys to change reconstruction angle (Current: {}, {}). [Space] to reset. [Q/Esc] to quit.",
        app.reconstruction_angle_x, app.reconstruction_angle_y
    ))
    .block(Block::default().borders(Borders::ALL))
    .style(Style::default().fg(Color::Gray));

    f.render_widget(instructions, chunks[1]);
}

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::Color,
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
use simulation::{World, Vec2};

// Novel Trait: Spectral Foraging
struct App {
    hologram: Hologram,
    world: World,
    reconstruction_angle_x: isize,
    reconstruction_angle_y: isize,
    status_msg: String,
    reconstruction_data: Vec<f64>,
}

impl App {
    fn new() -> Self {
        let mut app = Self {
            hologram: Hologram::new(256, 128),
            world: World::new(),
            reconstruction_angle_x: 0,
            reconstruction_angle_y: 0,
            status_msg: "Use Arrow Keys to adjust Angle. D to drop Firewall.".into(),
            reconstruction_data: vec![],
        };
        app.rebuild_hologram();
        app
    }

    fn rebuild_hologram(&mut self) {
        let width = self.hologram.width;
        let height = self.hologram.height;
        let mut grid = vec![0.0; width * height];

        for p in &self.world.agents {
            let x = (p.pos.x as usize).clamp(0, width - 1);
            let y = (p.pos.y as usize).clamp(0, height - 1);
            grid[y * width + x] += 0.5;
        }

        self.hologram.compute_hologram(&grid);
        self.reconstruction_data = self.hologram.reconstruct(
            self.reconstruction_angle_x,
            self.reconstruction_angle_y,
        );
    }

    fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()>
    where
        <B as Backend>::Error: Send + Sync + 'static,
    {
        loop {
            self.world.update();
            self.rebuild_hologram();

            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Left => self.reconstruction_angle_x -= 1,
                            KeyCode::Right => self.reconstruction_angle_x += 1,
                            KeyCode::Up => self.reconstruction_angle_y -= 1,
                            KeyCode::Down => self.reconstruction_angle_y += 1,
                            KeyCode::Char('d') => {
                                let mut rng = rand::thread_rng();
                                use rand::Rng;
                                let fw_x = rng.gen_range(50.0..200.0);
                                let fw_y = rng.gen_range(30.0..100.0);
                                self.world.add_firewall(Vec2::new(fw_x, fw_y), 20.0);
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn ui(&self, f: &mut ratatui::Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
            .split(f.area());

        let info = format!(
            "{} | Server Health: {:.0}/{:.0} | Angle: ({}, {})",
            self.status_msg, self.world.server_health, self.world.max_health, self.reconstruction_angle_x, self.reconstruction_angle_y
        );
        let header = Paragraph::new(info).block(
            Block::default()
                .borders(Borders::ALL)
                .title("Holographic Cyberwarfare"),
        );
        f.render_widget(header, chunks[0]);

        let width = self.hologram.width;
        let height = self.hologram.height;
        let r_data = &self.reconstruction_data;

        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL))
            .x_bounds([0.0, width as f64])
            .y_bounds([0.0, height as f64])
            .paint(|ctx| {
                let mut points = vec![];

                for (i, &val) in r_data.iter().enumerate() {
                    if val > 0.3 {
                        let x = (i % width) as f64;
                        let y = (height - 1 - i / width) as f64; // flip y
                        points.push((x, y));
                    }
                }

                ctx.draw(&Points {
                    coords: &points,
                    color: Color::Cyan,
                });

                for fw in &self.world.firewalls {
                    let mut fw_pts = vec![];
                    for i in 0..100 {
                        let angle = i as f64 * std::f64::consts::PI * 2.0 / 100.0;
                        let px = fw.0.x as f64 + fw.1 as f64 * angle.cos();
                        let py = (height as f64 - 1.0) - (fw.0.y as f64 + fw.1 as f64 * angle.sin());
                        fw_pts.push((px, py));
                    }
                    ctx.draw(&Points {
                        coords: &fw_pts,
                        color: Color::Red,
                    });
                }
            });

        f.render_widget(canvas, chunks[1]);
    }
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = app.run(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

use crate::grid::{Grid, HiddenLayer};
use crate::agent::Agent;
use crate::stego::generate_pattern;
use glam::Vec2;
use ratatui::widgets::canvas::{Canvas, Points};
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;

pub struct App {
    pub grid: Grid,
    pub hidden: HiddenLayer,
    pub agents: Vec<Agent>,
    pub running: bool,
    pub iter: usize,
    pub width: usize,
    pub height: usize,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = Grid::new(width, height);
        let hidden = generate_pattern(width, height);
        let mut agents = Vec::new();

        let mut rng = rand::thread_rng();
        use rand::Rng;

        // Spawn agents randomly
        for _ in 0..2000 {
            let x = rng.gen_range(0.0..width as f32);
            let y = rng.gen_range(0.0..height as f32);
            let angle = rng.gen_range(0.0..std::f32::consts::TAU);
            agents.push(Agent::new(Vec2::new(x, y), angle));
        }

        Self {
            grid,
            hidden,
            agents,
            running: true,
            iter: 0,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        if !self.running { return; }

        for agent in &mut self.agents {
            agent.update(&mut self.grid, &self.hidden);
        }

        self.grid.diffuse_and_decay(0.92); // Decay slightly slower to keep trails visible
        self.iter += 1;
    }

    pub fn render(&self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(frame.area());

        let canvas_area = chunks[0];
        let status_area = chunks[1];

        // Draw Simulation on Canvas
        let canvas = Canvas::default()
            .block(Block::default().borders(Borders::ALL).title(" Bio-Decryption: Physarum Steganography "))
            .x_bounds([0.0, self.width as f64])
            .y_bounds([0.0, self.height as f64])
            .paint(|ctx| {
                // Collect points by intensity
                let mut high_intensity = Vec::new();
                let mut med_intensity = Vec::new();
                let mut low_intensity = Vec::new();

                for y in 0..self.height {
                    for x in 0..self.width {
                        let val = self.grid.get(x, y);
                        if val > 0.8 {
                            high_intensity.push((x as f64, self.height as f64 - y as f64));
                        } else if val > 0.4 {
                            med_intensity.push((x as f64, self.height as f64 - y as f64));
                        } else if val > 0.1 {
                            low_intensity.push((x as f64, self.height as f64 - y as f64));
                        }
                    }
                }

                ctx.draw(&Points {
                    coords: &low_intensity,
                    color: Color::DarkGray,
                });
                ctx.draw(&Points {
                    coords: &med_intensity,
                    color: Color::Blue,
                });
                ctx.draw(&Points {
                    coords: &high_intensity,
                    color: Color::Cyan,
                });

                // Draw Agents as tiny white dots (optional, might be too cluttered)
                // for agent in &self.agents {
                //    ctx.draw(&Points {
                //        coords: &[(agent.pos.x as f64, self.height as f64 - agent.pos.y as f64)],
                //        color: Color::White,
                //    });
                // }
            });

        frame.render_widget(canvas, canvas_area);

        // Status Bar
        let status_text = vec![
            Span::raw("Iter: "),
            Span::styled(format!("{}", self.iter), Style::default().fg(Color::Yellow)),
            Span::raw(" | Agents: "),
            Span::styled(format!("{}", self.agents.len()), Style::default().fg(Color::Cyan)),
            Span::raw(" | Press "),
            Span::styled("q", Style::default().fg(Color::Red)),
            Span::raw(" to quit"),
        ];

        frame.render_widget(
            Paragraph::new(Line::from(status_text)).style(Style::default().bg(Color::Black)),
            status_area,
        );
    }
}

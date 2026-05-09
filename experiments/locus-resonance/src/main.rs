use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use locus::Topology;
use tui_shared::ratatui::style::{Color, Style};
use tui_shared::ratatui::text::{Line, Span};
use tui_shared::ratatui::widgets::{Block, Borders, Paragraph};
use tui_shared::ratatui::Frame;
use std::time::{Duration, Instant};
use tui_shared::Tui;

/// A custom physics grid that uses `locus::Topology` for wave propagation wrapping.
pub struct TopologyPhysicsGrid {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f32>,
    pub u_prev: Vec<f32>,
    pub u_next: Vec<f32>,
    pub energy_map: Vec<f32>,
    pub topology: Topology,
}

impl TopologyPhysicsGrid {
    pub fn new(width: usize, height: usize, topology: Topology) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![0.0; size],
            u_prev: vec![0.0; size],
            u_next: vec![0.0; size],
            energy_map: vec![0.0; size],
            topology,
        }
    }

    pub fn pluck(&mut self, x: usize, y: usize, strength: f32) {
        if x < self.width && y < self.height {
            self.u[y * self.width + x] += strength;
        }
    }

    pub fn get_idx(&self, x: i64, y: i64) -> Option<usize> {
        if let Some((ny, nx)) = self.topology.normalize(y, x, self.width, self.height) {
            Some(ny * self.width + nx)
        } else {
            None
        }
    }

    pub fn step(&mut self) {
        let w = self.width as i64;
        let h = self.height as i64;

        if w < 3 || h < 3 {
            return;
        }

        let c2 = 0.4;
        let damping = 0.999;

        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) as usize;

                let u_curr = self.u[idx];
                let u_prev = self.u_prev[idx];

                let up_idx = self.get_idx(x, y - 1);
                let down_idx = self.get_idx(x, y + 1);
                let left_idx = self.get_idx(x - 1, y);
                let right_idx = self.get_idx(x + 1, y);

                let u_up = up_idx.map(|i| self.u[i]).unwrap_or(0.0);
                let u_down = down_idx.map(|i| self.u[i]).unwrap_or(0.0);
                let u_left = left_idx.map(|i| self.u[i]).unwrap_or(0.0);
                let u_right = right_idx.map(|i| self.u[i]).unwrap_or(0.0);

                let laplacian = u_up + u_down + u_left + u_right - 4.0 * u_curr;

                let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
                val *= damping;

                self.u_next[idx] = val;
                self.energy_map[idx] = self.energy_map[idx] * 0.999 + val.abs() * 0.01;
            }
        }

        std::mem::swap(&mut self.u_prev, &mut self.u);
        std::mem::swap(&mut self.u, &mut self.u_next);
    }
}

fn ui(f: &mut Frame, grid: &TopologyPhysicsGrid) {
    let block = Block::default()
        .title(format!(
            " Topological Acoustic Morphogenesis | Topology: {:?} | Pluck: Space | Switch: Tab | Quit: q ",
            grid.topology
        ))
        .borders(Borders::ALL);

    let inner_area = block.inner(f.area());
    f.render_widget(block, f.area());

    if inner_area.width == 0 || inner_area.height == 0 {
        return;
    }

    let mut lines = Vec::with_capacity(inner_area.height as usize);
    let render_w = (inner_area.width as usize).min(grid.width);
    let render_h = (inner_area.height as usize).min(grid.height);

    for y in 0..render_h {
        let mut spans = Vec::with_capacity(render_w);
        for x in 0..render_w {
            let energy = grid.energy_map[y * grid.width + x];
            let intensity = (energy * 10.0).clamp(0.0, 1.0);

            let ch = if intensity > 0.8 {
                '#'
            } else if intensity > 0.5 {
                '*'
            } else if intensity > 0.2 {
                '+'
            } else if intensity > 0.05 {
                '.'
            } else {
                ' '
            };

            let color = if intensity > 0.8 {
                Color::LightRed
            } else if intensity > 0.5 {
                Color::LightYellow
            } else if intensity > 0.2 {
                Color::LightCyan
            } else {
                Color::DarkGray
            };

            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines);
    f.render_widget(paragraph, inner_area);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let headless = args.iter().any(|arg| arg == "--headless");

    let mut tui = if headless { None } else { Some(Tui::init()?) };

    let topologies = vec![
        Topology::Torus,
        Topology::Klein,
        Topology::Mobius,
        Topology::CylinderH,
        Topology::Plane,
    ];
    let mut topo_idx = 0;

    let width = 120;
    let height = 40;
    let mut grid = TopologyPhysicsGrid::new(width, height, topologies[topo_idx]);

    // Initial pluck
    grid.pluck(width / 2, height / 2, 50.0);

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    let mut frame_count = 0;

    loop {
        if headless {
            grid.step();
            frame_count += 1;
            if frame_count > 50 {
                println!("🧬 locus-resonance running in headless mode for 50 ticks.");
                break;
            }
            continue;
        }

        if let Some(tui) = &mut tui {
            tui.terminal.draw(|f| ui(f, &grid))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char(' ') => {
                                // Pluck center
                                grid.pluck(width / 2, height / 2, 100.0);
                            }
                            KeyCode::Tab => {
                                topo_idx = (topo_idx + 1) % topologies.len();
                                grid.topology = topologies[topo_idx];
                                grid.u.fill(0.0);
                                grid.u_prev.fill(0.0);
                                grid.u_next.fill(0.0);
                                grid.energy_map.fill(0.0);
                                grid.pluck(width / 2, height / 2, 50.0);
                            }
                            _ => {}
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                grid.step();
                last_tick = Instant::now();
            }
        }
    }

    Ok(())
}

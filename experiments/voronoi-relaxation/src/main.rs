use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rand::Rng;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

struct Site {
    position: Point,
    color: Color,
    // Accumulators for Lloyd's algorithm
    sum_x: f64,
    sum_y: f64,
    count: usize,
}

impl Site {
    fn new(x: f64, y: f64, color: Color) -> Self {
        Self {
            position: Point { x, y },
            color,
            sum_x: 0.0,
            sum_y: 0.0,
            count: 0,
        }
    }

    fn reset_accumulators(&mut self) {
        self.sum_x = 0.0;
        self.sum_y = 0.0;
        self.count = 0;
    }
}

struct Voronoi {
    sites: Vec<Site>,
    width: f64,
    height: f64,
    relaxing: bool,
}

impl Voronoi {
    fn new(count: usize, width: f64, height: f64) -> Self {
        let mut rng = rand::rngs::ThreadRng::default();
        let mut sites = Vec::with_capacity(count);
        let colors = [
            Color::Red,
            Color::Green,
            Color::Blue,
            Color::Yellow,
            Color::Magenta,
            Color::Cyan,
            Color::White,
            Color::LightRed,
            Color::LightGreen,
            Color::LightBlue,
        ];

        for _ in 0..count {
            sites.push(Site::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
                colors[rng.gen_range(0..colors.len())],
            ));
        }

        Self {
            sites,
            width,
            height,
            relaxing: false,
        }
    }

    fn reset(&mut self) {
        let mut rng = rand::rngs::ThreadRng::default();
        for site in &mut self.sites {
            site.position.x = rng.gen_range(0.0..self.width);
            site.position.y = rng.gen_range(0.0..self.height);
            site.reset_accumulators();
        }
    }

    // Lloyd's Algorithm Step
    fn relax(&mut self) {
        // Reset accumulators
        for site in &mut self.sites {
            site.reset_accumulators();
        }

        // Sampling resolution (trade-off between speed and accuracy)
        // For a TUI, checking every cell is feasible.
        // Canvas coordinate system is usually grid based.
        // We will sample continuously or use a grid approximation.
        // Let's use a fixed grid for integration.
        let grid_w = self.width as usize;
        let grid_h = self.height as usize;

        for y in 0..grid_h {
            for x in 0..grid_w {
                let px = x as f64 + 0.5;
                let py = y as f64 + 0.5;

                let mut min_dist_sq = f64::MAX;
                let mut nearest_idx = 0;

                for (i, site) in self.sites.iter().enumerate() {
                    let dx = px - site.position.x;
                    // Terminal cells are roughly 1:2 aspect ratio.
                    // To make "circular" cells visually, we treat Y distance as double.
                    let dy = (py - site.position.y) * 2.0;
                    let dist_sq = dx * dx + dy * dy;

                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest_idx = i;
                    }
                }

                // Add to centroid accumulator
                self.sites[nearest_idx].sum_x += px;
                self.sites[nearest_idx].sum_y += py;
                self.sites[nearest_idx].count += 1;
            }
        }

        // Move sites to centroids
        for site in &mut self.sites {
            if site.count > 0 {
                site.position.x = site.sum_x / site.count as f64;
                site.position.y = site.sum_y / site.count as f64;
            }
        }
    }
}

struct App {
    voronoi: Voronoi,
    should_quit: bool,
}

impl App {
    fn new() -> Self {
        Self {
            voronoi: Voronoi::new(20, 100.0, 100.0), // Initial size, will update on resize
            should_quit: false,
        }
    }

    fn on_tick(&mut self) {
        if self.voronoi.relaxing {
            self.voronoi.relax();
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let tick_rate = Duration::from_millis(50); // 20 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.should_quit = true,
                        KeyCode::Char(' ') => app.voronoi.relaxing = !app.voronoi.relaxing,
                        KeyCode::Char('r') => app.voronoi.reset(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }

        if app.should_quit {
            break;
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update Voronoi dimensions to match current view
    let canvas_area = chunks[0];
    if canvas_area.width as f64 != app.voronoi.width || canvas_area.height as f64 != app.voronoi.height {
         // If resized, we might want to scale positions or just clamp/wrap?
         // For simplicity, let's just update bounds and let points float.
         // Or better, re-initialize if drastically different?
         // Let's just update bounds. Points outside will be pulled back by Lloyd's eventually if we clamp sampling.
         app.voronoi.width = canvas_area.width as f64;
         app.voronoi.height = canvas_area.height as f64;
    }

    let voronoi_widget = VoronoiWidget { voronoi: &app.voronoi };
    f.render_widget(voronoi_widget, chunks[0]);


    let status_text = vec![
        Line::from(vec![
            " [Space] ".yellow().bold(),
            "Toggle Relaxation ".into(),
            " [R] ".yellow().bold(),
            "Reset ".into(),
            " [Q] ".yellow().bold(),
            "Quit ".into(),
            format!(" | Relaxing: {}", app.voronoi.relaxing).into(),
        ]),
    ];

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}

struct VoronoiWidget<'a> {
    voronoi: &'a Voronoi,
}

impl<'a> Widget for VoronoiWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        // Iterate over every cell in the area
        for y in 0..area.height {
            for x in 0..area.width {
                let px = x as f64;
                let py = y as f64;

                // Map to Voronoi space (if mismatched, scaling needed, but we synced them in ui)
                // We assume voronoi.width/height matches area.

                let mut min_dist_sq = f64::MAX;
                let mut nearest_color = Color::Reset;

                for site in &self.voronoi.sites {
                    let dx = px - site.position.x;
                    // Visual aspect ratio correction for distance
                    let dy = (py - site.position.y) * 2.0;
                    let dist_sq = dx*dx + dy*dy;

                    if dist_sq < min_dist_sq {
                        min_dist_sq = dist_sq;
                        nearest_color = site.color;
                    }
                }

                if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                    cell.set_bg(nearest_color);
                    // Add a character for texture?
                    // cell.set_char(' ');
                }
            }
        }

        // Draw seeds on top
        for site in &self.voronoi.sites {
            let sx = site.position.x.round() as u16;
            let sy = site.position.y.round() as u16;
            if sx < area.width && sy < area.height {
                if let Some(cell) = buf.cell_mut((area.x + sx, area.y + sy)) {
                    cell.set_fg(Color::Black);
                    cell.set_char('●');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        let dx = p1.x - p2.x;
        let dy = p1.y - p2.y;
        assert_eq!(dx*dx + dy*dy, 25.0);
    }

    #[test]
    fn test_relax_moves_points() {
        let mut v = Voronoi::new(2, 10.0, 10.0);
        // Force positions
        v.sites[0].position = Point { x: 1.0, y: 1.0 }; // Corner
        v.sites[1].position = Point { x: 9.0, y: 9.0 }; // Corner

        let old_pos = v.sites[0].position;
        v.relax(); // Should move towards center

        assert!(v.sites[0].position.x > old_pos.x);
        assert!(v.sites[0].position.y > old_pos.y);
    }
}

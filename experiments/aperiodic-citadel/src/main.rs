mod penrose;
mod tui;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
use tui_shared::Tui;
use penrose::{PenroseTiling, Point};
use tui::ui;

struct App {
    tiling: PenroseTiling,
    player_idx: usize,
    camera_offset: Point,
    zoom: f64,
    exit: bool,
}

impl App {
    fn new() -> Self {
        let mut tiling = PenroseTiling::generate_sun(300.0);
        // Subdivide a few times
        // 4 iterations -> 10 * 16 = 160 tiles. Fast.
        // 5 iterations -> 320.
        // 6 -> 640.
        for _ in 0..5 {
            tiling.subdivide();
        }
        tiling.build_adjacency();

        Self {
            tiling,
            player_idx: 0,
            camera_offset: Point::new(0.0, 0.0),
            zoom: 1.0,
            exit: false,
        }
    }

    fn run(&mut self, tui: &mut Tui) -> Result<()> {
        while !self.exit {
            tui.terminal.draw(|f| {
                 ui(f, &self.tiling, self.player_idx, self.camera_offset, self.zoom);
            })?;

            if event::poll(Duration::from_millis(16))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
                        KeyCode::Char('w') | KeyCode::Up => self.move_player(Point::new(0.0, 1.0)),
                        KeyCode::Char('s') | KeyCode::Down => self.move_player(Point::new(0.0, -1.0)),
                        KeyCode::Char('a') | KeyCode::Left => self.move_player(Point::new(-1.0, 0.0)),
                        KeyCode::Char('d') | KeyCode::Right => self.move_player(Point::new(1.0, 0.0)),
                        KeyCode::Char('+') | KeyCode::Char('=') => self.zoom *= 1.1,
                        KeyCode::Char('-') => self.zoom /= 1.1,
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn move_player(&mut self, dir: Point) {
        if let Some(next) = self.tiling.get_closest_neighbor(self.player_idx, dir) {
            self.player_idx = next;
            self.camera_offset = self.tiling.triangles[self.player_idx].center();
        }
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = app.run(&mut tui);
    tui.exit()?;
    res
}

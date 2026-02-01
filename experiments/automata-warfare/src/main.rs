use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{backend::Backend, layout::Rect, style::Color, widgets::Widget, Terminal};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

use automata_warfare::automata::{Cell, Grid};

fn main() -> Result<()> {
    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal);

    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut grid = Grid::new(100, 100); // Initial size, will resize
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(50); // 20 FPS
    let mut paused = false;

    loop {
        terminal.draw(|f| {
            let size = f.area();

            // Resize grid if terminal size changed significantly
            // Note: Resizing resets the grid in this simple implementation
            // Vertical resolution is 2x terminal height because we use half-blocks
            let target_width = size.width as usize;
            let target_height = (size.height * 2) as usize;

            if grid.width != target_width || grid.height != target_height {
                grid = Grid::new(target_width, target_height);
            }

            let widget = GridWidget { grid: &grid };
            f.render_widget(widget, size);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => paused = !paused,
                    KeyCode::Char('r') => {
                        grid = Grid::new(grid.width, grid.height);
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !paused {
                grid.update();
            }
            last_tick = Instant::now();
        }
    }
}

struct GridWidget<'a> {
    grid: &'a Grid,
}

impl<'a> Widget for GridWidget<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::buffer::Buffer) {
        if area.area() == 0 {
            return;
        }

        // We render 2 grid cells per terminal cell (vertically)
        // Top half block: ▀
        // Foreground color is top cell, Background color is bottom cell

        for y in 0..area.height {
            for x in 0..area.width {
                let grid_x = x as usize;
                let grid_y_top = (y * 2) as usize;
                let grid_y_bottom = (y * 2 + 1) as usize;

                if grid_x >= self.grid.width || grid_y_bottom >= self.grid.height {
                    continue;
                }

                let cell_top = self.grid.cells[self.grid.get_index(grid_x, grid_y_top)];
                let cell_bottom = self.grid.cells[self.grid.get_index(grid_x, grid_y_bottom)];

                let color_top = get_color(cell_top);
                let color_bottom = get_color(cell_bottom);

                let symbol = "▀";

                // Set the cell
                // buf.get_mut is deprecated, use buf.cell_mut
                if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                    cell.set_symbol(symbol)
                        .set_fg(color_top)
                        .set_bg(color_bottom);
                }
            }
        }
    }
}

fn get_color(cell: Cell) -> Color {
    match cell {
        Cell::Empty => Color::Black,
        Cell::Rock => Color::Red,      // Red
        Cell::Paper => Color::White,   // White
        Cell::Scissors => Color::Gray, // Silver/Gray
        Cell::Lizard => Color::Green,  // Green
        Cell::Spock => Color::Blue,    // Blue
    }
}

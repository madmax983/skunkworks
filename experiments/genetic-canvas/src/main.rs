use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::{
        canvas::{Canvas, Line as CanvasLine, Rectangle},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::collections::HashSet;

pub mod genome;
pub mod population;

use genome::ShapeType;
use population::Population;
use tui_shared::Tui;

struct App {
    population: Population,
    selected: HashSet<usize>,
    hovered: Option<usize>,
    grid_size: usize, // e.g., 3 for 3x3
}

impl App {
    fn new() -> Self {
        let grid_size = 3;
        Self {
            population: Population::new(grid_size * grid_size),
            selected: HashSet::new(),
            hovered: None,
            grid_size,
        }
    }

    fn toggle_selection(&mut self, idx: usize) {
        if self.selected.contains(&idx) {
            self.selected.remove(&idx);
        } else {
            self.selected.insert(idx);
        }
    }

    fn evolve(&mut self) {
        if self.selected.is_empty() {
            return;
        }
        let indices: Vec<usize> = self.selected.iter().cloned().collect();
        self.population.evolve(&indices);
        self.selected.clear();
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == crossterm::event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => break,
                            KeyCode::Char(' ') => app.evolve(),
                            KeyCode::Char('r') => app.population = Population::new(9),
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => match mouse.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        let width = tui.terminal.size()?.width;
                        let height = tui.terminal.size()?.height;
                        if let Some(idx) =
                            hit_test(mouse.column, mouse.row, width, height, app.grid_size)
                        {
                            app.toggle_selection(idx);
                        }
                    }
                    MouseEventKind::Moved => {
                        let width = tui.terminal.size()?.width;
                        let height = tui.terminal.size()?.height;
                        app.hovered =
                            hit_test(mouse.column, mouse.row, width, height, app.grid_size);
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }

    Ok(())
}

fn hit_test(x: u16, y: u16, width: u16, height: u16, grid_size: usize) -> Option<usize> {
    // Reconstruct layout logic to map coords to index
    // This is a bit fragile but standard TUI practice without event bubbling

    // Main layout
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(Rect::new(0, 0, width, height));

    let grid_area = main_layout[0];

    if x < grid_area.x
        || x >= grid_area.x + grid_area.width
        || y < grid_area.y
        || y >= grid_area.y + grid_area.height
    {
        return None;
    }

    // Grid layout
    // We split vertically then horizontally
    let row_height = grid_area.height / grid_size as u16;
    let col_width = grid_area.width / grid_size as u16;

    if row_height == 0 || col_width == 0 {
        return None;
    }

    let row = (y - grid_area.y) / row_height;
    let col = (x - grid_area.x) / col_width;

    if row < grid_size as u16 && col < grid_size as u16 {
        Some((row as usize) * grid_size + (col as usize))
    } else {
        None
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Render Grid
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints((0..app.grid_size).map(|_| Constraint::Ratio(1, app.grid_size as u32)))
        .split(chunks[0]);

    for r in 0..app.grid_size {
        let cols = Layout::default()
            .direction(Direction::Horizontal)
            .constraints((0..app.grid_size).map(|_| Constraint::Ratio(1, app.grid_size as u32)))
            .split(rows[r]);

        for c in 0..app.grid_size {
            let idx = r * app.grid_size + c;
            if idx < app.population.genomes.len() {
                let genome = &app.population.genomes[idx];

                let is_selected = app.selected.contains(&idx);
                let is_hovered = app.hovered == Some(idx);

                let border_style = if is_selected {
                    Style::default().fg(Color::Green).bg(Color::DarkGray)
                } else if is_hovered {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default().fg(Color::Gray)
                };

                let block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(border_style)
                    .title(if is_selected { " SELECTED " } else { "" });

                let canvas = Canvas::default()
                    .block(block)
                    .x_bounds([0.0, 1.0])
                    .y_bounds([0.0, 1.0])
                    .paint(|ctx| {
                        for shape in &genome.shapes {
                            match shape.shape_type {
                                ShapeType::Rect => {
                                    ctx.draw(&Rectangle {
                                        x: shape.x,
                                        y: shape.y,
                                        width: shape.w,
                                        height: shape.h,
                                        color: shape.color,
                                    });
                                }
                                ShapeType::Line => {
                                    ctx.draw(&CanvasLine {
                                        x1: shape.x,
                                        y1: shape.y,
                                        x2: shape.x + shape.w - 0.5,
                                        y2: shape.y + shape.h - 0.5,
                                        color: shape.color,
                                    });
                                }
                                ShapeType::Circle => {
                                    ctx.draw(&ratatui::widgets::canvas::Circle {
                                        x: shape.x,
                                        y: shape.y,
                                        radius: shape.w / 2.0,
                                        color: shape.color,
                                    });
                                }
                            }
                        }
                    });

                f.render_widget(canvas, cols[c]);
            }
        }
    }

    // Status Bar
    let status_text = format!(
        " Generation: {} | Selected: {} | [Click] Select | [Space] Evolve | [R]eset | [Q]uit ",
        app.population.generation,
        app.selected.len()
    );
    let status = Paragraph::new(Line::from(status_text)).block(
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Blue)),
    );

    f.render_widget(status, chunks[1]);
}

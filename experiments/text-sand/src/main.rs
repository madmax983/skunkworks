mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use physics::{Particle, ParticleKind, World};
use ratatui::{
    backend::Backend,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal);
    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }
    Ok(())
}

struct App {
    world: World,
    selected_kind: ParticleKind,
    mouse_x: u16,
    mouse_y: u16,
    is_drawing: bool,
    paused: bool,
    brush_size: usize,
}

impl App {
    fn new() -> Self {
        Self {
            world: World::new(10, 10), // Initial small size, auto-resizes
            selected_kind: ParticleKind::Sand,
            mouse_x: 0,
            mouse_y: 0,
            is_drawing: false,
            paused: false,
            brush_size: 1,
        }
    }

    fn spawn_at_mouse(&mut self, char: Option<char>) {
        // Map screen coordinates to grid coordinates.
        // We assume the grid starts at (1, 1) due to Block borders.
        // We need to sync this with render logic.
        // Let's assume the render area is known or we calculate it.
        // For simplicity, we'll ask the `WorldWidget` to handle offset,
        // but here we are in App.

        // Let's assume the block is at 0,0 and has borders.
        // So grid (0,0) is at screen (1,1).

        let grid_x = self.mouse_x.saturating_sub(1) as usize;
        let grid_y = self.mouse_y.saturating_sub(1) as usize;

        let kind = if char.is_some() {
            ParticleKind::Text
        } else {
            self.selected_kind
        };

        let p_char = char.unwrap_or(match kind {
            ParticleKind::Sand => 's',
            ParticleKind::Water => '~',
            ParticleKind::Wall => '#',
            ParticleKind::Empty => ' ',
            ParticleKind::Text => '?', // Should not happen
        });

        let color = match kind {
            ParticleKind::Sand => Color::Yellow,
            ParticleKind::Water => Color::Blue,
            ParticleKind::Wall => Color::Gray,
            ParticleKind::Empty => Color::Reset,
            ParticleKind::Text => Color::Green, // Text is Green (Matrix style)
        };

        // Brush size
        let r = self.brush_size as isize;
        for dy in 0..r {
            for dx in 0..r {
                // simple square brush
                self.world.set(
                    grid_x + dx as usize,
                    grid_y + dy as usize,
                    Particle::new(kind, p_char, color)
                );
            }
        }
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(33); // ~30 FPS

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let main_area = chunks[0];
            let status_area = chunks[1];

            // Resize world to fit the block (minus borders)
            let grid_width = (main_area.width.saturating_sub(2)) as usize;
            let grid_height = (main_area.height.saturating_sub(2)) as usize;

            app.world.resize(grid_width, grid_height);

            let block = Block::default()
                .borders(Borders::ALL)
                .title(" Text Sand - Creative Mode ")
                .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));

            f.render_widget(
                WorldWidget {
                    world: &app.world,
                },
                block.inner(main_area),
            );
            f.render_widget(block, main_area);

            let tool_name = match app.selected_kind {
                ParticleKind::Sand => "Sand (s)",
                ParticleKind::Water => "Water (w)",
                ParticleKind::Wall => "Wall (#)",
                ParticleKind::Empty => "Eraser (x)",
                ParticleKind::Text => "Text",
            };

            let status = Line::from(vec![
                Span::raw("Tool: "),
                Span::styled(tool_name, Style::default().fg(Color::Yellow)),
                Span::raw(" | [Space] Pause | [C]lear | Type to spawn text | Click to draw"),
            ]);
            f.render_widget(Paragraph::new(status).style(Style::default().bg(Color::DarkGray)), status_area);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                            KeyCode::Char(' ') => app.paused = !app.paused,
                            KeyCode::Char('c') => {
                                let w = app.world.width;
                                let h = app.world.height;
                                app.world = World::new(w, h);
                            },
                            KeyCode::Char('s') => app.selected_kind = ParticleKind::Sand,
                            KeyCode::Char('w') => app.selected_kind = ParticleKind::Water,
                            KeyCode::Char('#') => app.selected_kind = ParticleKind::Wall,
                            KeyCode::Char('x') => app.selected_kind = ParticleKind::Empty,
                            KeyCode::Char(c) => {
                                // Spawn text particle
                                app.spawn_at_mouse(Some(c));
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    app.mouse_x = mouse.column;
                    app.mouse_y = mouse.row;

                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.is_drawing = true;
                            app.spawn_at_mouse(None);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.is_drawing = false;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                             app.is_drawing = true; // Ensure drawing state
                             app.spawn_at_mouse(None);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            if !app.paused {
                app.world.update();
            }
            last_tick = Instant::now();
        }
    }
}

struct WorldWidget<'a> {
    world: &'a World,
}

impl<'a> Widget for WorldWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        for y in 0..area.height {
            for x in 0..area.width {
                let p = self.world.get(x as usize, y as usize);
                if p.kind != ParticleKind::Empty {
                     if let Some(cell) = buf.cell_mut((area.x + x, area.y + y)) {
                         cell.set_char(p.char).set_fg(p.color);
                     }
                }
            }
        }
    }
}

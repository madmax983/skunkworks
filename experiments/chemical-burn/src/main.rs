pub mod model;

use crossterm::event::{self, Event, KeyCode, MouseButton, MouseEventKind};
use model::GrayScott;
use ratatui::buffer::Buffer;
use ratatui::style::Color;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

fn main() -> anyhow::Result<()> {
    let mut tui = Tui::init()?;
    let mut app = App::new();
    let res = run_app(&mut tui.terminal, &mut app);
    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
        return Err(err.into());
    }
    tui.exit()?;
    Ok(())
}

struct App {
    model: GrayScott,
    running: bool,
    mouse_pressed: bool,
    preset_index: usize,
}

struct Preset {
    name: &'static str,
    feed: f64,
    kill: f64,
}

const PRESETS: &[Preset] = &[
    Preset {
        name: "Mitosis",
        feed: 0.0367,
        kill: 0.06418,
    },
    Preset {
        name: "Coral",
        feed: 0.0545,
        kill: 0.062,
    },
    Preset {
        name: "Maze",
        feed: 0.029,
        kill: 0.057,
    },
    Preset {
        name: "U-Skate",
        feed: 0.062,
        kill: 0.0609,
    },
    Preset {
        name: "Chaos",
        feed: 0.026,
        kill: 0.051,
    },
];

impl App {
    fn new() -> Self {
        Self {
            model: GrayScott::new(10, 10), // Will resize on first frame
            running: true,
            mouse_pressed: false,
            preset_index: 1, // Start with Coral
        }
    }

    fn update(&mut self, dt: f64) {
        // Apply preset
        self.model.feed = PRESETS[self.preset_index].feed;
        self.model.kill = PRESETS[self.preset_index].kill;

        // Multi-step for speed
        for _ in 0..10 {
            self.model.update(dt);
        }
    }

    fn toggle_preset(&mut self) {
        self.preset_index = (self.preset_index + 1) % PRESETS.len();
    }
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => app.running = false,
                            KeyCode::Char('r') => {
                                let w = app.model.width;
                                let h = app.model.height;
                                app.model = GrayScott::new(w, h);
                            }
                            KeyCode::Char('p') | KeyCode::Tab => {
                                app.toggle_preset();
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    let x = mouse.column;
                    let y = mouse.row;

                    // Mouse interaction handling relative to widget area would be better,
                    // but assuming fullscreen block for simplicity or strict layout.
                    // We'll calculate offset in ui function, but here we just store raw pos.
                    // Actually, we need to map to model coords here to inject.

                    // Let's assume the widget is at (1, 1) due to borders.
                    // And model size matches (width-2, height-2).

                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.mouse_pressed = true;
                            inject_chemical(app, x, y);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.mouse_pressed = false;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            if app.mouse_pressed {
                                inject_chemical(app, x, y);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(1.0);
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}

fn inject_chemical(app: &mut App, x: u16, y: u16) {
    // Map screen x,y to model x,y.
    // Assuming simple mapping where model (0,0) is screen (1,1).
    if x > 0 && y > 0 {
        let mx = (x - 1) as usize;
        let my = (y - 1) as usize;
        if mx < app.model.width && my < app.model.height {
            app.model.add_chemical(mx, my, 3);
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let sim_area = chunks[0];
    let status_area = chunks[1];

    // Render Simulation
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Chemical Burn");
    let inner_area = block.inner(sim_area);

    // Resize model if needed
    if app.model.width != inner_area.width as usize
        || app.model.height != inner_area.height as usize
    {
        app.model
            .resize(inner_area.width as usize, inner_area.height as usize);
    }

    f.render_widget(block, sim_area);
    f.render_widget(ChemicalWidget { model: &app.model }, inner_area);

    // Render Status
    let preset = &PRESETS[app.preset_index];
    let status = format!(
        "Preset: {} (f={:.4}, k={:.4}) | 'p': Next Preset | 'r': Reset | 'q': Quit | Click to Add",
        preset.name, preset.feed, preset.kill
    );
    f.render_widget(Paragraph::new(status), status_area);
}

struct ChemicalWidget<'a> {
    model: &'a GrayScott,
}

impl<'a> Widget for ChemicalWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chars = b" .:-=+*#%@";

        for y in 0..area.height {
            for x in 0..area.width {
                let mx = x as usize;
                let my = y as usize;

                if mx < self.model.width && my < self.model.height {
                    let idx = my * self.model.width + mx;
                    let v = self.model.v[idx];

                    // Map v (0..1) to char index
                    let char_idx = (v * 9.0).clamp(0.0, 9.0) as usize;
                    let c = chars[char_idx] as char;

                    let color = if v > 0.5 {
                        Color::Cyan
                    } else if v > 0.2 {
                        Color::Blue
                    } else {
                        Color::DarkGray
                    };

                    if let Some(cell) = buf.cell_mut((area.left() + x, area.top() + y)) {
                        cell.set_char(c).set_fg(color);
                    }
                }
            }
        }
    }
}

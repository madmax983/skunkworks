use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, MouseEventKind};
use rand::Rng;
use ratatui::{prelude::*, widgets::*};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod lorenz;
mod reaction;

use lorenz::{integrate, LorenzParams, LorenzState};
use reaction::ChemicalSystem;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    let mut tui = Tui::init()?;

    let res = run_app(&mut tui.terminal);

    // Restore terminal
    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Initialize system
    let width = 120;
    let height = 60;
    let mut chem = ChemicalSystem::new(width, height);

    // Seed random spots
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let rx = rng.gen_range(5..width - 5);
        let ry = rng.gen_range(5..height - 5);
        for y in ry - 2..=ry + 2 {
            for x in rx - 2..=rx + 2 {
                chem.add_chemical(x, y, 0.9);
            }
        }
    }

    // Initialize Driver (Lorenz)
    let mut lorenz_state = LorenzState::new(0.1, 0.0, 0.0);
    let lorenz_params = LorenzParams {
        sigma: 10.0,
        rho: 28.0,
        beta: 8.0 / 3.0,
    };
    let lorenz_dt = 0.01;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        terminal
            .draw(|f| {
                ui(f, &chem, &lorenz_state);
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if let KeyCode::Char('q') = key.code {
                        return Ok(());
                    }
                }
                Event::Mouse(mouse) => {
                    if mouse.kind == MouseEventKind::Down(crossterm::event::MouseButton::Left) {
                        let mx = mouse.column as usize;
                        let my = mouse.row as usize;
                        // Map to grid coordinates
                        let grid_x = mx;
                        let grid_y = my * 2;
                        // Add a splash of V
                        for dy in 0..6 {
                            for dx in 0..6 {
                                chem.add_chemical(grid_x + dx, grid_y + dy, 0.8);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // 1. Update Lorenz Driver
            lorenz_state = integrate(&lorenz_state, &lorenz_params, lorenz_dt);

            // 2. Map Genes (Lorenz -> Chem)
            // f nominal ~0.055. x range ~ +/- 20.
            // k nominal ~0.062. z range ~ 0..50.

            // Damping the chaos slightly to keep it in the "alive" zone
            // f = 0.055 + (x / 100.0) -> +/- 0.02 variation
            chem.f = 0.055 + (lorenz_state.x / 400.0);
            // k = 0.062 + ((z - 25.0) / 1000.0)
            chem.k = 0.062 + ((lorenz_state.z - 25.0) / 1000.0);

            // Clamp to sane values to prevent explosion
            chem.f = chem.f.clamp(0.01, 0.1);
            chem.k = chem.k.clamp(0.03, 0.08);

            // 3. Update Chemical System
            // Multiple updates per frame for speed
            for _ in 0..8 {
                chem.update(1.0);
            }

            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, chem: &ChemicalSystem, lorenz: &LorenzState) {
    let area = f.area();

    // Split into main view and sidebar
    let main_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(30)])
        .split(area);

    let canvas_area = main_layout[0];
    let sidebar_area = main_layout[1];

    // Render Simulation
    f.render_widget(ChemWidget { system: chem }, canvas_area);

    // Render Sidebar Info
    let stats = vec![
        Line::from(Span::styled(
            "🧬 System Bio-Dome",
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(Color::Magenta),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "Driver (Lorenz):",
            Style::default().fg(Color::Cyan),
        )),
        Line::from(format!("X: {:.2}", lorenz.x)),
        Line::from(format!("Y: {:.2}", lorenz.y)),
        Line::from(format!("Z: {:.2}", lorenz.z)),
        Line::from(""),
        Line::from(Span::styled(
            "Expression (Gray-Scott):",
            Style::default().fg(Color::Green),
        )),
        Line::from(format!("Feed (f): {:.5}", chem.f)),
        Line::from(format!("Kill (k): {:.5}", chem.k)),
        Line::from(""),
        Line::from(Span::styled(
            "Controls:",
            Style::default().fg(Color::Yellow),
        )),
        Line::from("Click: Seeding"),
        Line::from("Q: Quit"),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title("Genetics")
        .border_style(Style::default().fg(Color::DarkGray));

    let p = Paragraph::new(stats).block(block);
    f.render_widget(p, sidebar_area);
}

struct ChemWidget<'a> {
    system: &'a ChemicalSystem,
}

impl<'a> Widget for ChemWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We use half-block characters to double vertical resolution.
        for x in 0..area.width {
            for y in 0..area.height {
                let grid_x = x as usize;
                let grid_y_top = (y * 2) as usize;
                let grid_y_bot = (y * 2 + 1) as usize;

                if grid_x >= self.system.width || grid_y_bot >= self.system.height {
                    continue;
                }

                let idx_top = self.system.get_index(grid_x, grid_y_top);
                let idx_bot = self.system.get_index(grid_x, grid_y_bot);

                let v_top = self.system.v[idx_top];
                let v_bot = self.system.v[idx_bot];

                let color_top = value_to_color(v_top);
                let color_bot = value_to_color(v_bot);

                buf[(area.x + x, area.y + y)]
                    .set_char('▀')
                    .set_fg(color_top)
                    .set_bg(color_bot);
            }
        }
    }
}

fn value_to_color(v: f64) -> Color {
    if v < 0.1 {
        Color::Reset
    } else if v < 0.2 {
        Color::DarkGray
    } else if v < 0.3 {
        Color::Blue
    } else if v < 0.4 {
        Color::Cyan
    } else if v < 0.5 {
        Color::Green
    } else if v < 0.6 {
        Color::Yellow
    } else if v < 0.8 {
        Color::Red
    } else {
        Color::Magenta
    }
}

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rand::Rng;
use ratatui::{prelude::*, widgets::*};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

mod monitor;
mod reaction;

use monitor::SystemMonitor;
use reaction::ChemicalSystem;

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    // Initialize system
    // We'll set a fixed resolution that likely fits.
    // Or we can resize dynamically.
    // Let's start with 100x50.
    let width = 120;
    let height = 60;
    let mut chem = ChemicalSystem::new(width, height);
    let mut monitor = SystemMonitor::new();

    // Seed random spots
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let rx = rng.gen_range(5..width - 5);
        let ry = rng.gen_range(5..height - 5);

        // Add a blob
        for y in ry - 2..=ry + 2 {
            for x in rx - 2..=rx + 2 {
                chem.add_chemical(x, y, 0.9);
            }
        }
    }

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // ~60 FPS

    loop {
        terminal.draw(|f| {
            ui(f, &chem, &monitor);
        })?;

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

                        // Map to grid coordinates (assuming widget at 0,0)
                        // Vertical resolution is double
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
            monitor.update();
            let (f, k) = monitor.get_parameters();
            chem.f = f;
            chem.k = k;

            // Multiple updates per frame for speed
            // Gray-Scott needs diffusion to be fast relative to reaction.
            for _ in 0..8 {
                chem.update(1.0);
            }

            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, chem: &ChemicalSystem, monitor: &SystemMonitor) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(f.area());

    let (f_param, k_param) = monitor.get_parameters();
    let stats = monitor.get_stats_string();
    let status_bar = Paragraph::new(format!(
        "{} | f: {:.4} k: {:.4} | q: Quit",
        stats, f_param, k_param
    ))
    .style(Style::default().bg(Color::DarkGray).fg(Color::White));

    f.render_widget(status_bar, chunks[1]);

    // Render Simulation
    let canvas_area = chunks[0];
    f.render_widget(ChemWidget { system: chem }, canvas_area);
}

struct ChemWidget<'a> {
    system: &'a ChemicalSystem,
}

impl<'a> Widget for ChemWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // We use half-block characters to double vertical resolution.
        // x maps 1:1, y maps 1:2.

        for x in 0..area.width {
            for y in 0..area.height {
                // Corresponding grid coordinates
                // We center the grid in the area or stretch?
                // Let's just crop/clamp for now.
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
    // v is typically 0.0 to 1.0 (though usually < 0.5 in patterns)
    // Map to some nice heatmap
    if v < 0.1 {
        Color::Black
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

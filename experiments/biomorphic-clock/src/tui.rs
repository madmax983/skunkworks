use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

use biomorphic_clock::simulation::Grid;

pub struct App {
    pub grid: Grid,
    pub should_quit: bool,
    pub angle: f32,
}

impl App {
    pub fn new() -> Result<Self> {
        let (width, height) = crossterm::terminal::size()?;
        // Reduce by 2 for borders
        let width = (width as usize).saturating_sub(2);
        let height = (height as usize).saturating_sub(2);

        let mut grid = Grid::new(width, height);
        grid.random_seed_center();

        Ok(Self {
            grid,
            should_quit: false,
            angle: 0.0,
        })
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        let w = (width as usize).saturating_sub(2);
        let h = (height as usize).saturating_sub(2);
        if w != self.grid.width || h != self.grid.height {
            // Re-allocate grid
            self.grid = Grid::new(w, h);
            self.grid.random_seed_center();
        }
    }
}

pub fn run_app(mut app: App) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_loop<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS target
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => app.should_quit = true,
                            KeyCode::Char('r') => {
                                let w = app.grid.width;
                                let h = app.grid.height;
                                app.resize(w as u16 + 2, h as u16 + 2); // Hack to trigger reseed
                            }
                            _ => {}
                        }
                    }
                }
                Event::Resize(w, h) => {
                    app.resize(w, h);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            // Update simulation
            // 60 seconds cycle = 360 degrees
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs_f32();

            let cycle = 60.0;
            app.angle = (time % cycle) / cycle * 2.0 * std::f32::consts::PI;

            // Run multiple simulation steps per frame
            for _ in 0..10 {
                app.grid.update(1.0, app.angle);
            }

            last_tick = Instant::now();
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.area();

    // Gradient: " .:-=+*#%@"
    let gradient = " .:-=+*#%@";
    let gradient_len = gradient.len() as f32;

    let mut lines = Vec::with_capacity(app.grid.height);

    for y in 0..app.grid.height {
        let mut spans = Vec::new();
        let mut current_color = Color::Reset;
        let mut current_text = String::new();

        for x in 0..app.grid.width {
            let idx = y * app.grid.width + x;
            let v = app.grid.v[idx];
            // let u = app.grid.u[idx];

            let val = (v * 3.0).clamp(0.0, 1.0); // Boost contrast
            let char_idx = (val * (gradient_len - 1.0)) as usize;
            let ch = gradient.chars().nth(char_idx).unwrap_or(' ');

            let color = if v > 0.3 {
                Color::Cyan
            } else if v > 0.1 {
                Color::Blue
            } else {
                Color::DarkGray
            };

            if color != current_color {
                if !current_text.is_empty() {
                    spans.push(Span::styled(
                        current_text,
                        Style::default().fg(current_color),
                    ));
                    current_text = String::new();
                }
                current_color = color;
            }
            current_text.push(ch);
        }
        if !current_text.is_empty() {
            spans.push(Span::styled(
                current_text,
                Style::default().fg(current_color),
            ));
        }
        lines.push(Line::from(spans));
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Biomorphic Clock - Angle: {:.2} rad", app.angle));
    let paragraph = Paragraph::new(lines).block(block);
    f.render_widget(paragraph, size);
}

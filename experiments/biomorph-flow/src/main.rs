use anyhow::Result;
use crossterm::event::{
    self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Points},
        Block, Borders, Paragraph,
    },
    Frame,
};
use std::{
    time::{Duration, Instant},
};
use tui_shared::Tui;

mod presets;
mod reaction;

use presets::PRESETS;
use reaction::GrayScott;

struct App {
    sim: GrayScott,
    running: bool,
    current_preset_idx: usize,
    mouse_pressed: bool,
    steps_per_frame: usize,
}

impl App {
    fn new() -> Self {
        // Start with a reasonably sized grid
        // Canvas coordinate system: 0..width, 0..height
        // We'll set it to 100x100 for now, but UI will determine render scale.
        // Actually, let's make it denser. 120x80.
        let mut sim = GrayScott::new(120, 80);
        sim.seed_center();

        Self {
            sim,
            running: true,
            current_preset_idx: 0,
            mouse_pressed: false,
            steps_per_frame: 8, // Run physics faster than render
        }
    }

    fn on_tick(&mut self) {
        if self.running {
            for _ in 0..self.steps_per_frame {
                self.sim.update();
            }
        }
    }

    fn next_preset(&mut self) {
        self.current_preset_idx = (self.current_preset_idx + 1) % PRESETS.len();
        self.apply_preset();
    }

    fn prev_preset(&mut self) {
         if self.current_preset_idx == 0 {
             self.current_preset_idx = PRESETS.len() - 1;
         } else {
             self.current_preset_idx -= 1;
         }
         self.apply_preset();
    }

    fn apply_preset(&mut self) {
        let p = &PRESETS[self.current_preset_idx];
        self.sim.f = p.f;
        self.sim.k = p.k;
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    // Enable mouse capture
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    let mut app = App::new();
    // Apply initial preset
    app.apply_preset();

    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        tui.terminal.draw(|f| ui(f, &mut app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => break,
                            KeyCode::Char('r') => app.sim.reset(),
                            KeyCode::Char(' ') => app.running = !app.running,
                            KeyCode::Right | KeyCode::Char('n') => app.next_preset(),
                            KeyCode::Left | KeyCode::Char('p') => app.prev_preset(),
                            KeyCode::Up => app.sim.f += 0.001,
                            KeyCode::Down => app.sim.f -= 0.001,
                            KeyCode::PageUp => app.sim.k += 0.001,
                            KeyCode::PageDown => app.sim.k -= 0.001,
                            KeyCode::Char('+') => app.steps_per_frame += 1,
                            KeyCode::Char('-') => if app.steps_per_frame > 1 { app.steps_per_frame -= 1 },
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.mouse_pressed = true;
                            handle_mouse(&mut app, mouse.column, mouse.row);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.mouse_pressed = false;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            if app.mouse_pressed {
                                handle_mouse(&mut app, mouse.column, mouse.row);
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }

    crossterm::execute!(std::io::stdout(), crossterm::event::DisableMouseCapture)?;
    Ok(())
}

fn handle_mouse(app: &mut App, col: u16, row: u16) {
    // Map screen coordinates to grid coordinates.
    // This is tricky without knowing the exact layout rect.
    // For now, we assume a full screen map or close to it.
    // Better approach: In `ui`, we know the rect. But we handle events outside `ui`.
    // Approximation:
    // The canvas is in the top chunk.
    // Assume 1 char = 1 unit?
    // Or we can just seed relative to window size.
    // Let's rely on the user aiming roughly.

    // Correction for borders (1) and title (1)
    let x = (col as usize).saturating_sub(1);
    let y = (row as usize).saturating_sub(1);

    // If the canvas is scaled, we need to scale x/y.
    // Our canvas x_bounds is [0, width], y_bounds is [0, height].
    // But Render is usually 2x1 pixels per char.
    // Let's just try direct mapping and see if it feels right.
    // Since we forced sim size to 120x80, we should map based on that?
    // No, let's map based on the rendered area if possible.
    // Without Layout state, we guess.

    // Scale for standard terminal font aspect ratio (approx 1:2)
    // Ratatui Canvas handles this internally if we use the same bounds.

    // Just clamp to sim dimensions
    // Actually, let's just use raw coords and clamp.

    let sx = x.clamp(0, app.sim.width - 1);
    let sy = (y * 2).clamp(0, app.sim.height - 1); // Y * 2 because braille/block usually packs vertically

    app.sim.seed_at(sx, sy, 3);
}

fn ui(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Biomorph Flow"))
        .x_bounds([0.0, app.sim.width as f64])
        .y_bounds([0.0, app.sim.height as f64])
        .marker(ratatui::symbols::Marker::Block)
        .paint(|ctx| {
            // Optimization: Group points by color
            // 8 buckets for our gradient
            let mut buckets: Vec<Vec<(f64, f64)>> = vec![vec![]; 8];

            for y in 0..app.sim.height {
                for x in 0..app.sim.width {
                    let idx = y * app.sim.width + x;
                    let v = app.sim.v[idx];

                    if v > 0.1 {
                        let bucket_idx = if v < 0.2 { 0 }
                        else if v < 0.3 { 1 }
                        else if v < 0.4 { 2 }
                        else if v < 0.5 { 3 }
                        else if v < 0.6 { 4 }
                        else if v < 0.8 { 5 }
                        else { 6 };

                        // Invert Y for canvas
                        buckets[bucket_idx].push((x as f64, (app.sim.height - 1 - y) as f64));
                    }
                }
            }

            let colors = [
                Color::DarkGray,
                Color::Blue,
                Color::Cyan,
                Color::Green,
                Color::Yellow,
                Color::Red,
                Color::White
            ];

            for (i, points) in buckets.iter().enumerate() {
                if !points.is_empty() {
                    if i < colors.len() {
                        ctx.draw(&Points {
                            coords: points,
                            color: colors[i],
                        });
                    }
                }
            }
        });

    f.render_widget(canvas, chunks[0]);

    let preset = &PRESETS[app.current_preset_idx];
    let status_text = vec![
        Line::from(vec![
            Span::raw("Preset: "),
            Span::styled(preset.name, Style::default().fg(Color::Green).bold()),
            Span::raw(format!(" | F: {:.4} | K: {:.4} | ", app.sim.f, app.sim.k)),
            Span::raw(format!("Speed: {}x", app.steps_per_frame)),
        ]),
        Line::from(vec![
            Span::raw("[Arrows] Preset/Tweak | [Space] Pause | [R]eset | [Mouse] Draw | [Q]uit"),
        ]),
    ];

    let status = Paragraph::new(status_text)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}

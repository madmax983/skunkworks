pub mod game;
pub mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
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
use std::time::{Duration, Instant};
use tui_shared::Tui;

use crate::game::GameState;
use crate::physics::{PacketKind, PinKind};

struct App {
    game: GameState,
    running: bool,
}

impl App {
    fn new(width: f64, height: f64) -> Self {
        Self {
            game: GameState::new(width, height),
            running: true,
        }
    }

    fn on_tick(&mut self) {
        // Run physics at higher frequency if needed, but for now 1:1 is fine
        // dt = 1/60 approx 0.016
        self.game.tick(0.016);
    }
}

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    crossterm::execute!(std::io::stdout(), crossterm::event::EnableMouseCapture)?;

    // Get terminal size for initial game size
    let _size = tui.terminal.size()?;
    // Canvas area is usually a bit smaller than terminal due to borders
    // And Canvas scaling: Braille is 2x4 pixels per char.
    // Block is 1x1.
    // Let's assume logical coordinates 0..100x100 and map it to whatever we have.
    // OR we can make the logical world fit the terminal chars directly.

    // Let's use a fixed logical size for consistent physics
    let logical_width = 100.0;
    let logical_height = 100.0;

    let mut app = App::new(logical_width, logical_height);

    let tick_rate = Duration::from_millis(16);
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
                            KeyCode::Esc | KeyCode::Char('q') => app.running = false,
                            KeyCode::Char(' ') => app.game.place_pin(),
                            KeyCode::Tab => {
                                app.game.tool_kind = match app.game.tool_kind {
                                    PinKind::Bumper => PinKind::Blocker,
                                    PinKind::Blocker => PinKind::Bumper,
                                    _ => PinKind::Bumper,
                                }
                            }
                            KeyCode::Left => app.game.move_cursor(-2.0, 0.0),
                            KeyCode::Right => app.game.move_cursor(2.0, 0.0),
                            KeyCode::Up => app.game.move_cursor(0.0, -2.0),
                            KeyCode::Down => app.game.move_cursor(0.0, 2.0),
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    // Map mouse to game coordinates
                    // This is hard without knowing the Rect of the Canvas.
                    // But we can approximate.
                    // Or we just rely on Keyboard for precision.
                    // For now, let's skip mouse interaction to keep it simple and precise.
                    // Or implement it if we have time.
                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            // Not implemented mapping yet
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

        if !app.running {
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

    // Calculate aspect ratio correction if possible?
    // For now just stretch.

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Packet Pachinko "),
        )
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, app.game.width])
        .y_bounds([0.0, app.game.height])
        .paint(|ctx| {
            // Draw Bins
            let bin_w = app.game.width / 3.0;
            let bin_h = 5.0;
            let bin_y = app.game.height - bin_h;

            // Bin 1: DROP (Red)
            ctx.layer();
            // We can't draw filled rects easily in Canvas without many lines
            // Just draw labels
            ctx.print(
                bin_w * 0.5 - 2.0,
                bin_y + 2.0,
                Span::styled("DROP", Style::default().fg(Color::Red)),
            );

            // Bin 2: SSH (Blue)
            ctx.print(
                bin_w * 1.5 - 2.0,
                bin_y + 2.0,
                Span::styled("SSH", Style::default().fg(Color::Blue)),
            );

            // Bin 3: HTTP (Green)
            ctx.print(
                bin_w * 2.5 - 2.0,
                bin_y + 2.0,
                Span::styled("HTTP", Style::default().fg(Color::Green)),
            );

            // Draw Pins
            for pin in &app.game.pins {
                let color = match pin.kind {
                    PinKind::Bumper => Color::White,
                    PinKind::Blocker => Color::DarkGray,
                    _ => Color::White,
                };

                // Draw pin as 'O'
                // Invert Y for render? Canvas usually (0,0) is Bottom-Left.
                // My Physics has (0,0) Top-Left (Gravity positive Y).
                // So I need to invert Y when drawing: canvas_y = height - game_y
                let render_y = app.game.height - pin.pos.y;

                ctx.print(
                    pin.pos.x,
                    render_y,
                    Span::styled("O", Style::default().fg(color)),
                );
            }

            // Draw Particles
            for p in &app.game.particles {
                let color = match p.kind {
                    PacketKind::Malware => Color::Red,
                    PacketKind::Ssh => Color::Blue,
                    PacketKind::Http => Color::Green,
                };

                let render_y = app.game.height - p.pos.y;

                ctx.draw(&Points {
                    coords: &[(p.pos.x, render_y)],
                    color,
                });
            }

            // Draw Cursor
            let render_cursor_y = app.game.height - app.game.cursor_pos.y;
            let cursor_color = match app.game.tool_kind {
                PinKind::Bumper => Color::Yellow,
                PinKind::Blocker => Color::Gray,
                _ => Color::Yellow,
            };
            ctx.print(
                app.game.cursor_pos.x,
                render_cursor_y,
                Span::styled("+", Style::default().fg(cursor_color)),
            );
        });

    f.render_widget(canvas, chunks[0]);

    // Status Bar
    let tool_name = match app.game.tool_kind {
        PinKind::Bumper => "Bumper (Bounce)",
        PinKind::Blocker => "Blocker (Absorb)", // Logic not fully impl yet
        _ => "Unknown",
    };

    let status = vec![Line::from(vec![
        Span::raw("Score: "),
        Span::styled(
            format!("{}", app.game.score),
            Style::default().fg(Color::Cyan).bold(),
        ),
        Span::raw(" | Tool: "),
        Span::styled(tool_name, Style::default().fg(Color::Yellow)),
        Span::raw(" | "),
        Span::raw("Arrows: Move | Space: Place/Remove | Tab: Switch Tool | Q: Quit"),
    ])];

    f.render_widget(
        Paragraph::new(status).block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
}

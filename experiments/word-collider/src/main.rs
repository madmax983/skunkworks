mod app;
mod physics;

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
};
use std::time::{Duration, Instant};
use tui_shared::Tui;

use app::{App, InputMode};

fn main() -> Result<()> {
    let mut tui = Tui::init()?;
    let res = run_app(&mut tui.terminal, &mut App::new());

    if let Err(err) = res {
        tui.exit()?;
        println!("{:?}", err);
    }
    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<B>,
    app: &mut App,
) -> Result<()> {
    let tick_rate = Duration::from_millis(16); // ~60 FPS
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| ui(f, app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match app.input_mode {
                            InputMode::Normal => match key.code {
                                KeyCode::Char('q') => app.running = false,
                                KeyCode::Char('i') => app.input_mode = InputMode::Typing,
                                KeyCode::Char('c') => app.world.clear(),
                                KeyCode::Char('p') => app.physics_paused = !app.physics_paused,
                                KeyCode::Char('r') => *app = App::new(),
                                _ => {}
                            },
                            InputMode::Typing => match key.code {
                                KeyCode::Enter => app.spawn_word(),
                                KeyCode::Esc => app.input_mode = InputMode::Normal,
                                KeyCode::Backspace => {
                                    app.input_buffer.pop();
                                }
                                KeyCode::Char(c) => {
                                    app.input_buffer.push(c);
                                }
                                _ => {}
                            },
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    let x = mouse.column;
                    let y = mouse.row;

                    // We need to account for the canvas offset.
                    // The main block has borders, so (0,0) is (1,1).
                    // And we assume the canvas takes up the top chunk.
                    // Let's assume fullscreen minus 3 lines for help.

                    let canvas_x = x.saturating_sub(1) as f64;
                    let canvas_y = y.saturating_sub(1) as f64;
                    // Note: Canvas Y coordinates are usually inverted (0 at bottom) in math,
                    // but ratatui Canvas has (0,0) at bottom-left by default.
                    // HOWEVER, our physics assumes (0,0) is top-left because gravity is +Y.
                    // So we need to map physics Y to canvas Y.
                    // Canvas: 0 at bottom. Physics: 0 at top.
                    // If Physics Height is H. Canvas Y = H - Physics Y.

                    // BUT, when painting, we can map physics coords to canvas coords.
                    // When receiving mouse input, we map screen coords to physics coords.

                    // Let's simplify:
                    // Physics: 0..W, 0..H (Top-Left Origin)
                    // Canvas: 0..W, 0..H (Bottom-Left Origin)
                    //
                    // Mouse (Screen): 0..W, 0..H (Top-Left Origin)
                    //
                    // So Mouse Y == Physics Y (roughly).
                    // But Canvas Y == Height - Physics Y.

                    // Let's rely on the Canvas bounds matching the screen size roughly.
                    // We update app world size in the draw loop, but we need it here.
                    // Let's assume the previous frame's size is good enough or use a fixed coordinate system.

                    // Actually, ratatui Canvas `paint` closure uses a coordinate system we define.
                    // We defined x_bounds=[0, width], y_bounds=[0, height].
                    // If we set y_bounds=[0, height], then Y=0 is bottom.
                    // If we want Y=0 to be Top (to match screen/physics), we can set y_bounds=[height, 0].
                    // That flips the axis!

                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.handle_mouse_down(canvas_x, canvas_y);
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.handle_mouse_up();
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            app.handle_mouse_drag(canvas_x, canvas_y);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(0.016); // Fixed time step
            last_tick = Instant::now();
        }

        if !app.running {
            break;
        }
    }
    Ok(())
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    // Update World Bounds to match Screen (minus borders)
    let canvas_area = chunks[0];
    let width = (canvas_area.width.saturating_sub(2)) as f32;
    let height = (canvas_area.height.saturating_sub(2)) as f32;
    app.resize(width, height);

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Word Collider "),
        )
        .marker(ratatui::symbols::Marker::Braille)
        .x_bounds([0.0, width as f64])
        // Invert Y axis so 0 is Top (matching screen/physics)
        .y_bounds([height as f64, 0.0])
        .paint(|ctx| {
            // Draw Constraints (Springs)
            for c in &app.world.constraints {
                let p1 = &app.world.particles[c.p1];
                let p2 = &app.world.particles[c.p2];
                ctx.draw(&CanvasLine {
                    x1: p1.pos.x as f64,
                    y1: p1.pos.y as f64,
                    x2: p2.pos.x as f64,
                    y2: p2.pos.y as f64,
                    color: Color::DarkGray,
                });
            }

            // Draw Particles (Characters)
            // Canvas doesn't support drawing Text characters at arbitrary positions directly via `draw`.
            // But we can use `ctx.print`.

            for p in &app.world.particles {
                ctx.print(
                    p.pos.x as f64,
                    p.pos.y as f64,
                    Span::styled(p.char.to_string(), Style::default().fg(p.color)),
                );
            }

            // Draw debug points for positions if needed (using Points)
            /*
            for p in &app.world.particles {
                ctx.draw(&Points {
                    coords: &[(p.pos.x as f64, p.pos.y as f64)],
                    color: p.color,
                });
            }
            */
        });

    f.render_widget(canvas, chunks[0]);

    // Status / Input
    let status_text = match app.input_mode {
        InputMode::Normal => vec![
            Span::raw("Press "),
            Span::styled("i", Style::default().fg(Color::Yellow)),
            Span::raw(" to insert text, "),
            Span::styled("Mouse", Style::default().fg(Color::Green)),
            Span::raw(" to drag, "),
            Span::styled("c", Style::default().fg(Color::Red)),
            Span::raw(" clear, "),
            Span::styled("p", Style::default().fg(Color::Cyan)),
            Span::raw(" pause, "),
            Span::styled("q", Style::default().fg(Color::Magenta)),
            Span::raw(" quit."),
        ],
        InputMode::Typing => vec![
            Span::raw("Typing: "),
            Span::styled(&app.input_buffer, Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD)),
            Span::raw("_"),
            Span::raw(" (Enter to spawn, Esc to cancel)"),
        ],
    };

    let status = Paragraph::new(Line::from(status_text))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(status, chunks[1]);
}

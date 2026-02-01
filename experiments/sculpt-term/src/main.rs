mod app;
mod math;
mod render;
mod sdf;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};
use std::{io, time::Duration};

use app::App;
use math::Vec3;
use render::SdfView;
use sdf::{op_smooth_union, sd_sphere};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)])
                .split(f.area());

            // Define the scene function based on app state
            let p1 = app.sphere1_pos;
            let p2 = app.sphere2_pos;
            let k = app.blend_factor;

            let scene_sdf = move |p: Vec3| {
                let d1 = sd_sphere(p - p1, 1.0);
                let d2 = sd_sphere(p - p2, 0.8);
                op_smooth_union(d1, d2, k)
            };

            let view = SdfView::new(scene_sdf, app.camera_pos, app.camera_target, app.time);

            f.render_widget(view, chunks[0]);

            // Instructions
            let text = vec![Line::from(vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().fg(Color::Red)),
                Span::raw(" to quit, "),
                Span::styled("WASD", Style::default().fg(Color::Yellow)),
                Span::raw(" to move camera, "),
                Span::styled("Arrows", Style::default().fg(Color::Green)),
                Span::raw(" to move sphere."),
            ])];
            let info = Paragraph::new(text).block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        // Updates
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('w') => app.camera_pos.z += 0.5,
                        KeyCode::Char('s') => app.camera_pos.z -= 0.5,
                        KeyCode::Char('a') => app.camera_pos.x -= 0.5,
                        KeyCode::Char('d') => app.camera_pos.x += 0.5,
                        KeyCode::Up => app.sphere1_pos.y += 0.1,
                        KeyCode::Down => app.sphere1_pos.y -= 0.1,
                        KeyCode::Left => app.sphere1_pos.x -= 0.1,
                        KeyCode::Right => app.sphere1_pos.x += 0.1,
                        _ => {}
                    }
                }
            }
        }

        if app.should_quit {
            break;
        }

        app.tick();
    }
    Ok(())
}

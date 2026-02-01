pub mod solver;
pub mod view;

use std::{io, time::{Duration, Instant}};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, MouseButton, MouseEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use solver::Fluid;
use view::FluidWidget;

struct App {
    fluid: Fluid,
    running: bool,
    mouse_pressed: bool,
    last_mouse_pos: Option<(u16, u16)>,
}

impl App {
    fn new() -> Self {
        // Size 60x40 is decent for terminal
        Self {
            fluid: Fluid::new(60, 0.1, 0.0001, 0.0001),
            running: true,
            mouse_pressed: false,
            last_mouse_pos: None,
        }
    }

    fn on_tick(&mut self) {
        self.fluid.step();
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App
    let mut app = App::new();

    let res = run_app(&mut terminal, &mut app);

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(f.area());

            let fluid_area = chunks[0];

            // Render Fluid
            // We use a Block to draw borders?
            // Or just render FluidWidget in the area.
            // Let's wrap it in a Block for aesthetics
            let block = Block::default().borders(Borders::ALL).title("Fluidia");
            let inner_area = block.inner(fluid_area);

            f.render_widget(block, fluid_area);
            f.render_widget(FluidWidget { fluid: &app.fluid }, inner_area);

            // Instructions
            let help_text = "Drag Mouse to stir | 'q': Quit | 'r': Reset | 'c': Clear";
            f.render_widget(Paragraph::new(help_text), chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') => app.running = false,
                            KeyCode::Char('r') => app.fluid = Fluid::new(60, 0.1, 0.0001, 0.0001),
                            KeyCode::Char('c') => {
                                // Clear density but keep velocity? Or reset both?
                                // Reset is easier.
                                app.fluid = Fluid::new(60, 0.1, 0.0001, 0.0001);
                            }
                            _ => {}
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    let x = mouse.column;
                    let y = mouse.row;
                    // We need to map screen coordinates (x,y) to fluid coordinates.
                    // The FluidWidget is rendered inside a Block with Borders::ALL.
                    // So Fluid (0,0) is at (ScreenX+1, ScreenY+1).
                    // But we don't know ScreenX easily here without storing layout.
                    // Assuming fullscreen, chunks[0] is at (0,0).
                    // Block border is 1 char thick.
                    // So Fluid(0,0) is at (1,1).

                    // Also `mouse.column` is u16.

                    let fluid_x = x.saturating_sub(1) as usize;
                    let fluid_y = y.saturating_sub(1) as usize;

                    match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => {
                            app.mouse_pressed = true;
                            app.last_mouse_pos = Some((x, y));
                        }
                        MouseEventKind::Up(MouseButton::Left) => {
                            app.mouse_pressed = false;
                            app.last_mouse_pos = None;
                        }
                        MouseEventKind::Drag(MouseButton::Left) => {
                            if let Some((lx, ly)) = app.last_mouse_pos {
                                // Calculate velocity from drag
                                let dx = (x as f32 - lx as f32) * 5.0;
                                let dy = (y as f32 - ly as f32) * 5.0;

                                // Inject into fluid
                                app.fluid.add_density(fluid_x, fluid_y, 50.0);
                                app.fluid.add_velocity(fluid_x, fluid_y, dx, dy);

                                // Also inject neighbors for "brush" effect
                                if fluid_x > 0 { app.fluid.add_density(fluid_x-1, fluid_y, 25.0); }
                                if fluid_y > 0 { app.fluid.add_density(fluid_x, fluid_y-1, 25.0); }

                                app.last_mouse_pos = Some((x, y));
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

        if !app.running {
            break;
        }
    }
    Ok(())
}

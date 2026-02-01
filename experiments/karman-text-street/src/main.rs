mod lbm;
mod renderer;
mod scroller;

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use tui_shared::Tui;

use lbm::Fluid;
use renderer::FluidWidget;
use scroller::Scroller;

struct App {
    fluid: Fluid,
    scroller: Scroller,
    running: bool,
    paused: bool,
}

impl App {
    fn new(width: usize, height: usize) -> Self {
        // Fluid resolution is 2x width, 4x height of the TUI area
        let fluid_w = width * 2;
        let fluid_h = height * 4;

        Self {
            fluid: Fluid::new(fluid_w, fluid_h),
            scroller: Scroller::new(fluid_w, fluid_h),
            running: true,
            paused: false,
        }
    }

    fn on_tick(&mut self) {
        if !self.paused {
            self.scroller.tick(&mut self.fluid);
            self.fluid.step();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    let mut tui = Tui::init()?;

    // Initial size guess
    let size = tui.terminal.size()?;
    // Reserve space for UI?
    let width = size.width as usize;
    let height = size.height.saturating_sub(2) as usize; // -2 for borders/help

    let mut app = App::new(width, height);

    let res = run_app(&mut tui.terminal, &mut app);

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    // Target 60 FPS? 30 is fine for TUI.
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(1)])
                .split(f.area());

            let fluid_area = chunks[0];

            // Render Fluid
            // We need to ensure we don't render out of bounds of the fluid
            // The fluid was created based on initial size.
            // If window grows, we just show what we have.
            f.render_widget(FluidWidget { fluid: &app.fluid }, fluid_area);

            // Status bar
            let help_text = vec![Line::from(vec![
                Span::raw("Type to add text | "),
                Span::styled("ESC/q", Style::default().fg(Color::Red)),
                Span::raw(" Quit | "),
                Span::styled("Space", Style::default().fg(Color::Yellow)),
                Span::raw(" Pause"),
            ])];
            f.render_widget(
                Paragraph::new(help_text).block(Block::default().borders(Borders::TOP)),
                chunks[1],
            );
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Esc => app.running = false,
                        KeyCode::Char('q') if key.modifiers.is_empty() => app.running = false, // Allow 'q' in text? No, 'q' quits.
                        KeyCode::Char(' ') => app.paused = !app.paused,
                        KeyCode::Char(c) => {
                            // Add to scroller
                            app.scroller.add_char(c);
                        }
                        KeyCode::Backspace => {
                            app.scroller.pop_char();
                        }
                        _ => {}
                    }
                }
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

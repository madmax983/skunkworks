use std::io::stdout;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use glam::Vec3;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Line as CanvasLine},
        Block, Borders, Paragraph,
    },
    Terminal,
};

mod physics;

use physics::{is_vowel, LexicalString};

fn main() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()> {
    // Initialize with a phrase
    let initial_text = "InTheBeginningWasTheWord";
    let start = Vec3::new(-60.0, 0.0, 0.0);
    let end = Vec3::new(60.0, 0.0, 0.0);
    let mut string = LexicalString::new(initial_text, start, end, 15.0, 0.5);

    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            // Render Canvas
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Biomorphic Lexicon 🧬🗣️"),
                )
                .x_bounds([-80.0, 80.0])
                .y_bounds([-40.0, 40.0])
                .paint(|ctx| {
                    for i in 0..string.nodes.len() - 1 {
                        let n1 = &string.nodes[i];
                        let n2 = &string.nodes[i + 1];

                        // Draw line between nodes
                        ctx.draw(&CanvasLine {
                            x1: n1.pos.x as f64,
                            y1: n1.pos.y as f64,
                            x2: n2.pos.x as f64,
                            y2: n2.pos.y as f64,
                            color: Color::DarkGray,
                        });
                    }

                    // Draw characters
                    for node in &string.nodes {
                        let color = if node.mutated {
                            Color::Green // Highlight mutated
                        } else if is_vowel(node.char) {
                            Color::Red
                        } else {
                            Color::Cyan
                        };

                        ctx.print(
                            node.pos.x as f64,
                            node.pos.y as f64,
                            Span::styled(
                                node.char.to_string(),
                                Style::default().fg(color),
                            ),
                        );
                    }
                });
            f.render_widget(canvas, chunks[0]);

            // Status Bar
            let status = Paragraph::new(vec![
                Line::from(vec![
                    Span::raw("Controls: "),
                    Span::styled("Space", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Agitate (Mutate) | "),
                    Span::styled("Enter", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Reset | "),
                    Span::styled("Q", Style::default().fg(Color::Yellow)),
                    Span::raw(" to Quit"),
                ]),
                Line::from(vec![Span::raw(format!(
                    "Current Word: {} | Nodes: {} | Mass: {:.1}",
                    string,
                    string.nodes.len(),
                    string.nodes.iter().map(|n| n.mass).sum::<f32>()
                ))]),
            ])
            .block(Block::default().borders(Borders::ALL));
            f.render_widget(status, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => {
                        string = LexicalString::new(initial_text, start, end, 15.0, 0.5);
                    }
                    KeyCode::Char(' ') => {
                        string.pluck();
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            let dt = 0.016;
            // Sub-stepping
            for _ in 0..5 {
                string.update(dt / 5.0);
            }
            // Reset mutation flags after rendering a frame? No, keep them for a bit?
            // Actually, physics loop runs faster than render.
            // If I reset here, render might miss it if physics runs multiple times.
            // But main loop runs render then physics.
            // So if mutation happens in physics, next render shows green.
            // Then after render, I should probably reset.
            // But physics runs *after* render in my loop above.
            // So: Render (shows state from last frame) -> Input -> Physics (updates state) -> Loop.

            // To make "flash" visible, maybe I should decay the `mutated` flag or use a timer?
            // For now, I'll just let it flicker.
            string.reset_mutation_flags();

            last_tick = Instant::now();
        }
    }
}

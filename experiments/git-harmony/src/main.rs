use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git_associates::GitModel;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{
        canvas::{Canvas, Circle, Context},
        Block, Borders, Paragraph,
    },
    Terminal,
};
use std::io;

pub mod synthesizer;

use synthesizer::Synthesizer;

fn main() -> Result<()> {
    // Setup
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Logic
    // We try to get diff, if it fails (e.g. no git repo), we handle it
    let diff = match GitModel::open(".") {
        Ok(model) => model.diff_workdir().unwrap_or_default(),
        Err(_) => git_associates::model::DiffStats::default(),
    };

    let mut synth = Synthesizer::new(diff);

    // Loop
    let res = run_app(&mut terminal, &mut synth);

    // Teardown
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    synth: &mut Synthesizer,
) -> Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(f.area());

            // Canvas for visual notes
            let canvas = Canvas::default()
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" ⚛️ Git Harmony ⚛️ "),
                )
                .x_bounds([0.0, 100.0])
                .y_bounds([0.0, 100.0])
                .paint(|ctx: &mut Context| {
                    for note in &synth.active_notes {
                        let y = (note.pitch * 90.0 + 5.0) as f64; // Keep within bounds roughly

                        // Animation: Flow from center outwards or scroll?
                        // Let's do scrolling: New notes appear at right (100) and move left
                        // Wait, 'life' goes 1.0 -> 0.0.
                        // So x = life * 100.0 ? That means they start at right and move left.
                        let x = (note.life * 90.0 + 5.0) as f64;

                        let color = if note.is_add {
                            Color::Green
                        } else if note.is_remove {
                            Color::Red
                        } else {
                            Color::Blue
                        };

                        // Draw text
                        // Truncate text to avoid clutter
                        let display_text = if note.text.len() > 20 {
                            format!("{}...", &note.text[0..20])
                        } else {
                            note.text.clone()
                        };

                        ctx.print(x, y, Span::styled(display_text, Style::default().fg(color)));

                        // Draw circle
                        ctx.draw(&Circle {
                            x,
                            y,
                            radius: 1.0 + (note.life * 2.0) as f64,
                            color,
                        });
                    }
                });
            f.render_widget(canvas, chunks[0]);

            // Status bar
            let last_note_text = synth
                .active_notes
                .last()
                .map(|n| n.text.clone())
                .unwrap_or_else(|| "Waiting for git diff...".to_string());
            let count = synth.active_notes.len();

            let p = Paragraph::new(format!(
                "Active Notes: {} | Current: {}",
                count, last_note_text
            ))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Status (Press 'q' to quit)"),
            );
            f.render_widget(p, chunks[1]);
        })?;

        // Input
        if event::poll(std::time::Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        // Update
        synth.tick();
    }
}

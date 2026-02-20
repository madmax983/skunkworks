use crate::model::ThreadState;
use anyhow::Result;
use crossbeam_channel::Receiver;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Row, Table},
    Terminal,
};
use std::io::stdout;
use std::time::Duration;

pub fn run_tui(state_receiver: Receiver<(usize, ThreadState)>, thread_count: usize) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut thread_states = vec![ThreadState::Sleeping; thread_count];
    let mut running = true;

    while running {
        // Process all pending state updates
        while let Ok((id, state)) = state_receiver.try_recv() {
            if id < thread_states.len() {
                thread_states[id] = state;
            }
        }

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.area());

            let rows: Vec<Row> = thread_states.iter().enumerate().map(|(id, state)| {
                let color = match state {
                    ThreadState::Sleeping => Color::Blue,
                    ThreadState::Waiting => Color::Red, // Contention!
                    ThreadState::Playing => Color::Green, // Sound!
                    ThreadState::Finished => Color::Gray,
                };
                Row::new(vec![
                    format!("Thread {}", id),
                    format!("{:?}", state),
                ]).style(Style::default().fg(color))
            }).collect();

            let table = Table::new(rows, [Constraint::Length(15), Constraint::Length(20)])
                .block(Block::default().title("Chimera Syncopation").borders(Borders::ALL));

            f.render_widget(table, chunks[0]);

            let info = Paragraph::new("Press 'q' to quit. \nRed = Waiting (Contention / Jitter) \nGreen = Playing (Lock Acquired) \nBlue = Sleeping (Rest)")
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(info, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    running = false;
                }
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

use anyhow::Result;
use chimera_lang::vm::ChimeraVM;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, time::Duration};

pub fn run_tui(mut vm: ChimeraVM) -> Result<()> {
    // TUI initialization should happen before raw mode
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut vm);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, vm: &mut ChimeraVM) -> Result<()>
where
    <B as ratatui::backend::Backend>::Error: std::fmt::Debug + std::fmt::Display + Send + Sync + std::error::Error + 'static,
{
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            // Stack view
            let stack_str = vm.stack.iter()
                .map(|v| format!("{:?}", v))
                .collect::<Vec<_>>()
                .join("\n");

            let stack_paragraph = Paragraph::new(Text::from(stack_str))
                .block(Block::default().title("Stack").borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(stack_paragraph, chunks[0]);

            // Output / Log
            let output_str = vm.output.join("\n");
            let output_paragraph = Paragraph::new(Text::from(output_str))
                .block(Block::default().title("Log").borders(Borders::ALL))
                .style(Style::default().fg(Color::Yellow));
            f.render_widget(output_paragraph, chunks[1]);
        })?;

        if crossterm::event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        vm.step();
                    }
                    _ => {}
                }
            }
        }
    }
}

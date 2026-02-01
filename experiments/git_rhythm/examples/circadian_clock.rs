use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use git_rhythm::experimental::circadian::CircadianContext;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::{io, thread, time::Duration};

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut hour = 0;
    let mut minute = 0;

    // Simulate time passing: 15 minutes per frame
    loop {
        terminal.draw(|f| {
            let size = f.area();

            // Calculate timestamp for current simulated time (relative to a day)
            let timestamp = (hour * 3600 + minute * 60) as i64;
            let ctx = CircadianContext::new(timestamp);
            let (theme_color, mood) = ctx.theme();
            let phase = ctx.phase();

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(1),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);

            let title = Paragraph::new("Nova's Circadian Clock")
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));
            f.render_widget(title, chunks[0]);

            let time_str = format!("{:02}:{:02}", hour, minute);
            let phase_str = format!("{:?}", phase);

            let content = format!(
                "\n\nTime: {}\nPhase: {}\nMood: {}\n\n(Press 'q' to quit)",
                time_str, phase_str, mood
            );

            let main_block = Paragraph::new(content)
                .style(Style::default().fg(Color::Black).bg(theme_color))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL).title("Visualization"));

            f.render_widget(main_block, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }

        minute += 15;
        if minute >= 60 {
            minute = 0;
            hour += 1;
        }
        if hour >= 24 {
            hour = 0;
        }

        thread::sleep(Duration::from_millis(50));
    }
}

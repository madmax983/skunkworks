use crate::drummer::DrummerState;
use crossbeam::channel::Receiver;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::time::Duration;

pub struct App {
    pub drummer_count: usize,
    pub states: Vec<DrummerState>,
    pub rx: Receiver<(usize, DrummerState)>,
}

impl App {
    pub fn new(drummer_count: usize, rx: Receiver<(usize, DrummerState)>) -> Self {
        Self {
            drummer_count,
            states: vec![DrummerState::Sleeping; drummer_count],
            rx,
        }
    }
}

pub fn run_app(mut app: App) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_loop(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_loop<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // Drain channel
        while let Ok((id, state)) = app.rx.try_recv() {
            if id < app.states.len() {
                app.states[id] = state;
            }
        }

        terminal
            .draw(|f| {
                // Use area() instead of size() (deprecated)
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
                    .split(size);

                let title = Paragraph::new("⚛️ Genesis: Schrödinger's Beat ⚛️")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(title, chunks[0]);

                let constraints: Vec<Constraint> = (0..app.drummer_count)
                    .map(|_| Constraint::Ratio(1, app.drummer_count as u32))
                    .collect();

                let thread_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(chunks[1]);

                for (i, state) in app.states.iter().enumerate() {
                    let (color, text) = match state {
                        DrummerState::Sleeping => (Color::Gray, "SLEEPING"),
                        DrummerState::Trying => (Color::Yellow, "TRYING..."),
                        DrummerState::Acquired => (Color::Green, "!!! ACQUIRED !!!"),
                        DrummerState::Contested => (Color::Red, "XXX CONTESTED XXX"),
                        DrummerState::Releasing => (Color::Blue, "RELEASING"),
                    };

                    let p = Paragraph::new(format!("Thread #{}: {}", i, text))
                        .style(Style::default().fg(Color::White).bg(color))
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .title(format!("Thread {}", i)),
                        );

                    f.render_widget(p, thread_chunks[i]);
                }
            })
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{:?}", e)))?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}

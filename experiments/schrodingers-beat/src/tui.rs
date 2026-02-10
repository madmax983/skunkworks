use crate::drummer::{DrummerState, DrummerUpdate};
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
    widgets::{Block, Borders, Gauge, Paragraph},
    Terminal,
};
use std::io;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct App {
    pub drummer_count: usize,
    pub updates: Vec<Option<DrummerUpdate>>,
    pub rx: Receiver<DrummerUpdate>,
    pub cpu_load: Arc<AtomicU8>,
}

impl App {
    pub fn new(drummer_count: usize, rx: Receiver<DrummerUpdate>, cpu_load: Arc<AtomicU8>) -> Self {
        Self {
            drummer_count,
            updates: vec![None; drummer_count],
            rx,
            cpu_load,
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
        while let Ok(update) = app.rx.try_recv() {
            if update.id < app.updates.len() {
                let id = update.id;
                app.updates[id] = Some(update);
            }
        }

        terminal
            .draw(|f| {
                let size = f.area();

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Title
                            Constraint::Length(3), // CPU
                            Constraint::Min(0),    // Drummers
                        ]
                        .as_ref(),
                    )
                    .split(size);

                // Title
                let title = Paragraph::new("⚛️ Genesis: Schrödinger's Beat (Euclidean + Mutex) ⚛️")
                    .style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    )
                    .block(Block::default().borders(Borders::ALL));
                f.render_widget(title, chunks[0]);

                // CPU Load
                let load = app.cpu_load.load(Ordering::Relaxed);
                let label = format!("System Entropy (CPU): {}%", load);
                let gauge = Gauge::default()
                    .block(
                        Block::default()
                            .title("Metric Modulation")
                            .borders(Borders::ALL),
                    )
                    .gauge_style(Style::default().fg(Color::Magenta))
                    .ratio(load as f64 / 100.0)
                    .label(label);
                f.render_widget(gauge, chunks[1]);

                // Drummers
                let constraints: Vec<Constraint> = (0..app.drummer_count)
                    .map(|_| Constraint::Ratio(1, app.drummer_count as u32))
                    .collect();

                let thread_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(chunks[2]);

                for (i, update_opt) in app.updates.iter().enumerate() {
                    let block = Block::default()
                        .borders(Borders::ALL)
                        .title(format!("Thread #{}", i));

                    if let Some(update) = update_opt {
                        let (color, state_text) = match update.state {
                            DrummerState::Sleeping => (Color::Gray, "SLEEPING"),
                            DrummerState::Trying => (Color::Yellow, "TRYING..."),
                            DrummerState::Acquired => (Color::Green, "!!! ACQUIRED !!!"),
                            DrummerState::Contested => (Color::Red, "XXX CONTESTED XXX"),
                            DrummerState::Releasing => (Color::Blue, "RELEASING"),
                        };

                        // Visualization of Euclidean Ring
                        // Active Step highlighted
                        let mut ring_vis = String::new();
                        for (idx, &is_beat) in update.pattern.iter().enumerate() {
                            if idx == update.step {
                                if is_beat {
                                    ring_vis.push_str("[●]"); // Current Beat
                                } else {
                                    ring_vis.push_str("[○]"); // Current Rest
                                }
                            } else {
                                if is_beat {
                                    ring_vis.push_str(" • "); // Beat
                                } else {
                                    ring_vis.push_str(" · "); // Rest
                                }
                            }
                        }

                        let content = format!(
                            "{}\nState: {}\nPattern: {}",
                            ring_vis,
                            state_text,
                            update
                                .pattern
                                .iter()
                                .map(|&b| if b { '1' } else { '0' })
                                .collect::<String>()
                        );

                        let p = Paragraph::new(content)
                            .style(Style::default().fg(color))
                            .block(block);
                        f.render_widget(p, thread_chunks[i]);
                    } else {
                        let p = Paragraph::new("Waiting for thread...")
                            .style(Style::default().fg(Color::DarkGray))
                            .block(block);
                        f.render_widget(p, thread_chunks[i]);
                    }
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

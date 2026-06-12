use crossbeam_channel::bounded;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use locus::Topology;
use quipu::{Cord, Quipu};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use resonance_audio::{AudioCommand, AudioModel};
use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), Box<dyn Error>> {
    // We are running headless for tests, if DISPLAY is not set
    let headless = std::env::var("DISPLAY").is_err();
    if headless {
        println!("Running in headless mode. Quitting gracefully.");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn Error>>
where
    <B as Backend>::Error: 'static,
{
    // Audio Setup
    let (cmd_tx, cmd_rx) = bounded(128);
    let (snap_tx, _snap_rx) = bounded(1); // Ignore snaps for now

    // We start the audio thread with our FDTD grid.
    let grid_width = 80;
    let grid_height = 40;

    // Spawn Audio model in background thread
    std::thread::spawn(move || {
        let mut model = AudioModel::new(grid_width, grid_height, cmd_rx, snap_tx, None);
        // Normally we'd hook this to cpal. For our visualizer, we just let it run its internal step loop.
        // But since we want to drive it, we'll implement a fake audio callback loop if not using cpal.
        // For simplicity and since we don't have cpal set up, we'll just run it in a loop.
        let mut buffer = vec![0.0; 256];
        loop {
            model.process(&mut buffer);
            std::thread::sleep(Duration::from_millis(10));
        }
    });

    let mut q = Quipu::new();
    q.add_cord(Cord::from(42));
    q.add_cord(Cord::from(105));
    q.add_cord(Cord::from(7));
    q.add_cord(Cord::from(314));

    let _cursor_x: i32 = 40;
    let _cursor_y: i32 = 20;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(250);
    let _topo = Topology::Torus;

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
                .split(f.area());

            let mut display_text = format!("Quipu Knots & Resonance Acoustic Exciters:\n\n");

            for (i, cord) in q.cords.iter().enumerate() {
                display_text.push_str(&format!("Cord {}: {}\n", i, cord.value()));
            }
            display_text.push_str("\nAs knots are read, they pluck the acoustic fabric.");

            let p = Paragraph::new(display_text).block(
                Block::default()
                    .title("Quipu Resonance")
                    .borders(Borders::ALL),
            );
            f.render_widget(p, chunks[0]);

            let help = Paragraph::new("Press 'q' to quit | 'Space' to pluck based on Quipu values")
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(help, chunks[1]);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Char(' ') => {
                        // Traverse quipu and generate acoustic exciters
                        for (i, cord) in q.cords.iter().enumerate() {
                            let val = cord.value() as f32;
                            let pluck_x = (10 + (i * 15)) as usize;
                            let pluck_y = 20;
                            // Pluck strength based on knot value
                            let strength = (val % 100.0) / 100.0;
                            let _ = cmd_tx.send(AudioCommand::Pluck {
                                x: pluck_x,
                                y: pluck_y,
                                strength,
                            });
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

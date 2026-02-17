use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use crossbeam_channel::unbounded;

use thread_cycle::audio::{AudioEngine, AudioEvent};
use thread_cycle::simulation::{SharedResource, ThreadCycler};
use thread_cycle::tui::{App, draw};

fn main() -> Result<()> {
    // 1. Setup Audio
    let audio = AudioEngine::new()?;
    let audio_tx = audio.get_sender();

    // 2. Setup TUI Channel
    let (tui_tx, tui_rx) = unbounded::<AudioEvent>();

    // 3. Setup Simulation
    let resource = Arc::new(SharedResource::new());

    // Config: ID, Cycle(ms), Work(ms)
    // Primes: 307, 503, 701, 1103, 1301
    // Work times small to cause contention but not deadlock
    let configs = vec![
        (1, 307u64, 40u64),
        (2, 503u64, 60u64),
        (3, 701u64, 80u64),
        (4, 1103u64, 50u64),
        (5, 1301u64, 70u64),
    ];

    // TUI App State
    let mut app = App::new(
        configs.iter().map(|&(id, c, w)| (id, Duration::from_millis(c), Duration::from_millis(w))).collect()
    );

    // Spawn Threads
    for (id, cycle_ms, work_ms) in configs {
        let resource = resource.clone();
        let audio_tx = audio_tx.clone();
        let tui_tx = tui_tx.clone();

        thread::spawn(move || {
            let cycler = ThreadCycler::new(
                id,
                Duration::from_millis(cycle_ms),
                Duration::from_millis(work_ms),
                resource,
            );

            // Closure that sends to both
            cycler.run_loop(move |e| {
                // Ignore errors (if channels closed)
                let _ = audio_tx.send(e);
                let _ = tui_tx.send(e);
            });
        });
    }

    // 4. Run TUI Loop
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &mut app, tui_rx);

    // 5. Cleanup
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    rx: crossbeam_channel::Receiver<AudioEvent>,
) -> Result<()>
where B::Error: Send + Sync + 'static
{
    loop {
        terminal.draw(|f| draw(f, app))?;

        // Process all pending events (non-blocking)
        while let Ok(event) = rx.try_recv() {
            app.on_event(event);
        }

        // Input Handling
        if event::poll(Duration::from_millis(16))? { // ~60 FPS
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }
}

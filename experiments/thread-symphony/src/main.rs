use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[cfg(feature = "audio")]
use thread_symphony::audio::AudioEngine;
use thread_symphony::conductor::{Instrument, Stage};
use thread_symphony::tui::{App, MusicianState, TuiEvent};

fn main() -> anyhow::Result<()> {
    #[cfg(not(feature = "audio"))]
    {
        println!("Audio feature disabled. Run with --features audio to play.");
        return Ok(());
    }

    #[cfg(feature = "audio")]
    real_main()
}

#[cfg(feature = "audio")]
fn real_main() -> anyhow::Result<()> {
    // 1. Audio
    let audio_engine = AudioEngine::new()?;
    let audio_tx = audio_engine.get_sender();

    // 2. TUI Channel
    let (tui_tx, tui_rx) = crossbeam_channel::unbounded();

    // 3. Stage
    let stage = Arc::new(Stage::new());

    // 4. Musicians (Threads)
    // We define a band with different periods to create polymeters
    let musicians = vec![
        (0, Instrument::Kick, Duration::from_millis(500)), // 4/4 @ 120bpm
        (1, Instrument::Snare, Duration::from_millis(750)), // 3/2 polyrhythm
        (2, Instrument::Hat, Duration::from_millis(125)),  // 16th notes
        (3, Instrument::Hat, Duration::from_millis(333)),  // Triplet feel
        (4, Instrument::Crash, Duration::from_millis(2000)), // Long cycle
    ];

    let num_musicians = musicians.len();

    for (id, inst, period) in musicians {
        let stage = stage.clone();
        let audio = audio_tx.clone();
        let tui = tui_tx.clone();

        thread::spawn(move || {
            // Initial offset to stagger starts
            thread::sleep(Duration::from_millis((id as u64) * 100));

            loop {
                // Sleep (Rest period)
                let _ = tui.send(TuiEvent::StateChange {
                    id,
                    state: MusicianState::Sleeping,
                });
                thread::sleep(period);

                // Wait for the Stage (Contention)
                let _ = tui.send(TuiEvent::StateChange {
                    id,
                    state: MusicianState::Waiting,
                });

                {
                    // Lock the GLOBAL stage (Monophonic / Linear constraint)
                    // This forces beats that land near each other to serialize,
                    // creating "flams" and "shuffles".
                    let _guard = stage.lock_global();

                    // Playing!
                    let _ = tui.send(TuiEvent::StateChange {
                        id,
                        state: MusicianState::Playing,
                    });
                    let _ = tui.send(TuiEvent::Beat {
                        id,
                        instrument: inst,
                    });
                    audio.send(inst).unwrap();

                    // Hold the stage for a tiny bit (simulating physical motion)
                    // This "width" of the beat causes the blocking.
                    thread::sleep(Duration::from_millis(30));
                } // Release lock
            }
        });
    }

    // 5. TUI Loop
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(num_musicians, tui_rx);

    loop {
        app.update();
        app.draw(&mut terminal)?;

        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

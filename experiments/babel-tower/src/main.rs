pub mod audio;
pub mod phonology;
pub mod speaker;
pub mod tower;
pub mod tui;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::sync::{atomic::AtomicBool, Arc, Mutex};
use std::time::{Duration, Instant};

use crate::audio::AudioEngine;
use crate::speaker::{Speaker, SpeakerEvent};
use crate::tower::Tower;
use crate::tui::{draw_ui, SpeakerStat, TuiState};

const SCRIPT: &str = r#"
Now the whole world had one language and a common speech.
As people moved eastward, they found a plain in Shinar and settled there.
They said to each other, "Come, let's make bricks and bake them thoroughly."
They used brick instead of stone, and tar for mortar.
Then they said, "Come, let us build ourselves a city, with a tower that reaches to the heavens, so that we may make a name for ourselves; otherwise we will be scattered over the face of the whole earth."
But the Lord came down to see the city and the tower the people were building.
The Lord said, "If as one people speaking the same language they have begun to do this, then nothing they plan to do will be impossible for them.
Come, let us go down and confuse their language so they will not understand each other."
So the Lord scattered them from there over all the earth, and they stopped building the city.
That is why it was called Babel—because there the Lord confused the language of the whole world.
From there the Lord scattered them over the face of the whole earth.
"#;

fn main() -> Result<()> {
    // Setup Audio
    let audio_engine = Arc::new(AudioEngine::new()?);

    // Setup Tower (Shared State)
    let tower = Arc::new(Mutex::new(Tower::new(SCRIPT)));

    // Setup Channels
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let running = Arc::new(AtomicBool::new(true));

    // Setup Speakers
    let num_speakers = 8;
    let mut handles = Vec::new();
    let mut speaker_stats = Vec::new();

    for i in 0..num_speakers {
        let speaker = Speaker::new(
            i,
            tower.clone(),
            audio_engine.clone(),
            running.clone(),
            event_tx.clone(),
            Duration::from_millis(50 + (i as u64 * 10)), // Staggered rhythm
        );
        handles.push(speaker.spawn());

        speaker_stats.push(SpeakerStat {
            id: i,
            last_wait: Duration::from_millis(0),
            last_word: String::new(),
            total_words: 0,
            avg_wait_ms: 0.0,
        });
    }

    // TUI Setup
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(16); // 60 FPS UI

    let mut transcript_buffer = Vec::new();

    loop {
        // Handle incoming events from speakers
        while let Ok(event) = event_rx.try_recv() {
            match event {
                SpeakerEvent::Spoke { id, word, wait } => {
                    // Update stats
                    if let Some(stat) = speaker_stats.get_mut(id) {
                        stat.last_wait = wait;
                        stat.last_word = word.clone();
                        stat.total_words += 1;
                        // Simple moving average
                        let wait_ms = wait.as_millis() as f32;
                        if stat.avg_wait_ms == 0.0 {
                            stat.avg_wait_ms = wait_ms;
                        } else {
                            stat.avg_wait_ms = stat.avg_wait_ms * 0.9 + wait_ms * 0.1;
                        }
                    }

                    // Add to transcript buffer for display
                    let intensity = (wait.as_millis() as f32 / 50.0).min(5.0);
                    transcript_buffer.push((word, intensity));

                    // Keep buffer size manageable
                    if transcript_buffer.len() > 2000 {
                         transcript_buffer.drain(0..1000);
                    }
                }
            }
        }

        // Draw UI
        let state = TuiState {
            transcript: transcript_buffer.clone(),
            speaker_stats: speaker_stats // Move logic? No, stat contains String which is Clone.
                .iter()
                .map(|s| SpeakerStat {
                    id: s.id,
                    last_wait: s.last_wait,
                    last_word: s.last_word.clone(),
                    total_words: s.total_words,
                    avg_wait_ms: s.avg_wait_ms,
                })
                .collect(),
        };

        terminal.draw(|f| draw_ui(f, &state))?;

        // Input Handling
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }

    // Cleanup
    running.store(false, std::sync::atomic::Ordering::Relaxed);
    // Join threads? They sleep so might take a bit.
    // For now just exit.

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

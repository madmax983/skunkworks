use crate::audio::AudioEvent;
use crossbeam::channel::Sender;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MusicianState {
    Sleeping,
    Waiting,
    Playing,
    Contending,
}

pub struct Musician {
    pub id: usize,
    pub name: String,
    pub state: Arc<Mutex<MusicianState>>,
}

impl Musician {
    pub fn new(id: usize, name: String) -> Self {
        Self {
            id,
            name,
            state: Arc::new(Mutex::new(MusicianState::Sleeping)),
        }
    }
}

pub fn spawn_musician(
    musician: &Musician,
    interval_ms: u64,
    freq: f32,
    audio_tx: Sender<AudioEvent>,
    instrument: Arc<Mutex<()>>,
    running: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
    let state = musician.state.clone();
    let id = musician.id;
    let interval = Duration::from_millis(interval_ms);
    let note_duration = Duration::from_millis(100); // Short pluck

    thread::spawn(move || {
        // Offset start to avoid initial sync
        thread::sleep(Duration::from_millis((id as u64) * 50));

        while running.load(Ordering::Relaxed) {
            // Sleep for interval
            {
                let mut s = state.lock().unwrap();
                *s = MusicianState::Sleeping;
            }

            // Sleep in chunks to check running flag?
            // Or just sleep full interval.
            // If we quit during sleep, we wait up to interval_ms.
            // Acceptable for < 1s intervals.
            thread::sleep(interval);
            if !running.load(Ordering::Relaxed) { break; }

            // Try to play
            {
                let mut s = state.lock().unwrap();
                *s = MusicianState::Waiting;
            }

            // Attempt to acquire lock
            if let Ok(_guard) = instrument.try_lock() {
                // Success!
                {
                    let mut s = state.lock().unwrap();
                    *s = MusicianState::Playing;
                }

                if audio_tx.send(AudioEvent::NoteOn { freq, duration: 0.1 }).is_err() {
                    break;
                }

                thread::sleep(note_duration);
                if !running.load(Ordering::Relaxed) { break; }

                let _ = audio_tx.send(AudioEvent::NoteOff { freq });
            } else {
                // Contention!
                {
                    let mut s = state.lock().unwrap();
                    *s = MusicianState::Contending;
                }

                if audio_tx.send(AudioEvent::Contention).is_err() {
                    break;
                }

                thread::sleep(Duration::from_millis(50));
            }
        }
    })
}

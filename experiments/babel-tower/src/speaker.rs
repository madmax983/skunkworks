use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam_channel::Sender;
use crate::tower::Tower;
use crate::audio::{AudioEngine, Voice};

pub struct Speaker {
    id: usize,
    tower: Arc<Mutex<Tower>>,
    audio: Arc<AudioEngine>,
    running: Arc<AtomicBool>,
    event_tx: Sender<SpeakerEvent>,
    base_sleep: Duration,
}

#[derive(Debug, Clone)]
pub enum SpeakerEvent {
    Spoke { id: usize, word: String, wait: Duration },
}

impl Speaker {
    pub fn new(
        id: usize,
        tower: Arc<Mutex<Tower>>,
        audio: Arc<AudioEngine>,
        running: Arc<AtomicBool>,
        event_tx: Sender<SpeakerEvent>,
        base_sleep: Duration,
    ) -> Self {
        Self {
            id,
            tower,
            audio,
            running,
            event_tx,
            base_sleep,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            while self.running.load(Ordering::Relaxed) {
                // Try to acquire lock
                let start = Instant::now();

                // Block until we get the lock
                // We use standard Mutex which blocks.
                // Contention happens here.
                let mut tower_guard = self.tower.lock().unwrap();
                let wait_time = start.elapsed();

                // Speak
                let word = tower_guard.speak(wait_time);

                // Release lock immediately so other threads can proceed (or contend)
                drop(tower_guard);

                // Play audio
                // Distortion proportional to wait time (0.0 - 2.0 range)
                let distortion = (wait_time.as_millis() as f32 / 50.0).min(5.0);
                self.audio.play(Voice::Speaker(self.id), distortion);

                // Notify UI
                let _ = self.event_tx.send(SpeakerEvent::Spoke {
                    id: self.id,
                    word,
                    wait: wait_time,
                });

                // Sleep to simulate rhythm
                // Add some jitter?
                thread::sleep(self.base_sleep);
            }
        })
    }
}

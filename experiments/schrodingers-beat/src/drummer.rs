use crate::audio::SoundEvent;
use crossbeam::channel::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum DrummerState {
    Sleeping,
    Trying,
    Acquired,
    Contested,
    Releasing,
}

pub struct Drummer {
    pub id: usize,
    pub interval_ms: u64,
    pub resource: Arc<Mutex<()>>,
    pub audio_tx: Sender<SoundEvent>,
    pub state_tx: Sender<(usize, DrummerState)>,
}

impl Drummer {
    pub fn spawn(self) {
        thread::spawn(move || {
            loop {
                // Sleep (Rhythm phase)
                self.report(DrummerState::Sleeping);
                thread::sleep(Duration::from_millis(self.interval_ms));

                // Try Lock (The Beat)
                self.report(DrummerState::Trying);
                self.audio_tx.send(SoundEvent::Waiting).ok(); // Pre-beat click

                // Small delay to let the "Trying" state be visible/audible?
                // No, instant.

                // We use try_lock to keep the metronome steady.
                // If we used lock(), the rhythm would drag.
                match self.resource.try_lock() {
                    Ok(_guard) => {
                        // Acquired (Strong Beat)
                        self.report(DrummerState::Acquired);
                        self.audio_tx.send(SoundEvent::LockAcquired).ok();

                        // Hold for a percentage of the interval (Sustain)
                        // e.g., 20% of interval
                        thread::sleep(Duration::from_millis(self.interval_ms / 5));

                        // Release (Off-beat/Snare)
                        self.report(DrummerState::Releasing);
                        self.audio_tx.send(SoundEvent::LockReleased).ok();
                    }
                    Err(_) => {
                        // Contested (Clash/Dissonance)
                        self.report(DrummerState::Contested);
                        self.audio_tx.send(SoundEvent::Contention).ok();

                        // Still sleep the "hold" time to keep phase alignment roughly similar?
                        // Or just skip?
                        // Let's skip, so failure is shorter -> "Stumbling" rhythm.
                    }
                }
            }
        });
    }

    fn report(&self, state: DrummerState) {
        self.state_tx.send((self.id, state)).ok();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel::unbounded;

    #[test]
    fn test_drummer_spawn() {
        let (audio_tx, _) = unbounded();
        let (state_tx, state_rx) = unbounded();
        let resource = Arc::new(Mutex::new(()));

        let drummer = Drummer {
            id: 0,
            interval_ms: 10,
            resource,
            audio_tx,
            state_tx,
        };
        drummer.spawn();

        // Wait for at least one state report
        // We expect "Sleeping" first
        let report = state_rx.recv_timeout(Duration::from_secs(1));
        assert!(report.is_ok());
    }
}

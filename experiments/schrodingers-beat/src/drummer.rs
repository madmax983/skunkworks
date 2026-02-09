use crate::audio::SoundEvent;
use crate::euclidean::generate;
use crossbeam::channel::Sender;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrummerState {
    Sleeping,
    Trying,
    Acquired,
    Contested,
    Releasing,
}

#[derive(Clone, Debug)]
pub struct DrummerUpdate {
    pub id: usize,
    pub state: DrummerState,
    pub step: usize,
    pub pattern: Vec<bool>,
}

pub struct Drummer {
    pub id: usize,
    pub interval_ms: u64,
    pub resource: Arc<Mutex<()>>,
    pub audio_tx: Sender<SoundEvent>,
    pub state_tx: Sender<DrummerUpdate>,
    pub cpu_load: Arc<AtomicU8>,
    pub euclidean_k: usize,
    pub euclidean_n: usize,
}

impl Drummer {
    pub fn spawn(self) {
        thread::spawn(move || {
            let mut step = 0;
            loop {
                // 1. Calculate dynamic K based on CPU load
                let load = self.cpu_load.load(Ordering::Relaxed);
                // Map load (0-100) to range [k, n]
                // If load is 0 -> k
                // If load is 100 -> n
                // We use saturating_sub to avoid underflow if n < k (which shouldn't happen but safe is better)
                let range = self.euclidean_n.saturating_sub(self.euclidean_k);
                let extra_k = (load as usize * range) / 100;
                let effective_k = self.euclidean_k + extra_k;

                // 2. Generate Pattern
                let pattern = generate(effective_k, self.euclidean_n);

                // 3. Check if current step is a beat
                let is_beat = if self.euclidean_n > 0 {
                    pattern[step % self.euclidean_n]
                } else {
                    false
                };

                // Report State (Start of Step)
                self.report(DrummerState::Sleeping, step, &pattern);

                // Sleep for the pulse duration (minus execution time approx)
                thread::sleep(Duration::from_millis(self.interval_ms));

                if is_beat {
                    // Try Lock
                    self.report(DrummerState::Trying, step, &pattern);

                    match self.resource.try_lock() {
                        Ok(_guard) => {
                            self.report(DrummerState::Acquired, step, &pattern);
                            // Using new SoundEvent variant (requires update in audio.rs)
                            self.audio_tx.send(SoundEvent::LockAcquired(self.id)).ok();

                            // Hold for a bit (gate time)
                            thread::sleep(Duration::from_millis(self.interval_ms / 2));

                            self.report(DrummerState::Releasing, step, &pattern);
                            self.audio_tx.send(SoundEvent::LockReleased(self.id)).ok();
                        }
                        Err(_) => {
                            self.report(DrummerState::Contested, step, &pattern);
                            self.audio_tx.send(SoundEvent::Contention(self.id)).ok();
                        }
                    }
                }

                step = (step + 1) % self.euclidean_n.max(1);
            }
        });
    }

    fn report(&self, state: DrummerState, step: usize, pattern: &Vec<bool>) {
        self.state_tx
            .send(DrummerUpdate {
                id: self.id,
                state,
                step,
                pattern: pattern.clone(),
            })
            .ok();
    }
}

use crate::audio::{RhythmEvent, Voice};
use crossbeam_channel::Sender;
use rand::Rng;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc, Mutex,
};
use std::thread;
use std::time::Duration;

pub struct Beacon {
    pub last_beat: AtomicU64,
    pub is_clash: AtomicBool,
}

impl Beacon {
    pub fn new() -> Self {
        Beacon {
            last_beat: AtomicU64::new(0),
            is_clash: AtomicBool::new(false),
        }
    }
}

pub struct Musician {
    pub id: usize,
    #[allow(dead_code)]
    pub name: String,
    pub period_ms: u64, // Period in milliseconds
    pub voice: Voice,
    tx: Sender<RhythmEvent>,
    sample_rate: u32,
    current_time: Arc<AtomicU64>,
    shared_resource: Arc<Mutex<()>>, // The resource to contend for
    running: Arc<AtomicBool>,
    beacon: Arc<Beacon>,
    work_load: u64,
    drift_ms: u64,
}

impl Musician {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: usize,
        name: String,
        period_ms: u64,
        voice: Voice,
        tx: Sender<RhythmEvent>,
        sample_rate: u32,
        current_time: Arc<AtomicU64>,
        shared_resource: Arc<Mutex<()>>,
        running: Arc<AtomicBool>,
        beacon: Arc<Beacon>,
        work_load: u64,
        drift_ms: u64,
    ) -> Self {
        Musician {
            id,
            name,
            period_ms,
            voice,
            tx,
            sample_rate,
            current_time,
            shared_resource,
            running,
            beacon,
            work_load,
            drift_ms,
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let period_samples = (self.period_ms as f64 * self.sample_rate as f64 / 1000.0) as u64;

            // Initial synchronization
            let mut next_beat_sample =
                self.current_time.load(Ordering::Relaxed) + self.sample_rate as u64 / 2;

            while self.running.load(Ordering::Relaxed) {
                let now = self.current_time.load(Ordering::Relaxed);

                // Catch-up logic: If we missed the beat, skip to the next grid point
                // This ensures we stay on the Euclidean grid even if we hiccup
                while next_beat_sample < now {
                    next_beat_sample += period_samples;
                }

                if now < next_beat_sample {
                    let diff = next_beat_sample - now;
                    let diff_ms = diff as f64 * 1000.0 / self.sample_rate as f64;

                    // Apply negative jitter (wake up slightly early/late)
                    // If drift is enabled, we might wake up earlier or later.
                    // But we can only sleep. So we sleep less.
                    // Actually, "drift" usually means error.
                    // Let's implement drift by modifying the sleep time randomly.

                    let jitter = if self.drift_ms > 0 {
                        rand::thread_rng().gen_range(0..=self.drift_ms) as f64
                    } else {
                        0.0
                    };

                    // We target waking up 'jitter' ms late (or early? let's say late for simplicity/laziness simulation)
                    // Actually, let's just subtract jitter from the sleep time to be safe?
                    // No, let's randomize the target wake up time.

                    // Effective diff to sleep is diff_ms +/- jitter?
                    // Let's just sleep a bit less or more.
                    // If we sleep less, we spin.
                    // If we sleep more, we are late (which is the goal of "drift").

                    // Let's say we aim to be late by 'jitter' ms.
                    // So we sleep diff_ms + jitter.

                    // Wait, if we sleep too much, next_beat_sample < now logic triggers catch up?
                    // Only if we are *way* late (past the next beat).
                    // If we are slightly late, we just process late.

                    let sleep_ms = if diff_ms > 20.0 {
                        (diff_ms - 5.0) as u64 // Wake up 5ms early to spin
                    } else {
                        0
                    };

                    if sleep_ms > 0 {
                        // Add random jitter to the sleep
                        let randomized_sleep = sleep_ms.saturating_add(jitter as u64);
                        thread::sleep(Duration::from_millis(randomized_sleep));
                    } else if diff_ms > 1.0 {
                        thread::sleep(Duration::from_millis(1));
                    } else {
                        std::hint::spin_loop();
                    }

                    // Check time again after waking up
                    let now_after_sleep = self.current_time.load(Ordering::Relaxed);
                    if now_after_sleep < next_beat_sample {
                        continue; // Still early, loop again (spin)
                    }
                }

                // It's the beat! (or we are slightly late)

                // Attempt synchronization (contention)
                let (final_voice, volume) = match self.shared_resource.try_lock() {
                    Ok(guard) => {
                        // Simulate work based on work_load
                        let mut x: u64 = 0;
                        for _ in 0..self.work_load {
                            x = x.wrapping_add(1);
                            std::hint::black_box(x);
                        }
                        drop(guard);

                        (self.voice, 0.8)
                    }
                    Err(_) => {
                        // Resource busy - Contention!
                        // Play a "Clave" sound to indicate clash
                        (Voice::Clave, 0.6)
                    }
                };

                let now = self.current_time.load(Ordering::Relaxed);

                // Schedule event. Since we might be late, we schedule it ASAP (now + small buffer)
                // or at the *intended* time if possible?
                // If we schedule in the past, AudioEngine (new logic) starts it NOW.
                // So passing 'next_beat_sample' (the intended time) is correct if we want
                // the audio engine to handle "catch up" (play immediately if late).
                // But let's pass 'now + buffer' to be safe against glitches.

                // Actually, passing 'now' is safer for real-time generation.
                let schedule_time = now; // Play immediately

                let _ = self.tx.send(RhythmEvent {
                    timestamp: schedule_time,
                    voice: final_voice,
                    volume,
                    source_id: self.id,
                });

                // Update beacon
                self.beacon
                    .last_beat
                    .store(schedule_time, Ordering::Relaxed);
                self.beacon
                    .is_clash
                    .store(matches!(final_voice, Voice::Clave), Ordering::Relaxed);

                // Advance beat to next grid point
                next_beat_sample += period_samples;
            }
        })
    }
}

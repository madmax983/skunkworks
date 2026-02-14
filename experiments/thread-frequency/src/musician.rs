use crate::audio::{RhythmEvent, Voice};
use crossbeam_channel::Sender;
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
}

impl Musician {
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
        }
    }

    pub fn spawn(self) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let period_samples = (self.period_ms as f64 * self.sample_rate as f64 / 1000.0) as u64;
            // Start slightly in the future
            let start_time =
                self.current_time.load(Ordering::Relaxed) + self.sample_rate as u64 / 2;
            let mut next_beat_sample = start_time;

            while self.running.load(Ordering::Relaxed) {
                let now = self.current_time.load(Ordering::Relaxed);

                if now < next_beat_sample {
                    let diff = next_beat_sample - now;
                    let diff_ms = diff as f64 * 1000.0 / self.sample_rate as f64;

                    if diff_ms > 20.0 {
                        thread::sleep(Duration::from_millis((diff_ms - 10.0) as u64));
                    } else if diff_ms > 2.0 {
                        thread::sleep(Duration::from_millis(1));
                    } else {
                        std::hint::spin_loop();
                    }
                    continue;
                }

                // It's the beat!
                // Attempt synchronization (contention)
                let (final_voice, volume) = match self.shared_resource.try_lock() {
                    Ok(guard) => {
                        // Simulate work
                        let work_load = 5000;
                        let mut x: u64 = 0;
                        for _ in 0..work_load {
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

                // Schedule event slightly in future to avoid glitches
                // If we are late (now > next_beat_sample), we schedule ASAP
                let schedule_time = std::cmp::max(now + 500, next_beat_sample + 500);

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

                // Advance beat
                next_beat_sample += period_samples;

                // If we fell way behind, skip beats to catch up
                if next_beat_sample < now {
                    next_beat_sample = now + period_samples;
                }
            }
        })
    }
}

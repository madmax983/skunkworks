use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

#[derive(Debug)]
pub struct GrooveState {
    // Latency in microseconds for each instrument
    kick_latency_micros: AtomicU64,
    snare_latency_micros: AtomicU64,
    hat_latency_micros: AtomicU64,

    // Counters for visualization
    total_beats: AtomicU64,
}

impl Default for GrooveState {
    fn default() -> Self {
        Self {
            kick_latency_micros: AtomicU64::new(0),
            snare_latency_micros: AtomicU64::new(0),
            hat_latency_micros: AtomicU64::new(0),
            total_beats: AtomicU64::new(0),
        }
    }
}

impl GrooveState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_kick_latency(&self, duration: Duration) {
        self.kick_latency_micros.store(duration.as_micros() as u64, Ordering::Relaxed);
    }

    pub fn set_snare_latency(&self, duration: Duration) {
        self.snare_latency_micros.store(duration.as_micros() as u64, Ordering::Relaxed);
    }

    pub fn set_hat_latency(&self, duration: Duration) {
        self.hat_latency_micros.store(duration.as_micros() as u64, Ordering::Relaxed);
    }

    pub fn get_kick_latency(&self) -> Duration {
        Duration::from_micros(self.kick_latency_micros.load(Ordering::Relaxed))
    }

    pub fn get_snare_latency(&self) -> Duration {
        Duration::from_micros(self.snare_latency_micros.load(Ordering::Relaxed))
    }

    pub fn get_hat_latency(&self) -> Duration {
        Duration::from_micros(self.hat_latency_micros.load(Ordering::Relaxed))
    }

    pub fn increment_beat(&self) {
        self.total_beats.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_total_beats(&self) -> u64 {
        self.total_beats.load(Ordering::Relaxed)
    }
}

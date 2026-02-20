use parking_lot::RwLock;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub struct Params {
    pub areas: Vec<f32>,
    pub frequency: f32,
    pub is_speaking: bool,
}

pub struct Metrics {
    pub output_level_bits: AtomicU32,
}

impl Metrics {
    pub fn set_level(&self, level: f32) {
        self.output_level_bits.store(level.to_bits(), Ordering::Relaxed);
    }
    pub fn get_level(&self) -> f32 {
        f32::from_bits(self.output_level_bits.load(Ordering::Relaxed))
    }
}

pub struct SharedState {
    pub params: RwLock<Params>,
    pub metrics: Metrics,
}

impl SharedState {
    pub fn new(len: usize) -> Arc<Self> {
        Arc::new(Self {
            params: RwLock::new(Params {
                areas: vec![1.0; len], // Default uniform tube
                frequency: 110.0,      // Low A
                is_speaking: false,
            }),
            metrics: Metrics {
                output_level_bits: AtomicU32::new(0f32.to_bits()),
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_state_metrics() {
        let state = SharedState::new(10);
        state.metrics.set_level(0.5);
        assert!((state.metrics.get_level() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_shared_state_params() {
        let state = SharedState::new(10);
        {
            let mut params = state.params.write();
            params.frequency = 220.0;
        }
        let params = state.params.read();
        assert_eq!(params.frequency, 220.0);
    }
}

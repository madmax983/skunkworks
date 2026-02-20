use parking_lot::RwLock;
use std::sync::Arc;

pub struct SharedState {
    pub params: RwLock<VocalParams>,
    pub metrics: Arc<AudioMetrics>,
}

pub struct VocalParams {
    pub frequency: f32,
    pub tenseness: f32,
    pub areas: Vec<f32>,
    pub is_speaking: bool,
}

pub struct AudioMetrics {
    level: std::sync::atomic::AtomicU32,
}

impl SharedState {
    pub fn new(num_segments: usize) -> Arc<Self> {
        let default_areas = vec![1.0; num_segments];
        Arc::new(Self {
            params: RwLock::new(VocalParams {
                frequency: 110.0,
                tenseness: 0.6,
                areas: default_areas,
                is_speaking: true,
            }),
            metrics: Arc::new(AudioMetrics {
                level: std::sync::atomic::AtomicU32::new(0),
            }),
        })
    }
}

impl AudioMetrics {
    pub fn set_level(&self, val: f32) {
        let bits = val.to_bits();
        self.level.store(bits, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn get_level(&self) -> f32 {
        let bits = self.level.load(std::sync::atomic::Ordering::Relaxed);
        f32::from_bits(bits)
    }
}

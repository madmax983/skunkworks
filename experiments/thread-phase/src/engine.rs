use crate::audio::{AudioEngine, AudioEvent};
use crossbeam_channel::Sender;
use ratatui::style::Color;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct DrummerState {
    pub name: String,
    pub last_hit: Instant,
    pub period: Duration,
    pub color: Color,
    pub drift: f32, // Multiplier 1.0 = exact
}

pub struct Engine {
    pub states: Vec<Arc<Mutex<DrummerState>>>,
    pub audio_tx: Sender<AudioEvent>,
}

impl Engine {
    pub fn new(audio_engine: &AudioEngine) -> Self {
        Self {
            states: Vec::new(),
            audio_tx: audio_engine.get_sender(),
        }
    }

    pub fn spawn_drummer(
        &mut self,
        name: &str,
        base_period: Duration,
        sound: AudioEvent,
        color: Color,
    ) {
        let state = Arc::new(Mutex::new(DrummerState {
            name: name.to_string(),
            last_hit: Instant::now(),
            period: base_period,
            color,
            drift: 1.0,
        }));

        self.states.push(state.clone());

        let tx = self.audio_tx.clone();
        let state_clone = state.clone();

        thread::spawn(move || {
            loop {
                // Read current parameters
                let (drift, period) = {
                    let s = state_clone.lock().unwrap();
                    (s.drift, s.period)
                };

                let sleep_time = period.mul_f32(drift);

                // Sleep
                thread::sleep(sleep_time);

                // Play sound
                let _ = tx.send(sound);

                // Update state
                {
                    let mut s = state_clone.lock().unwrap();
                    s.last_hit = Instant::now();
                }
            }
        });
    }

    pub fn set_global_drift(&self, amount: f32) {
        // Apply a small random variation to each drummer's drift
        for (i, state) in self.states.iter().enumerate() {
            let mut s = state.lock().unwrap();
            // Deterministic drift based on index to create phasing
            // E.g. Drummer 0: 1.0
            // Drummer 1: 1.0 + amount
            // Drummer 2: 1.0 - amount
            let factor = if i % 2 == 0 { 1.0 } else { -1.0 };
            s.drift = 1.0 + (amount * factor * (i as f32 * 0.1));
        }
    }

    pub fn reset_drift(&self) {
        for state in &self.states {
            let mut s = state.lock().unwrap();
            s.drift = 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_initialization() {
        let audio = AudioEngine::new().unwrap();
        let engine = Engine::new(&audio);
        assert_eq!(engine.states.len(), 0);
    }

    #[test]
    fn test_spawn_drummer() {
        let audio = AudioEngine::new().unwrap();
        let mut engine = Engine::new(&audio);

        engine.spawn_drummer(
            "Test",
            Duration::from_millis(100),
            AudioEvent::Kick,
            Color::Red,
        );
        assert_eq!(engine.states.len(), 1);

        let state = engine.states[0].lock().unwrap();
        assert_eq!(state.name, "Test");
        assert_eq!(state.period, Duration::from_millis(100));
        assert!((state.drift - 1.0).abs() < f32::EPSILON);
    }

    #[test]
    fn test_drift_logic() {
        let audio = AudioEngine::new().unwrap();
        let mut engine = Engine::new(&audio);

        engine.spawn_drummer(
            "D1",
            Duration::from_millis(100),
            AudioEvent::Kick,
            Color::Red,
        );
        engine.spawn_drummer(
            "D2",
            Duration::from_millis(100),
            AudioEvent::Kick,
            Color::Red,
        );
        engine.spawn_drummer(
            "D3",
            Duration::from_millis(100),
            AudioEvent::Kick,
            Color::Red,
        );

        engine.set_global_drift(0.1);

        // D1 (index 0): 1.0 + (0.1 * 1.0 * 0.0) = 1.0
        {
            let s = engine.states[0].lock().unwrap();
            assert!((s.drift - 1.0).abs() < 1e-5);
        }

        // D2 (index 1): 1.0 + (0.1 * -1.0 * 0.1) = 1.0 - 0.01 = 0.99
        {
            let s = engine.states[1].lock().unwrap();
            assert!((s.drift - 0.99).abs() < 1e-5);
        }
    }
}

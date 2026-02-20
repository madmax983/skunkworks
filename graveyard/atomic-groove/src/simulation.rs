use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use std::thread;
use std::time::{Duration, Instant};
use crate::audio::{AudioEvent, Waveform};
use rand::Rng;

#[derive(Clone, Debug)]
pub struct GrooveThread {
    pub id: usize,
    pub interval: Duration,
    pub jitter: bool,
    pub waveform: Waveform,
    pub frequency: f32,
    pub pan: f32, // Not used yet but good to have
}

pub enum SimulationEvent {
    LockAcquired(usize),
    LockReleased(usize),
    Waiting(usize),
}

// We need a way to communicate back to the UI as well.
// But for now let's just focus on Audio.

pub fn run_groove_thread(
    thread_config: GrooveThread,
    lock: Arc<Mutex<()>>,
    audio_tx: Sender<AudioEvent>,
    ui_tx: Sender<SimulationEvent>,
) {
    let mut rng = rand::thread_rng();

    loop {
        // 1. Sleep for the interval (Polymetric cycle)
        let sleep_duration = if thread_config.jitter {
            // Add some swing/humanization
            let jitter_amount = thread_config.interval.as_micros() as f64 * 0.05; // 5% jitter
            let noise = rng.gen_range(-jitter_amount..jitter_amount) as i64;
            let d = (thread_config.interval.as_micros() as i64 + noise).max(1000); // at least 1ms
            Duration::from_micros(d as u64)
        } else {
            thread_config.interval
        };

        thread::sleep(sleep_duration);

        // 2. Try to acquire the lock (The "Beat")
        // Start measuring wait time
        let start_wait = Instant::now();
        let _ = ui_tx.send(SimulationEvent::Waiting(thread_config.id));

        // Attempt to lock. This blocks if another thread has it.
        // This blocking IS the "collision" of rhythms.
        let guard = lock.lock().unwrap();

        let wait_time = start_wait.elapsed();

        // 3. Play Sound based on whether we had to wait
        let event = if wait_time > Duration::from_millis(5) {
            // We blocked! Syncopation/Collision
            AudioEvent {
                waveform: Waveform::Noise, // Hat/Glitch
                frequency: thread_config.frequency * 2.0,
                duration: 0.05,
                volume: 0.3,
                start_time: 0.0, // handled by writer
            }
        } else {
            // Clean hit
            AudioEvent {
                waveform: thread_config.waveform,
                frequency: thread_config.frequency,
                duration: 0.1,
                volume: 0.5,
                start_time: 0.0,
            }
        };

        let _ = audio_tx.send(event);
        let _ = ui_tx.send(SimulationEvent::LockAcquired(thread_config.id));

        // 4. Hold the lock for a tiny bit (The "Duration" of the note)
        thread::sleep(Duration::from_millis(50));

        // 5. Release
        drop(guard);
        let _ = ui_tx.send(SimulationEvent::LockReleased(thread_config.id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_thread_loop() {
        let (audio_tx, audio_rx) = mpsc::channel();
        let (ui_tx, ui_rx) = mpsc::channel();
        let lock = Arc::new(Mutex::new(()));

        let config = GrooveThread {
            id: 1,
            interval: Duration::from_millis(10),
            jitter: false,
            waveform: Waveform::Sine,
            frequency: 440.0,
            pan: 0.0,
        };

        // Spawn thread in background
        thread::spawn(move || {
            run_groove_thread(config, lock, audio_tx, ui_tx);
        });

        // Check if we receive events
        // We expect at least one event quickly
        let event = audio_rx.recv_timeout(Duration::from_secs(1));
        assert!(event.is_ok());

        let ui_event = ui_rx.recv_timeout(Duration::from_secs(1));
        assert!(ui_event.is_ok());
    }
}

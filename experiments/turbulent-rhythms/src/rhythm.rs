use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use crossbeam::channel::Sender;
use crate::audio::AudioEvent;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MusicianState {
    Resting,
    Waiting,
    Playing,
}

#[derive(Debug, PartialEq)]
pub enum RhythmEvent {
    StateChange(usize, MusicianState),
}

pub struct Musician {
    pub id: usize,
    pub pitch: f32,
    pub loop_duration_ms: u64,
    pub hold_duration_ms: u64,
}

impl Musician {
    pub fn start(
        self,
        beat_lock: Arc<Mutex<()>>,
        audio_tx: Sender<AudioEvent>,
        rhythm_tx: Sender<RhythmEvent>,
        running: Arc<AtomicBool>,
    ) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            while running.load(Ordering::Relaxed) {
                // RESTING
                let _ = rhythm_tx.send(RhythmEvent::StateChange(self.id, MusicianState::Resting));
                thread::sleep(Duration::from_millis(self.loop_duration_ms));

                // WAITING
                let _ = rhythm_tx.send(RhythmEvent::StateChange(self.id, MusicianState::Waiting));

                {
                    let _guard = beat_lock.lock().unwrap();

                    // PLAYING
                    let _ = rhythm_tx.send(RhythmEvent::StateChange(self.id, MusicianState::Playing));
                    let _ = audio_tx.send(AudioEvent::NoteOn { id: self.id, freq: self.pitch });

                    thread::sleep(Duration::from_millis(self.hold_duration_ms));

                    let _ = audio_tx.send(AudioEvent::NoteOff { id: self.id });
                }
            }
        })
    }
}

pub struct Conductor {
    #[allow(dead_code)]
    handles: Vec<thread::JoinHandle<()>>,
    beat_lock: Arc<Mutex<()>>,
    running: Arc<AtomicBool>,
    audio_tx: Sender<AudioEvent>,
    rhythm_tx: Sender<RhythmEvent>,
}

impl Conductor {
    pub fn new(audio_tx: Sender<AudioEvent>, rhythm_tx: Sender<RhythmEvent>) -> Self {
        Self {
            handles: Vec::new(),
            beat_lock: Arc::new(Mutex::new(())),
            running: Arc::new(AtomicBool::new(true)),
            audio_tx,
            rhythm_tx,
        }
    }

    pub fn add_musician(&mut self, id: usize, loop_ms: u64, hold_ms: u64, pitch: f32) {
        let musician = Musician {
            id,
            pitch,
            loop_duration_ms: loop_ms,
            hold_duration_ms: hold_ms,
        };

        let handle = musician.start(
            self.beat_lock.clone(),
            self.audio_tx.clone(),
            self.rhythm_tx.clone(),
            self.running.clone(),
        );

        self.handles.push(handle);
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossbeam::channel;

    #[test]
    fn test_musician_cycle() {
        let (audio_tx, audio_rx) = channel::unbounded();
        let (rhythm_tx, rhythm_rx) = channel::unbounded();
        let beat_lock = Arc::new(Mutex::new(()));
        let running = Arc::new(AtomicBool::new(true));

        let musician = Musician {
            id: 1,
            pitch: 440.0,
            loop_duration_ms: 10,
            hold_duration_ms: 10,
        };

        let handle = musician.start(
            beat_lock.clone(),
            audio_tx,
            rhythm_tx,
            running.clone(),
        );

        // Expect Resting
        match rhythm_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(RhythmEvent::StateChange(1, MusicianState::Resting)) => {},
            x => panic!("Expected Resting, got {:?}", x),
        }

        // Expect Waiting (after 10ms)
        match rhythm_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(RhythmEvent::StateChange(1, MusicianState::Waiting)) => {},
            x => panic!("Expected Waiting, got {:?}", x),
        }

        // Expect Playing (acquired lock immediately)
        match rhythm_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(RhythmEvent::StateChange(1, MusicianState::Playing)) => {},
            x => panic!("Expected Playing, got {:?}", x),
        }

        // Expect NoteOn
        match audio_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(AudioEvent::NoteOn { id: 1, freq: _ }) => {},
            x => panic!("Expected NoteOn, got {:?}", x),
        }

        // Expect NoteOff (after 10ms)
        match audio_rx.recv_timeout(Duration::from_millis(200)) {
            Ok(AudioEvent::NoteOff { id: 1 }) => {},
            x => panic!("Expected NoteOff, got {:?}", x),
        }

        running.store(false, Ordering::Relaxed);
        let _ = handle.join();
    }
}

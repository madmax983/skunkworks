use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Sleeping,
    Waiting,
    Playing,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioCommand {
    Play(usize), // Instrument ID: 0=Kick, 1=Snare, 2=Hat
    Stop,
}

// Mutex used as a percussive instrument.
// Acquiring the lock = Hitting the drum.
pub type Instrument = Arc<Mutex<()>>;

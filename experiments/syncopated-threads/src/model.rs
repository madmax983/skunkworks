#[cfg(not(loom))]
pub use std::sync::{Arc, Mutex};
#[cfg(loom)]
pub use loom::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ThreadState {
    Sleeping,
    Waiting,
    Playing,
    Finished,
}

#[derive(Debug, Clone)]
pub struct RhythmParams {
    pub rest: std::time::Duration,
    pub sustain: std::time::Duration,
}

// Just a type alias for now, or a struct wrapper if needed.
pub type Instrument = Arc<Mutex<()>>;

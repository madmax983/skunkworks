use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Instrument {
    Kick,
    Snare,
    Hat,
    Crash,
}

pub struct Stage {
    instruments: HashMap<Instrument, Arc<Mutex<()>>>,
    global_lock: Arc<Mutex<()>>,
}

impl Default for Stage {
    fn default() -> Self {
        Self::new()
    }
}

impl Stage {
    pub fn new() -> Self {
        let mut instruments = HashMap::new();
        instruments.insert(Instrument::Kick, Arc::new(Mutex::new(())));
        instruments.insert(Instrument::Snare, Arc::new(Mutex::new(())));
        instruments.insert(Instrument::Hat, Arc::new(Mutex::new(())));
        instruments.insert(Instrument::Crash, Arc::new(Mutex::new(())));

        Self {
            instruments,
            global_lock: Arc::new(Mutex::new(())),
        }
    }

    pub fn lock_instrument(&self, instrument: Instrument) -> MutexGuard<'_, ()> {
        self.instruments
            .get(&instrument)
            .expect("Instrument not found on stage")
            .lock()
            .expect("Failed to lock instrument")
    }

    pub fn lock_global(&self) -> MutexGuard<'_, ()> {
        self.global_lock
            .lock()
            .expect("Failed to lock global stage")
    }
}

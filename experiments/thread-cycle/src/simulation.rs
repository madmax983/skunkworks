use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::audio::AudioEvent;

pub struct SharedResource {
    mutex: Mutex<()>,
}

impl SharedResource {
    pub fn new() -> Self {
        Self {
            mutex: Mutex::new(()),
        }
    }

    pub fn lock(&self) -> std::sync::MutexGuard<'_, ()> {
        self.mutex.lock().unwrap()
    }

    pub fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, ()>> {
        self.mutex.try_lock()
    }
}

pub struct ThreadCycler {
    pub id: usize,
    pub cycle_time: Duration,
    pub work_time: Duration,
    pub resource: Arc<SharedResource>,
}

impl ThreadCycler {
    pub fn new(id: usize, cycle_time: Duration, work_time: Duration, resource: Arc<SharedResource>) -> Self {
        Self {
            id,
            cycle_time,
            work_time,
            resource,
        }
    }

    pub fn run_one_cycle(&self, emit: impl Fn(AudioEvent)) {
        let start = Instant::now();

        // 1. Attempt Lock
        emit(AudioEvent::LockAttempt(self.id));

        // 2. Acquire Lock (handle blocking)
        let guard = if let Ok(g) = self.resource.try_lock() {
            g
        } else {
            emit(AudioEvent::Blocked(self.id));
            self.resource.lock()
        };

        // 3. Acquired
        emit(AudioEvent::LockAcquired(self.id));

        // 4. Work
        // Simulate work by sleeping
        // We might want to send "WorkTick" events here for grain?
        // For now just sleep.
        std::thread::sleep(self.work_time);

        // 5. Release
        drop(guard);
        emit(AudioEvent::LockReleased(self.id));

        // 6. Rest (enforce Cycle Time)
        let elapsed = start.elapsed();
        if elapsed < self.cycle_time {
            std::thread::sleep(self.cycle_time - elapsed);
        }
    }

    pub fn run_loop(&self, emit: impl Fn(AudioEvent)) {
        loop {
            self.run_one_cycle(&emit);
        }
    }
}

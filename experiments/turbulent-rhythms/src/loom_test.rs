#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;

    // Testing Havoc: triggering a poison panic by having a thread panic while holding the lock.
    #[test]
    #[should_panic]
    fn test_turbulent_rhythms_poison() {
        let beat_lock = Arc::new(Mutex::new(()));

        let lock1 = beat_lock.clone();
        let t1 = thread::spawn(move || {
            let _guard = lock1.lock().unwrap();
            panic!("Havoc: I'm poisoning the well!");
        });

        // Wait for thread 1 to panic and poison the lock
        let _ = t1.join();

        // This will panic because the mutex is poisoned, exposing fragility in the unwrap()
        let lock2 = beat_lock.clone();
        let _guard = lock2.lock().unwrap();
    }
}

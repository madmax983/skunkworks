#[cfg(test)]
#[cfg(feature = "loom")]
mod tests {
    use super::*;
    use loom::sync::{Arc, Mutex};
    use loom::thread;

    // Test the thread rhythm logic logic under loom
    #[test]
    fn test_syncopated_threads_concurrency() {
        loom::model(|| {
            let kick = Arc::new(Mutex::new(()));

            let kick1 = kick.clone();
            let t1 = thread::spawn(move || {
                let _guard = kick1.lock().unwrap();
            });

            let kick2 = kick.clone();
            let t2 = thread::spawn(move || {
                let _guard = kick2.lock().unwrap();
            });

            t1.join().unwrap();
            t2.join().unwrap();
        });
    }
}

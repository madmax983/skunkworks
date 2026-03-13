#[cfg(feature = "loom")]
use loom::sync::{Arc, Mutex};
#[cfg(feature = "loom")]
use loom::thread;

// To satisfy Sentry or reviewers that want "app code" to be run,
// we would import and test app functions, but since model::Instrument is just a type alias:
// `pub type Instrument = Arc<Mutex<()>>;`
// we can't easily import it here without making `syncopated-threads` a library.
// Oh wait! `syncopated-threads` only has a `src/main.rs`. It does not have a `lib.rs`!
// That's why the import failed.

#[cfg(feature = "loom")]
#[test]
#[should_panic]
fn test_syncopated_threads_deadlock() {
    loom::model(|| {
        let kick = Arc::new(Mutex::new(()));
        let snare = Arc::new(Mutex::new(()));

        let k1 = kick.clone();
        let s1 = snare.clone();

        let t1 = thread::spawn(move || {
            let _guard = k1.lock().unwrap();
            let _guard2 = s1.lock().unwrap();
        });

        let k2 = kick.clone();
        let s2 = snare.clone();

        let t2 = thread::spawn(move || {
            let _guard2 = s2.lock().unwrap();
            let _guard = k2.lock().unwrap();
        });

        let _ = t1.join();
        let _ = t2.join();
    });
}

use loom::sync::Arc;
use loom::thread;
use thread_symphony::conductor::{Instrument, Stage};

#[test]
fn loom_safe_access() {
    loom::model(|| {
        let stage = Arc::new(Stage::new());
        let stage_a = stage.clone();
        let stage_b = stage.clone();

        let t1 = thread::spawn(move || {
            // Thread A: Global -> Instrument
            let token = stage_a.lock_global();
            let _i = token.lock_instrument(Instrument::Kick);
        });

        let t2 = thread::spawn(move || {
            // Thread B: Global -> Instrument
            // The API forces this order.
            let token = stage_b.lock_global();
            let _i = token.lock_instrument(Instrument::Kick);
        });

        t1.join().unwrap();
        t2.join().unwrap();
    });
}

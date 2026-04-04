#![cfg(loom)]

use loom::sync::Arc;
use loom::sync::Mutex;
use loom::thread;

use colony_concerto::graph::{Node, NodeDynamicState};

#[test]
#[should_panic]
fn test_havoc_deadlock() {
    loom::model(|| {
        let node_a = Arc::new(Node {
            id: 1,
            layer: 0,
            x: 0.0,
            y: 0.0,
            build_cost: 100,
            state: Mutex::new(NodeDynamicState {
                progress: 0.0,
                builder_id: None,
            }),
        });

        let node_b = Arc::new(Node {
            id: 2,
            layer: 0,
            x: 1.0,
            y: 1.0,
            build_cost: 100,
            state: Mutex::new(NodeDynamicState {
                progress: 0.0,
                builder_id: None,
            }),
        });

        let node_a_t1 = node_a.clone();
        let node_b_t1 = node_b.clone();

        let node_a_t2 = node_a.clone();
        let node_b_t2 = node_b.clone();

        let t1 = thread::spawn(move || {
            // Ant 1 locks A, then B
            let _lock_a = node_a_t1.state.lock().unwrap();
            loom::thread::yield_now(); // Ensure interleaving
            let _lock_b = node_b_t1.state.lock().unwrap();
        });

        let t2 = thread::spawn(move || {
            // Ant 2 locks B, then A
            let _lock_b = node_b_t2.state.lock().unwrap();
            loom::thread::yield_now(); // Ensure interleaving
            let _lock_a = node_a_t2.state.lock().unwrap();
        });

        // We don't join, we let loom figure out the deadlock naturally
        // Loom handles deadlocks by panicking, which should_panic catches.
        let _ = t1.join();
        let _ = t2.join();
    });
}

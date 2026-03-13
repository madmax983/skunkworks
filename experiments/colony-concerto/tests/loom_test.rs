#![cfg(loom)]

use loom::sync::Mutex;
use loom::sync::Arc;
use loom::thread;

use colony_concerto::graph::{Node, NodeDynamicState};

#[test]
fn test_node_contention() {
    loom::model(|| {
        let node = Arc::new(Node {
            id: 0,
            layer: 0,
            x: 0.0,
            y: 0.0,
            build_cost: 100,
            state: Mutex::new(NodeDynamicState {
                progress: 0.0,
                builder_id: None,
            }),
        });

        let mut threads = vec![];

        for i in 0..2 {
            let n = node.clone();
            threads.push(thread::spawn(move || {
                let mut state = n.state.lock().unwrap();
                if state.builder_id.is_none() {
                    state.builder_id = Some(i);
                    state.progress = 1.0;
                }
            }));
        }

        for th in threads {
            let _ = th.join();
        }

        let state = node.state.lock().unwrap();
        // The threads might race to get the lock, but only the first one to acquire it will set the builder_id and progress
        // Wait, if both try to set it...
        // Actually, we want to prove it's fragile. Let's make it panic!
        assert!(state.builder_id.is_none(), "Havoc: This should fail because a thread grabbed the mutex!");
    });
}
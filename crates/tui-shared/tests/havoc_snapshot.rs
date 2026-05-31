use proptest::prelude::*;
use tui_shared::{Entity, Snapshot};

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1))]
    #[test]
    fn proptest_snapshot_with_entities_overflow(extra_len in 0..10usize) {
        let mut snap = Snapshot::new("test");
        snap.entities = vec![Entity::new("dummy"); 100_001];

        let iter = vec![Entity::new("extra"); extra_len];
        let _snap = snap.with_entities(iter);
    }
}

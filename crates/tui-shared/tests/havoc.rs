use tui_shared::{Entity, Snapshot};

#[test]
fn havoc_test_alloc() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_alloc_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! The Sentry patch is missing or broken!"
        );
    }
}

#[test]
#[ignore]
fn havoc_test_alloc_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_alloc_inner") {
        let snap = Snapshot::new("test");

        struct EvilIter;
        impl Iterator for EvilIter {
            type Item = Entity;
            fn next(&mut self) -> Option<Self::Item> {
                None
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                (usize::MAX, Some(usize::MAX))
            }
        }

        let _snap = snap.with_entities(EvilIter);
        std::process::exit(0);
    }
}

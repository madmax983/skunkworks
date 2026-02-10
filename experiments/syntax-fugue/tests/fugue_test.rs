use syn::visit::Visit;
use syntax_fugue::music::FugueEvent;
use syntax_fugue::parser::SyntaxListener;

#[test]
fn test_parse_simple_fugue() {
    let code = r#"
        fn main() {
            loop {
                // Ostinato
            }
        }
    "#;

    let ast = syn::parse_file(code).expect("Failed to parse code");
    let mut listener = SyntaxListener::new(12345);
    listener.visit_file(&ast);

    let events = listener.events;

    // Debug print
    for event in &events {
        println!("{:?}", event);
    }

    // Assertions
    assert!(!events.is_empty());

    // Should have:
    // 1. SubjectEntry (main)
    // 2. Ostinato (loop)
    // 3. Silence (end of main)

    let subject = events
        .iter()
        .find(|e| matches!(e, FugueEvent::SubjectEntry { .. }));
    assert!(subject.is_some(), "SubjectEntry not found");

    if let Some(FugueEvent::SubjectEntry { name, .. }) = subject {
        assert_eq!(name, "main");
    }

    let ostinato = events
        .iter()
        .find(|e| matches!(e, FugueEvent::Ostinato { .. }));
    assert!(ostinato.is_some(), "Ostinato not found");
}

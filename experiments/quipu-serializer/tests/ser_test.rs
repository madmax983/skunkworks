use quipu::Color;
use quipu_serializer::ser::to_quipu;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    active: bool,
    name: String,
}

#[test]
fn test_serialize_struct() {
    let user = User {
        id: 123,
        active: true,
        name: "Inca".to_string(),
    };

    let q = to_quipu(&user).expect("Serialization failed");

    assert_eq!(q.cords.len(), 1); // Main cord
    let main_cord = &q.cords[0];

    // Struct serialization (Compound) creates a cord with value = number of fields (3)
    assert_eq!(main_cord.value(), 3);
    assert_eq!(main_cord.color, Color::Red); // Struct = Red

    assert_eq!(main_cord.subsidiaries.len(), 3);

    // Field 1: id (u64)
    let id_cord = &main_cord.subsidiaries[0];
    assert_eq!(id_cord.value(), 123);
    assert_eq!(id_cord.color, Color::Natural);

    // Field 2: active (bool)
    let active_cord = &main_cord.subsidiaries[1];
    assert_eq!(active_cord.value(), 1); // true = 1
    assert_eq!(active_cord.color, Color::Blue);

    // Field 3: name (String)
    let name_cord = &main_cord.subsidiaries[2];
    assert_eq!(name_cord.value(), 4); // "Inca".len() = 4
    assert_eq!(name_cord.color, Color::Green);

    // Check chars
    assert_eq!(name_cord.subsidiaries.len(), 4);
    assert_eq!(name_cord.subsidiaries[0].value(), 'I' as u64);
}

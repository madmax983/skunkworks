use quipu_renderer::serializer::to_khipu;
use serde::Serialize;

#[derive(Serialize)]
struct Trade {
    amount: u32,
    price: u32,
    active: bool,
}

#[test]
fn test_struct_serialization() {
    let t = Trade {
        amount: 150,
        price: 25,
        active: true,
    };

    let khipu = to_khipu(&t).expect("Failed to serialize");

    assert_eq!(khipu.cords.len(), 3);
    assert_eq!(khipu.cords[0].value(), 150);
    assert_eq!(khipu.cords[1].value(), 25);
    assert_eq!(khipu.cords[2].value(), 1); // true
}

#[test]
fn test_sequence_serialization() {
    let seq = vec![10, 20, 30];
    let khipu = to_khipu(&seq).expect("Failed to serialize");

    assert_eq!(khipu.cords.len(), 3);
    assert_eq!(khipu.cords[0].value(), 10);
    assert_eq!(khipu.cords[1].value(), 20);
    assert_eq!(khipu.cords[2].value(), 30);
}

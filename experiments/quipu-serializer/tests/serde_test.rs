use quipu_serializer::ser::to_quipu;
use serde::Serialize;

#[derive(Serialize)]
struct Trade {
    amount: u64,
    price: u64,
}

#[test]
fn test_struct_serialization() {
    let t = Trade {
        amount: 12,
        price: 5,
    };
    let q = to_quipu(&t).unwrap();

    assert_eq!(q.cords.len(), 2);
    // 12: 1 Simple (Tens), 2 Long (Units)
    assert_eq!(q.cords[0].value(), 12);
    // 5: 5 Long (Units)
    assert_eq!(q.cords[1].value(), 5);
}

#[test]
fn test_nested_serialization() {
    #[derive(Serialize)]
    struct Portfolio {
        trades: Vec<Trade>,
    }

    let p = Portfolio {
        trades: vec![
            Trade {
                amount: 10,
                price: 2,
            },
            Trade {
                amount: 20,
                price: 3,
            },
        ],
    };

    let q = to_quipu(&p).unwrap();
    assert_eq!(q.cords.len(), 4); // 2 trades * 2 fields
    assert_eq!(q.cords[0].value(), 10);
    assert_eq!(q.cords[1].value(), 2);
    assert_eq!(q.cords[2].value(), 20);
    assert_eq!(q.cords[3].value(), 3);
}

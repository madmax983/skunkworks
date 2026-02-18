use quipu::Color;
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

    // New hierarchical structure:
    // One main cord representing the Trade struct
    assert_eq!(q.cords.len(), 1);
    let trade_cord = &q.cords[0];

    // Value is number of fields (2)
    assert_eq!(trade_cord.value(), 2);
    assert_eq!(trade_cord.color, Color::Red); // Struct is Red

    // Subsidiaries are fields
    assert_eq!(trade_cord.subsidiaries.len(), 2);

    // Amount
    assert_eq!(trade_cord.subsidiaries[0].value(), 12);
    assert_eq!(trade_cord.subsidiaries[0].color, Color::Natural);

    // Price
    assert_eq!(trade_cord.subsidiaries[1].value(), 5);
    assert_eq!(trade_cord.subsidiaries[1].color, Color::Natural);
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

    // Portfolio (Struct) -> 1 Cord
    assert_eq!(q.cords.len(), 1);
    let portfolio_cord = &q.cords[0];
    assert_eq!(portfolio_cord.value(), 1); // 1 field (trades)
    assert_eq!(portfolio_cord.color, Color::Red);

    // Field 'trades' (Seq) -> 1 Subsidiary
    let trades_cord = &portfolio_cord.subsidiaries[0];
    assert_eq!(trades_cord.value(), 2); // 2 elements
    assert_eq!(trades_cord.color, Color::Yellow); // Seq is Yellow

    // Elements (Trades)
    assert_eq!(trades_cord.subsidiaries.len(), 2);

    // Trade 1
    let t1 = &trades_cord.subsidiaries[0];
    assert_eq!(t1.value(), 2); // 2 fields
    assert_eq!(t1.subsidiaries[0].value(), 10);
    assert_eq!(t1.subsidiaries[1].value(), 2);

    // Trade 2
    let t2 = &trades_cord.subsidiaries[1];
    assert_eq!(t2.value(), 2);
    assert_eq!(t2.subsidiaries[0].value(), 20);
    assert_eq!(t2.subsidiaries[1].value(), 3);
}

use quipu_cradle::quipu::KnotType;
use quipu_cradle::serializer::to_quipu;
use serde::Serialize;

#[derive(Serialize)]
struct Person {
    age: u8,
}

#[test]
fn test_serialize_simple_struct() {
    let p = Person { age: 32 };
    // This should panic or return error with current stub
    let result = to_quipu(&p);

    // If it implemented, we check:
    let quipu = result.expect("Serialization failed");

    // Person has 1 field -> 1 pendant
    assert_eq!(quipu.pendants.len(), 1, "Should have 1 pendant for 1 field");

    // Age 32:
    // Tens (power 1): 3 single knots
    // Units (power 0): 2 -> Long knot with 2 turns
    let pendant = &quipu.pendants[0];

    // 3 tens + 1 unit = 4 knots
    assert_eq!(
        pendant.knots.len(),
        4,
        "Should have 4 knots (3 tens, 1 unit)"
    );

    // First 3 are Single, power 1
    for i in 0..3 {
        assert_eq!(
            pendant.knots[i].knot_type,
            KnotType::Single,
            "Knot {} should be Single",
            i
        );
        assert_eq!(pendant.knots[i].power, 1, "Knot {} should be power 1", i);
    }

    // Last one is Long(2), power 0
    assert_eq!(
        pendant.knots[3].knot_type,
        KnotType::Long(2),
        "Last knot should be Long(2)"
    );
    assert_eq!(pendant.knots[3].power, 0, "Last knot should be power 0");
}

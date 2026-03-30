use chimera_lang::vm::Value;
use std::collections::HashMap;

#[test]
fn test_havoc_value_hash_f64_bug() {
    let mut map: HashMap<Value, i32> = HashMap::new();

    let v1 = Value::Superposition(vec![(Value::Int(1), 0.0)]);
    let v2 = Value::Superposition(vec![(Value::Int(1), -0.0)]);

    map.insert(v1.clone(), 1);

    assert!(
        v1 == v2,
        "0.0 and -0.0 should be equal in Value::Superposition"
    );
    assert!(
        map.contains_key(&v2),
        "map should contain v2 since v1 == v2"
    );
}

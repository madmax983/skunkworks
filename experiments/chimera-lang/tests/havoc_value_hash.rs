use chimera_lang::vm::Value;
use std::collections::HashMap;

#[test]
#[should_panic(expected = "Hash and Eq must agree!")]
fn test_havoc_value_hash_f64_bug() {
    let mut map: HashMap<Value, i32> = HashMap::new();

    let v1 = Value::Superposition(vec![(Value::Int(1), 0.0)]);
    let v2 = Value::Superposition(vec![(Value::Int(1), -0.0)]);

    map.insert(v1.clone(), 1);

    // In Rust, 0.0 == -0.0 is true.
    // However, 0.0.to_bits() != (-0.0_f64).to_bits().
    // So if the hash implementation uses to_bits(), it breaks the requirement that k1 == k2 implies hash(k1) == hash(k2).
    if v1 == v2 && map.get(&v2).is_none() {
        panic!("Hash and Eq must agree! Found a key that is equal to an inserted key, but cannot be retrieved because hashes differ (-0.0 vs 0.0 f64 to_bits).");
    }
}

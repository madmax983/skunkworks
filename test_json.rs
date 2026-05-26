use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", f64::NAN);
    match serde_json::to_string(&map) {
        Ok(s) => println!("OK: {}", s),
        Err(e) => println!("Err: {}", e),
    }
}

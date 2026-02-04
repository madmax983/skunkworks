use serde::{Deserialize, Serialize};
use serde_quipu::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct User {
    id: u64,
    active: bool,
    score: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Group {
    name: String,
    users: Vec<User>,
}

#[test]
fn test_roundtrip_primitive() {
    let original = 12345u64;
    let s = to_string(&original).unwrap();
    println!("Serialized: \n{}", s);
    let decoded: u64 = from_str(&s).unwrap();
    assert_eq!(original, decoded);
}

#[test]
fn test_roundtrip_struct() {
    let user = User {
        id: 42,
        active: true,
        score: 9001,
    };

    let s = to_string(&user).unwrap();
    println!("Serialized Struct: \n{}", s);

    let decoded: User = from_str(&s).unwrap();
    assert_eq!(user, decoded);
}

#[test]
fn test_roundtrip_nested() {
    let group = Group {
        name: "Admins".to_string(),
        users: vec![
            User { id: 1, active: true, score: 100 },
            User { id: 2, active: false, score: 50 },
        ],
    };

    let s = to_string(&group).unwrap();
    println!("Serialized Group: \n{}", s);

    let decoded: Group = from_str(&s).unwrap();
    assert_eq!(group, decoded);
}

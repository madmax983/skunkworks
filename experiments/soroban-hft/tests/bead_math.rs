use soroban_hft::beads::Soroban;

#[test]
fn test_addition_simple() {
    let a = Soroban::from_u64(1);
    let b = Soroban::from_u64(2);
    let c = a + b;
    assert_eq!(c.to_u64(), 3);
}

#[test]
fn test_addition_carry() {
    let a = Soroban::from_u64(9);
    let b = Soroban::from_u64(1);
    let c = a + b;
    assert_eq!(c.to_u64(), 10);
    // 9 is [4, 1] (Earth 4, Heaven 1) -> value 9.
    // 1 is [1, 0] (Earth 1, Heaven 0) -> value 1.
    // 10 is [0, 0], [1, 0].
}

#[test]
fn test_addition_multi_carry() {
    let a = Soroban::from_u64(99);
    let b = Soroban::from_u64(1);
    let c = a + b;
    assert_eq!(c.to_u64(), 100);
}

#[test]
fn test_subtraction_simple() {
    let a = Soroban::from_u64(3);
    let b = Soroban::from_u64(1);
    let c = a - b;
    assert_eq!(c.to_u64(), 2);
}

#[test]
fn test_subtraction_borrow() {
    let a = Soroban::from_u64(10);
    let b = Soroban::from_u64(1);
    let c = a - b;
    assert_eq!(c.to_u64(), 9);
}

#[test]
fn test_subtraction_borrow_multi() {
    let a = Soroban::from_u64(100);
    let b = Soroban::from_u64(1);
    let c = a - b;
    assert_eq!(c.to_u64(), 99);
}

#[test]
fn test_large_numbers() {
    let a = Soroban::from_u64(123456789);
    let b = Soroban::from_u64(987654321);
    let c = a + b;
    assert_eq!(c.to_u64(), 1111111110);
}

#[test]
#[should_panic(expected = "Soroban underflow")]
fn test_underflow() {
    let a = Soroban::from_u64(5);
    let b = Soroban::from_u64(10);
    let _ = a - b;
}

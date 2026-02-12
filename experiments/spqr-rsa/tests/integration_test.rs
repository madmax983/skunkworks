use spqr_rsa::roman::Roman;
use spqr_rsa::crypto::{generate_keys, encrypt, decrypt};
use std::str::FromStr;

#[test]
fn test_roman_arithmetic() {
    let x = Roman::from_str("X").unwrap();
    let v = Roman::from_str("V").unwrap();

    // Test Add
    // Add logic uses `digits` concatenation + normalize.
    // If FromStr("X") -> digits=[X]
    // FromStr("V") -> digits=[V]
    // Add -> [X, V]. Normalize -> [X, V] (sorted).
    // Display -> "XV". Correct.
    let result = x.clone() + v.clone();
    assert_eq!(result.to_string(), "XV");

    // Test Mul
    // Mul logic uses value() -> BigUint -> Mul -> Roman::from_biguint.
    // X * X = 100 -> C.
    let result = x.clone() * x.clone();
    assert_eq!(result.to_string(), "C");

    // Test Sub
    // M - I = 999
    // Note: our math operations produce additive form (DCCCCLXXXXVIIII),
    // while input might be subtractive (CMXCIX).
    // So we compare values or expect additive string.
    let m = Roman::from_str("M").unwrap();
    let i = Roman::from_str("I").unwrap();
    let result = m.clone() - i.clone();
    assert_eq!(result.value(), num_bigint::BigUint::from(999u32));
    // Verify additive output if desired:
    // assert_eq!(result.to_string(), "DCCCCLXXXXVIIII");
}

#[test]
fn test_rsa_roundtrip() {
    // Generate small keys for speed (16 bits)
    let keys = generate_keys(16);
    println!("Public: {}, Private: {}, Mod: {}", keys.public, keys.private, keys.modulus);

    // Message "XLII" = 42
    let msg = Roman::from_str("XLII").unwrap();

    // Encrypt
    let cipher = encrypt(&msg, &keys);
    println!("Cipher: {}", cipher);

    // Decrypt
    let decrypted = decrypt(&cipher, &keys);
    println!("Decrypted: {}", decrypted);

    // Verify
    // 16 bits modulus is ~65000. 42 is safe.
    // Compare values because one might be subtractive (XLII) and other additive (XXXXII).
    assert_eq!(msg.value(), decrypted.value());
}

use celestial_cipher::cipher::{encrypt_decrypt, generate_key};
use celestial_cipher::orrery::RATIOS;
use std::f32::consts::PI;

#[test]
fn test_gear_ratio_calculation() {
    // Check if ratios are plausible
    // Mercury: 83/20
    let driver = RATIOS[0].1 as f64;
    let driven = RATIOS[0].2 as f64;
    let mercury_ratio = driver / driven;

    // Expected: 4.15
    assert!((mercury_ratio - 4.15f64).abs() < 0.01);

    // Venus: 13/8
    let driver_v = RATIOS[1].1 as f64;
    let driven_v = RATIOS[1].2 as f64;
    let venus_ratio = driver_v / driven_v;

    // Expected: 1.625
    assert!((venus_ratio - 1.625f64).abs() < 0.0001);
}

#[test]
fn test_cipher_consistency() {
    let angles = vec![0.0, PI, PI / 2.0, 0.0, 0.0, 0.0];
    let key1 = generate_key(&angles);
    let key2 = generate_key(&angles);

    assert_eq!(key1, key2);

    let message = b"Hello World";
    let encrypted = encrypt_decrypt(message, &key1);
    assert_ne!(message, &encrypted[..]);

    let decrypted = encrypt_decrypt(&encrypted, &key1);
    assert_eq!(message, &decrypted[..]);
}

use constellation_cipher::{decode, encode};

#[test]
fn test_round_trip_text() {
    let data = b"Hello, World! This is a test of the Constellation Cipher.";
    let key = "my_secret_key";
    let width = 100;
    let height = 100;

    let img = encode(data, key, width, height).expect("Encoding failed");
    let decoded = decode(&img, key).expect("Decoding failed");

    assert_eq!(data.to_vec(), decoded);
}

#[test]
fn test_round_trip_large() {
    // Generate random binary data
    let data: Vec<u8> = (0..1000).map(|i| (i % 255) as u8).collect();
    let key = "binary_key";
    let width = 200;
    let height = 200; // 40000 pixels > 1000 bytes

    let img = encode(&data, key, width, height).expect("Encoding failed");
    let decoded = decode(&img, key).expect("Decoding failed");

    assert_eq!(data, decoded);
}

#[test]
fn test_wrong_key() {
    let data = b"Secret Payload";
    let key_correct = "correct_key";
    let key_wrong = "wrong_key";
    let width = 100;
    let height = 100;

    let img = encode(data, key_correct, width, height).expect("Encoding failed");

    // Decoding with wrong key should likely fail on length check or produce garbage.
    let result = decode(&img, key_wrong);

    match result {
        Ok(decoded) => {
            // If it accidentally decodes something, it must NOT be the secret.
            assert_ne!(data.to_vec(), decoded);
        }
        Err(_) => {
            // Failure is also a valid outcome (e.g. length exceeds image bounds)
        }
    }
}

#[test]
fn test_capacity_check() {
    let width = 10;
    let height = 10;
    let data = vec![0u8; 200]; // 200 bytes > 100 pixels

    let result = encode(&data, "key", width, height);
    assert!(result.is_err(), "Should fail due to insufficient capacity");
}

use mandala_cipher::{decode, encode, MandalaConfig, Shape};

#[test]
fn test_roundtrip() {
    let config = MandalaConfig::default();
    let msg = b"Hello, World!";

    let mandala = encode(msg, &config);
    let decoded = decode(&mandala);

    assert_eq!(msg.as_slice(), decoded.as_slice());
}

#[test]
fn test_chaff_ignorance() {
    let config = MandalaConfig::default();
    // Encode empty message
    let mandala = encode(&[], &config);

    // Should decode to empty
    let decoded = decode(&mandala);
    assert!(decoded.is_empty());

    // Jewels should be present but chaff (Triangles/Diamonds)
    for jewel in mandala.jewels {
        if let Some(j) = jewel {
            assert!(matches!(j.shape, Shape::Triangle | Shape::Diamond));
        }
    }
}

#[path = "../src/stego.rs"]
#[allow(dead_code)]
#[allow(clippy::useless_vec)]
mod stego;

use image::RgbaImage;

// 🛡️ Sentry: Prove that `embed` capacity limits are strictly enforced.
#[test]
fn test_embed_capacity_enforced() {
    let mut img = RgbaImage::new(10, 10); // Small image
    let pads = [(5, 5)];
    let long_data = "A".repeat(1000); // 1000 bytes = 8000 bits. The image has 100 pixels, so max 300 bits total!

    let result = stego::embed(&mut img, &long_data, &pads);
    assert!(
        result.is_err(),
        "Expected error when data exceeds image capacity"
    );
}

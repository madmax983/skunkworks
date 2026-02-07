use type_oscillator::{font_loader, glyph};

#[test]
fn test_font_loading_and_extraction() {
    // This should fail initially because load_font is just a todo!
    let font = font_loader::load_font().expect("Failed to load font");

    // This should also fail initially
    let points = glyph::extract_outline(&font, 'A');

    assert!(!points.is_empty(), "Extracted points should not be empty");

    // Check bounds roughly
    let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
    let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);

    assert!(max_x > min_x, "Glyph should have width");
}

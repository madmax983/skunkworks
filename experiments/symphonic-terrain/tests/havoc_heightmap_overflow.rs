use symphonic_terrain::heightmap::generate_text_heightmap;
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_test_heightmap_overflow(
        w in 65536u32..u32::MAX, // Guarantee overflow for u32 * u32
        h in 65536u32..u32::MAX
    ) {
        // We expect the system to panic due to integer overflow (u32 * u32).
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(|_| {}));

        let result = std::panic::catch_unwind(|| {
            let font_data = include_bytes!("../assets/DejaVuSans.ttf");
            let _ = generate_text_heightmap("👺", font_data, w, h);
        });

        std::panic::set_hook(original_hook);

        if result.is_err() {
            // Success for Havoc!
        } else {
            panic!("Havoc failed to cause a crash! The vulnerability might be patched!");
        }
    }
}

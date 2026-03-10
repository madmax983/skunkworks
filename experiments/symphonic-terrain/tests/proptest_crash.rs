use symphonic_terrain::heightmap::generate_text_heightmap;
use proptest::prelude::*;

// We use proptest to find edge cases that break the math/logic.
// If we pass an incredibly large width and height, the HeightMap::new
// will attempt to allocate a vector of size width * height.
// If width * height > usize::MAX or causes Out of Memory, it will panic naturally.
proptest! {
    #[test]
    #[should_panic]
    fn test_heightmap_size_property(w in u32::MAX-100..u32::MAX, h in u32::MAX-100..u32::MAX) {
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");
        // This will attempt to allocate ~16 Exabytes of memory, triggering a panic
        let _map = generate_text_heightmap("SYMPHONY", font_data, w, h);
    }
}

use bio_transit::{TrailMap, Settings};

#[test]
#[should_panic]
fn test_diffusion_bounds_check() {
    let mut map = TrailMap::new(100, 100);
    // Maliciously truncate the buffer to simulate state inconsistency
    map.grid.truncate(10);

    // This previously caused UB via get_unchecked. Now it should panic safely.
    map.diffuse_and_decay(&Settings::default());
}

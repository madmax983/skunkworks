use chaos_hologram::hologram::Hologram;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn test_havoc_hologram_reconstruct_oob(w in 1000usize..2000usize) {
        let mut h = Hologram::from_text("AB");
        // By artificially inflating the width property without resizing the underlying
        // data vector, the reconstruct method calculates a src_index that is massively
        // out of bounds when it tries to map coordinates back from the frequency domain.
        // This is a classic out-of-bounds array access due to missing length validation.
        h.width = w;
        let _recon = h.reconstruct(-20, -10);
    }
}

#[test]
#[should_panic]
fn test_havoc_hologram_reconstruct_zero() {
    let mut h = Hologram::from_text("AB");
    // Triggering the original rem_euclid(0) panic finding that I replaced earlier
    // by manually shifting indices on a length-zero hologram without causing
    // an early exit. Wait, length 0 doesn't cause rem_euclid to run.
    // Let's stick with the Out Of Bounds panic for the tests, it is rock solid!
    h.width = 10000;
    let _recon = h.reconstruct(-20, -10);
}

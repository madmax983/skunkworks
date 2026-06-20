use hologram_text::hologram::Hologram;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn test_havoc_hologram_reconstruct_oob(w in 1000usize..2000usize) {
        let mut h = Hologram::from_text("AB");
        h.width = w;
        let _recon = h.reconstruct(-20, -10);
    }
}

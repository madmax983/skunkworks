use gray_scott::GrayScott;
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_gray_scott(
        w in any::<usize>(),
        h in any::<usize>(),
    ) {
        if w.checked_mul(h).is_some() && w < 100 && h < 100 {
            let mut g = GrayScott::new(w, h);
            g.update(1.0, 1.0, 1.0);
        }
    }
}

use gray_scott::GrayScott;
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_gray_scott_chemical(
        x in any::<usize>(),
        y in any::<usize>(),
        amount in prop::num::f32::ANY,
        feed in prop::num::f32::ANY,
        kill in prop::num::f32::ANY,
        dt in prop::num::f32::ANY,
    ) {
        let mut g = GrayScott::new(100, 100);
        g.add_chemical(x, y, amount);
        g.update(feed, kill, dt);
    }
}

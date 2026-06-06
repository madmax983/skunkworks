use market_sim::{Grid, Particle};
use proptest::prelude::*;

proptest! {
    #[test]
    fn havoc_market_sim(
        w in any::<usize>(),
        h in any::<usize>(),
        x in any::<usize>(),
        y in any::<usize>(),
        buyer in any::<usize>(),
        seller in any::<usize>(),
    ) {
        if w.checked_mul(h).is_some() && w < 1000 && h < 1000 {
            let mut g = Grid::new(w, h);
            g.set(x, y, Particle::Bid(buyer));
            g.set(x, y, Particle::Ask(seller));
            let _ = g.update();
        }
    }
}

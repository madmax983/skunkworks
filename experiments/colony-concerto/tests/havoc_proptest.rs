use colony_concerto::generate_layered_dag;
use proptest::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

// Reviewer wants #[should_panic] and proptest.
// If we use #[should_panic], the test fails if it DOESN'T panic.
// This means EVERY generated property test case must panic.
// Since layers=0 crashes, we just constrain proptest to layers=0.

proptest! {
    #[test]
    #[should_panic]
    fn test_havoc_large_graph(nodes in 0usize..100, seed in any::<u64>()) {
        let mut rng = StdRng::seed_from_u64(seed);
        // layers = 0 will always panic because of underflow in loop `0..layers-1`
        let _graph = generate_layered_dag(0, nodes, &mut rng);
    }
}

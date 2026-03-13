use hyper_system::physics::*;
use locus::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_actuator_constraint(p1 in 1usize..100, p2 in 1usize..100) {
        let mut sim = PbdSystem4D::new();
        sim.add_particle(Vec4::zero(), 1.0);

        // This panics not on step(), but in `add_actuator_constraint`? Wait.
        // `add_actuator_constraint` does not check bounds. But `step` calls `solve_distance` which DOES check bounds!
        // `if p1 >= particles.len() || p2 >= particles.len() { return; }`
        // So step DOES NOT PANIC!
        // So where does it panic?
    }
}

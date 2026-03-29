use physics_pbd::PbdSystem;

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 100")]
fn test_havoc_add_distance_constraint_oob() {
    // 👺 HAVOC: Proving that `add_distance_constraint` panics if given invalid indices.
    // The method calculates `dist = self.particles[p1].pos.distance(...)` without
    // checking if `p1` or `p2` actually exist in the `particles` vector.
    // An attacker or erroneous simulation logic can easily crash the physics engine.

    let mut system = PbdSystem::new();

    // System is empty. Asking for particles 100 and 200 should cause a deterministic panic.
    system.add_distance_constraint(100, 200, 1.0);
}

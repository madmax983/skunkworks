use market_sim::{DEFAULT_TRADE_AGE, Grid, Particle};

#[test]
fn test_sideways_movement_when_blocked() {
    // 3x3 grid
    // Row 0: Empty
    // Row 1: Bid(2) (Blocker)
    // Row 2: Bid(1) (Mover)
    let mut grid = Grid::new(3, 3);
    grid.set(1, 1, Particle::Bid(2));
    grid.set(1, 2, Particle::Bid(1));

    // Update.
    // Bid(1) at (1, 2) tries to move UP to (1, 1).
    // (1, 1) is occupied by Bid(2).
    // It should try to move sideways to (0, 2) or (2, 2).
    // Bid(2) at (1, 1) moves to (1, 0).

    // Note: Pass 1 iterates 0..height.
    // y=0: Nothing.
    // y=1: Bid(2) moves to (1, 0). (1, 1) becomes Empty.
    // y=2: Bid(1) moves to (1, 1).
    // So if the blocker moves out of the way, the blocked particle moves FORWARD, not sideways!

    // To test sideways movement, we need a STATIONARY blocker.
    // But Bids always move up unless blocked or at top.
    // Let's use an Ask as a blocker? No, Ask triggers collision.
    // Let's use a Trade as a blocker! Trade doesn't move.

    grid.set(1, 1, Particle::Trade { age: DEFAULT_TRADE_AGE });
    grid.set(1, 2, Particle::Bid(1));

    // Update.
    // y=1: Trade stays put (Pass 3 decays it, but Pass 1 sees it as occupied).
    // y=2: Bid(1) tries (1, 1). Occupied by Trade.
    // Tries sideways: (0, 2) or (2, 2).

    grid.update();

    let p_left = grid.get(0, 2);
    let p_mid = grid.get(1, 2);
    let p_right = grid.get(2, 2);

    // Original spot should be empty (moved sideways) OR contain the bid (if sideways failed due to RNG or bounds, but bounds are fine here).
    // Wait, sideways logic:
    // let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
    // It tries both directions. If one is valid and empty, it moves.
    // So it should DEFINITELY move sideways if space is available.

    let moved_sideways = matches!(
        (p_left, p_right),
        (Particle::Bid(1), _) | (_, Particle::Bid(1))
    );

    assert!(
        moved_sideways,
        "Bid should have moved sideways around the Trade particle. Grid state: {:?}",
        grid.cells
    );
    assert_eq!(p_mid, Particle::Empty, "Original spot should be empty");
}

#[test]
fn test_boundary_cleanup() {
    let mut grid = Grid::new(3, 3);
    // Bid at top (0, 0)
    grid.set(1, 0, Particle::Bid(1));
    // Ask at bottom (1, 2)
    grid.set(1, 2, Particle::Ask(2));

    grid.update();

    // Bid should be gone (moved off top)
    assert_eq!(
        grid.get(1, 0),
        Particle::Empty,
        "Bid at top should be removed"
    );

    // Ask should be gone (moved off bottom)
    assert_eq!(
        grid.get(1, 2),
        Particle::Empty,
        "Ask at bottom should be removed"
    );

    // Check that they didn't wrap around
    assert_eq!(
        grid.get(1, 1),
        Particle::Empty,
        "Particles should not wrap around"
    );
}

#[test]
fn test_trade_decay_lifecycle() {
    let mut grid = Grid::new(3, 3);
    // Manually place a Trade with age 2
    grid.set(1, 1, Particle::Trade { age: 2 });

    // Update 1: Age 2 -> 1
    grid.update();
    match grid.get(1, 1) {
        Particle::Trade { age } => assert_eq!(age, 1, "Age should decay from 2 to 1"),
        p => panic!("Expected Trade, found {:?}", p),
    }

    // Update 2: Age 1 -> 0
    grid.update();
    match grid.get(1, 1) {
        Particle::Trade { age } => assert_eq!(age, 0, "Age should decay from 1 to 0"),
        p => panic!("Expected Trade, found {:?}", p),
    }

    // Update 3: Age 0 -> Empty
    grid.update();
    assert_eq!(
        grid.get(1, 1),
        Particle::Empty,
        "Trade with age 0 should be removed"
    );
}

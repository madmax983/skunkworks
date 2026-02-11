use market_sim::{Grid, Particle};

#[test]
fn test_sideways_blocked_left_move_right() {
    let mut grid = Grid::new(3, 10);
    // Place a Bid at (1, 5)
    grid.set(1, 5, Particle::Bid(1));
    // Place a block at (1, 4) (above) - Use Trade so it doesn't move
    grid.set(1, 4, Particle::Trade { age: 10 });

    // Block Left at (0, 5) - Use Trade
    grid.set(0, 5, Particle::Trade { age: 10 });

    // Open Right at (2, 5) - Empty (default)

    grid.update();

    // Bid 1 (at 1,5) moves UP to (1,4). Blocked by Trade.
    // Tries sideways.
    // Left (0,5) is blocked by Trade.
    // Right (2,5) is open.
    // Should move to (2,5).

    assert_eq!(grid.get(2, 5), Particle::Bid(1));
    assert_eq!(grid.get(1, 5), Particle::Empty);
}

#[test]
fn test_sideways_blocked_right_move_left() {
    let mut grid = Grid::new(3, 10);
    // Place a Bid at (1, 5)
    grid.set(1, 5, Particle::Bid(1));
    // Place a block at (1, 4) (above)
    grid.set(1, 4, Particle::Trade { age: 10 });

    // Open Left at (0, 5)

    // Block Right at (2, 5)
    grid.set(2, 5, Particle::Trade { age: 10 });

    grid.update();

    // Bid 1 moves UP (blocked).
    // Tries sideways.
    // Right blocked. Left open.
    // Should move to (0, 5).

    assert_eq!(grid.get(0, 5), Particle::Bid(1));
    assert_eq!(grid.get(1, 5), Particle::Empty);
}

#[test]
fn test_sideways_blocked_both_no_move() {
    let mut grid = Grid::new(3, 10);
    // Place a Bid at (1, 5)
    grid.set(1, 5, Particle::Bid(1));
    // Place a block at (1, 4) (above)
    grid.set(1, 4, Particle::Trade { age: 10 });

    // Block Left at (0, 5)
    grid.set(0, 5, Particle::Trade { age: 10 });

    // Block Right at (2, 5)
    grid.set(2, 5, Particle::Trade { age: 10 });

    grid.update();

    // Bid 1 moves UP (blocked).
    // Tries sideways. Both blocked.
    // Should stay at (1, 5).

    assert_eq!(grid.get(1, 5), Particle::Bid(1));
}

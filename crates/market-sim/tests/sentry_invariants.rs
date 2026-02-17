use market_sim::{Grid, Particle};
use rand::Rng;

#[test]
fn test_zero_dimensions() {
    let mut grid = Grid::new(0, 0);
    // Ensure set() doesn't panic on zero-sized grid (it should just do nothing)
    grid.set(0, 0, Particle::Bid(1));

    let events = grid.update();
    assert!(events.is_empty(), "Should not return events for empty grid");
    assert_eq!(grid.trade_count, 0);
    assert_eq!(grid.total_bids, 0);
    assert_eq!(grid.total_asks, 0);
    // Grid::new initializes center_of_mass to height / 2.0 which is 0.0
    assert_eq!(grid.center_of_mass, 0.0);
}

#[test]
fn test_stats_consistency() {
    // Fuzz test: Randomly populate grid and verify stats match reality
    let width = 20;
    let height = 50;
    let mut grid = Grid::new(width, height);
    let mut rng = rand::thread_rng();

    // Populate with random particles
    for y in 0..height {
        for x in 0..width {
            let p = match rng.gen_range(0..10) {
                0 => Particle::Bid(rng.gen()),
                1 => Particle::Ask(rng.gen()),
                2 => Particle::Trade { age: 5 },
                _ => Particle::Empty,
            };
            if p != Particle::Empty {
                grid.set(x, y, p);
            }
        }
    }

    // Run update
    grid.update();

    // Verify stats
    let mut actual_bids = 0;
    let mut actual_asks = 0;

    for y in 0..height {
        for x in 0..width {
            match grid.get(x, y) {
                Particle::Bid(_) => actual_bids += 1,
                Particle::Ask(_) => actual_asks += 1,
                _ => {}
            }
        }
    }

    assert_eq!(grid.total_bids, actual_bids, "Total Bids stat mismatch");
    assert_eq!(grid.total_asks, actual_asks, "Total Asks stat mismatch");
}

#[test]
fn test_trade_event_consistency() {
    let height = 10;
    let mut grid = Grid::new(10, height);
    // Force a trade: Bid(1) at (5, 5), Ask(2) at (5, 4).
    // Bid moves UP to 4.
    grid.set(5, 5, Particle::Bid(1));
    grid.set(5, 4, Particle::Ask(2));

    let events = grid.update();

    assert_eq!(events.len(), 1);
    assert_eq!(
        grid.trade_count, 1,
        "Grid.trade_count should match events.len()"
    );

    let event = events[0];
    // Calculate expected Y from Price
    // Price = Height - 1 - Y
    // Y = Height - 1 - Price
    let expected_y = (height as f32 - 1.0 - event.price) as usize;

    // Verify a Trade particle exists at that location
    match grid.get(5, expected_y) {
        Particle::Trade { .. } => {}
        p => panic!(
            "Expected Trade particle at (5, {}), found {:?}",
            expected_y, p
        ),
    }
}

#[test]
fn test_trapped_particle() {
    // Surround a Bid with Trades so it can't move
    let mut grid = Grid::new(3, 3);
    // (1, 1) is our hero
    grid.set(1, 1, Particle::Bid(1));

    // Walls
    grid.set(1, 0, Particle::Trade { age: 5 }); // Top - Block Up
    grid.set(0, 1, Particle::Trade { age: 5 }); // Left - Block Sideways
    grid.set(2, 1, Particle::Trade { age: 5 }); // Right - Block Sideways

    // Update
    grid.update();

    // Should still be at (1, 1) because it has nowhere to go
    assert_eq!(
        grid.get(1, 1),
        Particle::Bid(1),
        "Particle should be trapped"
    );
}

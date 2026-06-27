use flocking::{compute_force, FlockingParams};
use locus::Vec2;

#[test]
fn havoc_test_flocking_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_test_flocking_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        if !status.success() {
            println!("👺 Havoc SUCCESS: Flocking engine panics when processing unequal lengths!");
        } else {
            panic!("Havoc failed to cause a crash!");
        }
    }
}

#[test]
#[ignore]
fn havoc_test_flocking_panic_inner() {
    if std::env::args().any(|arg| arg == "havoc_test_flocking_panic_inner") {
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let positions = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)];
        let velocities = vec![Vec2::zero()];

        let _ = compute_force(&positions, &velocities, 0, &params);
        std::process::exit(0);
    }
}

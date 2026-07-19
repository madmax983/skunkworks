use hydrothermal_locks::FluidSim;
use hydrothermal_locks::Grid;
use std::process;

// We simulate the underflow crash using an inner subprocess.
#[test]
fn test_havoc_fluid_underflow() {
    if std::env::var("RUN_HAVOC_INNER").is_ok() {
        // Trigger the crash: Width and height less than 2
        let mut fluid = FluidSim::new(0, 0);
        let grid = Grid::new(0, 0);

        // This will panic due to underflow: `1..self.width - 1` when width is 0 -> `1..-1` -> panic in range or subtraction
        fluid.update(&grid, 0.1);
        return;
    }

    let output = process::Command::new(std::env::current_exe().unwrap())
        .arg("--nocapture")
        .env("RUN_HAVOC_INNER", "1")
        .output()
        .unwrap();

    assert!(
        !output.status.success(),
        "Havoc expected fluid underflow crash, but it passed safely?!"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("attempt to subtract with overflow") || stderr.contains("panicked at"),
        "Expected overflow panic, got: {}",
        stderr
    );
}

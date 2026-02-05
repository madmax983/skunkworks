use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use celestial_cipher::physics::PhysicsPlugin;
use celestial_cipher::cipher::{CipherPlugin, CipherState};
use std::time::Duration;
use std::thread;

#[test]
fn test_cipher_generation() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(TransformPlugin);
    app.add_plugins(HierarchyPlugin);
    app.add_plugins(AssetPlugin::default());

    // Configure Rapier to run on Update
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0));

    app.add_plugins(PhysicsPlugin);
    app.add_plugins(CipherPlugin);

    // We need to simulate time passing for physics to work.
    // Bevy's default time system uses Instant::now().
    // Running app.update() in a tight loop results in near-zero delta time.
    // Sleeping forces the clock to advance.

    for _ in 0..500 {
        app.update();
        thread::sleep(Duration::from_millis(10));

        if app.world().resource::<CipherState>().generated_count > 0 {
            break;
        }
    }

    let cipher_state = app.world().resource::<CipherState>();
    println!("Final Key: {}", cipher_state.key);
    println!("Generated Count: {}", cipher_state.generated_count);

    assert!(cipher_state.generated_count > 0, "Should have generated at least one key segment");
}

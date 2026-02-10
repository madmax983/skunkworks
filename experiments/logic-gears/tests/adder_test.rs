use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use logic_gears::mechanism::{spawn_differential_adder, OutputIndicator, Rack};

#[test]
fn test_adder_logic() {
    let mut app = App::new();

    // Use MinimalPlugins for headless execution
    app.add_plugins(MinimalPlugins);

    // Add necessary plugins for Transform and Assets (needed by SpriteBundle)
    app.add_plugins(AssetPlugin::default());
    app.add_plugins(HierarchyPlugin);
    app.add_plugins(TransformPlugin);
    // TypeRegistrationPlugin and FrameCountPlugin are already in MinimalPlugins

    // Add Physics
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    // Initialize Resources
    app.init_resource::<Assets<Image>>();
    app.init_resource::<Assets<Mesh>>();

    app.add_systems(Startup, setup_test);

    app.update();

    for _ in 0..10 {
        app.update();
    }

    let output_y_initial = get_output_y(&mut app);
    println!("Initial Output Y: {}", output_y_initial);

    for _ in 0..60 {
        {
            let world = app.world_mut();
            let mut query = world.query::<(&mut Transform, &Rack)>();
            for (mut transform, _rack) in query.iter_mut(world) {
                if transform.translation.x < 0.0 {
                     transform.translation.y += 1.0;
                }
            }
        }
        app.update();
    }

    let output_y_final = get_output_y(&mut app);
    println!("Final Output Y: {}", output_y_final);

    // Assert movement
    assert!(output_y_final > output_y_initial + 10.0, "Output did not move up significantly. Diff: {}", output_y_final - output_y_initial);
}

fn setup_test(mut commands: Commands) {
    spawn_differential_adder(&mut commands, Vec2::ZERO);
}

fn get_output_y(app: &mut App) -> f32 {
    let world = app.world_mut();
    let mut query = world.query::<(&Transform, &OutputIndicator)>();
    if let Some((transform, _)) = query.iter(world).next() {
        return transform.translation.y;
    }
    0.0
}

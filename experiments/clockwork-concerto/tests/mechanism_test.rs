use bevy::prelude::*;
use clockwork_concerto::*;

#[test]
fn test_spawn_gear_exists() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_systems(Startup, |mut commands: Commands| {
        spawn_gear(&mut commands, Vec2::ZERO, 12, 5.0, 1.0);
    });

    app.update();

    let mut query = app.world.query::<&EscapeWheel>();
    let count = query.iter(&app.world).len();
    assert_eq!(count, 1);
}

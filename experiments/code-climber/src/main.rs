use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod ik;
mod parser;
mod climber;
mod brain;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_code_world)
        .add_systems(Startup, climber::spawn_climber)
        .add_systems(Update, brain::climb_control)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.transform.translation = Vec3::new(200.0, -200.0, 0.0);
    camera.projection.scale = 0.5;
    commands.spawn(camera);
}

fn spawn_code_world(mut commands: Commands) {
    let code = include_str!("main.rs");
    let scale = 20.0;

    for (i, line) in code.lines().enumerate() {
        let tokens = parser::parse_line(line, i);
        for token in tokens {
            let color = match token.token_type {
                parser::TokenType::Keyword => Color::rgb(1.0, 0.0, 1.0), // Magenta
                parser::TokenType::Identifier => Color::rgb(0.0, 1.0, 1.0), // Cyan
                parser::TokenType::Symbol => Color::WHITE,
                parser::TokenType::Comment => Color::GRAY,
                _ => Color::RED,
            };

            let char_w = 0.6;

            let w_px = token.width * scale * char_w;
            let h_px = scale * 0.8;

            let x_px = token.x * scale * char_w;
            let y_px = token.y * scale;

            let center_x = x_px + w_px / 2.0;
            let center_y = y_px;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(w_px, h_px)),
                        ..default()
                    },
                    transform: Transform::from_xyz(center_x, center_y, 0.0),
                    ..default()
                },
                Collider::cuboid(w_px / 2.0, h_px / 2.0),
                RigidBody::Fixed,
                Name::new(token.text),
                brain::Hold,
            ));
        }
    }
}

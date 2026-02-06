use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod actor;
mod laban;
mod particles;

use actor::{actor_move_system, Actor};
use laban::{director_system, Director};
use particles::{particle_emitter_system, particle_update_system, ParticleEmitter};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Genesis: Laban Machine".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .init_resource::<Director>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                director_system,
                actor_move_system,
                particle_emitter_system,
                particle_update_system,
                update_ui,
            ),
        )
        .run();
}

#[derive(Component)]
struct LabanText;

fn setup(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Actor
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(20.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 10.0)), // z=10 to be on top
            ..default()
        },
        Fill::color(Color::WHITE),
        Stroke::new(Color::srgb(0.0, 1.0, 1.0), 2.0),
        Actor::default(),
        ParticleEmitter::default(),
    ));

    // UI
    commands.spawn((
        TextBundle::from_section(
            "Laban State",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        LabanText,
    ));
}

fn update_ui(director: Res<Director>, mut query: Query<&mut Text, With<LabanText>>) {
    let e = &director.current_effort;

    for mut text in query.iter_mut() {
        text.sections[0].value = format!(
            "SPACE:  {:.2} ({})\nWEIGHT: {:.2} ({})\nTIME:   {:.2} ({})\nFLOW:   {:.2} ({})\n\nSpeed: {:.2}",
            e.space, label(e.space, "Direct", "Indirect"),
            e.weight, label(e.weight, "Strong", "Light"),
            e.time, label(e.time, "Sudden", "Sustained"),
            e.flow, label(e.flow, "Bound", "Free"),
            director.transition_speed
        );
    }
}

fn label(val: f32, low: &'static str, high: &'static str) -> &'static str {
    if val < 0.5 {
        low
    } else {
        high
    }
}

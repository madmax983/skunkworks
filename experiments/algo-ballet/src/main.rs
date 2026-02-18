use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::collections::VecDeque;

mod dancer;
mod kinematics;
mod choreographer;
mod laban;

use choreographer::{SortDirector, SortState, BubbleSort, director_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Algo Ballet ⚛️💃".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .insert_resource(SortDirector {
            array: vec![5, 3, 8, 1, 9, 2, 7, 4, 6],
            dancers: vec![],
            state: SortState::Running,
            algorithm: Box::new(BubbleSort::new(9)),
            instruction_queue: VecDeque::new(),
            current_wait: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (kinematics::pose_interpolator, director_system))
        .run();
}

fn setup(mut commands: Commands, mut director: ResMut<SortDirector>) {
    commands.spawn(Camera2dBundle::default());

    // UI
    commands.spawn((
        TextBundle::from_section(
            "Algorithm Ballet: Bubble Sort\n\nObserve the 'Effort' of the Dancers.\nCompare: Light, Sustained\nSwap: Strong, Sudden",
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
    ));

    // Spawn dancers corresponding to the array
    let start_x = -320.0;
    let spacing = 80.0;

    let mut entities = vec![];

    for (i, &value) in director.array.iter().enumerate() {
        let x = start_x + i as f32 * spacing;
        let hue = (value as f32 / 10.0) * 360.0;
        let color = Color::hsl(hue, 0.8, 0.5);

        let id = dancer::spawn_dancer(&mut commands, Vec3::new(x, 0.0, 0.0), value, color);
        entities.push(id);
    }

    director.dancers = entities;
}

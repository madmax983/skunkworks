use bevy::prelude::*;
use crate::simulation::SimulationImage;

#[derive(Component)]
struct SimulationSprite;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_visuals)
           .add_systems(Update, update_sprite_texture);
    }
}

fn setup_visuals(mut commands: Commands) {
    // We need to wait for the simulation image to be created?
    // SimulationPlugin creates it in Startup.
    // If VisualsPlugin runs after, it's fine.
    // But both are Startup systems. Ordering is not guaranteed.
    // We should check if resource exists.
    // Or better: Spawn a sprite with a placeholder, and let update system handle it.

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(1024.0, 1024.0)),
                ..default()
            },
            ..default()
        },
        SimulationSprite,
    ));

    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "Synesthesia Diffusion\nControls: [Click] Paint | [G] Toggle Ghost Mode",
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
    );
}

fn update_sprite_texture(
    image: Option<Res<SimulationImage>>,
    mut query: Query<&mut Handle<Image>, With<SimulationSprite>>,
) {
    if let Some(image) = image {
        for mut handle in query.iter_mut() {
            if *handle != image.0 {
                *handle = image.0.clone();
            }
        }
    }
}

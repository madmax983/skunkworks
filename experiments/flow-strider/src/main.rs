use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod gait;
mod graph;
mod input;
mod particles;
mod strider;

use gait::GaitPlugin;
use graph::GraphPlugin;
use input::InputPlugin;
use particles::ParticlePlugin;
use strider::StriderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(GraphPlugin)
        .add_plugins(StriderPlugin)
        .add_plugins(GaitPlugin)
        .add_plugins(ParticlePlugin)
        .add_plugins(InputPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

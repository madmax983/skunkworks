use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod graph;
mod strider;
mod gait;
mod particles;
mod input;

use graph::GraphPlugin;
use strider::StriderPlugin;
use gait::GaitPlugin;
use particles::ParticlePlugin;
use input::InputPlugin;

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

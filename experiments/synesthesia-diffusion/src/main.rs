use bevy::prelude::*;

mod audio;
mod simulation;
mod visuals;

use audio::AudioPlugin;
use simulation::SimulationPlugin;
use visuals::VisualsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Genesis: Synesthesia Diffusion".into(),
                resolution: (1024., 1024.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(AudioPlugin)
        .add_plugins(SimulationPlugin)
        .add_plugins(VisualsPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    info!("Synesthesia Diffusion Initialized.");
}

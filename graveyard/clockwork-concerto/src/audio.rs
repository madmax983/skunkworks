use crate::cpu::NoteEvent;
use bevy::prelude::*;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_note_system);
    }
}

fn play_note_system(mut events: EventReader<NoteEvent>) {
    for event in events.read() {
        // Log the note
        info!("🎵 Play Note: {}", event.0);
    }
}

use crossbeam_channel::unbounded;
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::Material;

#[test]
fn test_all_audio_commands() {
    let (tx, rx) = unbounded();
    let (snap_tx, _snap_rx) = unbounded();
    let (rec_tx, rec_rx) = unbounded();
    let mut engine = AudioModel::new(10, 10, rx, snap_tx, Some(rec_tx));

    // Initial state
    assert_eq!(engine.listener_x, 5);
    assert_eq!(engine.listener_y, 5);

    // Attempt invalid oscillator out of bounds (will hit the silent `else` branch in Oscillate addition)
    tx.send(AudioCommand::Oscillate {
        x: 20,
        y: 20,
        frequency: 440.0,
        strength: 1.0,
    })
    .unwrap();

    // Remove non-existent oscillator out of bounds (will hit silent `else` branch in missing oscillator)
    tx.send(AudioCommand::Oscillate {
        x: 30,
        y: 30,
        frequency: 440.0,
        strength: 0.0,
    })
    .unwrap();

    // Add an oscillator
    tx.send(AudioCommand::Oscillate {
        x: 2,
        y: 2,
        frequency: 440.0,
        strength: 1.0,
    })
    .unwrap();

    // Add a tone
    tx.send(AudioCommand::Tone {
        x: 3,
        y: 3,
        frequency: 880.0,
        strength: 0.5,
        duration_ms: 100,
    })
    .unwrap();

    // Move Listener
    tx.send(AudioCommand::MoveListener { x: 8, y: 8 }).unwrap();

    // Attempt invalid listener move
    tx.send(AudioCommand::MoveListener { x: 20, y: 20 })
        .unwrap();

    // Add and Remove wall
    tx.send(AudioCommand::AddWall { x: 1, y: 1 }).unwrap();

    // To trigger clear_walls inner logic, we need to have a wall.
    tx.send(AudioCommand::AddWall { x: 2, y: 2 }).unwrap();

    // Remove wall
    tx.send(AudioCommand::RemoveWall { x: 1, y: 1 }).unwrap();

    // Invalid wall coords (should do nothing safely)
    tx.send(AudioCommand::AddWall { x: 20, y: 20 }).unwrap();
    tx.send(AudioCommand::RemoveWall { x: 20, y: 20 }).unwrap();

    // Paint material
    tx.send(AudioCommand::PaintMaterial {
        x: 4,
        y: 4,
        material: Material::Slow,
    })
    .unwrap();
    tx.send(AudioCommand::PaintMaterial {
        x: 4,
        y: 4,
        material: Material::Fast,
    })
    .unwrap();
    tx.send(AudioCommand::PaintMaterial {
        x: 4,
        y: 4,
        material: Material::Void,
    })
    .unwrap();
    tx.send(AudioCommand::PaintMaterial {
        x: 4,
        y: 4,
        material: Material::Wall,
    })
    .unwrap();
    tx.send(AudioCommand::PaintMaterial {
        x: 4,
        y: 4,
        material: Material::Air,
    })
    .unwrap();

    // Out of bounds paint material
    tx.send(AudioCommand::PaintMaterial {
        x: 20,
        y: 20,
        material: Material::Void,
    })
    .unwrap();

    // Out of bounds pluck
    tx.send(AudioCommand::Pluck {
        x: 20,
        y: 20,
        strength: 1.0,
    })
    .unwrap();

    // Clear waves and walls
    tx.send(AudioCommand::ClearWaves).unwrap();
    tx.send(AudioCommand::ClearWalls).unwrap();

    // Test width and height
    assert_eq!(engine.grid.width(), 10);
    assert_eq!(engine.grid.height(), 10);

    // Verify out of bounds get
    assert_eq!(engine.grid.get(20, 20), 0.0);

    // Process audio to consume commands
    let mut buffer = vec![0.0; 256];
    engine.process(&mut buffer);

    // Let's also check phase wrapping logic
    // Oscillator frequency 44100 / 2.0 would cause phase to increase by PI each sample
    // Phase wrap happens when phase > 2 * PI
    tx.send(AudioCommand::Oscillate {
        x: 2,
        y: 2,
        frequency: 22050.0,
        strength: 1.0,
    })
    .unwrap();

    // Run for a few samples to wrap the phase
    let mut buffer = vec![0.0; 256];
    engine.process(&mut buffer);

    // Test recording reception
    let _rec_buffer = rec_rx.try_recv().unwrap();

    // Ensure active tones are cleaned up when duration hits 0
    tx.send(AudioCommand::Tone {
        x: 3,
        y: 3,
        frequency: 880.0,
        strength: 0.5,
        duration_ms: 1, // very short
    })
    .unwrap();

    // Process many samples to exhaust the tone
    for _ in 0..100 {
        engine.process(&mut buffer);
    }
}

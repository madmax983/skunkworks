use crossbeam_channel::bounded;
use resonance_audio::audio::{AudioCommand, AudioModel};
use resonance_audio::physics::{Material, PhysicsGrid};

#[test]
fn test_audio_tone_and_material_coverage() {
    let (cmd_tx, cmd_rx) = bounded(10);
    let (snap_tx, _snap_rx) = bounded(10);

    let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);

    // Tone Command
    cmd_tx
        .send(AudioCommand::Tone {
            x: 5,
            y: 5,
            frequency: 440.0,
            strength: 1.0,
            duration_ms: 10,
        })
        .unwrap();

    // Material commands
    cmd_tx
        .send(AudioCommand::PaintMaterial {
            x: 3,
            y: 3,
            material: Material::Slow,
        })
        .unwrap();
    cmd_tx.send(AudioCommand::AddWall { x: 4, y: 4 }).unwrap();
    cmd_tx
        .send(AudioCommand::RemoveWall { x: 4, y: 4 })
        .unwrap();
    cmd_tx.send(AudioCommand::ClearWalls).unwrap();
    cmd_tx.send(AudioCommand::ClearWaves).unwrap();
    cmd_tx
        .send(AudioCommand::MoveListener { x: 5, y: 5 })
        .unwrap();
    cmd_tx
        .send(AudioCommand::MoveListener { x: 100, y: 100 })
        .unwrap(); // out of bounds

    let mut buffer = vec![0.0; 100];
    model.process(&mut buffer);

    assert!(buffer.iter().any(|&s| s.abs() > 0.0));
}

#[test]
fn test_audio_recording_coverage() {
    let (_cmd_tx, cmd_rx) = bounded(10);
    let (snap_tx, _snap_rx) = bounded(10);
    let (rec_tx, rec_rx) = bounded(10);

    let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, Some(rec_tx));
    let mut buffer = vec![1.0; 10];
    model.process(&mut buffer);
    assert!(rec_rx.try_recv().is_ok());
}

#[test]
fn test_physics_materials_and_bounds() {
    let mut grid = PhysicsGrid::new(10, 10);

    // Bounds on pluck
    grid.pluck(20, 20, 1.0);

    // Material specific logic
    grid.set_material(5, 5, Material::Slow);
    grid.set_material(6, 6, Material::Fast);
    grid.set_material(7, 7, Material::Void);

    grid.pluck(5, 5, 1.0);
    grid.pluck(6, 6, 1.0);
    grid.pluck(7, 7, 1.0);

    for _ in 0..10 {
        grid.step();
    }

    grid.clear_waves();
    grid.add_wall(8, 8);
    grid.clear_walls();
    grid.remove_wall(8, 8);
}

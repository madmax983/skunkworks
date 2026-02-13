use crossbeam_channel::bounded;
use resonance_audio::audio::{AudioCommand, AudioModel};

#[test]
fn test_audio_simulation_integration() {
    let (cmd_tx, cmd_rx) = bounded(10);
    let (snap_tx, _snap_rx) = bounded(10);

    // Create a small model
    let mut model = AudioModel::new(20, 20, cmd_rx, snap_tx);

    // Send a pluck command
    cmd_tx
        .send(AudioCommand::Pluck {
            x: 10,
            y: 10,
            strength: 1.0,
        })
        .unwrap();

    // Process a buffer of samples
    let mut buffer = vec![0.0; 100];
    model.process(&mut buffer);

    // Verify that the simulation produced some output (wave propagation)
    // The pluck is at (10,10). The listener is at (10,10) by default (width/2, height/2).
    // So the first samples should reflect the pluck.
    let has_signal = buffer.iter().any(|&x| x.abs() > 0.001);
    assert!(has_signal, "Buffer should contain audio signal after pluck");
}

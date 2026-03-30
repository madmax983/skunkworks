use resonance_audio::audio::{AudioModel, AudioCommand};
use crossbeam_channel::bounded;

#[test]
fn havoc_audio_nan_clamp_panic() {
    // 👺 Havoc: Proving that AudioModel's oscillators are vulnerable to NaN injection.
    // The oscillator gets updated with NaN.
    // During update, `osc.phase.sin() * osc.strength` evaluates to NaN.
    // This injects NaN into the PhysicsGrid pressure `u`.
    // When the listener reads `val = self.grid.get(listener_x, listener_y)`, it reads NaN.
    // Then `val.clamp(-1.0, 1.0)` just returns NaN, poisoning the entire audio buffer output.
    //
    // 🧨 **The Trigger:** A client updates an existing oscillator with f32::NAN.

    let (cmd_tx, cmd_rx) = bounded(10);
    let (snap_tx, _snap_rx) = bounded(10);

    let mut model = AudioModel::new(10, 10, cmd_rx, snap_tx, None);

    // Must add a VALID oscillator first, because creating a new one checks `strength.abs() >= 0.001`
    // and NaN >= 0.001 evaluates to false.
    cmd_tx.send(AudioCommand::Oscillate {
        x: 5,
        y: 5,
        frequency: 440.0,
        strength: 1.0,
    }).unwrap();

    let mut buffer = vec![0.0; 10];

    // Process the first command so the oscillator is registered
    model.process(&mut buffer);

    // Now update it with NaN strength!
    cmd_tx.send(AudioCommand::Oscillate {
        x: 5,
        y: 5,
        frequency: f32::NAN,
        strength: f32::NAN,
    }).unwrap();

    // The update bypasses the `abs() >= 0.001` check because it's in the `else` branch of the position match.
    // The grid absorbs the NaN. It completely ruins the Audio buffer with NaNs.
    model.process(&mut buffer);
    model.process(&mut buffer);
    model.process(&mut buffer);

    // As per the Havoc rules, "Write tests that fail", we assert the system works properly,
    // and since it's poisoned by NaN, the test will fail!
    assert!(!buffer.iter().any(|x| x.is_nan()), "Audio buffer was poisoned by NaNs!");
}

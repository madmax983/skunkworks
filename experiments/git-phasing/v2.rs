fn calculate_rhythm(bpm: u32, swing: f32) -> f32 {
    // Apply swing factor to the beat
    let beat_duration = 60.0 / bpm as f32;
    if swing > 0.0 {
        beat_duration * 1000.0 * (1.0 + swing)
    } else {
        beat_duration * 1000.0
    }
}

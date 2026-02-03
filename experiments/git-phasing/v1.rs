fn calculate_rhythm(bpm: u32) -> f32 {
    let beat_duration = 60.0 / bpm as f32;
    beat_duration * 1000.0
}

use crate::mesh::{GlyphMesh, GlyphVertex};
use crate::synth::Spectrum;
use macroquad::prelude::Vec2;

pub fn apply_distortion(base_mesh: &GlyphMesh, spectrum: &Spectrum, time: f32) -> GlyphMesh {
    let mut distorted_mesh = GlyphMesh::new();

    let bass_scale = 1.0 + (spectrum.bass * 0.2); // Up to 20% growth
    let mid_ripple_amp = spectrum.mid * 5.0; // 5px ripple
    let mid_ripple_freq = 0.5 + spectrum.mid * 0.5; // Frequency relative to index
    let treble_jitter_amp = spectrum.treble * 2.0; // 2px jitter

    for strip in &base_mesh.strips {
        let mut new_strip = Vec::new();

        // Calculate strip centroid for scaling (approximate center of mass)
        let mut centroid = Vec2::ZERO;
        if !strip.is_empty() {
             for v in strip { centroid += v.position; }
             centroid /= strip.len() as f32;
        }

        for (i, v) in strip.iter().enumerate() {
            let mut pos = v.position;

            // 1. Bass: Scale from centroid
            // This makes the whole letter pulse
            let from_center = pos - centroid;
            pos = centroid + from_center * bass_scale;

            // 2. Mid: Sine wave ripple along normal
            // Phase depends on time and position (uv or index)
            // Use index for spatial coherence along the path
            let phase = time * 5.0 + (i as f32 * 0.1 * mid_ripple_freq);
            let ripple = phase.sin() * mid_ripple_amp;
            pos += v.normal * ripple;

            // 3. Treble: High frequency noise/jitter
            // Deterministic noise based on position and time
            let noise_x = (time * 50.0 + (i as f32) * 1.3 + pos.x * 0.1).sin();
            let noise_y = (time * 43.0 + (i as f32) * 2.1 + pos.y * 0.1).cos();
            pos.x += noise_x * treble_jitter_amp;
            pos.y += noise_y * treble_jitter_amp;

            new_strip.push(GlyphVertex {
                position: pos,
                normal: v.normal, // Keep original normal (or approximate)
                uv: v.uv,
            });
        }
        distorted_mesh.strips.push(new_strip);
    }
    distorted_mesh
}

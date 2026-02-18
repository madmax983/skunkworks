use bevy::prelude::*;
use bevy::color::LinearRgba;
use crate::audio::AudioString;
use crate::scanner::{CodeString, get_color_from_ext};
use crate::camera::FlyCam;

pub fn interaction_system(
    mut query: Query<(&Transform, &mut AudioString, &Handle<StandardMaterial>, &CodeString)>,
    camera_query: Query<&Transform, With<FlyCam>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if let Ok(cam_transform) = camera_query.get_single() {
        let cam_pos = cam_transform.translation;

        for (transform, mut audio, mat_handle, code) in query.iter_mut() {
            let dist = (Vec2::new(transform.translation.x, transform.translation.z)
                      - Vec2::new(cam_pos.x, cam_pos.z)).length();

            if dist < 0.5 {
                audio.pluck(0.1 * time.delta_seconds());

                if let Some(mat) = materials.get_mut(mat_handle.id()) {
                    let base = get_color_from_ext(code.path.extension().and_then(|e| e.to_str()).unwrap_or(""));
                    let pulse = (time.elapsed_seconds() * 10.0).sin() * 0.5 + 1.5;
                    mat.emissive = LinearRgba::from(base) * pulse * 5.0;
                }
            } else {
                 if let Some(mat) = materials.get_mut(mat_handle.id()) {
                     let base = get_color_from_ext(code.path.extension().and_then(|e| e.to_str()).unwrap_or(""));
                     mat.emissive = LinearRgba::from(base) * 2.0;
                 }
            }
        }
    }
}

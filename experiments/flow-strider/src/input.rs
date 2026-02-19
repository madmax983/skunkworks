use bevy::prelude::*;
use crate::graph::Graph;
use crate::strider::Strider;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_click);
    }
}

fn handle_click(
    mouse: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform)>,
    mut striders: Query<&mut Strider>,
    graph: Res<Graph>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Ok(window) = windows.get_single() {
            if let Some(position) = window.cursor_position() {
                if let Ok((camera, camera_transform)) = camera.get_single() {
                    if let Some(world_position) = camera.viewport_to_world_2d(camera_transform, position) {
                        // Find nearest node
                        let mut nearest_node = None;
                        let mut min_dist = f32::MAX;

                        for (i, node_pos) in graph.nodes.iter().enumerate() {
                            let dist = node_pos.distance(world_position);
                            if dist < min_dist {
                                min_dist = dist;
                                nearest_node = Some(i);
                            }
                        }

                        if let Some(idx) = nearest_node {
                            if min_dist < 40.0 { // Click radius
                                for mut strider in striders.iter_mut() {
                                    strider.target_node = idx;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

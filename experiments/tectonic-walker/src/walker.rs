use bevy::prelude::*;
use crate::components::*;

pub struct WalkerPlugin;

impl Plugin for WalkerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, walker_system);
    }
}

#[derive(Component)]
pub struct Walker {
    pub target_entity: Entity, // The IKTarget entity this walker controls
    pub current_commit_idx: usize,
    pub speed: f32,
}

fn walker_system(
    time: Res<Time>,
    mut walker_query: Query<&mut Walker>,
    mut target_transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    commit_nodes: Query<(Entity, &CommitNode)>,
) {
    for mut walker in walker_query.iter_mut() {
        // Find the target node for the current index
        let mut target_node_entity = None;
        let mut max_idx = 0;

        // Naive search (optimization: store sorted list in a Resource)
        for (e, node) in commit_nodes.iter() {
            if node.index == walker.current_commit_idx {
                target_node_entity = Some(e);
            }
            if node.index > max_idx {
                max_idx = node.index;
            }
        }

        if let Some(target_e) = target_node_entity {
            if let Ok(node_tf) = global_transforms.get(target_e) {
                let target_pos = node_tf.translation();

                if let Ok(mut t) = target_transforms.get_mut(walker.target_entity) {
                    let current = t.translation;
                    let direction = target_pos - current;
                    let dist = direction.length();

                    // Speed modulation based on stress?
                    // Let's just use constant speed for now.

                    if dist > 5.0 {
                         let move_step = direction.normalize() * walker.speed * time.delta_seconds();
                         // Lerp towards it
                         if move_step.length() > dist {
                             t.translation = target_pos;
                         } else {
                             t.translation += move_step;
                         }
                    } else {
                        // Reached target
                        // Move to next commit
                        if walker.current_commit_idx < max_idx {
                            walker.current_commit_idx += 1;
                        } else {
                            // Reset to start
                            walker.current_commit_idx = 0;
                            // Teleport IK target back to start?
                            // Or just let it walk back? Walking back is funny.
                            // But usually we want to see the timeline.
                            // Teleporting might break IK if it stretches too far.
                        }
                    }
                }
            }
        } else {
            // Target not found (maybe index skipped?), try next
             if walker.current_commit_idx < max_idx {
                walker.current_commit_idx += 1;
            }
        }
    }
}

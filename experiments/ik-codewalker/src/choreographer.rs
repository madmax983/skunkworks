use bevy::prelude::*;
use rand::Rng;
use crate::components::*;
use crate::code_graph::CodeGraph;
use crate::ik::IKChain;

pub struct ChoreographerPlugin;

impl Plugin for ChoreographerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, choreographer_system);
    }
}

#[derive(Component)]
pub struct Choreographer {
    pub target_entity: Entity, // The IKTarget entity this choreographer controls
    pub current_target_node: Option<Entity>,
    pub next_target_time: f32,
    pub speed: f32,
}

fn choreographer_system(
    time: Res<Time>,
    mut choreographer_query: Query<&mut Choreographer>,
    mut target_transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    code_graphs: Query<&CodeGraph>,
) {
    let mut rng = rand::thread_rng();

    for mut choreo in choreographer_query.iter_mut() {
        // Decrease timer
        choreo.next_target_time -= time.delta_seconds();

        // Check if we need a new target node
        if choreo.next_target_time <= 0.0 {
            // Pick a random node from the graph
            // We assume there's one main graph for now
            if let Ok(graph) = code_graphs.get_single() {
                if !graph.nodes.is_empty() {
                    let idx = rng.gen_range(0..graph.nodes.len());
                    let target_node = graph.nodes[idx];
                    choreo.current_target_node = Some(target_node);
                    choreo.next_target_time = rng.gen_range(1.0..3.0); // Wait 1-3 seconds
                }
            }
        }

        // Move IK Target entity towards the current target node position
        if let Some(target_node_entity) = choreo.current_target_node {
             if let Ok(node_tf) = global_transforms.get(target_node_entity) {
                 let target_dest = node_tf.translation();

                 if let Ok(mut t) = target_transforms.get_mut(choreo.target_entity) {
                     let current = t.translation;
                     let direction = target_dest - current;
                     let dist = direction.length();

                     if dist > 1.0 {
                         let move_step = direction.normalize() * choreo.speed * time.delta_seconds();
                         if move_step.length() > dist {
                             t.translation = target_dest;
                         } else {
                             t.translation += move_step;
                         }
                     }
                 }
             }
        }
    }
}

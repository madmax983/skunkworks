use petgraph::Direction;
use petgraph::visit::EdgeRef;
use ratatui::style::Color;
use ratatui::widgets::canvas::{Context, Line};
use std::collections::{HashMap, HashSet, VecDeque};

use crate::graph::DependencyGraph;
use petgraph::graph::NodeIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodePos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl NodePos {
    pub fn origin() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

pub struct RenderNode {
    pub index: NodeIndex,
    pub pos: NodePos,
    pub label: String,
    pub is_focused: bool,
}

// Isometric projection constants
const ISO_WIDTH: f64 = 4.0;
const ISO_HEIGHT_Y: f64 = 2.0;
const ISO_HEIGHT_Z: f64 = 4.0;

pub fn project(pos: NodePos) -> (f64, f64) {
    let u = (pos.x as f64 - pos.y as f64) * ISO_WIDTH;
    let v = ((pos.x as f64 + pos.y as f64) * ISO_HEIGHT_Y) - (pos.z as f64 * ISO_HEIGHT_Z);
    (u, -v) // Flip Y for screen coords usually, but let's keep it mathematical first.
    // In TUI canvas, Y increases upwards? No, usually downwards.
    // Canvas: (0,0) is bottom-left if we set bounds.
    // Let's assume (0,0) is center and Y increases upwards.
}

pub fn build_scene(
    graph: &DependencyGraph,
    center_node: NodeIndex,
    depth: usize,
) -> Vec<RenderNode> {
    let mut scene = Vec::new();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    // (Node, Position, PathLength)
    queue.push_back((center_node, NodePos::origin(), 0));
    visited.insert((center_node, NodePos::origin())); // Visit is keyed by (Node, Pos) to allow same node at diff pos?
    // Actually, if we want to show cycles (Escher stairs), we MUST allow same node at different positions.
    // So `visited` should perhaps be just `pos` to avoid overlapping blocks?
    // Or `(Node, Pos)`?
    // If we visit A at (0,0,0), then B at (1,0,1). Then C at (1,1,2). Then A at (0,1,3).
    // We want to draw A at (0,1,3).
    // So we should track `(Node, Pos)` to avoid infinite loops in BFS if we don't limit depth.
    // But we limit depth.

    // Let's use `visited_pos` to prevent two nodes occupying same space.
    let mut visited_pos = HashSet::new();
    visited_pos.insert(NodePos::origin());

    scene.push(RenderNode {
        index: center_node,
        pos: NodePos::origin(),
        label: graph[center_node].clone(),
        is_focused: true,
    });

    while let Some((curr, pos, d)) = queue.pop_front() {
        if d >= depth {
            continue;
        }

        // Neighbors (Dependencies - Going UP)
        let deps: Vec<_> = graph
            .edges_directed(curr, Direction::Outgoing)
            .map(|e| e.target())
            .collect();

        for (i, &dep) in deps.iter().enumerate() {
            // Heuristic to distribute neighbors
            let offset = match i % 2 {
                0 => (1, 0, 1),
                _ => (0, 1, 1),
            };
            let new_pos = NodePos {
                x: pos.x + offset.0,
                y: pos.y + offset.1,
                z: pos.z + offset.2,
            };

            if !visited_pos.contains(&new_pos) {
                visited_pos.insert(new_pos);
                scene.push(RenderNode {
                    index: dep,
                    pos: new_pos,
                    label: graph[dep].clone(),
                    is_focused: false,
                });
                queue.push_back((dep, new_pos, d + 1));
            }
        }

        // Reverse Neighbors (Dependents - Going DOWN)
        let rev_deps: Vec<_> = graph
            .edges_directed(curr, Direction::Incoming)
            .map(|e| e.source())
            .collect();

        for (i, &rev) in rev_deps.iter().enumerate() {
            let offset = match i % 2 {
                0 => (-1, 0, -1),
                _ => (0, -1, -1),
            };
            let new_pos = NodePos {
                x: pos.x + offset.0,
                y: pos.y + offset.1,
                z: pos.z + offset.2,
            };

            if !visited_pos.contains(&new_pos) {
                visited_pos.insert(new_pos);
                scene.push(RenderNode {
                    index: rev,
                    pos: new_pos,
                    label: graph[rev].clone(),
                    is_focused: false,
                });
                queue.push_back((rev, new_pos, d + 1));
            }
        }
    }

    scene
}

pub fn draw_scene(ctx: &mut Context<'_>, scene: &[RenderNode], graph: &DependencyGraph) {
    for node in scene {
        let (u, v) = project(node.pos);

        // Draw Node (Block)
        let color = if node.is_focused {
            Color::Yellow
        } else {
            Color::Blue
        };

        ctx.draw(&ratatui::widgets::canvas::Circle {
            x: u,
            y: v,
            radius: 2.0,
            color,
        });
    }

    // Map NodeIndex -> Vec<Pos> (since a node can appear multiple times)
    let mut node_positions: HashMap<NodeIndex, Vec<NodePos>> = HashMap::new();
    for node in scene {
        node_positions.entry(node.index).or_default().push(node.pos);
    }

    for node in scene {
        // Outgoing edges
        for edge in graph.edges_directed(node.index, Direction::Outgoing) {
            let target = edge.target();
            if let Some(positions) = node_positions.get(&target) {
                for &target_pos in positions {
                    // Check if they are adjacent in our grid logic
                    // (dx, dy, dz) should be (1,0,1) or (0,1,1)
                    let dx = target_pos.x - node.pos.x;
                    let dy = target_pos.y - node.pos.y;
                    let dz = target_pos.z - node.pos.z;

                    if (dx == 1 && dy == 0 && dz == 1) || (dx == 0 && dy == 1 && dz == 1) {
                        let (u1, v1) = project(node.pos);
                        let (u2, v2) = project(target_pos);
                        ctx.draw(&Line {
                            x1: u1,
                            y1: v1,
                            x2: u2,
                            y2: v2,
                            color: Color::Gray,
                        });
                    }
                }
            }
        }

        // Also draw label on top
        let (u, v) = project(node.pos);
        ctx.print(u, v + 2.0, ratatui::text::Span::raw(node.label.clone()));
    }
}

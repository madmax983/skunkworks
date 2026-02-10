use macroquad::prelude::*;
use petgraph::Directed;
use petgraph::stable_graph::{NodeIndex, StableGraph};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ObjectState {
    Allocated,
    Marked,
    Rotting,
}

#[derive(Clone, Debug)]
pub struct Object {
    pub pos: Vec2,
    pub state: ObjectState,
}

pub struct Heap {
    pub graph: StableGraph<Object, (), Directed>,
    pub roots: Vec<NodeIndex>,
}

impl Heap {
    pub fn new() -> Self {
        Self {
            graph: StableGraph::new(),
            roots: Vec::new(),
        }
    }

    pub fn allocate(&mut self, pos: Vec2) -> NodeIndex {
        self.graph.add_node(Object {
            pos,
            state: ObjectState::Allocated,
        })
    }

    pub fn link(&mut self, from: NodeIndex, to: NodeIndex) {
        self.graph.add_edge(from, to, ());
    }

    pub fn add_root(&mut self, node: NodeIndex) {
        self.roots.push(node);
    }

    pub fn mark(&mut self) {
        // Reset previously marked nodes to Allocated so we can re-verify reachability
        let nodes: Vec<_> = self.graph.node_indices().collect();
        for node in nodes {
            if self.graph[node].state == ObjectState::Marked {
                self.graph[node].state = ObjectState::Allocated;
            }
        }

        // BFS from roots
        let mut queue = self.roots.clone();
        let mut visited = std::collections::HashSet::new();

        for root in &self.roots {
            visited.insert(*root);
            // Roots are always marked if they exist in graph
            if let Some(obj) = self.graph.node_weight_mut(*root) {
                obj.state = ObjectState::Marked;
            }
        }

        while let Some(node_idx) = queue.pop() {
            let mut neighbors = self.graph.neighbors(node_idx).detach();
            while let Some(neighbor_idx) = neighbors.next_node(&self.graph) {
                if !visited.contains(&neighbor_idx) {
                    visited.insert(neighbor_idx);
                    if let Some(obj) = self.graph.node_weight_mut(neighbor_idx) {
                        if obj.state != ObjectState::Rotting {
                            // Don't revive rotting nodes? Or should we?
                            // Standard GC: If it's reachable, it's alive. Even if it was rotting?
                            // Usually rotting means "waiting to be collected". If it becomes reachable again (resurrection), it should be marked.
                            obj.state = ObjectState::Marked;
                        }
                    }
                    queue.push(neighbor_idx);
                }
            }
        }
    }

    pub fn decay(&mut self) {
        let nodes: Vec<_> = self.graph.node_indices().collect();
        for node in nodes {
            if self.graph[node].state == ObjectState::Allocated {
                self.graph[node].state = ObjectState::Rotting;
            }
        }
    }

    pub fn sweep(&mut self) {
        // StableGraph::retain_nodes retains nodes for which the closure returns true
        self.graph
            .retain_nodes(|g, n| g[n].state != ObjectState::Rotting);
        // Note: retain_nodes invalidates NodeIndices. Roots vector might become invalid.
        // We need to update roots or handle invalidation.
        // For this simple simulation, we might need to be careful.
        // If roots are removed (unlikely as they are roots), we have issues.
        // But roots are seeds of reachability, so they should be Marked, thus not Rotting.

        // However, NodeIndex values change in `petgraph`'s `Graph` when nodes are removed (swap and pop).
        // This is problematic for `roots` vector which holds indices.
        // Solution: Use `StableGraph` or rebuild `roots`.
        // Or just map the indices if possible.

        // For now, let's assume roots are always safe because they are marked?
        // No, even if roots are safe, other nodes moving might shift indices that roots point to if `roots` stored indices to *other* things?
        // No, `roots` stores indices of root objects.
        // If node 5 is removed, node 10 might become node 5.
        // If a root was node 10, it is now node 5. But our `roots` vec still says 10.
        // So next `mark()` will look at 10 (which might be empty or something else).

        // Switch to `StableGraph`? It keeps indices stable.
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate() {
        let mut heap = Heap::new();
        let idx = heap.allocate(vec2(10.0, 10.0));
        assert_eq!(heap.graph.node_count(), 1);
        assert_eq!(heap.graph[idx].state, ObjectState::Allocated);
    }

    #[test]
    fn test_mark_reachability() {
        let mut heap = Heap::new();
        let root = heap.allocate(vec2(0.0, 0.0));
        let child = heap.allocate(vec2(10.0, 10.0));
        let orphan = heap.allocate(vec2(20.0, 20.0));

        heap.add_root(root);
        heap.link(root, child);

        heap.mark();

        assert_eq!(heap.graph[root].state, ObjectState::Marked);
        assert_eq!(heap.graph[child].state, ObjectState::Marked);
        assert_eq!(heap.graph[orphan].state, ObjectState::Allocated);
    }

    #[test]
    fn test_decay() {
        let mut heap = Heap::new();
        let root = heap.allocate(vec2(0.0, 0.0));
        let orphan = heap.allocate(vec2(20.0, 20.0));

        heap.add_root(root);
        heap.mark();

        heap.decay();

        assert_eq!(heap.graph[root].state, ObjectState::Marked);
        assert_eq!(heap.graph[orphan].state, ObjectState::Rotting);
    }

    #[test]
    fn test_sweep() {
        let mut heap = Heap::new();
        let rotting = heap.allocate(vec2(0.0, 0.0));
        heap.graph[rotting].state = ObjectState::Rotting;

        heap.sweep();

        assert_eq!(heap.graph.node_count(), 0);
    }
}

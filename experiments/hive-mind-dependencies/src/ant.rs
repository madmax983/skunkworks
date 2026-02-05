use petgraph::graph::{EdgeIndex, NodeIndex};

#[derive(Clone, Debug)]
pub struct Ant {
    pub current_node: NodeIndex,
    pub carrying_artifact: bool,
    pub path_history: Vec<EdgeIndex>,
}

impl Ant {
    pub fn new(start_node: NodeIndex) -> Self {
        Self {
            current_node: start_node,
            carrying_artifact: false,
            path_history: Vec::with_capacity(50),
        }
    }
}

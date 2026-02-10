use crate::grammar::LSystem;

pub type TreeID = usize;

#[derive(Clone, Debug)]
pub struct Tree {
    pub id: TreeID,
    pub full_dna: String,
    pub growth_cursor: usize,
    pub energy: f32,
    pub x_pos: f32,
    pub color: (f32, f32, f32), // r, g, b
}

impl Tree {
    pub fn new(id: TreeID, lsystem: &LSystem, iterations: u32, x_pos: f32, color: (f32, f32, f32)) -> Self {
        let full_dna = lsystem.expand(iterations);
        Self {
            id,
            full_dna,
            growth_cursor: 0,
            energy: 0.0,
            x_pos,
            color,
        }
    }

    pub fn grow(&mut self, amount: usize) {
        self.growth_cursor = (self.growth_cursor + amount).min(self.full_dna.len());
    }
}

pub struct Forest {
    pub trees: Vec<Tree>,
}

impl Forest {
    pub fn new() -> Self {
        Self { trees: Vec::new() }
    }

    pub fn add_tree(&mut self, tree: Tree) {
        self.trees.push(tree);
    }

    pub fn get_tree_mut(&mut self, id: TreeID) -> Option<&mut Tree> {
        self.trees.iter_mut().find(|t| t.id == id)
    }
}

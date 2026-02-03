use std::time::Duration;
use tui_shared::math::Vec2;

#[derive(Debug, Clone)]
pub struct Node {
    pub pos: Vec2,
    pub length: f64,
    pub angle: f64, // Absolute angle in radians
    pub depth: usize,
    #[allow(dead_code)]
    pub function_name: String,
    #[allow(dead_code)]
    pub args: String,
    pub return_value: Option<String>,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(
        pos: Vec2,
        angle: f64,
        length: f64,
        depth: usize,
        name: String,
        args: String,
    ) -> Self {
        Self {
            pos,
            length,
            angle,
            depth,
            function_name: name,
            args,
            return_value: None,
            children: Vec::new(),
        }
    }

    pub fn end_pos(&self) -> Vec2 {
        let dx = self.angle.cos() * self.length;
        let dy = self.angle.sin() * self.length;
        self.pos + Vec2::new(dx, dy)
    }
}

pub struct Plant {
    pub root: Option<Node>,
    /// Path to the current active node (the top of the stack).
    /// Each usize is the index in the parent's `children` vector.
    pub cursor_path: Vec<usize>,
}

impl Default for Plant {
    fn default() -> Self {
        Self::new()
    }
}

impl Plant {
    pub fn new() -> Self {
        Self {
            root: None,
            cursor_path: Vec::new(),
        }
    }

    /// Adds a child to the node at `cursor_path` and moves the cursor to it.
    /// If root is None, creates the root.
    pub fn push_call(
        &mut self,
        pos: Vec2,
        angle: f64,
        length: f64,
        depth: usize,
        name: String,
        args: String,
    ) {
        let node = Node::new(pos, angle, length, depth, name, args);

        if self.root.is_none() {
            self.root = Some(node);
            self.cursor_path.clear(); // pointing to root (which is depth 0, so path is empty? or path is empty means we are AT root?
                                      // Let's say: path empty = "holding the root handle"? No.
                                      // Let's say: path is indices to reach the CURRENT node.
                                      // If we are at root, path is empty.
                                      // But we just added root. So we are AT root.
            return;
        }

        // Navigate to current node
        let mut current = self.root.as_mut().unwrap();
        for &idx in &self.cursor_path {
            current = &mut current.children[idx];
        }

        // Add child
        current.children.push(node);
        let new_idx = current.children.len() - 1;

        // Move cursor into child
        self.cursor_path.push(new_idx);
    }

    /// Moves the cursor up one level and sets the return value of the just-exited node.
    pub fn pop_return(&mut self, value: String) {
        if self.cursor_path.is_empty() {
            // We are popping the root?
            if let Some(root) = self.root.as_mut() {
                root.return_value = Some(value);
            }
            return;
        }

        // We want to set the return value of the node we are LEAVING (current cursor).
        // Then pop the path.
        let mut current = self.root.as_mut().unwrap();
        for &idx in &self.cursor_path {
            current = &mut current.children[idx];
        }
        current.return_value = Some(value);

        self.cursor_path.pop();
    }

    pub fn get_cursor_node(&self) -> Option<&Node> {
        let mut current = self.root.as_ref()?;
        for &idx in &self.cursor_path {
            if idx >= current.children.len() {
                return None; // Should not happen if logic is correct
            }
            current = &current.children[idx];
        }
        Some(current)
    }
}

#[derive(Debug)]
pub enum Event {
    Call {
        name: String,
        args: String,
    },
    Return {
        value: String,
    },
    #[allow(dead_code)]
    Sleep(Duration),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plant_growth() {
        let mut plant = Plant::new();
        // Call Root
        plant.push_call(Vec2::new(0.0, 0.0), 0.0, 10.0, 0, "root".into(), "".into());
        assert!(plant.root.is_some());
        assert_eq!(plant.cursor_path.len(), 0); // At root

        // Call Child 1
        plant.push_call(
            Vec2::new(0.0, 0.0),
            0.0,
            10.0,
            1,
            "child1".into(),
            "".into(),
        );
        assert_eq!(plant.cursor_path.len(), 1);
        assert_eq!(plant.cursor_path[0], 0);

        // Call Child 1.1
        plant.push_call(
            Vec2::new(0.0, 0.0),
            0.0,
            10.0,
            2,
            "child1.1".into(),
            "".into(),
        );
        assert_eq!(plant.cursor_path.len(), 2);

        // Return from 1.1
        plant.pop_return("ret1.1".into());
        assert_eq!(plant.cursor_path.len(), 1);

        let root = plant.root.as_ref().unwrap();
        let child1 = &root.children[0];
        let child1_1 = &child1.children[0];
        assert_eq!(child1_1.return_value, Some("ret1.1".into()));
    }
}

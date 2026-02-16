use macroquad::prelude::Color;
use rand::prelude::*;

#[derive(Clone, Debug)]
pub struct TesseractNode {
    pub id: u64,
    pub size: u64,
    pub children: Vec<TesseractNode>,
    pub color: Color,
    pub name: String,
    pub layout_seed: u64, // For deterministic layout
}

impl TesseractNode {
    pub fn total_size(&self) -> u64 {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<u64>()
    }
}

pub fn generate_mock_heap(seed: u64) -> TesseractNode {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut id_counter = 0;

    let mut root = generate_recursive(&mut rng, 3, &mut id_counter);
    root.name = "HEAP_ROOT".to_string();
    root
}

fn generate_recursive(rng: &mut StdRng, depth: u32, id_counter: &mut u64) -> TesseractNode {
    let id = *id_counter;
    *id_counter += 1;
    let layout_seed = rng.next_u64();

    let mut children = Vec::new();
    let mut self_size = rng.gen_range(64..256); // Meta overhead

    if depth > 0 {
        let num_children = rng.gen_range(4..12); // More children to fill the room
        for _ in 0..num_children {
            children.push(generate_recursive(rng, depth - 1, id_counter));
        }
    } else {
        self_size += rng.gen_range(1024..65536);
    }

    let r = rng.gen_range(0.2..1.0);
    let g = rng.gen_range(0.2..1.0);
    let b = rng.gen_range(0.2..1.0);
    let color = Color::new(r, g, b, 1.0);

    TesseractNode {
        id,
        size: self_size,
        children,
        color,
        name: format!("Alloc_{:04X}", id),
        layout_seed,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_generation() {
        let root = generate_mock_heap(42);

        // Assert root exists
        assert_eq!(root.name, "HEAP_ROOT");

        // Assert structure
        assert!(!root.children.is_empty(), "Root should have children");

        // Check depths
        let first_child = &root.children[0];
        assert!(!first_child.children.is_empty(), "First child should have sub-children");

        // Check consistency
        let total = root.total_size();
        assert!(total > 10000, "Heap should be substantial, got {}", total);
    }
}

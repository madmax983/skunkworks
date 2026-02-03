use crate::level_gen::{BlockType, LevelProfile};

#[derive(Debug, Clone)]
pub struct Block {
    pub start: usize,
    pub size: usize,
    pub block_type: BlockType,
    pub is_solid: bool, // If false, it's a gap/hole (freed memory)
    pub code: String,
}

#[derive(Debug)]
pub struct Heap {
    pub blocks: Vec<Block>,
    pub total_size: usize,
}

impl Heap {
    pub fn new(total_size: usize) -> Self {
        // Fallback for empty init
        Self {
            blocks: vec![],
            total_size,
        }
    }

    pub fn from_profile(profile: &LevelProfile) -> Self {
        let mut blocks = Vec::new();
        let mut cursor = 0;

        // Add a safety start block
        blocks.push(Block {
            start: cursor,
            size: 10,
            block_type: BlockType::Solid,
            is_solid: true,
            code: "// Entry Point".to_string(),
        });
        cursor += 10;

        for seg in &profile.segments {
            // If it's a Gap type from generator, we might map it to a non-solid block or just skip?
            // Let's map it to a non-solid block so we keep the "memory address" space consistent.
            let is_solid = !matches!(seg.block_type, BlockType::Gap);

            blocks.push(Block {
                start: cursor,
                size: seg.width,
                block_type: seg.block_type,
                is_solid,
                code: seg.code.clone(),
            });
            cursor += seg.width;
        }

        // Add safety end block
        blocks.push(Block {
            start: cursor,
            size: 50,
            block_type: BlockType::Solid,
            is_solid: true,
            code: "// Return Address".to_string(),
        });
        cursor += 50;

        Self {
            blocks,
            total_size: cursor,
        }
    }

    /// "Frees" a block (makes it non-solid/hole)
    pub fn free(&mut self, index: usize) {
        if index < self.blocks.len() {
            self.blocks[index].is_solid = false;
        }
    }

    /// "Allocates" a block (makes it solid)
    pub fn allocate_at(&mut self, index: usize) {
        if index < self.blocks.len() {
            self.blocks[index].is_solid = true;
        }
    }
}

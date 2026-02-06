use chimera_lang::vm::ChimeraVM;
use macroquad::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Portal {
    pub target_room_id: usize,
    pub rect: Rect,
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Room {
    pub id: usize,
    pub rect: Rect,
    pub color: Color,
    pub portals: Vec<Portal>,
    pub label: String,
    pub description: String,
}

pub struct World {
    pub rooms: HashMap<usize, Room>,
    pub root_id: usize,
}

fn hsl_to_color(h: f32, s: f32, l: f32) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 1.0 / 6.0 {
        (c, x, 0.0)
    } else if h < 2.0 / 6.0 {
        (x, c, 0.0)
    } else if h < 3.0 / 6.0 {
        (0.0, c, x)
    } else if h < 4.0 / 6.0 {
        (0.0, x, c)
    } else if h < 5.0 / 6.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Color::new(r + m, g + m, b + m, 1.0)
}

impl World {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            root_id: 0,
        }
    }

    pub fn from_vm(vm: &ChimeraVM) -> Self {
        let mut world = Self::new();
        let room_w = 400.0;
        let room_h = 300.0;

        // Collect stack frames: (strand_idx, gene_idx)
        // vm.call_stack stores return addresses.
        // We should also include the *current* execution point as the deepest room.
        let mut frames = vm.call_stack.clone();
        frames.push(vm.ip);

        for (i, frame) in frames.iter().enumerate() {
            let (strand_idx, gene_idx) = *frame;

            // Get op info
            let mut label = format!("Frame {}", i);
            let mut description = format!("Strand {} Gene {}", strand_idx, gene_idx);

            if strand_idx < vm.dna.helix.strands.len() {
                let strand = &vm.dna.helix.strands[strand_idx];
                if gene_idx < strand.genes.len() {
                    let gene = &strand.genes[gene_idx];
                    label = format!("{}", gene.op);
                    description = format!("S{} G{} ({})", strand_idx, gene_idx, gene.op);

                    // Add args to description
                    for arg in &gene.args {
                        description.push_str(&format!(" {:?}", arg));
                    }
                } else {
                    label = "EOF".to_string();
                }
            } else {
                label = "VOID".to_string();
            }

            // Color based on Strand Index to show "thread" context
            // Use a golden ratio offset to separate strand colors nicely
            let hue = (strand_idx as f32 * 0.61803398875) % 1.0;
            let room_color = hsl_to_color(hue, 0.5, 0.2); // Darker background

            let mut portals = Vec::new();

            // Link to next frame
            if i + 1 < frames.len() {
                let next_strand = frames[i + 1].0;
                let next_hue = (next_strand as f32 * 0.61803398875) % 1.0;
                let portal_color = hsl_to_color(next_hue, 0.8, 0.5); // Brighter portal

                let portal_w = 150.0;
                let portal_h = 100.0;

                // Spiral offset
                let offset_x = (i as f32 * 0.5).sin() * 50.0;
                let offset_y = (i as f32 * 0.5).cos() * 50.0;

                let portal_rect = Rect::new(
                    (room_w - portal_w) / 2.0 + offset_x,
                    (room_h - portal_h) / 2.0 + offset_y,
                    portal_w,
                    portal_h,
                );

                portals.push(Portal {
                    target_room_id: i + 1,
                    rect: portal_rect,
                    color: portal_color,
                });
            }

            let room = Room {
                id: i,
                rect: Rect::new(0.0, 0.0, room_w, room_h),
                color: room_color,
                portals,
                label,
                description,
            };

            world.rooms.insert(i, room);
        }

        world
    }
}

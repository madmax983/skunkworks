use macroquad::prelude::*;
use std::collections::HashMap;
use crate::logic::{TraceSegment, SegmentType, parse_trace};

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
    pub segment_type: SegmentType,
}

pub struct World {
    pub rooms: HashMap<usize, Room>,
    pub root_id: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            root_id: 0,
        }
    }

    pub fn from_trace(trace_input: &str) -> Self {
        let segments = parse_trace(trace_input);
        let mut world = Self::new();

        // Base room dimensions
        let room_w = 400.0;
        let room_h = 300.0;

        for (i, segment) in segments.iter().enumerate() {
            let room_color = match segment.segment_type {
                SegmentType::System => Color::new(0.2, 0.2, 0.3, 1.0), // Dark Blue-ish
                SegmentType::User => Color::new(0.4, 0.2, 0.2, 1.0),   // Dark Red-ish
            };

            let mut portals = Vec::new();

            // If there is a next segment, add a portal to it
            if i + 1 < segments.len() {
                // Determine portal color based on *next* room type
                let next_type = &segments[i+1].segment_type;
                let portal_color = match next_type {
                    SegmentType::System => BLUE,
                    SegmentType::User => RED,
                };

                // Portal dimensions relative to room
                let portal_w = 100.0;
                let portal_h = 80.0;

                // Position portal in center for the infinite tunnel effect
                // Or maybe slightly offset to create a spiral?
                // Let's do a slight spiral.
                let angle = (i as f32) * 0.5; // Rotate placement?
                // No, alloc-tardis uses Axis Aligned Rects. We can only translate.

                let offset_x = (i as f32 * 10.0).sin() * 50.0;
                let offset_y = (i as f32 * 10.0).cos() * 50.0;

                let portal_rect = Rect::new(
                    (room_w - portal_w) / 2.0 + offset_x,
                    (room_h - portal_h) / 2.0 + offset_y,
                    portal_w,
                    portal_h
                );

                portals.push(Portal {
                    target_room_id: i + 1,
                    rect: portal_rect,
                    color: portal_color,
                });
            }

            let room = Room {
                id: i,
                rect: Rect::new(0.0, 0.0, room_w, room_h), // All rooms same local coords
                color: room_color,
                portals,
                label: segment.content.clone(),
                segment_type: segment.segment_type.clone(),
            };

            world.rooms.insert(i, room);
        }

        world
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::logic::SegmentType;

    #[test]
    fn test_world_from_trace() {
        let input = "
stack backtrace:
   0: std::sys::unix::process::process_common::Command::new
             at /rustc/std/src/sys/unix/process/process_common.rs:163
   1: my_app::main
             at src/main.rs:10
";
        let world = World::from_trace(input);
        assert_eq!(world.rooms.len(), 2);

        let room0 = world.rooms.get(&0).unwrap();
        assert_eq!(room0.segment_type, SegmentType::System);
        assert_eq!(room0.portals.len(), 1);
        assert_eq!(room0.portals[0].target_room_id, 1);

        let room1 = world.rooms.get(&1).unwrap();
        assert_eq!(room1.segment_type, SegmentType::User);
        assert_eq!(room1.portals.len(), 0);
    }
}

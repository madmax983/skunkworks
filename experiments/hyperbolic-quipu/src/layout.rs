use macroquad::prelude::*;
use poincare_disk::{mobius_add, mobius_sub, Point};
use quipu::Cord;
use crate::fs::DirNode;
use std::f64::consts::PI;

pub struct QuipuNode {
    pub cord: Cord,
    pub start_pos: Point,
    pub end_pos: Point,
    pub children: Vec<QuipuNode>,
    pub name: String,
    pub info: String,
    pub color: Color,
}

impl QuipuNode {
    pub fn new(node: &DirNode, parent_end: Point, angle: f64, depth: usize) -> Self {
        // 1. Create Cord from directory size (sum of files in this dir)
        // Note: node.self_size is the size of the directory entry itself usually.
        // We want size of files *in* this directory.
        // scan_dir logic: total_size includes children.
        // self_size is just the dir entry.
        // We need to sum file children.
        let mut files_size = 0;
        for child in &node.children {
            if !child.is_dir {
                files_size += child.total_size;
            }
        }

        let cord = Cord::from(files_size);

        // 2. Determine Length based on number of knots + number of subdirs
        // Visual length.
        let num_clusters = cord.clusters.len();
        // Count subdirectories
        let subdirs: Vec<&DirNode> = node.children.iter().filter(|c| c.is_dir).collect();
        let num_subdirs = subdirs.len();

        // Base length
        let len = 0.2 + (num_clusters as f64 * 0.05) + (num_subdirs as f64 * 0.05);
        // Clamp length to avoid going too close to edge too fast
        let len = len.min(0.8);

        // 3. Calculate End Point
        // We move 'len' distance in 'angle' direction from 'parent_end' (which is our start_pos).
        // To do this in hyperbolic space:
        // Map start_pos to origin.
        // Move 'len' in 'angle' direction (which is just polar coords in disk).
        // Map back.

        // Wait, angle is tricky.
        // If we just use global angle, it distorts.
        // Better:
        // 1. Transform parent_end to Origin.
        // 2. The "incoming" direction is determined by mapping parent_start to Origin.
        // 3. We want to branch "outwards" from the incoming direction.

        // Let's assume for Root, start is (0,0), angle is -PI/2 (Down).
        // For others, we calculate based on parent.

        // Simpler approach for "Finder":
        // Just use `mobius_add`.
        // The offset vector (in the tangent space of origin) is rotated by `angle`.
        // Then we apply the translation to `parent_end`.

        // But `angle` needs to be relative to something.
        // Let's pass the "global" angle, but adjust it.
        // Actually, let's use the same logic as fractal trees.

        use num_complex::Complex;
        let offset_local = Complex::from_polar(len, angle);
        // The point in the disk corresponding to this offset from origin
        let p_local = Point::new(offset_local.re, offset_local.im);

        // Now translate this point to start_pos
        // P_global = start_pos (+) P_local
        let start_pos = parent_end;
        let end_pos = mobius_add(start_pos, p_local); // Note: mobius_add(a, b) is (a+b)/(1+ab_bar)
                                                     // This effectively moves b "from" a.

        let mut quipu_node = QuipuNode {
            cord,
            start_pos,
            end_pos,
            children: Vec::new(),
            name: node.name.clone(),
            info: format!("{}", files_size),
            color: match depth % 4 {
                0 => RED,
                1 => BLUE,
                2 => GREEN,
                _ => YELLOW,
            },
        };

        // 4. Create Children (Subdirectories)
        // Spread them along the angle fan?
        // Or spread them along the cord length?
        // Quipu: Subsidiary cords hang FROM the main cord.
        // So we pick points ALONG the geodesic (start_pos -> end_pos).
        // And branch off from there.

        if !subdirs.is_empty() {
            // Distribute attachment points along the cord
            // We have `num_subdirs`.
            // Let's place them from 20% to 100% of the length.

            for (i, subdir) in subdirs.iter().enumerate() {
                let t = 0.2 + (i as f64 / num_subdirs as f64) * 0.8;

                // Point along geodesic
                // To find point at fraction t:
                // Map start to origin. End maps to some point P'.
                // The point we want is t * P' (linear scaling in disk works for geodesics through origin).
                // Map back.

                let m_end = mobius_sub(end_pos, start_pos);
                let p_t_local = m_end * t;
                let attach_point = mobius_add(start_pos, p_t_local);

                // Branch Angle
                // We want to branch "out" from the cord.
                // The cord direction at Origin was `angle`.
                // Let's branch at `angle + PI/2` and `angle - PI/2`?
                // Or fan them out.
                // Let's vary angle slightly.
                // Branch Angle: Alternate left/right slightly
                let branch_angle = angle + if i % 2 == 0 { PI / 3.0 } else { -PI / 3.0 };

                let child_node = QuipuNode::new(subdir, attach_point, branch_angle, depth + 1);
                quipu_node.children.push(child_node);
            }
        }

        quipu_node
    }
}

pub fn layout_quipu(root: &DirNode) -> QuipuNode {
    // Root starts at (0,0) and goes Down (-PI/2)
    // Actually, let's start it slightly up so we can see it hanging.
    // Start at (0, 0.5) (top) and go down.
    // But (0,0) is center.
    // Let's start at (0,0) for simplicity of camera.

    // We pass (0,0) as "parent_end" so the root starts there.
    QuipuNode::new(root, Point::new(0.0, 0.0), -PI / 2.0, 0)
}

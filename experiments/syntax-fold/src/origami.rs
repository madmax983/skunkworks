use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub pos: Vec3,
    pub old_pos: Vec3,
    pub acc: Vec3,
    pub mass: f32,
    pub fixed: bool,
    pub uv: Vec2,
}

impl Vertex {
    pub fn new(pos: Vec3, uv: Vec2, fixed: bool) -> Self {
        Self {
            pos,
            old_pos: pos,
            acc: Vec3::ZERO,
            mass: 1.0,
            fixed,
            uv,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Edge {
    pub a: usize,
    pub b: usize,
    pub length: f32,
    pub wings: [Option<usize>; 2],
}

#[derive(Clone, Debug)]
pub struct Face {
    pub indices: [usize; 3],
    pub color: Color,
}

#[derive(Clone, Debug)]
pub struct Crease {
    pub edge_index: usize,
    pub design_angle: f32, // Target when fully folded
    pub current_target: f32, // Current physics target
    pub stiffness: f32,
    pub is_mountain: bool,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub creases: Vec<Crease>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
            faces: Vec::new(),
            creases: Vec::new(),
        }
    }

    pub fn add_vertex(&mut self, pos: Vec3, uv: Vec2, fixed: bool) -> usize {
        let idx = self.vertices.len();
        self.vertices.push(Vertex::new(pos, uv, fixed));
        idx
    }

    pub fn add_triangle(&mut self, a: usize, b: usize, c: usize, color: Color) {
        self.faces.push(Face { indices: [a, b, c], color });

        self.register_edge(a, b, c);
        self.register_edge(b, c, a);
        self.register_edge(c, a, b);
    }

    fn register_edge(&mut self, a: usize, b: usize, wing: usize) {
        let (min, max) = if a < b { (a, b) } else { (b, a) };

        if let Some(idx) = self.edges.iter().position(|e| e.a == min && e.b == max) {
            let edge = &mut self.edges[idx];
            if edge.wings[0].is_none() {
                edge.wings[0] = Some(wing);
            } else if edge.wings[1].is_none() {
                edge.wings[1] = Some(wing);
            }
        } else {
            let v_a = self.vertices[min].pos;
            let v_b = self.vertices[max].pos;
            let length = v_a.distance(v_b);
            self.edges.push(Edge {
                a: min,
                b: max,
                length,
                wings: [Some(wing), None]
            });
        }
    }

    pub fn get_edge_index(&self, a: usize, b: usize) -> Option<usize> {
        let (min, max) = if a < b { (a, b) } else { (b, a) };
        self.edges.iter().position(|e| e.a == min && e.b == max)
    }

    pub fn add_crease(&mut self, edge_idx: usize, design_angle: f32, stiffness: f32, is_mountain: bool) {
        self.creases.push(Crease {
            edge_index: edge_idx,
            design_angle,
            current_target: std::f32::consts::PI, // Start flat
            stiffness,
            is_mountain
        });
    }
}

use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn dist(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn add(&self, other: &Point) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn sub(&self, other: &Point) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn scale(&self, s: f64) -> Self {
        Self {
            x: self.x * s,
            y: self.y * s,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TriangleType {
    Acute,  // 36-72-72 (half thick rhombus)
    Obtuse, // 108-36-36 (half thin rhombus)
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub a: Point, // Apex (36 for Acute, 108 for Obtuse)
    pub b: Point, // Base Left
    pub c: Point, // Base Right
    pub t_type: TriangleType,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point, t_type: TriangleType) -> Self {
        Self { a, b, c, t_type }
    }

    pub fn center(&self) -> Point {
        Point {
            x: (self.a.x + self.b.x + self.c.x) / 3.0,
            y: (self.a.y + self.b.y + self.c.y) / 3.0,
        }
    }
}

pub struct PenroseTiling<T> {
    pub triangles: Vec<Triangle>,
    pub adjacency: Vec<Vec<usize>>,
    pub data: Vec<T>,
}

impl<T: Default + Clone> PenroseTiling<T> {
    // Generate a sun pattern (5 thick rhombi -> 10 acute triangles)
    pub fn generate_sun(radius: f64) -> Self {
        let mut triangles = Vec::new();
        let center = Point::new(0.0, 0.0);

        for i in 0..10 {
            // 10 triangles. 2*PI / 10 = PI/5 = 36 deg.
            let t_angle1 = (i as f64) * PI / 5.0;
            let t_angle2 = (i + 1) as f64 * PI / 5.0;

            let b = Point::new(radius * t_angle1.cos(), radius * t_angle1.sin());
            let c = Point::new(radius * t_angle2.cos(), radius * t_angle2.sin());

            // We use 10 Acute triangles with Apex at center.
            // This forms a Decagon.

            if i % 2 == 0 {
                triangles.push(Triangle::new(center, b, c, TriangleType::Acute));
            } else {
                // Mirror it for symmetry
                triangles.push(Triangle::new(center, c, b, TriangleType::Acute));
            }
        }

        let len = triangles.len();
        Self {
            triangles,
            adjacency: Vec::new(),
            data: vec![T::default(); len],
        }
    }

    pub fn build_adjacency(&mut self) {
        let n = self.triangles.len();
        let mut adj = vec![Vec::new(); n];
        let epsilon = 1e-4;

        for i in 0..n {
            for j in (i + 1)..n {
                // Check if they share an edge (2 shared vertices)
                let t1 = &self.triangles[i];
                let t2 = &self.triangles[j];

                let mut shared = 0;
                let pts1 = [t1.a, t1.b, t1.c];
                let pts2 = [t2.a, t2.b, t2.c];

                for p1 in &pts1 {
                    for p2 in &pts2 {
                        if p1.dist(p2) < epsilon {
                            shared += 1;
                            break; // matched p1
                        }
                    }
                }

                if shared >= 2 {
                    adj[i].push(j);
                    adj[j].push(i);
                }
            }
        }
        self.adjacency = adj;
    }

    pub fn get_closest_neighbor(&self, current: usize, dir: Point) -> Option<usize> {
        if current >= self.adjacency.len() {
            return None;
        }

        let neighbors = &self.adjacency[current];
        if neighbors.is_empty() {
            return None;
        }

        let current_center = self.triangles[current].center();

        // Find neighbor whose center is closest to the direction vector relative to current center
        // Better: Maximize dot product (Center_N - Center_C) . Dir

        let mut best_idx = None;
        let mut max_score = -f64::INFINITY;

        for &n_idx in neighbors {
            let n_center = self.triangles[n_idx].center();
            let vec_to_n = n_center.sub(&current_center);

            let score = vec_to_n.x * dir.x + vec_to_n.y * dir.y;

            if score > max_score {
                max_score = score;
                best_idx = Some(n_idx);
            }
        }

        if max_score > 0.0 {
            best_idx
        } else {
            None
        }
    }

    pub fn get_closest_triangle(&self, p: Point) -> Option<usize> {
        let mut min_dist = f64::INFINITY;
        let mut best_idx = None;
        for (i, t) in self.triangles.iter().enumerate() {
            let dist = t.center().dist(&p);
            if dist < min_dist {
                min_dist = dist;
                best_idx = Some(i);
            }
        }
        best_idx
    }

    pub fn subdivide(&mut self) {
        let mut new_triangles = Vec::new();
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

        for t in &self.triangles {
            match t.t_type {
                TriangleType::Acute => {
                    let p = t.c.add(&t.a.sub(&t.c).scale(1.0 / phi));
                    new_triangles.push(Triangle::new(t.b, p, t.c, TriangleType::Acute));
                    new_triangles.push(Triangle::new(p, t.a, t.b, TriangleType::Obtuse));
                }
                TriangleType::Obtuse => {
                    let p = t.b.add(&t.c.sub(&t.b).scale(1.0 / phi));
                    new_triangles.push(Triangle::new(p, t.b, t.a, TriangleType::Obtuse));
                    new_triangles.push(Triangle::new(t.c, t.a, p, TriangleType::Acute));
                }
            }
        }

        self.triangles = new_triangles;
        // Resize data to match new count. Since subdivision doubles count exactly in this implementation:
        self.data = vec![T::default(); self.triangles.len()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdivision_counts() {
        let mut tiling: PenroseTiling<()> = PenroseTiling::generate_sun(100.0);
        assert_eq!(tiling.triangles.len(), 10);
        assert_eq!(tiling.data.len(), 10);

        // Iteration 1: 10 * 2 = 20
        tiling.subdivide();
        assert_eq!(tiling.triangles.len(), 20);
        assert_eq!(tiling.data.len(), 20);

        // Iteration 2: 20 * 2 = 40
        tiling.subdivide();
        assert_eq!(tiling.triangles.len(), 40);
        assert_eq!(tiling.data.len(), 40);
    }

    #[test]
    fn test_connectivity() {
        let mut tiling: PenroseTiling<()> = PenroseTiling::generate_sun(100.0);
        tiling.subdivide();
        tiling.build_adjacency();

        // Check that every tile has neighbors
        for i in 0..tiling.triangles.len() {
            assert!(
                !tiling.adjacency[i].is_empty(),
                "Tile {} has no neighbors",
                i
            );
        }

        let start = 0;
        let dir = Point::new(1.0, 0.0);
        // Check movement returns a result
        let _next = tiling.get_closest_neighbor(start, dir);
    }
}

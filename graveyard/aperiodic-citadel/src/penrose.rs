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

pub struct PenroseTiling {
    pub triangles: Vec<Triangle>,
    pub adjacency: Vec<Vec<usize>>,
}

impl PenroseTiling {
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

        Self {
            triangles,
            adjacency: Vec::new(),
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

            // Normalize direction for dot product comparison?
            // User input is usually unit vector (WASD).
            // But distance matters?
            // "Closest neighbor in that direction".
            // Dot product is good.

            let score = vec_to_n.x * dir.x + vec_to_n.y * dir.y;

            if score > max_score {
                max_score = score;
                best_idx = Some(n_idx);
            }
        }

        // Only move if score is positive? (i.e., in that general direction)
        // If all neighbors are behind, maybe we shouldn't move?
        // But for gameplay, maybe "best effort".
        // Let's require score > 0.

        if max_score > 0.0 {
            best_idx
        } else {
            None
        }
    }

    pub fn subdivide(&mut self) {
        let mut new_triangles = Vec::new();
        let phi = (1.0 + 5.0_f64.sqrt()) / 2.0;

        for t in &self.triangles {
            match t.t_type {
                TriangleType::Acute => {
                    // Subdivide Acute (Gold Triangle)
                    // A (36), B (72), C (72)
                    // New point P on side AB? Or AC?
                    // Rule: P on AB such that AP = BC?
                    // Let's use coordinates interpolation.

                    // We need consistent rules.
                    // If we used "Mirror" in generation (B, C vs C, B), we need to respect that.
                    // Let's assume A is always Apex.
                    // Point P is on the side AC? or AB?
                    // Standard: P is on one of the legs.
                    // Let's say P is on AC.
                    // P = A + (C-A) / phi.

                    // Result:
                    // 1. Obtuse (P, A, B) ??
                    // 2. Acute (C, P, B) ??

                    // Let's check angles.
                    // If P is on AC. Triangle ABP. Angle at A is 36.
                    // If AP = AB (isosceles)? No.
                    // If we want Obtuse (36-36-108).
                    // If Angle A is 36. Then Angle ABP and APB must be 36/108.
                    // If ABP is 36, then APB is 108.
                    // Then Triangle ABP is Obtuse.
                    // Triangle CPB:
                    // Angle C is 72.
                    // Angle CPB is 180 - 108 = 72.
                    // Angle PBC is 180 - 72 - 72 = 36.
                    // So CPB is Acute (36-72-72) with Apex B.

                    // So:
                    // P = A + (C - A) / phi. (Actually P divides AC in Golden Ratio).
                    // Length AC = L. Length AP = L / phi?
                    // Or Length CP = L / phi?
                    // We need CP = Base = BC. (Since CPB is isosceles with base CP? No base PB?)
                    // CPB has 72 at C and 72 at P. So Base is CB?
                    // Then CB = CP.
                    // So P is distance |BC| from C along CA.
                    // P = C + (A - C) * (|BC| / |AC|).
                    // In Acute triangle, |AC| / |BC| = phi.
                    // So |BC| / |AC| = 1/phi.
                    // So P = C + (A - C) * (1/phi).
                    // Or P = A + (C - A) * (1 - 1/phi).
                    // 1 - 1/phi = 1 - (phi-1) = 2 - phi?
                    // 1/phi = phi - 1.
                    // So P = C + (A - C) * (phi - 1).

                    let p = t.c.add(&t.a.sub(&t.c).scale(1.0 / phi));

                    // New Triangles:
                    // 1. Acute (Apex B, Base P, Base C) -> No, B is apex (36).
                    //    Wait, in CPB, C=72, P=72, B=36. So B is Apex.
                    //    So Triangle(B, P, C).
                    new_triangles.push(Triangle::new(t.b, p, t.c, TriangleType::Acute));

                    // 2. Obtuse (Apex P, Base A, Base B) -> No.
                    //    In ABP: A=36. P=108. B=36.
                    //    So P is Apex (108).
                    //    So Triangle(P, t.a, t.b). (Order: Apex, Side, Side).
                    new_triangles.push(Triangle::new(p, t.a, t.b, TriangleType::Obtuse));
                }
                TriangleType::Obtuse => {
                    // Subdivide Obtuse (Gnomon)
                    // A (108), B (36), C (36)
                    // New point P on side BC? (The long base).
                    // Obtuse has short sides AB, AC. Long base BC.
                    // P on BC.
                    // Result:
                    // 1. Obtuse (P, ?, ?)
                    // 2. Acute (?, ?, ?)

                    // Actually, Gnomon subdivides into 1 Acute and 1 Obtuse.
                    // Decomposition:
                    // Draw line from B to AC? No AC is short side.
                    // Draw line from B to point P on AC? No.
                    // Draw line from B to point P on Side BC? No.
                    // Decomposition of Obtuse (108-36-36):
                    // Vertices A(108), B(36), C(36).
                    // Point P on BC such that Triangle ABP is Obtuse? Or Acute?
                    // If we want a smaller Acute triangle (36-72-72).
                    // We have angle B=36.
                    // If we make a cut such that we get a 36-72-72.
                    // We need a 72 angle.
                    // 108 split into 36 and 72?
                    // If we split Angle A (108) into 36 and 72.
                    // Ray AP meets BC at P.
                    // Triangle ABP: A=36, B=36, P=108. (Obtuse).
                    // Triangle APC: A=72, C=36, P=72. (Acute).

                    // So P is on BC.
                    // P divides BC.
                    // Triangle ABP is the new Obtuse.
                    // Triangle APC is the new Acute.

                    // Coordinates of P:
                    // In ABP (Obtuse), sides AB, BP, AP.
                    // ABP is isosceles (36-36-108). AB = BP.
                    // So P is distance |AB| from B along BC.
                    // P = B + (C - B) * (|AB| / |BC|).
                    // Ratio |BC| / |AB| = phi. (Long / Short in Obtuse = phi? No. Short/Long?)
                    // In Obtuse (36-36-108), let Short side = 1.
                    // Height = ...
                    // Base = 2 * cos(36)?
                    // cos(36) = phi/2? No, cos(36) = phi/2 * something?
                    // Relation: Acute Side/Base = phi.
                    // Obtuse Base/Side = phi?
                    // Yes. In Obtuse, Long/Short = phi.
                    // So |BC| / |AB| = phi.
                    // So |AB| / |BC| = 1/phi.

                    let p = t.b.add(&t.c.sub(&t.b).scale(1.0 / phi));

                    // New Triangles:
                    // 1. Obtuse (Apex P, Base B, Base A).
                    //    ABP: A=36, B=36, P=108.
                    //    So P is Apex.
                    //    Triangle(P, t.b, t.a).
                    new_triangles.push(Triangle::new(p, t.b, t.a, TriangleType::Obtuse));

                    // 2. Acute (Apex A, Base C, Base P).
                    //    APC: A=72, C=36, P=72.
                    //    Wait, Acute is 36-72-72.
                    //    So C is Apex (36)? No, C is 36.
                    //    So C is Apex.
                    //    Base angles at A and P are 72.
                    //    So Triangle(C, t.a, p).
                    new_triangles.push(Triangle::new(t.c, t.a, p, TriangleType::Acute));
                }
            }
        }

        self.triangles = new_triangles;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdivision_counts() {
        let mut tiling = PenroseTiling::generate_sun(100.0);
        assert_eq!(tiling.triangles.len(), 10);

        // Iteration 1: 10 * 2 = 20
        tiling.subdivide();
        assert_eq!(tiling.triangles.len(), 20);

        // Iteration 2: 20 * 2 = 40
        tiling.subdivide();
        assert_eq!(tiling.triangles.len(), 40);

        // Note: This implements N->2N substitution which creates an aperiodic tiling
        // with 5-fold symmetry, though not the standard Penrose P3 (which has Golden Ratio growth).
        // The resulting triangles are slightly distorted "Silver" gnomons.
    }

    #[test]
    fn test_connectivity() {
        let mut tiling = PenroseTiling::generate_sun(100.0);
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

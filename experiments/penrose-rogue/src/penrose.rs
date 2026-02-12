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
}

impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Point {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RhombusType {
    Thick, // 72-108
    Thin,  // 36-144
}

#[derive(Clone, Debug)]
pub struct Rhombus {
    pub vertices: [Point; 4],
    pub kind: RhombusType,
    pub grid_indices: (usize, usize), // Which grids intersected (r, s)
}

impl Rhombus {
    pub fn center(&self) -> Point {
        let sum = self.vertices[0] + self.vertices[1] + self.vertices[2] + self.vertices[3];
        Point::new(sum.x / 4.0, sum.y / 4.0)
    }
}

pub struct Pentagrid {
    gammas: [f64; 5],
    vectors: [Point; 5],
}

impl Pentagrid {
    pub fn new(seed: u64) -> Self {
        // Use random gammas based on seed to generate different tilings.
        // Condition: sum of gammas should not result in integer intersections.
        use rand::{Rng, SeedableRng, rngs::StdRng};
        let mut rng = StdRng::seed_from_u64(seed);
        let mut gammas = [0.0; 5];
        for i in 0..5 {
            gammas[i] = rng.r#gen::<f64>();
        }

        let mut vectors = [Point::new(0.0, 0.0); 5];
        for i in 0..5 {
            let theta = 2.0 * PI * (i as f64) / 5.0;
            vectors[i] = Point::new(theta.cos(), theta.sin());
        }

        Self { gammas, vectors }
    }

    /// Generate rhombi within a bounding box defined by range of grid lines.
    /// This is an approximation; we iterate grid lines that *might* intersect in the view.
    /// bounds: (min_x, max_x, min_y, max_y) in world space?
    /// Actually, let's take a range of grid indices (approx -20 to 20).
    pub fn generate_tiling(&self, range: i32) -> Vec<Rhombus> {
        let mut rhombi = Vec::new();

        // Iterate all pairs of grids (r, s)
        for r in 0..5 {
            for s in (r + 1)..5 {
                // Find intersections of grid lines
                // Line in grid r: x . v_r + gamma_r = k_r
                // Line in grid s: x . v_s + gamma_s = k_s

                // Solve linear system for x (Point):
                // x*vr_x + y*vr_y = kr - gr
                // x*vs_x + y*vs_y = ks - gs

                // Determinant
                let det =
                    self.vectors[r].x * self.vectors[s].y - self.vectors[r].y * self.vectors[s].x;
                if det.abs() < 1e-9 {
                    continue;
                } // Parallel (should not happen for 72 deg)

                for kr in -range..=range {
                    for ks in -range..=range {
                        let c_r = (kr as f64) - self.gammas[r];
                        let c_s = (ks as f64) - self.gammas[s];

                        let ix = (c_r * self.vectors[s].y - c_s * self.vectors[r].y) / det;
                        let iy = (self.vectors[r].x * c_s - self.vectors[s].x * c_r) / det;
                        let intersection = Point::new(ix, iy);

                        // Check if intersection is within reasonable world bounds if needed.
                        // Here we just accept all in index range.

                        // Calculate indices km for m != r, s
                        let mut k = [0; 5];
                        k[r] = kr; // Note: kr is the line index.
                        k[s] = ks;

                        // For Pentagrid, the integer coordinates of the tile are determined by ceiling.
                        // The intersection lies on lines r and s.
                        // For other grids, we evaluate ceil(x . v_m + gamma_m).

                        for m in 0..5 {
                            if m != r && m != s {
                                let val = intersection.x * self.vectors[m].x
                                    + intersection.y * self.vectors[m].y
                                    + self.gammas[m];
                                k[m] = val.ceil() as i32;
                            }
                        }

                        // The 4 vertices of the Rhombus are formed by summing basis vectors.
                        // The intersection defines a transition.
                        // The "indices" of the region shift by 1 when crossing a line.
                        // At intersection of r and s, we cross both.
                        // Vertices correspond to the 4 regions around the intersection.
                        // Region 1: (..., kr, ..., ks, ...) -> Sum K v
                        // Region 2: (..., kr-1, ..., ks, ...) -> Sum K' v
                        // ...

                        // Wait, de Bruijn formula:
                        // Center of Rhombus? Or Vertices?
                        // Vertices are sums of integers n_i * v_i.
                        // At the intersection, the four adjacent open regions have indices:
                        // k (base), then k with r-1, k with s-1, k with r-1 and s-1.
                        // The integers k_m we computed (using ceil) are the indices of the region "just above" the lines.
                        // So the region (kr, ks) corresponds to one vertex.

                        let base_k = k;
                        let mut v_points = [Point::new(0.0, 0.0); 4];

                        // Vertex 0: base_k
                        v_points[0] = self.compute_vertex(&base_k);

                        // Vertex 1: base_k with r-1
                        let mut k_r_minus = base_k;
                        k_r_minus[r] -= 1;
                        v_points[1] = self.compute_vertex(&k_r_minus);

                        // Vertex 2: base_k with r-1, s-1
                        let mut k_rs_minus = base_k;
                        k_rs_minus[r] -= 1;
                        k_rs_minus[s] -= 1;
                        v_points[2] = self.compute_vertex(&k_rs_minus);

                        // Vertex 3: base_k with s-1
                        let mut k_s_minus = base_k;
                        k_s_minus[s] -= 1;
                        v_points[3] = self.compute_vertex(&k_s_minus);

                        // Determine Rhombus Type
                        // Depends on (s - r) mod 5.
                        // If (s-r)%5 == 1 or 4 -> Thin?
                        // If (s-r)%5 == 2 or 3 -> Thick?
                        let diff = (s as i32 - r as i32).abs() % 5;
                        // diff 1 (72 deg) -> Thick
                        // diff 2 (144 deg) -> Thin
                        let kind = if diff == 1 || diff == 4 {
                            RhombusType::Thick
                        } else {
                            RhombusType::Thin
                        };

                        rhombi.push(Rhombus {
                            vertices: v_points,
                            kind,
                            grid_indices: (r, s),
                        });
                    }
                }
            }
        }
        rhombi
    }

    fn compute_vertex(&self, k: &[i32; 5]) -> Point {
        let mut sum = Point::new(0.0, 0.0);
        for i in 0..5 {
            sum = sum + self.vectors[i] * (k[i] as f64);
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pentagrid_generation() {
        let grid = Pentagrid::new(0);
        let tiles = grid.generate_tiling(5);
        println!("Generated {} tiles.", tiles.len());
        assert!(tiles.len() > 0);

        // Basic consistency check
        for tile in tiles.iter().take(5) {
            println!("Tile: {:?}", tile.kind);
        }
    }
}

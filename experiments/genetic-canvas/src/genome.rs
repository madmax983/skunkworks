use rand::Rng;
use ratatui::style::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum ShapeType {
    Rect,
    Line,
    Circle,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    // Coordinates are normalized 0.0 to 1.0
    pub x: f64,
    pub y: f64,
    pub w: f64, // or x2 for line, or radius for circle
    pub h: f64, // or y2 for line
    pub color: Color,
}

impl Shape {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let shape_type = match rng.gen_range(0..3) {
            0 => ShapeType::Rect,
            1 => ShapeType::Line,
            2 => ShapeType::Circle,
            _ => ShapeType::Rect,
        };

        Shape {
            shape_type,
            x: rng.gen(),
            y: rng.gen(),
            w: rng.gen(),
            h: rng.gen(),
            color: random_color(),
        }
    }

    pub fn mutate(&mut self, rate: f64) {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < rate {
            // Mutate Position
            self.x = (self.x + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
            self.y = (self.y + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            // Mutate Size
            self.w = (self.w + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
            self.h = (self.h + rng.gen_range(-0.1..0.1)).clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            // Mutate Color
            self.color = random_color();
        }
    }
}

fn random_color() -> Color {
    let colors = [
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::LightRed,
        Color::LightGreen,
        Color::LightYellow,
        Color::LightBlue,
        Color::LightMagenta,
        Color::LightCyan,
        Color::Gray,
        Color::DarkGray,
    ];
    let mut rng = rand::thread_rng();
    colors[rng.gen_range(0..colors.len())]
}

#[derive(Clone, Debug)]
pub struct Genome {
    pub shapes: Vec<Shape>,
}

impl Genome {
    pub fn random(size: usize) -> Self {
        let shapes = (0..size).map(|_| Shape::random()).collect();
        Genome { shapes }
    }

    pub fn mutate(&mut self, rate: f64) {
        let mut rng = rand::thread_rng();
        for shape in &mut self.shapes {
            shape.mutate(rate);
        }

        // Add/Remove shapes
        if rng.gen::<f64>() < rate {
            if rng.gen_bool(0.5) && self.shapes.len() < 50 {
                self.shapes.push(Shape::random());
            } else if self.shapes.len() > 1 {
                self.shapes.remove(rng.gen_range(0..self.shapes.len()));
            }
        }
    }

    pub fn crossover(parent1: &Genome, parent2: &Genome) -> Genome {
        let mut rng = rand::thread_rng();
        // Uniform crossover
        let len = std::cmp::max(parent1.shapes.len(), parent2.shapes.len());
        let mut child_shapes = Vec::new();

        for i in 0..len {
            if i < parent1.shapes.len() && i < parent2.shapes.len() {
                if rng.gen_bool(0.5) {
                    child_shapes.push(parent1.shapes[i].clone());
                } else {
                    child_shapes.push(parent2.shapes[i].clone());
                }
            } else if i < parent1.shapes.len() && rng.gen_bool(0.5) {
                child_shapes.push(parent1.shapes[i].clone());
            } else if i < parent2.shapes.len() && rng.gen_bool(0.5) {
                child_shapes.push(parent2.shapes[i].clone());
            }
        }

        Genome {
            shapes: child_shapes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_genome() {
        let genome = Genome::random(10);
        assert_eq!(genome.shapes.len(), 10);
    }

    #[test]
    fn test_mutation_changes_genome() {
        let mut genome = Genome::random(10);
        let original = genome.clone();
        // High mutation rate to ensure change
        genome.mutate(1.0);

        // It's possible but unlikely that nothing changed or it mutated back.
        // We check if at least something is different.
        // Actually, with rate 1.0, positions should shift.
        let mut changed = false;
        if genome.shapes.len() != original.shapes.len() {
            changed = true;
        } else {
            for (s1, s2) in genome.shapes.iter().zip(original.shapes.iter()) {
                if s1.x != s2.x || s1.y != s2.y || s1.color != s2.color {
                    changed = true;
                    break;
                }
            }
        }
        assert!(changed, "Genome should have mutated");
    }

    #[test]
    fn test_crossover() {
        let p1 = Genome::random(10);
        let p2 = Genome::random(10);
        let child = Genome::crossover(&p1, &p2);

        // Child length should be roughly 10 (or exactly 10 in this implementation since lengths match)
        assert!(child.shapes.len() <= 10);
    }
}

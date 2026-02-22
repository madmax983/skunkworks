use chimera_lang::prelude::*;
use chimera_lang::vm::ChimeraVM;
use nalgebra::Point2;
use rapier2d::prelude::*;

pub struct GeneticDesigner {
    pub vm: ChimeraVM,
    pub dna: Dna,
}

impl GeneticDesigner {
    pub fn new(genes: Vec<OpCode>) -> Self {
        // Convert OpCodes to Dna
        // If OpCode is Push, we need to generate a random argument
        let strand_genes: Vec<Gene> = genes
            .into_iter()
            .map(|op| {
                let args = if matches!(op, OpCode::Push) {
                    // Generate a random number 0-100
                    let n = macroquad::rand::gen_range(0, 100);
                    vec![Nucleotide::Number(n as i64)]
                } else {
                    vec![]
                };
                Gene { op, args }
            })
            .collect();

        let dna = Dna {
            helix: Helix {
                strands: vec![Strand {
                    genes: strand_genes,
                }],
            },
        };

        let vm = ChimeraVM::new(dna.clone());
        Self { vm, dna }
    }

    pub fn generate_cam_shape(&mut self) -> SharedShape {
        // Run the VM
        for _ in 0..500 {
            if self.vm.halted {
                break;
            }
            self.vm.step();
        }

        // Interpret stack
        let stack: &Vec<Value> = &self.vm.stack;
        let mut points = Vec::new();

        let mut iter = stack.iter();
        while let (Some(r_val), Some(a_val)) = (iter.next(), iter.next()) {
            let radius = match r_val {
                Value::Int(n) => (*n as f32).abs().clamp(0.5, 3.0),
                _ => 1.0,
            };

            let angle = match a_val {
                Value::Int(n) => (*n as f32) * 0.1, // Scale int to angle
                _ => 0.0,
            };

            let x = radius * angle.cos();
            let y = radius * angle.sin();
            points.push(Point2::new(x, y));
        }

        if points.len() < 3 {
            points.push(Point2::new(1.0, 0.0));
            points.push(Point2::new(-0.5, 1.0));
            points.push(Point2::new(-0.5, -1.0));
        }

        SharedShape::convex_hull(&points).unwrap_or(SharedShape::ball(1.0))
    }
}

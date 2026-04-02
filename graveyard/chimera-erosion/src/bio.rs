use crate::leaf::LeafMap;
use chimera_lang::compiler::compile;
use chimera_lang::prelude::*;

#[derive(Debug, Clone)]
pub enum BioAction {
    None,
    GrowRoots(f32),
    Drink(f32),
    Reproduce,
}

pub struct Plant {
    pub x: usize,
    pub y: usize,
    pub vm: ChimeraVM,
    pub energy: i64,
    pub age: u64,
    pub species: String,
    pub dna_source: String, // Keep source for reproduction
}

impl Plant {
    pub fn new(x: usize, y: usize, species: &str, source: &str) -> Self {
        // Compile the source code
        let dna = compile(source, None).unwrap_or_else(|e| {
            println!("Failed to compile plant DNA for {}: {}", species, e);
            // Return empty DNA as fallback
            Dna {
                helix: Helix { strands: vec![] },
            }
        });

        let vm = ChimeraVM::new(dna);

        Self {
            x,
            y,
            vm,
            energy: 50,
            age: 0,
            species: species.to_string(),
            dna_source: source.to_string(),
        }
    }

    pub fn step(&mut self, map: &mut LeafMap) -> BioAction {
        if self.energy <= 0 {
            return BioAction::None;
        }

        self.age += 1;
        self.energy -= 1; // Metabolic cost

        // Ensure plant is still on map
        if !map.is_inside(self.x, self.y) {
            self.energy = 0;
            return BioAction::None;
        }

        let idx = self.y * map.width + self.x;
        let h = map.heightmap[idx];
        let w = map.water[idx];

        // Photosynthesis
        if w < 0.2 {
            // Exposed to sun
            self.energy += 2;
        } else {
            // Underwater
            self.energy -= 1;
        }

        // Prepare Inputs for VM
        // Stack: [Height, Water, Energy, Age]
        self.vm.stack.push(Value::Int((h * 100.0) as i64));
        self.vm.stack.push(Value::Int((w * 100.0) as i64));
        self.vm.stack.push(Value::Int(self.energy));
        self.vm.stack.push(Value::Int(self.age as i64));

        // Execute VM
        for _ in 0..20 {
            self.vm.step();
            if self.vm.halted {
                break;
            }
        }

        // Interpret Output
        if let Some(val) = self.vm.stack.pop() {
            match val {
                Value::Int(1) => {
                    // Grow Roots
                    if self.energy >= 10 {
                        self.energy -= 10;
                        return BioAction::GrowRoots(0.01);
                    }
                }
                Value::Int(2) => {
                    // Drink
                    return BioAction::Drink(0.05);
                }
                Value::Int(3) => {
                    // Reproduce
                    if self.energy >= 30 {
                        self.energy -= 30;
                        return BioAction::Reproduce;
                    }
                }
                _ => {}
            }
        }

        BioAction::None
    }
}

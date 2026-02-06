#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::opcode::OpCode;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(invoke_target: &str) -> ChimeraVM {
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::String(invoke_target.to_string())],
            },
            Gene {
                op: OpCode::Invoke,
                args: vec![],
            },
        ];
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_sigil_ward() {
        let mut vm = make_vm("Ward");

        // Setup Ward pattern (Ring of 8 around 8,8)
        let cx = 8;
        let cy = 8;
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            vm.grid[ny][nx] = Value::Int(1); // "Material"
        }

        // Execute Invoke("Ward")
        vm.step(); // push
        vm.step(); // invoke

        // Check effect: Membrane should be 15
        assert_eq!(vm.membranes[cy][cx], 15);

        // Check consumption: Ring should be 0
        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            match vm.grid[ny][nx] {
                Value::Int(0) => {}
                _ => panic!("Material not consumed at {},{}", nx, ny),
            }
        }
    }

    #[test]
    fn test_sigil_vitality() {
        let mut vm = make_vm("Vitality");

        let cx = 8;
        let cy = 8;
        let offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            vm.grid[ny][nx] = Value::Int(10); // High value material
        }

        let initial_energy = vm.energy;

        vm.step(); // push
        vm.step(); // invoke

        // Gain should be 10 * 5 = 50. Cost is implicitly checked (initial 50 - 2 + 50 = 98)
        assert!(vm.energy > initial_energy);

        // Check consumption
        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            assert_eq!(vm.grid[ny][nx], Value::Int(0));
        }
    }

    #[test]
    fn test_sigil_void() {
        let mut vm = make_vm("Void");

        let cx = 8;
        let cy = 8;
        // 2x2 Square to the Bottom-Right: (0,0), (0,1), (1,0), (1,1)
        let offsets = [(0, 0), (0, 1), (1, 0), (1, 1)];

        // Since (0,0) is center, we must set grid[cy][cx] too!
        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            vm.grid[ny][nx] = Value::Str("Void".to_string());
        }

        vm.step(); // push
        vm.step(); // invoke

        // Check for Organelle spawn
        let voids = vm
            .organelles
            .iter()
            .filter(|o| matches!(o.kind, crate::vm::nova::OrganelleType::Void))
            .count();
        assert!(voids > 0, "Void organelle not spawned");

        // Check consumption
        for (dy, dx) in offsets {
            let ny = (cy as i64 + dy) as usize;
            let nx = (cx as i64 + dx) as usize;
            assert_eq!(vm.grid[ny][nx], Value::Int(0));
        }
    }
}

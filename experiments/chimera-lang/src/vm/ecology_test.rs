#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::vm::ChimeraVM;
    use crate::ast::{Dna, Helix, Strand};
    use crate::vm::nova_ecology;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_spawn_ecology() {
        let mut vm = make_vm();
        nova_ecology::spawn_random_ecology(&mut vm, 10);
        assert_eq!(vm.organelles.len(), 10);
        assert_eq!(vm.dna.helix.strands.len(), 10);
    }

    #[test]
    fn test_spawn_food() {
        let mut vm = make_vm();
        nova_ecology::spawn_food(&mut vm);
        // We can't easily assert where it spawned, but we can check if grid changed
        // But grid is random spawn.
        // Let's spawn 100 food, statistically some should appear.
        for _ in 0..100 {
            nova_ecology::spawn_food(&mut vm);
        }

        let mut food_count = 0;
        for row in &vm.grid {
            for cell in row {
                if let crate::vm::Value::Int(n) = cell {
                    if *n > 0 {
                        food_count += 1;
                    }
                }
            }
        }
        assert!(food_count > 0);
    }
}

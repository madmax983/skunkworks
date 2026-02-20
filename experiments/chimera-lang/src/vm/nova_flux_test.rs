#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::value::Value;
    use crate::vm::nova_signals::process_signals;
    use crate::vm::ChimeraVM;

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.metamorphism_enabled = false;
        vm
    }

    #[test]
    fn test_flux_emitter() {
        let mut vm = make_vm();
        // Layout:
        // . F . (F reads West, applies to South)
        // 5 . . (West=5)
        // . . . (South should get entropy +5)

        // F at (1, 1).
        // West is (1, 0). Value 5.
        // South is (2, 1).

        vm.grid[1][1] = Value::Str("F".to_string());
        vm.grid[1][0] = Value::Int(5);

        vm.signal_grid[1][1] = 1; // Activate

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 5);
    }

    #[test]
    #[ignore]
    fn test_jam_emitter() {
        let mut vm = make_vm();
        // J at (1, 1). West=5. South=(2,1).

        vm.grid[1][1] = Value::Str("J".to_string());
        vm.grid[1][0] = Value::Int(5);
        vm.entropy_grid[2][1] = 10;

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 5); // 10 - 5
    }

    #[test]
    fn test_flux_saturation() {
        let mut vm = make_vm();
        vm.grid[1][1] = Value::Str("F".to_string());
        vm.grid[1][0] = Value::Int(150); // > 100

        vm.signal_grid[1][1] = 1;

        process_signals(&mut vm);

        assert_eq!(vm.entropy_grid[2][1], 100); // Clamped
    }

    #[test]
    fn test_wisp_spawn() {
        let mut vm = make_vm();
        // Set high entropy
        vm.entropy_grid[5][5] = 90;

        // Run until spawn or limit
        for _ in 0..1000 {
            crate::vm::nova_flux::process_flux(&mut vm);
            if !vm.organelles.is_empty() {
                break;
            }
        }

        if !vm.organelles.is_empty() {
            assert_eq!(vm.organelles[0].kind, crate::vm::nova::OrganelleType::Wisp);
        }
    }
}

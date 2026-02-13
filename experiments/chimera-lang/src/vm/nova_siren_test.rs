#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_psi_observer() {
        let mut vm = make_vm();
        // Layout:
        // . 5 .  (Target Value: 5)
        // . Ψ .  (Observer)
        // . . .

        vm.hologram_grid[1][1] = (0.05, 0.0);

        vm.grid[0][1] = Value::Str("5".to_string());
        vm.grid[1][1] = Value::Str("Ψ".to_string());

        process_signals(&mut vm);

        assert!(vm.signal_grid[2][1] > 0, "Psi did not fire bang on match");
    }

    #[test]
    fn test_phi_resonance() {
        let mut vm = make_vm();
        // Layout:
        // . 1 .
        // . Φ 2
        // . 3 .
        // N=1, E=2, S=3. 1+2=3.

        vm.grid[0][1] = Value::Str("1".to_string()); // N
        vm.grid[1][1] = Value::Str("Φ".to_string());
        vm.grid[1][2] = Value::Str("2".to_string()); // E
        vm.grid[2][1] = Value::Str("3".to_string()); // S

        let start_energy = vm.energy;
        process_signals(&mut vm);

        assert!(vm.energy > start_energy, "Phi did not generate energy");
    }

    #[test]
    fn test_omega_entropy_sink() {
        let mut vm = make_vm();
        // Layout:
        // . . .
        // . Ω .
        // . . .

        vm.entropy_grid[1][1] = 50;
        vm.grid[1][1] = Value::Str("Ω".to_string());

        process_signals(&mut vm);

        assert!(vm.entropy_grid[1][1] < 50, "Omega did not reduce entropy");
        assert!(vm.hologram_grid[1][1].0 != 0.0 || vm.hologram_grid[1][1].1 != 0.0, "Omega did not update hologram");
    }
}

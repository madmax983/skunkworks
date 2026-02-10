#[cfg(feature = "elektra")]
mod tests {
    use chimera_lang::ast::{Dna, Helix, Strand};
    use chimera_lang::vm::{ChimeraVM, Value};

    fn make_empty_vm() -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes: vec![] }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_and_gate() {
        let mut vm = make_empty_vm();
        vm.grid[5][5] = Value::Str("&".to_string());

        // North Input
        vm.voltage_grid[4][5] = 100.0;
        vm.resistance_grid[4][5] = -1.0;
        vm.step();
        // Only 1 input -> Output Low
        assert_eq!(vm.voltage_grid[5][5], 0.0);

        // South Input
        vm.voltage_grid[6][5] = 100.0;
        vm.resistance_grid[6][5] = -1.0;
        vm.step();
        // 2 inputs -> Output High
        assert_eq!(vm.voltage_grid[5][5], 100.0);
    }

    #[test]
    fn test_or_gate() {
        let mut vm = make_empty_vm();
        vm.grid[5][5] = Value::Str("|".to_string());

        vm.voltage_grid[4][5] = 100.0;
        vm.resistance_grid[4][5] = -1.0;
        vm.step();
        assert_eq!(vm.voltage_grid[5][5], 100.0);
    }

    #[test]
    fn test_xor_gate() {
        let mut vm = make_empty_vm();
        vm.grid[5][5] = Value::Str("^".to_string());

        // 1 Input
        vm.voltage_grid[4][5] = 100.0;
        vm.resistance_grid[4][5] = -1.0;
        vm.step();
        assert_eq!(vm.voltage_grid[5][5], 100.0);

        // 2 Inputs
        vm.voltage_grid[6][5] = 100.0;
        vm.resistance_grid[6][5] = -1.0;
        vm.step();
        assert_eq!(vm.voltage_grid[5][5], 0.0);
    }

    #[test]
    fn test_not_gate() {
        let mut vm = make_empty_vm();
        vm.grid[5][5] = Value::Str("!".to_string());

        // 0 Inputs
        vm.step();
        assert_eq!(vm.voltage_grid[5][5], 100.0);

        // 1 Input
        vm.voltage_grid[4][5] = 100.0;
        vm.resistance_grid[4][5] = -1.0;
        vm.step();
        assert_eq!(vm.voltage_grid[5][5], 0.0);
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::prologue::elemental::*;
    use crate::vm::prologue::exec_prologue_tick;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna { evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.prologue_state.active = true;
        vm
    }

    #[test]
    fn test_element_generation() {
        let mut vm = make_vm();
        // Δ (Fire), ∇ (Water), ◊ (Earth), ○ (Air), ☆ (Aether)
        vm.grid[5][0] = Value::Str("Δ".to_string());
        vm.grid[5][1] = Value::Str("∇".to_string());
        vm.grid[5][2] = Value::Str("◊".to_string());
        vm.grid[5][3] = Value::Str("○".to_string());
        vm.grid[5][4] = Value::Str("☆".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(
            vm.prologue_state.signal_grid[5][0],
            Some(Value::Symbol(ELEM_FIRE))
        );
        assert_eq!(
            vm.prologue_state.signal_grid[5][1],
            Some(Value::Symbol(ELEM_WATER))
        );
        assert_eq!(
            vm.prologue_state.signal_grid[5][2],
            Some(Value::Symbol(ELEM_EARTH))
        );
        assert_eq!(
            vm.prologue_state.signal_grid[5][3],
            Some(Value::Symbol(ELEM_AIR))
        );
        assert_eq!(
            vm.prologue_state.signal_grid[5][4],
            Some(Value::Symbol(ELEM_AETHER))
        );
    }

    #[test]
    fn test_fire_propagation() {
        let mut vm = make_vm();
        // Δ -> ~
        vm.grid[5][5] = Value::Str("Δ".to_string());
        vm.grid[6][5] = Value::Str("~".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(
            vm.prologue_state.signal_grid[6][5],
            Some(Value::Symbol(ELEM_FIRE))
        );
    }

    #[test]
    fn test_elemental_mixing_steam() {
        let mut vm = make_vm();
        // Δ ☿ ∇
        vm.grid[5][4] = Value::Str("Δ".to_string());
        vm.grid[5][5] = Value::Str("☿".to_string());
        vm.grid[5][6] = Value::Str("∇".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(
            vm.prologue_state.signal_grid[5][5],
            Some(Value::Symbol(ELEM_STEAM))
        );
    }

    #[test]
    fn test_elemental_mixing_lava() {
        let mut vm = make_vm();
        // Δ ☿ ◊
        vm.grid[5][4] = Value::Str("Δ".to_string());
        vm.grid[5][5] = Value::Str("☿".to_string());
        vm.grid[5][6] = Value::Str("◊".to_string());

        exec_prologue_tick(&mut vm);

        assert_eq!(
            vm.prologue_state.signal_grid[5][5],
            Some(Value::Symbol(ELEM_LAVA))
        );
    }
}

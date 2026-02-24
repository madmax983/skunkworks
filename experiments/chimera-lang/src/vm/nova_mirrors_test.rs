#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Helix};
    use crate::vm::nova_signals::process_signals;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm() -> ChimeraVM {
        let dna = Dna {
            evolution_config: None,
            helix: Helix { strands: vec![] },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_laser_mirror_slash() {
        let mut vm = make_vm();
        // Layout:
        // . N . .  (N should receive signal)
        // . / . .  (Mirror at 1,1)
        // . . . .
        // 4 L . .  (Laser at 3,1 firing North)
        //
        // Wait, Laser (L) needs:
        // North: Direction
        // West: Length
        //
        // Let's setup:
        // L at (2, 1).
        // North input (1, 1) is Direction. (We want East -> 1)
        // No, L reads North for Direction.
        // If we want L to fire East:
        // . 1 .
        // 4 L .
        // L fires East.
        // Mirror / at (2, 3).
        // Beam hits / from West (moving East).
        // / reflects East -> North.
        // Target at (1, 3).

        // Layout:
        // . . . T  (Target at 0,3)
        // . . . .
        // . . . /  (Mirror at 2,3)
        // . 1 . .  (Dir=1=East at 1,1)
        // 4 L . .  (Len=4, L at 2,1)

        vm.grid[2][0] = Value::Str("4".to_string()); // Length
        vm.grid[1][1] = Value::Str("1".to_string()); // Direction (East)
        vm.grid[2][1] = Value::Str("L".to_string()); // Laser

        vm.grid[2][3] = Value::Str("/".to_string()); // Mirror

        // Signal L
        vm.signal_grid[2][1] = 1;

        process_signals(&mut vm);

        // Check trace.
        // (2,2) should have signal (Beam path).
        // (2,3) is mirror.
        // (1,3) should have signal (Reflected North).
        // (0,3) should have signal.

        assert!(vm.signal_grid[2][2] > 0, "Beam failed to reach (2,2)");
        assert!(
            vm.signal_grid[2][3] > 0,
            "Beam failed to reach mirror at (2,3)"
        );
        assert!(
            vm.signal_grid[1][3] > 0,
            "Beam failed to reflect North to (1,3)"
        );
        assert!(vm.signal_grid[0][3] > 0, "Beam failed to reach (0,3)");
    }

    #[test]
    fn test_laser_mirror_backslash() {
        let mut vm = make_vm();
        // Layout:
        // . . . .
        // . . . \  (Mirror at 1,3)
        // . 1 . .
        // 4 L . T  (Target at 2,3 - South of Mirror)
        //
        // L fires East. Hits \ at (2,3).
        // East -> South.
        // Wait, \ reflects East -> South.
        // Let's verify standard Orca:
        // > \
        //   v
        // Yes.

        // Layout:
        // . . . .
        // . 1 . .
        // 4 L . \  (Mirror at 2,3)
        // . . . T  (Target at 3,3)

        vm.grid[2][0] = Value::Str("4".to_string());
        vm.grid[1][1] = Value::Str("1".to_string());
        vm.grid[2][1] = Value::Str("L".to_string());

        vm.grid[2][3] = Value::Str("\\".to_string());

        vm.signal_grid[2][1] = 1;

        process_signals(&mut vm);

        assert!(vm.signal_grid[2][2] > 0, "Beam failed to reach (2,2)");
        assert!(
            vm.signal_grid[3][3] > 0,
            "Beam failed to reflect South to (3,3)"
        );
    }
}

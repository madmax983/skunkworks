#[cfg(feature = "nova")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Strand};
    use crate::opcode::OpCode;
    use crate::vm::nova_astrology::Star;
    use crate::vm::{ChimeraVM, Value};

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        let dna = Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        };
        ChimeraVM::new(dna)
    }

    #[test]
    fn test_gaze_hit() {
        let genes = vec![Gene {
            op: OpCode::Gaze,
            args: vec![],
        }];
        let mut vm = make_vm(genes);

        // Place a star directly over (8,8)
        // Center is 7.5, 7.5. (8,8) is at +0.5, +0.5.
        // Dist = sqrt(0.5^2 + 0.5^2) / 8.0 = sqrt(0.5) / 8.0 = 0.707 / 8 = 0.088
        // Angle = 45 deg = PI/4.

        // Actually, let's reverse engineer project().
        // x = 7.5 + dist * 8 * cos(a)
        // 8 = 7.5 + dist * 8 * cos(a) => 0.5 = dist*8*cos(a)
        // 8 = 7.5 + dist * 8 * sin(a) => 0.5 = dist*8*sin(a)
        // cos(a) = sin(a) -> a = PI/4.
        // 0.5 = dist * 8 * 0.7071
        // dist = 0.5 / (8 * 0.7071) = 0.5 / 5.6568 = 0.088388

        vm.sky.stars.push(Star {
            angle: std::f64::consts::PI / 4.0,
            dist: 0.088388,
            color: 0xFF0000,
            power: 100,
            name: "TestStar".to_string(),
        });

        // Reset rotation
        vm.sky.rotation = 0.0;
        vm.context_loc = (8, 8); // y=8, x=8

        vm.step();

        // Stack should have intensity and color
        assert_eq!(vm.stack.len(), 2);
        let color_val = vm.stack.pop().unwrap();
        let intensity_val = vm.stack.pop().unwrap();

        if let (Value::Int(p), Value::Int(c)) = (intensity_val, color_val) {
            assert!((99..=101).contains(&p), "Power should be approx 100, got {}", p); // Floating point tolerance
            assert_eq!(c, 0xFF0000);
        } else {
            panic!("Gaze returned wrong types");
        }
    }

    #[test]
    fn test_starfall_hit() {
        let genes = vec![Gene {
            op: OpCode::Starfall,
            args: vec![],
        }];
        let mut vm = make_vm(genes);
        vm.energy = 100;

        // Same star setup
        vm.sky.stars.push(Star {
            angle: std::f64::consts::PI / 4.0,
            dist: 0.088388,
            color: 0xFF0000,
            power: 150,
            name: "DoomStar".to_string(),
        });
        vm.sky.rotation = 0.0;
        vm.context_loc = (8, 8);

        // Put something on grid to destroy
        vm.grid[8][8] = Value::Int(999);

        vm.step();

        let last_msg = vm.output.last().unwrap();
        assert!(last_msg.contains("Summoned DoomStar"), "Msg: {}", last_msg);

        // Check crater
        assert_eq!(vm.grid[8][8], Value::Int(0));

        // Check entropy (center + neighbors)
        assert!(vm.entropy_grid[8][8] >= 20);
        assert!(vm.entropy_grid[8][9] >= 20);
    }

    #[test]
    fn test_align() {
        let genes = vec![Gene {
            op: OpCode::Align,
            args: vec![],
        }];
        let mut vm = make_vm(genes);

        // Star at 90 degrees (PI/2)
        vm.sky.stars.clear();
        vm.sky.stars.push(Star {
            angle: std::f64::consts::PI / 2.0,
            dist: 0.5,
            color: 0,
            power: 0,
            name: "North".to_string(),
        });
        vm.sky.rotation = 0.0;

        // Organism at center (7.5, 7.5) approx.
        // If we are at (8, 8), our angle from center (7.5, 7.5) is 45 deg (PI/4).
        vm.context_loc = (8, 8);

        vm.step();

        // Expect nearest star angle 90
        let angle_val = vm.stack.pop().unwrap();
        if let Value::Int(deg) = angle_val {
            assert_eq!(deg, 90);
        } else {
            panic!("Align returned non-int");
        }
    }
}

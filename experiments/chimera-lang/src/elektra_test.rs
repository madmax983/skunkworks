#[cfg(feature = "elektra")]
#[cfg(test)]
mod tests {
    use crate::ast::{Dna, Gene, Helix, Nucleotide, Strand};
    use crate::prelude::*;
    use crate::vm::elektra;
    use crate::vm::ChimeraVM;

    fn make_dna(genes: Vec<Gene>) -> Dna {
        Dna {
            helix: Helix {
                strands: vec![Strand { genes }],
            },
        }
    }

    fn make_vm(genes: Vec<Gene>) -> ChimeraVM {
        ChimeraVM::new(make_dna(genes))
    }

    #[test]
    fn test_battery_op() {
        // [ push(10) push(5) push(5) battery() ]
        // Should set V=10 at (5,5) and mark as source
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            }, // Voltage
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // X
            Gene {
                op: OpCode::Battery,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // Execute
        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        assert_eq!(vm.voltage_grid[5][5], 10.0);
        assert_eq!(vm.resistance_grid[5][5], -1.0); // Battery marker
    }

    #[test]
    fn test_ground_op() {
        // [ push(5) push(5) ground() ]
        // Should set V=0 at (5,5) and mark as ground
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // X
            Gene {
                op: OpCode::Ground,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // Execute
        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        assert_eq!(vm.voltage_grid[5][5], 0.0);
        assert_eq!(vm.resistance_grid[5][5], -2.0); // Ground marker
    }

    #[test]
    fn test_sense_volt() {
        // [ push(5) push(5) sense_volt() ]
        // Should push voltage at (5,5)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // Y
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(5)],
            }, // X
            Gene {
                op: OpCode::SenseVolt,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // Manually set voltage
        vm.voltage_grid[5][5] = 12.5;
        vm.resistance_grid[5][5] = -1.0; // Mark as Battery to prevent decay

        // Execute
        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        let result = vm.stack.pop().expect("Stack empty");
        // Voltage is cast to int
        if let Value::Int(v) = result {
            assert_eq!(v, 12);
        } else {
            panic!("Expected Int value");
        }
    }

    #[test]
    fn test_shock_blows_fuse() {
        // [ push(100) push(2) shock() ]
        // Shock with power 100, radius 2 at context (8,8)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(100)],
            }, // Power
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(2)],
            }, // Radius
            Gene {
                op: OpCode::Shock,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);

        // Set a component (Battery) at (8,8)
        vm.resistance_grid[8][8] = -1.0;

        // Execute
        while !vm.halted && vm.ip.0 == 0 {
            vm.step();
        }

        // Should be reset to 1.0 (Air)
        assert_eq!(vm.resistance_grid[8][8], 1.0);
    }

    #[test]
    fn test_circuit_solver() {
        // Setup a simple circuit: Battery(10V) -> Wire -> Ground(0V)
        // (5,5) = 10V
        // (5,6) = Wire
        // (5,7) = 0V
        let genes = vec![]; // No genes needed, testing update_circuit directly
        let mut vm = make_vm(genes);

        // Setup components
        // Ensure Battery and Ground are on conductive material (Wire) so they couple
        vm.grid[5][5] = Value::Int(1);
        vm.voltage_grid[5][5] = 10.0;
        vm.resistance_grid[5][5] = -1.0; // Battery

        vm.grid[5][6] = Value::Int(1); // Wire (conductivity=10.0)

        vm.grid[5][7] = Value::Int(1);
        vm.voltage_grid[5][7] = 0.0;
        vm.resistance_grid[5][7] = -2.0; // Ground

        // Run simulation steps
        for _ in 0..50 {
            elektra::update_circuit(&mut vm);
        }

        // Voltage at wire should be average of neighbors (approx 5V)
        let v_wire = vm.voltage_grid[5][6];
        println!("Wire Voltage: {}", v_wire);
        assert!(
            v_wire > 4.0 && v_wire < 6.0,
            "Voltage should settle around 5V"
        );
    }

    #[test]
    fn test_insulator_logic() {
        // Battery(10V) -> Air -> Ground(0V)
        // Air has low conductivity, so potential should drop off sharply or be close to 0 if not connected
        let genes = vec![];
        let mut vm = make_vm(genes);

        vm.voltage_grid[5][5] = 10.0;
        vm.resistance_grid[5][5] = -1.0;

        // (5,6) is Air (Value::Int(0) or Empty)

        vm.voltage_grid[5][7] = 0.0;
        vm.resistance_grid[5][7] = -2.0;

        for _ in 0..50 {
            elektra::update_circuit(&mut vm);
        }

        let v_air = vm.voltage_grid[5][6];
        println!("Air Voltage: {}", v_air);
        // Should be much lower than wire case due to high resistance/low conductivity
        assert!(v_air < 2.0, "Air should insulate");
    }

    #[test]
    fn test_electrogenesis_induction() {
        // [ push(10) electrogenesis() induction() ]
        // Context at (8,8) by default
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(10)],
            },
            Gene {
                op: OpCode::Electrogenesis,
                args: vec![],
            },
            Gene {
                op: OpCode::Induction,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.energy = 50;

        // Execute Electrogenesis
        vm.step(); // push(10)
        vm.step(); // electrogenesis

        // Voltage is set to 10.0.
        assert_eq!(vm.voltage_grid[8][8], 10.0);

        // Execute Induction
        vm.step();

        // Note: update_circuit runs before gene execution, causing voltage decay in Air.
        // 10.0 decays to approx 3 (0.9^10 ~ 0.34).

        assert_eq!(vm.voltage_grid[8][8], 0.0);
        let val = vm.stack.pop().unwrap();
        // Allow for decay
        if let Value::Int(v) = val {
            assert!(
                v >= 3 && v <= 10,
                "Voltage {} out of expected decay range",
                v
            );
        } else {
            panic!("Expected Int value");
        }
    }

    #[test]
    fn test_wire_growth() {
        // [ push(1) wire_growth() ] (Direction 1 = East)
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)],
            },
            Gene {
                op: OpCode::WireGrowth,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.context_loc = (8, 8);

        vm.step(); // push
        vm.step(); // grow

        // (8, 9) should be wire (Int(1))
        assert_eq!(vm.grid[8][9], Value::Int(1));
    }

    #[test]
    fn test_circuit_breaker() {
        // [ push(5) push(1) circuit_breaker() ]
        // Threshold 5, Jump to Strand 1
        let strand0 = Strand {
            genes: vec![
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(5)],
                },
                Gene {
                    op: OpCode::Push,
                    args: vec![Nucleotide::Number(1)],
                },
                Gene {
                    op: OpCode::CircuitBreaker,
                    args: vec![],
                },
            ],
        };
        let strand1 = Strand {
            genes: vec![Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(99)],
            }],
        };

        let dna = Dna {
            helix: Helix {
                strands: vec![strand0, strand1],
            },
        };
        let mut vm = ChimeraVM::new(dna);
        vm.context_loc = (8, 8);
        vm.voltage_grid[8][8] = 10.0; // Higher than threshold 5
        vm.resistance_grid[8][8] = -1.0; // Mark as Battery to prevent decay during steps

        vm.step(); // push 5
        vm.step(); // push 1
        vm.step(); // breaker

        // Should have jumped to strand 1
        assert_eq!(vm.ip.0, 1);

        vm.step(); // push 99
        assert_eq!(vm.stack.last(), Some(&Value::Int(99)));
    }

    #[test]
    #[cfg(feature = "nova")]
    fn test_railgun() {
        // [ push(1) push(0) railgun() ]
        // dy=1, dx=0 -> Fire South
        let genes = vec![
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(1)], // dy
            },
            Gene {
                op: OpCode::Push,
                args: vec![Nucleotide::Number(0)], // dx
            },
            Gene {
                op: OpCode::Railgun,
                args: vec![],
            },
        ];
        let mut vm = make_vm(genes);
        vm.context_loc = (8, 8);
        vm.voltage_grid[8][8] = 50.0; // 50V -> Power 5
        vm.resistance_grid[8][8] = -1.0; // Mark as Battery to prevent decay

        vm.step(); // push 1
        vm.step(); // push 0
        vm.step(); // railgun

        // Check projectile
        assert_eq!(vm.projectiles.len(), 1);
        let p = &vm.projectiles[0];
        assert_eq!(p.power, 5); // 50.0 / 10.0
        assert_eq!(p.vx, 0.0);
        assert_eq!(p.vy, 1.0);

        // Check discharge
        assert_eq!(vm.voltage_grid[8][8], 0.0);
    }
}

use macroquad::prelude::*;
use rapier2d::prelude::*;
use chimera_lang::vm::{ChimeraVM, Value};
use chimera_lang::ast::{Dna, Helix, Strand, Gene, Nucleotide};
use chimera_lang::opcode::OpCode;

mod physics;
mod vehicle;
use physics::PhysicsWorld;
use vehicle::Vehicle;

fn create_dna() -> Dna {
    // Strand 0: Drive Forward
    // Registers: Grid[0][0] = Left Motor, Grid[0][1] = Right Motor
    let genes = vec![
        // Left Motor = 15
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // X
        Gene { op: OpCode::GWrite, args: vec![] },

        // Right Motor = 15
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(15)] },
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(0)] }, // Y
        Gene { op: OpCode::Push, args: vec![Nucleotide::Number(1)] }, // X
        Gene { op: OpCode::GWrite, args: vec![] },

        // Consume to avoid starvation (logic included in step, but good to be explicit/safe or just loop)
        Gene { op: OpCode::Jump, args: vec![Nucleotide::Number(0)] },
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes }],
        },
    }
}

#[macroquad::main("Chimera Automaton")]
async fn main() {
    let mut world = PhysicsWorld::new();
    let mut vm = ChimeraVM::new(create_dna());

    // Setup Ground
    let ground_size = 100.0;
    let ground_rb = RigidBodyBuilder::fixed().translation(vector![0.0, -2.0]).build();
    let ground = world.rigid_body_set.insert(ground_rb);
    let ground_coll = ColliderBuilder::cuboid(ground_size, 1.0).build();
    world.collider_set.insert_with_parent(ground_coll, ground, &mut world.rigid_body_set);

    // Spawn Vehicle
    let vehicle = Vehicle::spawn(&mut world, 0.0, 5.0);

    // Camera
    let mut cam_target = vec2(0.0, 0.0);

    loop {
        clear_background(LIGHTGRAY);

        // 1. Step VM
        // Step multiple times per frame to ensure responsiveness
        for _ in 0..10 {
            if !vm.halted {
                vm.step();
            }
        }
        // Refuel vm to keep it running forever for this demo
        vm.energy = 100;

        // 2. Read VM Output -> Drive Physics
        let left_val = &vm.grid[0][0];
        let right_val = &vm.grid[0][1];

        let left_speed = match left_val {
            Value::Int(v) => *v as f32,
            _ => 0.0,
        };
        let right_speed = match right_val {
            Value::Int(v) => *v as f32,
            _ => 0.0,
        };

        vehicle.set_motor_speeds(&mut world, left_speed, right_speed);

        // 3. Step Physics
        world.step();

        // 4. Update Camera to follow vehicle
        let chassis_pos = world.rigid_body_set[vehicle.chassis].translation();
        cam_target = vec2(chassis_pos.x, chassis_pos.y);

        set_camera(&Camera2D {
            zoom: vec2(0.05, 0.05),
            target: cam_target,
            ..Default::default()
        });

        // 5. Render
        draw_line(-ground_size, -1.0, ground_size, -1.0, 0.2, DARKGRAY);

        for (_handle, body) in world.rigid_body_set.iter() {
            for collider_handle in body.colliders() {
                if let Some(collider) = world.collider_set.get(*collider_handle) {
                    let shape = collider.shared_shape();
                    let pos = body.position() * collider.position_wrt_parent().unwrap();
                    let c_pos = pos.translation.vector;
                    let rot = pos.rotation.angle();

                    if let Some(ball) = shape.as_ball() {
                        draw_circle(c_pos.x, c_pos.y, ball.radius, BLACK);
                        draw_circle_lines(c_pos.x, c_pos.y, ball.radius, 0.1, WHITE);
                        // Rotation marker
                         let end_x = c_pos.x + ball.radius * rot.cos();
                         let end_y = c_pos.y + ball.radius * rot.sin();
                         draw_line(c_pos.x, c_pos.y, end_x, end_y, 0.1, WHITE);
                    } else if let Some(cuboid) = shape.as_cuboid() {
                        let w = cuboid.half_extents.x * 2.0;
                        let h = cuboid.half_extents.y * 2.0;
                        draw_rectangle_ex(c_pos.x, c_pos.y, w, h, DrawRectangleParams {
                            offset: vec2(0.5, 0.5),
                            rotation: rot,
                            color: BLUE,
                        });
                    }
                }
            }
        }

        // Draw HUD
        set_default_camera();
        draw_text("Chimera Automaton", 20.0, 30.0, 30.0, BLACK);
        draw_text(&format!("Motor L: {:.1} R: {:.1}", left_speed, right_speed), 20.0, 60.0, 20.0, DARKGRAY);

        next_frame().await
    }
}

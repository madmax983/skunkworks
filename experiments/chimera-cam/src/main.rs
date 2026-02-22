use chimera_lang::opcode::OpCode;
use chimera_lang::prelude::*;
use macroquad::prelude::*;
use nalgebra::Vector2 as Vec2N;
use rapier2d::prelude::*;

mod genetic_design;
mod mechanism;
mod physics;
mod puppet;

use genetic_design::GeneticDesigner;
use mechanism::Mechanism;
use physics::PhysicsWorld;
use puppet::Puppet;

const POP_SIZE: usize = 12;
const GEN_TICKS: usize = 300; // 5 seconds at 60fps frame time (simulated)

#[derive(Clone)]
struct Agent {
    dna: Vec<OpCode>,
    fitness: f32,
    cam_shape: Option<SharedShape>,
}

impl Agent {
    fn new_random() -> Self {
        let mut dna = Vec::new();
        for _ in 0..30 {
            dna.push(Self::random_opcode());
        }
        Self {
            dna,
            fitness: 0.0,
            cam_shape: None,
        }
    }

    fn random_opcode() -> OpCode {
        let choices = [
            OpCode::Push,
            OpCode::Add,
            OpCode::Sub,
            OpCode::Mul,
            OpCode::Div,
            OpCode::Dup,
            OpCode::Swap,
            OpCode::Drop,
            OpCode::Eq,
            OpCode::Gt,
            OpCode::Lt,
            OpCode::Nop,
        ];
        let idx = macroquad::rand::gen_range(0, choices.len());
        let op = choices[idx].clone();

        // If push, we need an arg?
        // Actually OpCode::Push doesn't carry the arg in the enum variant usually?
        // Wait, OpCode is just the Enum. The Gene has `args`.
        // My `GeneticDesigner` takes `Vec<OpCode>` and makes `Gene { op, args: vec![] }`.
        // If I want args, I need to change `GeneticDesigner` or how I construct genes.
        // `OpCode` variants usually don't have data (except Unknown).
        // The `args` are in the `Gene`.
        // My `GeneticDesigner::new` sets args to empty vec.
        // This means `Push` will have no args and fail/do nothing?
        // Ah, `execute_gene_inner` checks `args`.

        op
    }
}

// Helper to mutate DNA
fn mutate(dna: &mut Vec<OpCode>) {
    let len = dna.len();
    if len == 0 {
        return;
    }

    // 10% mutation rate per gene? No, simpler.
    let mutations = 1 + macroquad::rand::gen_range(0, 3);
    for _ in 0..mutations {
        let idx = macroquad::rand::gen_range(0, len);
        dna[idx] = Agent::random_opcode();
    }
}

// Crossover
fn crossover(parent_a: &Agent, parent_b: &Agent) -> Agent {
    let split = macroquad::rand::gen_range(0, parent_a.dna.len().min(parent_b.dna.len()));
    let mut new_dna = Vec::new();
    new_dna.extend_from_slice(&parent_a.dna[0..split]);
    new_dna.extend_from_slice(&parent_b.dna[split..]);

    // Ensure length constraint?
    new_dna.truncate(50);

    Agent {
        dna: new_dna,
        fitness: 0.0,
        cam_shape: None,
    }
}

// Because GeneticDesigner expects genes with args for Push,
// but I'm passing just OpCodes.
// I should fix GeneticDesigner or fix Agent to produce Dna directly.
// Let's modify Agent to produce Dna-friendly vectors?
// Or better: GeneticDesigner::new should construct args for Push.
// But GeneticDesigner receives `Vec<OpCode>`. It doesn't know args.
// I'll update GeneticDesigner::new in memory later if needed?
// No, I'll update `GeneticDesigner` to handle `OpCode::Push` by auto-generating args?
// Or I change `Agent` to store `Vec<Gene>`. That's cleaner.

// Redefining Agent to hold Genes.
/*
struct Agent {
    genes: Vec<Gene>,
    ...
}
*/
// But `GeneticDesigner::new` takes `Vec<OpCode>`.
// I will just update `GeneticDesigner` to handle this.
// Wait, `OpCode` variants don't store values.
// If `GeneticDesigner` creates `Gene { op, args: vec![] }`, `Push` will fail.
// So `GeneticDesigner` must be smarter.

fn setup_world(
    world: &mut PhysicsWorld,
    cam_shape: SharedShape,
) -> (RigidBodyHandle, RigidBodyHandle) {
    // 1. Setup Camshaft
    let shaft_pos = Vec2N::new(0.0, -5.0);
    let shaft = Mechanism::create_camshaft(world, shaft_pos);

    // Cam 1 (Left Leg Driver) - DNA Designed
    Mechanism::add_cam(world, shaft, cam_shape.clone(), Vec2N::new(0.0, 1.0));

    // Cam 2 (Right Leg Driver) - Phase Shifted (Rotated 180)
    // Actually add_cam doesn't support rotation arg easily,
    // but `ColliderBuilder::position` takes Isometry.
    // Mechanism::add_cam hardcodes rotation to 0.0.
    // I should modify Mechanism or just accept they are same phase.
    // Same phase means hopping. That's fine.
    Mechanism::add_cam(world, shaft, cam_shape, Vec2N::new(0.0, -1.0));

    // 2. Setup Followers
    let f1 = Mechanism::create_follower(world, -2.0, -2.0);
    let f2 = Mechanism::create_follower(world, 2.0, -2.0);

    // 3. Setup Puppet
    let puppet = Puppet::spawn(world, 0.0, 5.0);

    // 4. Connect Linkages
    Puppet::attach_rod(world, f1, puppet.left_leg);
    Puppet::attach_rod(world, f2, puppet.right_leg);

    (puppet.torso, shaft)
}

fn evaluate_fitness(shape: SharedShape) -> f32 {
    let mut world = PhysicsWorld::new();
    let (torso, shaft) = setup_world(&mut world, shape);

    let mut total_h = 0.0;

    for i in 0..GEN_TICKS {
        let angle = (i as f32) * 0.1; // Speed
        if let Some(body) = world.rigid_body_set.get_mut(shaft) {
            body.set_next_kinematic_rotation(Rotation::new(angle));
        }

        world.step();

        // Measure torso height relative to start
        let pos = world.rigid_body_set[torso].translation().y;
        total_h += pos;
    }

    // Average height
    total_h / (GEN_TICKS as f32)
}

#[macroquad::main("Chimera Cam")]
async fn main() {
    // Initial Population
    let mut population: Vec<Agent> = (0..POP_SIZE).map(|_| Agent::new_random()).collect();

    let mut generation = 0;
    let mut sim_frame = 0;
    let mut vis_world = PhysicsWorld::new();
    let mut shaft_handle = RigidBodyHandle::invalid();

    // Initial Vis World
    {
        let mut designer = GeneticDesigner::new(population[0].dna.clone());
        let shape = designer.generate_cam_shape();
        population[0].cam_shape = Some(shape.clone());
        let (_, s) = setup_world(&mut vis_world, shape);
        shaft_handle = s;
    }

    loop {
        // Evolution Step
        if sim_frame == 0 {
            // Evaluate
            for agent in &mut population {
                let mut designer = GeneticDesigner::new(agent.dna.clone());
                let shape = designer.generate_cam_shape();
                agent.cam_shape = Some(shape.clone());
                agent.fitness = evaluate_fitness(shape);
            }

            // Sort
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            println!(
                "Gen {} Best Fitness: {:.2}",
                generation, population[0].fitness
            );

            // Selection & Reproduction
            let mut new_pop = Vec::new();
            // Elitism: Keep top 2
            new_pop.push(population[0].clone());
            new_pop.push(population[1].clone());

            // Fill rest
            while new_pop.len() < POP_SIZE {
                let parent_a = &population[macroquad::rand::gen_range(0, POP_SIZE / 2)];
                let parent_b = &population[macroquad::rand::gen_range(0, POP_SIZE / 2)];
                let mut child = crossover(parent_a, parent_b);
                mutate(&mut child.dna);
                new_pop.push(child);
            }
            population = new_pop;

            generation += 1;

            // Reset Vis
            vis_world = PhysicsWorld::new();
            let (_, s) = setup_world(&mut vis_world, population[0].cam_shape.clone().unwrap());
            shaft_handle = s;
        }

        // Visualization
        clear_background(LIGHTGRAY);

        // Update Shaft
        let speed = 0.1; // Same speed as simulation
        let angle = (sim_frame as f32) * speed;
        if let Some(body) = vis_world.rigid_body_set.get_mut(shaft_handle) {
            body.set_next_kinematic_rotation(Rotation::new(angle));
        }

        vis_world.step();

        // Draw
        draw_physics_world(&vis_world);

        // UI
        draw_text(&format!("Gen: {}", generation), 10.0, 30.0, 30.0, BLACK);
        draw_text(
            &format!("Fit: {:.2}", population[0].fitness),
            10.0,
            60.0,
            30.0,
            BLACK,
        );

        sim_frame += 1;
        if sim_frame > GEN_TICKS {
            sim_frame = 0;
        }

        next_frame().await
    }
}

// Drawing Helper (Adapted from original main.rs)
fn draw_physics_world(world: &PhysicsWorld) {
    // Camera
    set_camera(&Camera2D {
        zoom: vec2(0.05, 0.05),
        target: vec2(0.0, 2.0),
        ..Default::default()
    });

    draw_line(-10.0, -5.0, 10.0, -5.0, 0.1, BLACK); // Ground? Shaft line?

    for (_handle, body) in world.rigid_body_set.iter() {
        for collider_handle in body.colliders() {
            if let Some(collider) = world.collider_set.get(*collider_handle) {
                let shape = collider.shared_shape();
                let iso = collider.position_wrt_parent().unwrap();
                let collider_pos = body.position() * iso;
                let c_pos = collider_pos.translation.vector;
                let c_rot = collider_pos.rotation.angle();

                if let Some(ball) = shape.as_ball() {
                    draw_circle(c_pos.x, c_pos.y, ball.radius, RED);
                    let end_x = c_pos.x + ball.radius * c_rot.cos();
                    let end_y = c_pos.y + ball.radius * c_rot.sin();
                    draw_line(c_pos.x, c_pos.y, end_x, end_y, 0.1, BLACK);
                } else if let Some(cuboid) = shape.as_cuboid() {
                    let w = cuboid.half_extents.x * 2.0;
                    let h = cuboid.half_extents.y * 2.0;
                    draw_rectangle_ex(
                        c_pos.x,
                        c_pos.y,
                        w,
                        h,
                        DrawRectangleParams {
                            offset: vec2(0.5, 0.5),
                            rotation: c_rot,
                            color: BLUE,
                        },
                    );
                } else if let Some(poly) = shape.as_convex_polygon() {
                    // Draw Polygon (Cam)
                    // We need to transform points
                    // This is for our Cam!
                    let points: Vec<Vec2> = poly
                        .points()
                        .iter()
                        .map(|p| {
                            let transformed = collider_pos * p;
                            vec2(transformed.x, transformed.y)
                        })
                        .collect();

                    // Draw polygon lines
                    if !points.is_empty() {
                        for i in 0..points.len() {
                            let p1 = points[i];
                            let p2 = points[(i + 1) % points.len()];
                            draw_line(p1.x, p1.y, p2.x, p2.y, 0.1, DARKGREEN);
                        }
                        // Fill center?
                        draw_circle(c_pos.x, c_pos.y, 0.2, GREEN);
                    }
                }
            }
        }
    }

    // Draw Joints
    for (_handle, joint) in world.impulse_joint_set.iter() {
        let b1 = world.rigid_body_set[joint.body1].translation();
        let b2 = world.rigid_body_set[joint.body2].translation();
        draw_line(b1.x, b1.y, b2.x, b2.y, 0.15, ORANGE);
    }
}

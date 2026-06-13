use rand::Rng;
use thermo_termites::world::{Agent, World, HEIGHT, WIDTH};

#[test]
fn test_termite_cooling_efficiency() {
    // 1. Setup Control World (No Termites)
    let mut control = World::new();
    setup_world(&mut control, false);

    // 2. Setup Experiment World (With Termites)
    let mut experiment = World::new();
    setup_world(&mut experiment, true);

    // 3. Run Simulation
    let steps = 2000;
    println!("Running for {} steps...", steps);
    for i in 0..steps {
        if i % 100 == 0 {
            // Optional progress log
        }
        control.update();
        experiment.update();
    }

    // 4. Measure
    let t_control = control.get_average_server_temp();
    let t_experiment = experiment.get_average_server_temp();

    println!("Control Temp: {:.2}", t_control);
    println!("Experiment Temp: {:.2}", t_experiment);

    // 5. Assert
    // We expect termites to make it cooler
    assert!(
        t_experiment < t_control,
        "Termites failed to cool the servers! Control: {}, Exp: {}",
        t_control,
        t_experiment
    );
}

fn setup_world(world: &mut World, add_termites: bool) {
    let mut rng = rand::thread_rng();

    // Add Server Rack (Center)
    let cx = WIDTH / 2;
    let cy = HEIGHT / 2;
    world.add_server_block(cx - 20, cy - 20, 40, 40);

    // Add Air
    for _ in 0..50000 {
        let x = rng.gen_range(0.0..WIDTH as f32);
        let y = rng.gen_range(0.0..HEIGHT as f32);
        world.agents.push(Agent::new_air(x, y));
    }

    // Add Termites (if exp)
    if add_termites {
        for _ in 0..5000 {
            let x = rng.gen_range(0.0..WIDTH as f32);
            let y = rng.gen_range(0.0..HEIGHT as f32);
            world.agents.push(Agent::new_termite(x, y));
        }
    }
}

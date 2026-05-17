with open("experiments/myco-resonance/src/main.rs", "r") as f:
    code = f.read()

import re

# Fix world setup
world_setup = """    let (world, agents) = MycoWorld::with_cities_and_agents(GRID_W, GRID_H, 5);
    let myco_world = Arc::new(Mutex::new(world));
    let myco_agents = Arc::new(Mutex::new(agents));

    let world_clone = Arc::clone(&myco_world);
    let agents_clone = Arc::clone(&myco_agents);

    // Run Myco simulation in background thread
    thread::spawn(move || {
        loop {
            {
                let mut world = world_clone.lock().unwrap();
                let mut agents = agents_clone.lock().unwrap();
                world.diffuse_and_decay();
                world.update_agents_parallel(&mut agents);
            }
            thread::sleep(std::time::Duration::from_millis(16)); // ~60fps logic
        }
    });"""

code = re.sub(r"    let \(mut world, mut agents\) = MycoWorld::with_cities_and_agents.*?}\n    }\);", world_setup, code, flags=re.DOTALL)

with open("experiments/myco-resonance/src/main.rs", "w") as f:
    f.write(code)

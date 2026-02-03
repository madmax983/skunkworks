use crate::physics::World;
use glam::Vec2;

pub enum InputMode {
    Normal,
    Typing,
}

pub struct App {
    pub world: World,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub running: bool,
    pub mouse_pressed: bool,
    pub dragged_particle: Option<usize>,
    pub physics_paused: bool,
}

impl App {
    pub fn new() -> Self {
        // Default size, will be updated by resize
        let mut world = World::new(100.0, 50.0);

        // Initial Demo
        world.add_word("HELLO", Vec2::new(10.0, 10.0), Vec2::new(20.0, 0.0));
        world.add_word("WORLD", Vec2::new(90.0, 10.0), Vec2::new(-20.0, 0.0));

        Self {
            world,
            input_mode: InputMode::Normal,
            input_buffer: String::new(),
            running: true,
            mouse_pressed: false,
            dragged_particle: None,
            physics_paused: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.physics_paused {
            self.world.update(dt);
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.world.width = width;
        self.world.height = height;
    }

    pub fn handle_mouse_down(&mut self, x: f64, y: f64) {
        self.mouse_pressed = true;
        // Find closest particle
        let mouse_pos = Vec2::new(x as f32, y as f32);
        let mut closest_dist = 5.0; // Selection radius
        let mut closest_idx = None;

        for (i, p) in self.world.particles.iter().enumerate() {
            let dist = p.pos.distance(mouse_pos);
            if dist < closest_dist {
                closest_dist = dist;
                closest_idx = Some(i);
            }
        }

        if let Some(idx) = closest_idx {
            self.dragged_particle = Some(idx);
            self.world.particles[idx].locked = true;
        }
    }

    pub fn handle_mouse_up(&mut self) {
        self.mouse_pressed = false;
        if let Some(idx) = self.dragged_particle {
            self.world.particles[idx].locked = false;
            // Fling?
            // velocity is automatically handled by Verlet if we just unlock it
            // but we might want to reset prev_pos to give it a kick?
            // For now, let Verlet handle it (it will infer velocity from the drag movement)
        }
        self.dragged_particle = None;
    }

    pub fn handle_mouse_drag(&mut self, x: f64, y: f64) {
        if let Some(idx) = self.dragged_particle {
            self.world.particles[idx].pos = Vec2::new(x as f32, y as f32);
        }
    }

    pub fn spawn_word(&mut self) {
        if self.input_buffer.is_empty() {
            return;
        }

        // Spawn at center top with random velocity
        let x = self.world.width / 2.0 - (self.input_buffer.len() as f32);
        let y = 5.0;

        let vel_x = (fastrand::f32() - 0.5) * 20.0;
        let vel_y = 10.0; // Downwards

        self.world
            .add_word(&self.input_buffer, Vec2::new(x, y), Vec2::new(vel_x, vel_y));
        self.input_buffer.clear();
        self.input_mode = InputMode::Normal;
    }
}

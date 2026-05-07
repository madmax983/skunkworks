use crate::audio::NeuronHit;
use crate::physics::{resolve_collision, NeuronPin, PacketKind, Particle};
use locus::Vec2;
use rand::Rng;

/// The overseer of the Pachinko arena.
///
/// `GameState` holds the entire simulation space: the falling [`crate::physics::Particle`]s (network packets)
/// and the static grid of [`crate::physics::NeuronPin`]s. It tracks the score and the synchronized visual
/// representation of the neuron voltages, updated via [`crate::audio::Snapshot`]s from the audio engine.
///
/// ## Examples
/// ```
/// use synaptic_pachinko::game::GameState;
///
/// let mut game = GameState::new(800.0, 600.0);
/// game.spawn_packet(); // Drop a new packet into the arena
/// assert_eq!(game.particles.len(), 1);
/// ```
pub struct GameState {
    /// The logical width of the game area.
    pub width: f64,
    /// The logical height of the game area.
    pub height: f64,
    /// Dropping network packets currently traversing the grid.
    pub particles: Vec<Particle>,
    /// The static layout of neural pins.
    pub pins: Vec<NeuronPin>,
    /// A synchronized view of all neuron voltages.
    pub neuron_voltages: Vec<f32>,
    /// The averaged electrical field across the entire pin network.
    pub mean_field: f32,
    /// The number of collisions recorded.
    pub score: u64,
}

impl GameState {
    /// Instantiates the game state and procedurally generates the lattice layout of [`crate::physics::NeuronPin`]s.
    ///
    /// This establishes the play area boundaries and sets up the structural network that will translate collisions into audio synthesis.
    ///
    /// ## Arguments
    ///
    /// * `width` - Total logical width defining the horizontal boundaries.
    /// * `height` - Total logical height defining the drop ceiling.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::game::GameState;
    /// let game = GameState::new(100.0, 100.0);
    /// assert_eq!(game.score, 0);
    /// ```
    pub fn new(width: f64, height: f64) -> Self {
        // Generate Pins
        let mut pins = Vec::new();
        let rows = 12;
        let cols = 16;
        let spacing_x = width / (cols as f64 + 1.0);
        let spacing_y = (height * 0.6) / rows as f64;
        let start_y = height * 0.2;

        let mut index = 0;
        for r in 0..rows {
            let offset = if r % 2 == 0 { 0.0 } else { spacing_x * 0.5 };
            for c in 0..cols {
                let x = spacing_x + (c as f64 * spacing_x) + offset;
                let y = start_y + (r as f64 * spacing_y);
                pins.push(NeuronPin::new(x, y, index));
                index += 1;
            }
        }

        Self {
            width,
            height,
            particles: Vec::new(),
            pins,
            neuron_voltages: vec![-65.0; index],
            mean_field: -65.0,
            score: 0,
        }
    }

    /// Injects a new data packet into the system to challenge the neural lattice.
    ///
    /// The packet is spawned near the top center with a randomized protocol type ([`crate::physics::PacketKind`]).
    /// This simulates incoming network traffic hitting the firewall/neural board.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::game::GameState;
    /// let mut game = GameState::new(100.0, 100.0);
    /// game.spawn_packet();
    /// assert_eq!(game.particles.len(), 1);
    /// ```
    pub fn spawn_packet(&mut self) {
        let mut rng = rand::thread_rng();
        let x = self.width * 0.5 + rng.gen_range(-5.0..5.0);
        let kind = match rng.gen_range(0..3) {
            0 => PacketKind::Http,
            1 => PacketKind::Ssh,
            _ => PacketKind::Malware,
        };
        self.particles.push(Particle::new(x, 0.0, kind));
    }

    /// Advances the chaotic physical simulation of the arena.
    ///
    /// In this phase, gravity pulls the packets downward. If they strike a pin, the collision is resolved mathematically,
    /// and the resulting kinetic energy is fired down the `hit_tx` channel as a [`crate::audio::NeuronHit`] to wake up the audio thread.
    ///
    /// ## Arguments
    ///
    /// * `dt` - Time delta (the frame step duration).
    /// * `hit_tx` - Channel used to communicate collision energy into the audio engine.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::game::GameState;
    /// use crossbeam_channel::unbounded;
    /// let mut game = GameState::new(100.0, 100.0);
    /// let (tx, rx) = unbounded();
    /// game.tick(0.016, &tx);
    /// ```
    pub fn tick(&mut self, dt: f64, hit_tx: &crossbeam_channel::Sender<crate::audio::NeuronHit>) {
        let gravity = Vec2::new(0.0, 40.0);

        for p in &mut self.particles {
            p.update(dt, gravity);

            // Wall collisions
            if p.pos.x < p.radius {
                p.pos.x = p.radius;
                p.vel.x *= -0.7;
            }
            if p.pos.x > self.width - p.radius {
                p.pos.x = self.width - p.radius;
                p.vel.x *= -0.7;
            }
        }

        // Pin collisions
        for p in &mut self.particles {
            if !p.active {
                continue;
            }
            for pin in &self.pins {
                if let Some(idx) = resolve_collision(p, pin) {
                    // Send Hit Event
                    let strength = match p.kind {
                        PacketKind::Http => 10.0,
                        PacketKind::Ssh => 15.0,
                        PacketKind::Malware => 25.0,
                    };
                    let _ = hit_tx.send(NeuronHit {
                        index: idx,
                        strength,
                    });
                    self.score += 1;
                }
            }
        }

        // Remove out of bounds
        self.particles.retain(|p| p.pos.y < self.height + 10.0);
    }

    /// Synchronizes the visual rendering loop with the high-frequency audio simulation.
    ///
    /// Since the audio thread is integrating Izhikevich equations constantly, the UI thread must occasionally pull
    /// a [`crate::audio::Snapshot`] to know what colors to paint the pins without bottlenecking the sound.
    ///
    /// ## Arguments
    ///
    /// * `snap` - The frozen network state captured from the audio thread.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::game::GameState;
    /// use synaptic_pachinko::audio::Snapshot;
    /// let mut game = GameState::new(100.0, 100.0);
    /// // Create a snapshot with voltages matching the pin count
    /// let len = game.pins.len();
    /// game.update_voltages(Snapshot { voltages: vec![-65.0; len], mean_field: -40.0 });
    /// assert_eq!(game.mean_field, -40.0);
    /// ```
    pub fn update_voltages(&mut self, snap: crate::audio::Snapshot) {
        if snap.voltages.len() == self.pins.len() {
            self.neuron_voltages = snap.voltages;
        }
        self.mean_field = snap.mean_field;
    }
}

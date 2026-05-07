pub use locus::Vec2;

#[derive(Clone, Copy, Debug, PartialEq)]
/// The protocol or classification of a network packet falling through the game.
///
/// Different protocols carry different amounts of "kinetic mass". When they strike
/// a [`crate::physics::NeuronPin`], they transfer different amounts of charge to the underlying neuron.
///
/// ## Examples
/// ```
/// use synaptic_pachinko::physics::PacketKind;
///
/// let malicious = PacketKind::Malware;
/// // Malware causes larger voltage spikes on impact
/// ```
pub enum PacketKind {
    /// Safe web traffic, impacts lightly. Rendered Green.
    Http,
    /// Encrypted shell traffic, impacts moderately. Rendered Blue.
    Ssh,
    /// Malicious traffic, strikes with high energy. Rendered Red.
    Malware,
}

#[derive(Clone, Debug)]
/// A discrete network packet traversing the physical simulation space.
///
/// Packets fall under the influence of gravity, bounce off the walls, and most importantly,
/// collide with [`crate::physics::NeuronPin`]s. Their behavior and visual representation depend on their [`crate::physics::PacketKind`].
///
/// ## Examples
/// ```
/// use synaptic_pachinko::physics::{Particle, PacketKind};
///
/// // Spawn a safe HTTP packet
/// let packet = Particle::new(50.0, 0.0, PacketKind::Http);
/// assert!(packet.active);
/// ```
pub struct Particle {
    /// Current 2D position vector.
    pub pos: Vec2,
    /// Current 2D velocity vector.
    pub vel: Vec2,
    /// The type of packet determining interaction behavior.
    pub kind: PacketKind,
    /// The collision radius.
    pub radius: f64,
    /// Active flag. Deactivated particles are culled.
    pub active: bool,
}

impl Particle {
    /// Manifests a new packet into the digital gravity well.
    ///
    /// By default, a particle spawns with zero velocity, waiting for the tick cycle's gravity to seize it.
    ///
    /// ## Arguments
    ///
    /// * `x` - The horizontal insertion point.
    /// * `y` - The vertical insertion point (usually near 0).
    /// * `kind` - The protocol classification dictating its weight/impact.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::physics::{Particle, PacketKind};
    /// let p = Particle::new(10.0, 0.0, PacketKind::Http);
    /// assert_eq!(p.pos.x, 10.0);
    /// ```
    pub fn new(x: f64, y: f64, kind: PacketKind) -> Self {
        Self {
            pos: Vec2::new(x, y),
            vel: Vec2::new(0.0, 0.0),
            kind,
            radius: 0.5,
            active: true,
        }
    }

    /// Surrenders the particle to the forces of digital physics.
    ///
    /// This method integrates velocity using the supplied gravity vector and applies a slight atmospheric friction
    /// to keep terminal velocity bounded. Inactive particles ignore these forces.
    ///
    /// ## Arguments
    ///
    /// * `dt` - The integration timestep.
    /// * `gravity` - The downward acceleration vector.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::physics::{Particle, PacketKind};
    /// use locus::Vec2;
    /// let mut p = Particle::new(0.0, 0.0, PacketKind::Http);
    /// p.update(1.0, Vec2::new(0.0, 9.8));
    /// // Velocity is subject to atmospheric friction: 9.8 * 0.99 = 9.702
    /// assert_eq!(p.vel.y, 9.702);
    /// ```
    pub fn update(&mut self, dt: f64, gravity: Vec2) {
        if !self.active {
            return;
        }
        self.vel += gravity * dt;
        self.pos += self.vel * dt;
        self.vel *= 0.99; // Friction
    }
}

#[derive(Clone, Debug)]
/// A physical obstacle in the Pachinko board that represents a living neuron.
///
/// When a [`crate::physics::Particle`] collides with a `NeuronPin`, the pin deflects the packet
/// and emits a `NeuronHit` event. The audio engine uses the `neuron_index` to route
/// the impact energy to the correct Izhikevich neuron in the simulation.
///
/// ## Examples
/// ```
/// use synaptic_pachinko::physics::NeuronPin;
///
/// // Create a pin located at (100.0, 200.0) bound to neuron index #5
/// let pin = NeuronPin::new(100.0, 200.0, 5);
/// assert_eq!(pin.neuron_index, 5);
/// ```
pub struct NeuronPin {
    /// Static 2D position.
    pub pos: Vec2,
    /// Static collision radius.
    pub radius: f64,
    /// The backing neuron index in the audio engine array.
    pub neuron_index: usize,
}

impl NeuronPin {
    /// Forges a static pin and binds it to a specific neuron index in the audio array.
    ///
    /// ## Arguments
    ///
    /// * `x` - The lattice X coordinate.
    /// * `y` - The lattice Y coordinate.
    /// * `neuron_index` - The routing identifier used to deliver kinetic payloads to the correct simulation instance.
    ///
    /// ## Examples
    /// ```
    /// use synaptic_pachinko::physics::NeuronPin;
    /// let pin = NeuronPin::new(5.0, 5.0, 42);
    /// assert_eq!(pin.neuron_index, 42);
    /// ```
    pub fn new(x: f64, y: f64, neuron_index: usize) -> Self {
        Self {
            pos: Vec2::new(x, y),
            radius: 1.0,
            neuron_index,
        }
    }
}

/// Resolves collision between particle and pin.
/// Returns Some(neuron_index) if collision occurred.
pub fn resolve_collision(particle: &mut Particle, pin: &NeuronPin) -> Option<usize> {
    let diff = particle.pos - pin.pos;
    let dist_sq = diff.magnitude_squared();
    let min_dist = particle.radius + pin.radius;

    if dist_sq < min_dist * min_dist {
        // Collision!
        let dist = dist_sq.sqrt();
        let normal = if dist == 0.0 {
            Vec2::new(0.0, 1.0)
        } else {
            diff * (1.0 / dist)
        };

        let overlap = min_dist - dist;
        particle.pos += normal * overlap;

        let restitution = 0.8;
        let v_dot_n = particle.vel.dot(normal);

        if v_dot_n < 0.0 {
            let j = -(1.0 + restitution) * v_dot_n;
            particle.vel += normal * j;
            return Some(pin.neuron_index);
        }
    }
    None
}

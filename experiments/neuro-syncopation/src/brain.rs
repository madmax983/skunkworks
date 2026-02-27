use parking_lot::Mutex;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::thread;
use std::time::{Duration, Instant};
use neuro_sim::Izhikevich;

/// Shared state for visualization
#[derive(Clone, Debug)]
pub struct NeuronState {
    pub voltage: f32,
    pub spiked: bool,
    pub contention_events: usize, // How many times we blocked on a lock
    pub last_spike: Option<Instant>,
}

impl Default for NeuronState {
    fn default() -> Self {
        Self {
            voltage: -65.0,
            spiked: false,
            contention_events: 0,
            last_spike: None,
        }
    }
}

/// Buffer for incoming signals, protected by a Mutex
#[derive(Default, Debug)]
pub struct InputBuffer {
    pub current: f32,
}

/// A connection to another neuron
#[derive(Clone)]
pub struct Synapse {
    pub target: Arc<Mutex<InputBuffer>>,
    pub weight: f32,
}

pub struct Neuron {
    #[allow(dead_code)]
    pub id: usize,
    pub input: Arc<Mutex<InputBuffer>>,
    pub synapses: Vec<Synapse>,
    pub physics: Izhikevich,
    pub state: Arc<Mutex<NeuronState>>,
    pub running: Arc<AtomicBool>,
}

impl Neuron {
    pub fn new(
        id: usize,
        input: Arc<Mutex<InputBuffer>>,
        state: Arc<Mutex<NeuronState>>,
        running: Arc<AtomicBool>,
    ) -> Self {
        Self {
            id,
            input,
            synapses: Vec::new(),
            physics: Izhikevich::new_regular_spiking(), // Default to RS
            state,
            running,
        }
    }

    pub fn run(mut self) {
        // Time step for the physics simulation
        let dt = 1.0;

        // We sleep to keep the simulation somewhat real-time,
        // but the "rhythm" comes from lock contention.
        let sleep_duration = Duration::from_millis(1);

        while self.running.load(Ordering::Relaxed) {
            // 1. Read Input
            let input_current = {
                let mut buffer = self.input.lock();
                let current = buffer.current;
                // Decay the input in the buffer (simple model: clear it, or exponential decay?)
                // If we clear it, it means inputs are instantaneous pulses.
                // If we decay it, it lingers. Izhikevich `inject` does decay internally if we use it properly,
                // but `update` takes `extra_current` which is instantaneous.
                // Let's assume the buffer holds "charge arrived since last step".
                buffer.current = 0.0;
                current
            };

            // 2. Update Physics
            // We use `update` which adds `extra_current` to the internal calculation.
            // Note: `input_current` here acts as a pulse for this time step.
            let (v, spiked) = self.physics.update(dt, input_current);

            // 3. Update Visualization State
            {
                let mut state = self.state.lock();
                state.voltage = v;
                state.spiked = spiked;
                if spiked {
                    state.last_spike = Some(Instant::now());
                }
            }

            // 4. Propagate Spikes
            if spiked {
                for synapse in &self.synapses {
                    // This is where "Syncopation" happens.
                    // We attempt to lock the target. If it's locked, we wait.
                    // The waiting is the physical cost of communication.

                    // We can track if we had to wait to measure contention.
                    // `parking_lot` doesn't easily tell us "did we wait", but `try_lock` does.

                    // Try to acquire lock without blocking first to check for contention
                    if let Some(mut target_buffer) = synapse.target.try_lock() {
                        target_buffer.current += synapse.weight;
                    } else {
                        // Contention!
                        {
                            let mut state = self.state.lock();
                            state.contention_events += 1;
                        }
                        // Now block until we get it
                        let mut target_buffer = synapse.target.lock();
                        target_buffer.current += synapse.weight;
                    }
                }
            }

            // 5. Temporal Regulation
            thread::sleep(sleep_duration);
        }
    }
}

pub struct Brain {
    pub neuron_states: Vec<Arc<Mutex<NeuronState>>>,
    // We keep input buffers to connect new synapses if we wanted dynamic rewiring
    pub input_buffers: Vec<Arc<Mutex<InputBuffer>>>,
    pub running: Arc<AtomicBool>,
    pub handles: Vec<thread::JoinHandle<()>>,
}

impl Brain {
    pub fn new(size: usize) -> Self {
        let running = Arc::new(AtomicBool::new(true));
        let mut neuron_states = Vec::new();
        let mut input_buffers = Vec::new();

        for _ in 0..size {
            neuron_states.push(Arc::new(Mutex::new(NeuronState::default())));
            input_buffers.push(Arc::new(Mutex::new(InputBuffer::default())));
        }

        Self {
            neuron_states,
            input_buffers,
            running,
            handles: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        let size = self.neuron_states.len();

        // Create Neurons and Synapses
        // For demonstration, we create a fully connected network or random connections
        // But we need to move Neurons into threads.

        // We need to construct the neurons first, then connect them, then spawn them.
        let mut neurons = Vec::new();

        for i in 0..size {
            let n = Neuron::new(
                i,
                self.input_buffers[i].clone(),
                self.neuron_states[i].clone(),
                self.running.clone(),
            );
            neurons.push(n);
        }

        // Connect Neurons (Random Small World or just Random)
        let mut rng = rand::thread_rng();
        use rand::Rng;

        // Clone input buffers to avoid borrow checker issues when iterating neurons
        let buffers = self.input_buffers.clone();

        for (i, neuron) in neurons.iter_mut().enumerate() {
            // Randomize physics parameters
            neuron.physics = Izhikevich::random(&mut rng);

            // Connect to random other neurons
            let num_connections = 5; // Fixed out-degree for now
            for _ in 0..num_connections {
                let target_idx = rng.gen_range(0..size);
                if target_idx != i {
                    let weight = if rng.gen_bool(0.8) {
                        30.0 // Excitatory (strong to ensure propagation)
                    } else {
                        -10.0 // Inhibitory
                    };

                    neuron.synapses.push(Synapse {
                        target: buffers[target_idx].clone(),
                        weight,
                    });
                }
            }
        }

        // Spawn Threads
        for neuron in neurons {
            let handle = thread::spawn(move || {
                neuron.run();
            });
            self.handles.push(handle);
        }
    }

    pub fn stop(&mut self) {
        self.running.store(false, Ordering::Relaxed);
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

impl Drop for Brain {
    fn drop(&mut self) {
        self.stop();
    }
}

use crate::audio::{AudioCommand, AudioEngine, Snapshot};
use crate::lattice::{Lattice, LatticeType};
use nalgebra::Point3;
use neuro_sim::Izhikevich;
use rand::prelude::*;

pub struct Brain {
    pub lattice: Lattice,
    pub neurons: Vec<Izhikevich>,
    pub adj: Vec<Vec<usize>>,
    pub weights: Vec<Vec<f32>>, // For now simple weight
    pub audio_engine: AudioEngine,
}

impl Brain {
    pub fn new(l_type: LatticeType, size: usize) -> anyhow::Result<Self> {
        let lattice = Lattice::new(l_type, size, 4.0);
        let n = lattice.points.len();
        let mut neurons = Vec::with_capacity(n);
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            neurons.push(Izhikevich::random(&mut rng));
        }

        let (adj, weights) = build_connectivity(&lattice, 5.0); // connect within 5.0 units

        let audio_engine = AudioEngine::new()?;

        // Send initial state to audio engine
        // We need to construct the adjacency list with weights
        let mut connections = Vec::with_capacity(n);
        for (i, neighbors) in adj.iter().enumerate() {
            let mut w_vec = Vec::new();
            for (j_idx, &neighbor) in neighbors.iter().enumerate() {
                w_vec.push((neighbor, weights[i][j_idx]));
            }
            connections.push(w_vec);
        }

        let _ = audio_engine.cmd_tx.send(AudioCommand::UpdateNetwork {
            neurons: neurons.clone(),
            connections,
        });

        Ok(Self {
            lattice,
            neurons,
            adj,
            weights,
            audio_engine,
        })
    }

    pub fn change_lattice(&mut self, l_type: LatticeType, size: usize) {
        self.lattice = Lattice::new(l_type, size, 4.0);
        let n = self.lattice.points.len();

        // Resize neurons if needed or re-init?
        // Let's keep old neurons if possible to preserve state, but mapping is hard.
        // Re-init is cleaner.
        let mut rng = rand::thread_rng();
        self.neurons.clear();
        for _ in 0..n {
            self.neurons.push(Izhikevich::random(&mut rng));
        }

        let (adj, weights) = build_connectivity(&self.lattice, 5.0);
        self.adj = adj;
        self.weights = weights;

        // Send update to audio
        let mut connections = Vec::with_capacity(n);
        for (i, neighbors) in self.adj.iter().enumerate() {
            let mut w_vec = Vec::new();
            for (j_idx, &neighbor) in neighbors.iter().enumerate() {
                w_vec.push((neighbor, self.weights[i][j_idx]));
            }
            connections.push(w_vec);
        }

        let _ = self.audio_engine.cmd_tx.send(AudioCommand::UpdateNetwork {
            neurons: self.neurons.clone(),
            connections,
        });
    }

    pub fn update(&mut self) {
        // Poll for snapshots from audio engine
        // Drain the queue to get latest
        #[cfg(feature = "audio")]
        {
            while let Some(snap) = self.audio_engine.snapshot_rx.pop() {
                // Update local neuron voltages for display
                if snap.voltages.len() == self.neurons.len() {
                    for (i, v) in snap.voltages.iter().enumerate() {
                        self.neurons[i].v = *v;
                    }
                }
            }
        }

        #[cfg(not(feature = "audio"))]
        {
            while let Ok(snap) = self.audio_engine.snapshot_rx.try_recv() {
                if snap.voltages.len() == self.neurons.len() {
                    for (i, v) in snap.voltages.iter().enumerate() {
                        self.neurons[i].v = *v;
                    }
                }
            }
            // If no audio, simulate locally?
            // Yes, otherwise nothing happens.
            let dt = 1.0;
            for neuron in &mut self.neurons {
                let _ = neuron.update(dt, 0.0);
            }
            // We ignore propagation in fallback for simplicity unless needed.
        }
    }

    pub fn inject(&mut self, index: usize, current: f32) {
        // Send injection command
        let _ = self
            .audio_engine
            .cmd_tx
            .send(AudioCommand::Inject { index, current });

        // Also update local state for immediate feedback if laggy?
        // No, let snapshot handle it.
        // But for non-audio mode:
        #[cfg(not(feature = "audio"))]
        if index < self.neurons.len() {
            self.neurons[index].inject(current);
        }
    }
}

fn build_connectivity(lattice: &Lattice, radius: f64) -> (Vec<Vec<usize>>, Vec<Vec<f32>>) {
    let n = lattice.points.len();
    let mut adj = vec![Vec::new(); n];
    let mut weights = vec![Vec::new(); n];

    for i in 0..n {
        let p1 = lattice.points[i];
        for j in (i + 1)..n {
            let p2 = lattice.points[j];
            let dist_sq = (p1 - p2).norm_squared();
            if dist_sq < radius * radius {
                let w = 1.0 / (dist_sq.sqrt() + 0.1); // Closer = stronger
                adj[i].push(j);
                weights[i].push(w as f32);
                adj[j].push(i);
                weights[j].push(w as f32);
            }
        }
    }
    (adj, weights)
}

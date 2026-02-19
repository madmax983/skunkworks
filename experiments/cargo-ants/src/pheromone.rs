use petgraph::graph::EdgeIndex;
use rayon::prelude::*;

pub struct PheromoneMap {
    values: Vec<f32>,
}

impl PheromoneMap {
    pub fn new(capacity: usize) -> Self {
        PheromoneMap {
            values: vec![0.0; capacity],
        }
    }

    pub fn ensure_capacity(&mut self, capacity: usize) {
        if capacity > self.values.len() {
            self.values.resize(capacity, 0.0);
        }
    }

    pub fn get(&self, edge: EdgeIndex) -> f32 {
        if edge.index() < self.values.len() {
            self.values[edge.index()]
        } else {
            0.0
        }
    }

    pub fn deposit(&mut self, edge: EdgeIndex, amount: f32) {
        if edge.index() >= self.values.len() {
            self.ensure_capacity(edge.index() + 1);
        }
        self.values[edge.index()] += amount;
    }

    pub fn evaporate(&mut self, rate: f32) {
        self.values.par_iter_mut().for_each(|v| {
            *v *= rate;
            if *v < 0.01 { *v = 0.0; }
        });
    }
}

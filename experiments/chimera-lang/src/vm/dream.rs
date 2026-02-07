use crate::ast::Strand;
use crate::vm::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DreamTrace {
    pub timestamp: usize,
    pub strand_idx: usize,
    pub duration: usize,
    pub energy_cost: i64,
    pub result_energy: i64,
    pub status: u8, // 1=Alive, 0=Dead
    pub mutation_desc: String,
    pub mutated_strand: Option<Strand>,
    pub accepted: bool,
    pub output_log: Vec<String>,
    pub grid_snapshot: Option<Vec<Vec<Value>>>,
}

impl DreamTrace {
    pub fn new(
        timestamp: usize,
        strand_idx: usize,
        duration: usize,
        energy_cost: i64,
        result_energy: i64,
        status: u8,
        mutation_desc: String,
        mutated_strand: Option<Strand>,
        accepted: bool,
        output_log: Vec<String>,
        grid_snapshot: Option<Vec<Vec<Value>>>,
    ) -> Self {
        Self {
            timestamp,
            strand_idx,
            duration,
            energy_cost,
            result_energy,
            status,
            mutation_desc,
            mutated_strand,
            accepted,
            output_log,
            grid_snapshot,
        }
    }
}

use crate::ast::Strand;
use crate::value::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Represents a `DreamTrace`.
pub struct DreamTrace {
    /// The `timestamp` field.
    pub timestamp: usize,
    /// The `strand_idx` field.
    pub strand_idx: usize,
    /// The `duration` field.
    pub duration: usize,
    /// The `energy_cost` field.
    pub energy_cost: i64,
    /// The `result_energy` field.
    pub result_energy: i64,
    /// The `status` field.
    pub status: u8, // 1=Alive, 0=Dead
    /// The `mutation_desc` field.
    pub mutation_desc: String,
    /// The `mutated_strand` field.
    pub mutated_strand: Option<Strand>,
    /// The `accepted` field.
    pub accepted: bool,
    /// The `is_nightmare` field.
    pub is_nightmare: bool,
    /// The `output_log` field.
    pub output_log: Vec<String>,
    /// The `grid_snapshot` field.
    pub grid_snapshot: Option<Vec<Vec<Value>>>,
}

impl DreamTrace {
    #[allow(clippy::too_many_arguments)]
    /// Creates a new instance.
    ///
    /// ## Examples
    ///
    /// ```text
    /// // Example usage of new()
    /// ```
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
        is_nightmare: bool,
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
            is_nightmare,
            output_log,
            grid_snapshot,
        }
    }
}

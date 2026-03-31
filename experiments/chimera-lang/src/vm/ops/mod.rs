//! Basic Operation execution logic for the Chimera Virtual Machine.
//!
//! This module groups the fundamental low-level instructions (OpCodes) used by Chimera strands
//! into logical categories. Instead of cluttering the main VM code with hundreds of execution
//! branches, these submodules handle specific domains of operations:
//!
//! * [`bio`] - Biological operations (e.g., replication, mutation, apoptosis).
//! * [`flow`] - Control flow operations (e.g., jumps, branches, loops).
//! * [`grid`] - Grid interaction operations (e.g., reading/writing cells, moving).
//! * [`io`] - Input/Output operations (e.g., printing to logs).
//! * [`math`] - Mathematical operations (e.g., arithmetic, comparisons).
//! * [`stack`] - Stack manipulation operations (e.g., push, pop, dup, swap).

pub mod bio;
pub mod flow;
pub mod grid;
pub mod io;
pub mod math;
pub mod misc;
pub mod stack;

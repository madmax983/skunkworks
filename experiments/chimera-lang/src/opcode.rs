//! # The Genetic Alphabet 🧬
//!
//! This module defines the `OpCode` enum, which represents the set of all possible enzymes (instructions)
//! in the Chimera language.
//!
//! ## Categories
//!
//! The genetic alphabet is divided into several domains:
//!
//! ### 1. Primary Metabolism (Core)
//! Basic stack manipulation, arithmetic, and control flow.
//! - `Push`, `Pop`, `Add`, `Sub`
//! - `Jump`, `Brz`
//! - `Photosynthesize`, `Consume`
//!
//! ### 2. Spatial Interaction (Grid)
//! Interacting with the 2D Petri Dish environment.
//! - `GRead`, `GWrite` (Cell access)
//! - `Radiate`, `Siphon` (Area of Effect)
//! - `Migrate`, `Osmosis` (Movement)
//!
//! ### 3. Genetics & Evolution (Nova)
//! Self-modification and reproduction.
//! - `Mitosis` (Cloning), `Apoptosis` (Death)
//! - `Transcribe`, `Methylate` (Epigenetics)
//! - `Splice`, `Recombine`, `CrisprScan` (Gene editing)
//!
//! ### 4. Physics & Reality (Nova)
//! Manipulating the simulation constants.
//! - `Gravity`, `Lumine` (Fields)
//! - `Relativity`, `TimeWarp` (Time dilation)
//! - `PhaseShift`, `Isomerize` (State changes)
//!
//! ### 5. Metaphysics (Nova)
//! Interactions with the "Spirit" (User) and "Egregore" (Collective).
//! - `Spirit` (Input)
//! - `Pray`, `Sacrifice` (Egregore)
//! - `Simulate`, `Dream`, `Prophecy` (Future prediction)
//!
//! ### 6. Specialized Systems
//! - **Cortex**: Neural networks (`Link`, `Spark`).
//! - **Silicon**: Logic gates (`Wire`, `NAND`).
//! - **Market**: Economic transactions (`Buy`, `Sell`).

use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{AsRefStr, EnumIter, EnumString};

/// Instructions for the Chimera Virtual Machine.
///
/// Each opcode represents a fundamental action that the organism can perform,
/// ranging from basic arithmetic to genetic engineering and inter-dimensional travel.
#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr, EnumIter,
)]
#[strum(serialize_all = "snake_case")]
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
/// Enum for `OpCode`.
pub enum OpCode {
    /// Pushes a value onto the stack.
    ///
    /// **Args:** `[Nucleotide::Number(n) | Nucleotide::String(s)]`
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    Push,
    /// Pops two values, adds them, and pushes the result.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a + b ]`
    Add,
    /// Pops two values, subtracts the second from the first, and pushes the result.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a - b ]`
    Sub,
    /// Pops two values, multiplies them, and pushes the result.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a * b ]`
    Mul,
    /// Pops two values, divides the first by the second, and pushes the result.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a / b ]`
    /// **Error:** Pushes error message if `b` is 0.
    Div,
    /// Pops two values, computes remainder of division.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a % b ]`
    Mod,
    /// Pops two values, computes bitwise AND.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a & b ]`
    BitAnd,
    /// Pops two values, computes bitwise OR.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a | b ]`
    BitOr,
    /// Pops two values, computes bitwise XOR.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a ^ b ]`
    BitXor,
    /// Pops one value, computes bitwise NOT.
    ///
    /// **Stack:** `[ ..., a ] -> [ ..., !a ]`
    BitNot,
    /// Pops two values, shifts first left by second.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a << b ]`
    Shl,
    /// Pops two values, shifts first right by second.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., a >> b ]`
    Shr,
    /// Pops two values, checks if they are equal, and pushes 1 (true) or 0 (false).
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., 1 if a == b else 0 ]`
    Eq,
    /// Pops two values, checks if first is greater than second.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., 1 if a > b else 0 ]`
    Gt,
    /// Pops two values, checks if first is less than second.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., 1 if a < b else 0 ]`
    Lt,
    /// Duplicates the top value of the stack.
    ///
    /// **Stack:** `[ ..., a ] -> [ ..., a, a ]`
    Dup,
    /// Swaps the top two values of the stack.
    ///
    /// **Stack:** `[ ..., a, b ] -> [ ..., b, a ]`
    Swap,
    /// Discards the top value of the stack.
    ///
    /// **Stack:** `[ ..., a ] -> [ ... ]`
    Drop,
    /// Pops a value and prints it to the VM output log.
    ///
    /// **Stack:** `[ ..., val ] -> [ ... ]`
    Print,
    /// Jumps to a specific strand index.
    ///
    /// **Args:** `[Nucleotide::Number(strand_idx)]`
    /// **Effect:** Sets IP to `(strand_idx, 0)`.
    Jump,
    /// Branches if Zero. Jumps if the top of the stack is 0.
    ///
    /// **Args:** `[Nucleotide::Number(strand_idx)]`
    /// **Stack:** `[ ..., val ] -> [ ... ]`
    /// **Effect:** If `val == 0`, jump to `(strand_idx, 0)`.
    Brz,
    /// Generates energy from "sunlight".
    ///
    /// **Effect:** Adds 5 Energy.
    Photosynthesize,
    /// Consumes a value from the stack to gain energy.
    ///
    /// **Stack:** `[ ..., val ] -> [ ... ]`
    /// **Effect:** Adds `val` (if Int) or `val.len()` (if Str) to Energy.
    Consume,
    /// Reads a value from the grid.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., grid[y][x] ]`
    GRead,
    /// Writes a value to the grid.
    ///
    /// **Stack:** `[ ..., val, y, x ] -> [ ... ]`
    /// **Effect:** Sets `grid[y][x] = val`.
    GWrite,
    /// Radiates a value into a circular area on the grid.
    ///
    /// **Stack:** `[ ..., val, radius, y, x ] -> [ ... ]`
    /// **Effect:** Fills circle with `val`. Costs energy proportional to area.
    Radiate,
    /// Siphons values from a circular area, summing them up.
    ///
    /// **Stack:** `[ ..., radius, y, x ] -> [ ..., sum ]`
    /// **Effect:** Clears circle, pushes sum of integers.
    Siphon,
    /// Pushes the organism's own genome onto the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., len, op_strings... ]`
    /// **Effect:** Pushes length of current strand, then all ops as strings.
    Genome,
    /// Executes a grid cell as an instruction.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]` (pushes result of op if any)
    /// **Effect:** Parses `grid[y][x]` as OpCode and executes it.
    Virus,
    /// Modifies the arguments of a gene in a strand.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx, arg_idx, new_value ] -> [ ... ]`
    /// **Effect:** Changes the DNA. Can trigger entanglement effects (Nova).
    Transcribe,
    /// Jumps to a strand index specified on the stack.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    JumpS,
    /// Branches if Zero (Stack-based target).
    ///
    /// **Stack:** `[ ..., condition, strand_idx ] -> [ ... ]`
    BrzS,
    /// Pushes the current stack length.
    ///
    /// **Stack:** `[ ... ] -> [ ..., len ]`
    SLen,
    /// Pushes the number of strands in the genome.
    ///
    /// **Stack:** `[ ... ] -> [ ..., helix_len ]`
    HelixLen,
    /// Pushes the number of genes in a specific strand.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., gene_len ]`
    GeneLen,

    // Cortex Features
    /// **`Cortex`** Links two strands with a neural synapse.
    ///
    /// **Stack:** `[ ..., target_strand_idx ]` (uses current IP as source)
    Link,
    /// **`Cortex`** Severs a synapse between the current strand and a target.
    ///
    /// **Stack:** `[ ..., target_strand_idx ]`
    Sever,
    /// **`Cortex`** Fires a signal across all synapses from the current strand.
    ///
    /// **Stack:** `[ ..., amount ]`
    /// **Effect:** Increases activation level of target strands.
    Spark,
    /// **`Cortex`** Reads the current strand's activation level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., activation_level ]`
    Sense,
    /// **`Cortex`** Gates execution based on activation level.
    ///
    /// **Args:** `[Nucleotide::Number(threshold)]`
    /// **Effect:** Skips next instruction if `activation < threshold`.
    Gate,

    // Biophysics Features
    /// **`Biophysics`** Spawns a Hodgkin-Huxley neuron at the specified grid location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    NeuroGenesis,
    /// **`Biophysics`** Injects current into a neuron.
    ///
    /// **Stack:** `[ ..., amount, y, x ] -> [ ... ]`
    Stimulate,
    /// **`Biophysics`** Reads the membrane potential (voltage) of a neuron.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., voltage ]`
    Dendrite,
    /// **`Biophysics`** Connects a neuron to another target (Synapse/Output).
    ///
    /// **Stack:** `[ ..., y_target, x_target, y_source, x_source ] -> [ ... ]`
    Axon,
    /// **`Biophysics`** Adds a chemical receptor to a neuron.
    ///
    /// **Stack:** `[ ..., channel, sensitivity, threshold, y, x ] -> [ ... ]`
    Receptor,
    /// **`Biophysics`** Sets the bidirectional coupling coefficient between Neuron and Grid Voltage.
    ///
    /// **Stack:** `[ ..., weight, y, x ] -> [ ... ]`
    NeuroCoupling,
    /// **`Biophysics`** Connects a neuron to a Cortex Strand.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    NeuroSynapse,

    // Silicon Features
    /// **`Silicon`** Runs one step of Wireworld on the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Conduct,
    /// **`Silicon`** Writes a conductor (1) to the grid.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Wire,
    /// **`Silicon`** Writes an electron head (2) to the grid (Pulse).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Pulse,
    /// **`Silicon`** Toggles automatic conduction mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    /// Silicon
    Silicon,
    /// **`Silicon`** Constructs a logic gate on the grid.
    ///
    /// **Stack:** `[ ..., type, dir, y, x ] -> [ ... ]`
    /// **Types:** 0=AND, 1=OR, 2=XOR, 3=NAND, 4=NOT.
    /// **Dirs:** 0=N, 1=E, 2=S, 3=W.
    Construct,
    /// **`Silicon`** A logic gate instruction (usually on grid, not in DNA).
    ///
    /// **Args:** `[Nucleotide::String(type), Nucleotide::Number(dir)]`
    LogicGate,
    /// **`Silicon`** Creates an Input Pin (Reads from Stack -> Grid).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    PinIn,
    /// **`Silicon`** Creates an Output Pin (Reads from Grid -> Stack).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    PinOut,
    /// **`Silicon`** Creates an Emitter cell that pulses periodically.
    ///
    /// **Stack:** `[ ..., freq, y, x ] -> [ ... ]`
    Emitter,
    /// **`Silicon`** Creates a Receiver cell that triggers a strand when powered.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    Receiver,
    /// **`Silicon`** Creates a stateful D-Latch on the grid.
    ///
    /// **Stack:** `[ ..., state, y, x ] -> [ ... ]`
    Latch,
    /// **`Silicon`** Reads 4-bit signal from neighbors and pushes value to stack (Digital-to-Analog).
    ///
    /// **Stack:** `[ ... ] -> [ ..., val ]`
    DAC,
    /// **`Silicon`** Pops value and writes 4-bit signal to neighbors (Analog-to-Digital).
    ///
    /// **Stack:** `[ ..., val ] -> [ ... ]`
    ADC,
    /// **`Silicon`** Traces a connected circuit and compiles it into DNA (Biologize).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., new_strand_idx ]`
    Trace,
    /// **`Silicon`** Executes a strand as a construction blueprint (Siliconize).
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    Fabricate,

    // Havoc Features (Chaos Engineering)
    /// **`Havoc`** Sets the fault injection rate.
    ///
    /// **Stack:** `[ ..., rate ] -> [ ... ]`
    /// **Rate:** 0.0 to 1.0 (Probability per tick).
    HavocRate,
    /// **`Havoc`** Sets the scope of fault injection.
    ///
    /// **Stack:** `[ ..., mask ] -> [ ... ]`
    /// **Mask:** 1=Memory, 2=Stack, 4=Execution.
    HavocScope,

    // Elektra Features (Circuitry)
    /// **`Elektra`** Creates a Voltage Source (Battery) on the grid.
    ///
    /// **Stack:** `[ ..., voltage, y, x ] -> [ ... ]`
    Battery,
    /// **`Elektra`** Creates a Ground (0V Sink) on the grid.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Ground,
    /// **`Elektra`** Reads the Voltage at a grid location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., voltage ]`
    SenseVolt,
    /// **`Elektra`** Discharges built-up charge, damaging neighbors.
    ///
    /// **Stack:** `[ ..., power, radius ] -> [ ... ]`
    Shock,
    /// **`Elektra`** Creates a lightning strike effect.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Lightning,

    /// **`Elektra`** Converts internal Energy to Voltage at the current location.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Electrogenesis,

    /// **`Elektra`** Absorb Voltage at the current location to gain Energy.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amount ]`
    Induction,

    /// **`Elektra`** Grows a wire (Value::Int(1)) in a direction.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ... ]`
    WireGrowth,

    /// **`Elektra`** Branches if Voltage at current location is > threshold.
    ///
    /// **Stack:** `[ ..., threshold, strand_idx ] -> [ ... ]`
    CircuitBreaker,

    /// **`Elektra`** Discharges high voltage to damage/charge nearby entities.
    ///
    /// **Stack:** `[ ..., power, radius ] -> [ ... ]`
    TeslaCoil,

    /// **[Elektra + Nova]** Resurrects a dead strand using high voltage.
    ///
    /// **Stack:** `[ ..., graveyard_idx ] -> [ ..., new_strand_idx ]`
    #[cfg(all(feature = "elektra", feature = "nova"))]
    Galvanize,

    /// **[Elektra + Nova]** Fires a projectile with power derived from local voltage.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    #[cfg(all(feature = "elektra", feature = "nova"))]
    Railgun,

    /// **`Elektra`** Creates a Diode (One-way conductor) on the grid.
    ///
    /// **Stack:** `[ ..., direction, y, x ] -> [ ... ]`
    Diode,

    /// **`Elektra`** Creates a Transistor (Switch) on the grid.
    ///
    /// **Stack:** `[ ..., base_direction, y, x ] -> [ ... ]`
    Transistor,

    /// **`Elektra`** Creates a Muscle (Actuator) on the grid.
    ///
    /// **Stack:** `[ ..., threshold, y, x ] -> [ ... ]`
    Muscle,

    /// **`Elektra`** Creates a Sensor (Source) on the grid.
    ///
    /// **Stack:** `[ ..., mode, y, x ] -> [ ... ]`
    Sensor,

    /// **`Elektra`** Patches a grid location to a VM parameter (Voltage Control).
    ///
    /// **Stack:** `[ ..., source_type, y, x, target_param ] -> [ ... ]`
    /// **Source:** 0=Voltage.
    /// **Target:** 0=EnergyRegen, 1=MutationRate.
    Patch,

    /// **`Elektra`** Moves the organism based on the local voltage gradient (Electrophoresis).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Moves towards lower voltage potential.
    Electrophoresis,

    /// **`Elektra`** Modifies the electrical resistance of the current grid cell.
    ///
    /// **Stack:** `[ ..., resistance ] -> [ ... ]`
    Modulate,

    /// **`Elektra`** Creates a Capacitor (Energy Storage) on the grid.
    ///
    /// **Stack:** `[ ..., capacitance, y, x ] -> [ ... ]`
    Capacitor,

    /// **`Elektra`** Creates a Memristor (Memory Resistor) on the grid.
    ///
    /// **Stack:** `[ ..., initial_memristance, y, x ] -> [ ... ]`
    Memristor,

    // Nova Features
    /// **`Nova`** Expands an L-System axiom using rules and iterations.
    ///
    /// **Stack:** `[ ..., axiom, rules, iterations ] -> [ ..., result_string ]`
    Morph,
    /// **`Nova`** Grows a structure on the grid using Turtle graphics commands.
    ///
    /// **Stack:** `[ ..., instruction_string, start_y, start_x ] -> [ ... ]`
    Grow,
    /// **`Nova`** Spawns a Seed organelle that grows procedurally over time (The Garden).
    ///
    /// **Stack:** `[ ..., rules, axiom ] -> [ ... ]`
    Plant,

    /// **`Nova`** Creates a "time-travel" snapshot (Spore) of the VM state.
    ///
    /// **Stack:** `[ ... ] -> [ ..., spore_id ]`
    /// **Cost:** 50 Energy.
    Sporulate,
    /// **`Nova`** Restores the VM state from a Spore.
    ///
    /// **Stack:** `[ ..., spore_id ] -> [ ... ]`
    /// **Effect:** Reverts *everything* (Grid, DNA, Stack) to the spore's state.
    Germinate,
    /// **`Nova`** Creates a named Time Loop anchor.
    ///
    /// **Stack:** `[ ..., loop_id ] -> [ ... ]`
    /// **Effect:** Saves state to a specific slot.
    TimeLoop,
    /// **`Nova`** Triggers a Paradox, rewinding time but keeping a value.
    ///
    /// **Stack:** `[ ..., loop_id, value ] -> [ ..., value ]`
    /// **Effect:** Restores state from loop_id, but pushes value to stack.
    /// **Risk:** Increases Paradox counter. Too much Paradox causes issues.
    Paradox,
    /// **`Nova`** Splices a strand from a saved Spore (timeline) into the current genome.
    ///
    /// **Stack:** `[ ..., spore_id, strand_idx ] -> [ ..., new_strand_idx ]`
    ChronosSplice,
    /// **`Nova`** Creates a new DNA strand from a sequence of values on the grid.
    ///
    /// **Stack:** `[ ..., len, y, x ] -> [ ... ]`
    /// **Effect:** Reads `len` cells starting at `(x, y)` and compiles them into a new strand.
    Incubate,
    /// **`Nova`** Marks a gene as methylated (epigenetics).
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    /// **Effect:** Adds an epigenetic marker.
    Methylate,
    /// **`Nova`** Removes a methylation marker.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    Demethylate,
    /// **`Nova`** Extends the lifespan (telomeres) of the current strand.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Cost:** 25 Energy.
    Telomerase,
    /// **`Nova`** Reads the remaining telomere length of the current strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., length ]`
    TLen,
    /// **`Nova`** Swaps the tails of two strands at a split point.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, split_idx ] -> [ ... ]`
    Recombine,
    /// **`Nova`** Slices and recombines two strands into a new one (Laboratory Splicing).
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, method ] -> [ ..., new_strand_idx ]`
    /// **Methods:** 0=Interleave, 1=Uniform Crossover, 2=Midpoint Split.
    Splice,
    /// **`Nova`** Stitches two strands together with high-voltage seams.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, stitches ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates a new strand by alternating chunks of A and B, inserting Sparks/Glitches at seams.
    Frankenstein,
    /// **`Nova`** Performs a single-point crossover at a random index.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b ] -> [ ..., new_strand_1, new_strand_2 ]`
    Crossover,
    /// **`Nova`** Pushes the index of the currently executing strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., current_strand_idx ]`
    SIndex,
    /// **`Nova`** Scans a target strand for a pattern matching a guide strand.
    ///
    /// **Stack:** `[ ..., target_idx, guide_idx ] -> [ ..., match_index ]`
    /// **Effect:** Returns index of first match or -1.
    CrisprScan,
    /// **`Nova`** Cuts a strand into two at a specific index.
    ///
    /// **Stack:** `[ ..., strand_idx, cut_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** The tail becomes a new strand.
    Cas9Cut,
    /// **`Nova`** Joins two strands together.
    ///
    /// **Stack:** `[ ..., recipient_idx, donor_idx ] -> [ ... ]`
    /// **Effect:** Appends donor genes to recipient. Donor becomes empty.
    Ligase,
    /// **`Nova`** Clones a strand perfectly.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Creates a new identical strand.
    Mitosis,
    /// **`Nova`** Destroys a strand.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Clears genes and epigenetics of the strand.
    Apoptosis,
    /// **`Nova`** Inserts a new gene into a strand.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx, op_name, arg ] -> [ ... ]`
    Integrase,
    /// **`Nova`** Removes a gene from a strand.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    Excision,
    /// **`Nova`** Secretes hormones into the environment.
    ///
    /// **Stack:** `[ ..., channel, amount ] -> [ ... ]`
    /// **Effect:** Adds to hormone grid at current location.
    Secrete,
    /// **`Nova`** Detects hormone levels.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., intensity ]`
    Detect,
    /// **`Nova`** Absorbs hormones from the environment.
    ///
    /// **Stack:** `[ ..., channel, amount ] -> [ ..., absorbed_amount ]`
    Absorb,
    /// **`Nova`** Moves the execution context (spatial location).
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    /// **Cost:** 5 Energy. Blocked by walls/membranes.
    Migrate,
    /// **`Nova`** Cleans waste from the environment.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Detox,
    /// **`Nova`** Reads local waste level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., waste_level ]`
    WRead,
    /// **`Nova`** Calls another strand as a subroutine.
    ///
    /// **Args:** `[Nucleotide::Number(strand_idx)]`
    /// **Effect:** Pushes return address to `call_stack` and jumps.
    Call,
    /// **`Nova`** Calls a strand index popped from the stack.
    ///
    /// **Stack:** `[ ..., strand_idx ]`
    Exec,
    /// **`Nova`** Returns from a subroutine.
    ///
    /// **Effect:** Pops address from `call_stack` and jumps.
    Ret,
    /// **`Nova`** Binds a keyboard input to a strand.
    ///
    /// **Stack:** `[ ..., char_code, strand_idx ] -> [ ... ]`
    /// **Effect:** Pressing the key will trigger an interrupt on that strand.
    Bind,
    /// **`Nova`** Unbinds a keyboard input.
    ///
    /// **Stack:** `[ ..., char_code ] -> [ ... ]`
    Unbind,
    /// **`Nova`** Registers an interrupt handler for internal events.
    ///
    /// **Stack:** `[ ..., event_id, strand_idx ] -> [ ... ]`
    Reflex,
    /// **`Nova`** Quantum entangles two strands.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b ] -> [ ... ]`
    /// **Effect:** Mutations/Transcriptions on one strand affect the other.
    Entangle,
    /// **`Nova`** Breaks quantum entanglement.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Decohere,
    /// **`Nova`** Moves the current gene to a new location in the strand (Transposon).
    ///
    /// **Stack:** `[ ..., offset ] -> [ ... ]`
    /// **Effect:** Moves current instruction `offset` genes away, replacing self with Nop. Jumps to new location.
    Transposon,
    /// **`Nova`** Writes a strand's code onto the grid physically.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x, direction ] -> [ ... ]`
    Conjugate,
    /// **`Nova`** Pulls items on the grid towards the center.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Gravitate,
    /// **`Nova`** Emits light into the environment.
    ///
    /// **Stack:** `[ ..., intensity, radius ] -> [ ... ]`
    Lumine,
    /// **`Nova`** Senses light level at current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., intensity ]`
    SenseLight,
    /// **`Nova`** Sets the bioluminescent color and intensity at the current location.
    ///
    /// **Stack:** `[ ..., r, g, b, intensity ] -> [ ... ]`
    Luciferin,
    /// **`Nova`** Emits the current location's bioluminescence to neighbors.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Photophore,
    /// **`Nova`** Spawns an Organelle (sub-process).
    ///
    /// **Stack:** `[ ..., strand_idx, type ] -> [ ... ]`
    /// **Types:** 1=Chloroplast, 2=Mitochondria, 3=Lysosome, 4=Ribosome.
    Spawn,
    /// **`Nova`** Runs a simulation of a strand in a sandboxed VM.
    ///
    /// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., result, energy, status ]`
    /// **Cost:** High energy cost.
    Simulate,
    /// **`Nova`** Simulates a mutated version of a strand; adopts if beneficial.
    ///
    /// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., success ]`
    Dream,
    /// **`Nova`** Reduces local entropy to prevent Nightmares.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Lucid,
    /// **`Nova`** Calculates direction towards highest chemical concentration.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., dy, dx ]`
    Chemotaxis,
    /// **`Nova`** Returns the identity (Organelle Type) of the current executor.
    ///
    /// **Stack:** `[ ... ] -> [ ..., type_id ]`
    Identity,
    /// **`Nova`** Changes the type of the current organelle.
    ///
    /// **Stack:** `[ ..., type_id ] -> [ ... ]`
    Differentiate,
    /// **`Nova`** Scans a line on the grid for non-empty cells.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ..., distance, value ]`
    Sonar,
    /// **`Nova`** Opens a spatial portal between two points.
    ///
    /// **Stack:** `[ ..., y1, x1, y2, x2 ] -> [ ... ]`
    Rift,
    /// **`Nova`** Closes a portal.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Seal,
    /// **`Nova`** Warps the topology of the grid (e.g., Torus, Klein Bottle).
    ///
    /// **Stack:** `[ ..., topology_type ] -> [ ... ]`
    /// **Cost:** 100 Energy.
    Shape,
    /// **`Nova`** Broadcasts a value to a global radio channel.
    ///
    /// **Stack:** `[ ..., channel, value ] -> [ ... ]`
    Broadcast,
    /// **`Nova`** Receives a value from a global radio channel.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., value ]`
    Tune,

    // Retina Features (Vision)
    /// **`Nova`** Draws a pixel to the Retina buffer.
    ///
    /// **Stack:** `[ ..., packed_color, char_code, y, x ] -> [ ... ]`
    /// **Packed Color:** `(R << 16) | (G << 8) | B`.
    RetinaDraw,
    /// **`Nova`** Clears the Retina buffer with a color.
    ///
    /// **Stack:** `[ ..., packed_color ] -> [ ... ]`
    RetinaClear,
    /// **`Nova`** Pushes the dimensions of the Retina buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., width, height ]`
    RetinaSize,
    /// **`Nova`** Reads a row of the Retina buffer.
    ///
    /// **Stack:** `[ ..., row_index ] -> [ ..., junction_of_pixels ]`
    Scanline,
    /// **`Nova`** Writes a Junction of values to the Retina buffer with glitch effects.
    ///
    /// **Stack:** `[ ..., y, x, junction, mode ] -> [ ... ]`
    /// **Mode:** 0=Linear, 1=Scatter, 2=XOR, 3=Sort.
    Rasterize,

    // Quantum Features (Superposition)
    /// **`Nova`** Instantly jumps to the entangled partner strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Sets IP to partner strand at current gene index.
    QuantumJump,
    /// **`Nova`** Creates a quantum superposition of the top two values.
    ///
    /// **Stack:** `[ ..., val_a, val_b ] -> [ ..., Ψ(val_a:0.5, val_b:0.5) ]`
    Superpose,
    /// **`Nova`** Collapses a superposition into a single value based on probability.
    ///
    /// **Stack:** `[ ..., superposition ] -> [ ..., collapsed_val ]`
    Collapse,
    /// **`Nova`** Observes the value, collapsing it and logging the result.
    ///
    /// **Stack:** `[ ..., superposition ] -> [ ..., collapsed_val ]`
    Observe,

    /// **`Nova`** Sings a note into the Chorus Buffer.
    ///
    /// **Stack:** `[ ..., note_string ] -> [ ... ]`
    /// **Effect:** Checks for Chords (magic spells).
    Sing,
    /// **`Nova`** Listens to the Chorus Buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., chorus_junction ]`
    Listen,
    /// **`Nova`** Registers a chord to trigger a strand.
    ///
    /// **Stack:** `[ ..., chord_junction, strand_idx ] -> [ ... ]`
    Harmonize,
    /// **`Nova`** Spawns a Choir organelle that sings a song.
    ///
    /// **Stack:** `[ ..., song_junction ] -> [ ... ]`
    Choir,

    /// **`Nova`** Pauses execution and requests input from the Spirit (User).
    ///
    /// **Stack:** `[ ... ] -> [ ..., input_value ]`
    Spirit,

    /// **`Nova`** Shifts the organism's phase of matter.
    ///
    /// **Stack:** `[ ..., phase_id ] -> [ ... ]`
    /// **Modes:** 0=Corporeal, 1=Ethereal, 2=Crystalline, 3=Flux.
    /// **Cost:** 50 Energy.
    PhaseShift,
    /// **`Nova`** Toggles the organism's chirality (L-isomer <-> D-isomer).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Inverts arithmetic and control flow logic.
    Isomerize,
    /// **`Nova`** Modifies cellular membranes (walls) at current location.
    ///
    /// **Stack:** `[ ..., mask ] -> [ ... ]`
    /// **Mask:** 1=N, 2=S, 4=E, 8=W.
    Membrane,
    /// **`Nova`** Moves through membranes/walls at high cost.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    Osmosis,
    /// **`Nova`** Merges an organelle back into the main organism (Symbiote).
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    Symbiosis,
    /// **`Nova`** Ejects a Symbiote as a free-roaming Organelle.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Lysis,

    // Necromancy Features
    /// **`Nova`** Buries a strand in the graveyard (copies and clears).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Bury,
    /// **`Nova`** Exhumes a strand from the graveyard (restores it).
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    Exhume,
    /// **`Nova`** Executes a dead strand from the graveyard (ephemeral).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Seance,
    /// **`Nova`** Gains energy from the graveyard.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    Mourn,

    /// **`Nova`** Kills the target strand and spawns a mutated copy.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., new_strand_idx ]`
    Reincarnate,

    /// **`Nova`** Splits a strand into a Horcrux object on the grid.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    /// **Effect:** Kills strand, writes "Horcrux:..." to grid.
    Horcrux,

    /// **`Nova`** Consumes a Horcrux to resurrect the strand.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Reads Horcrux, restores strand, clears grid cell.
    Rebirth,

    /// **`Nova`** Compiles a string into a strand.
    ///
    /// **Stack:** `[ ..., source_string ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Parses source and creates new strand.
    Compile,
    /// **`Nova`** Decompiles a strand into a string.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., source_string ]`
    Decompile,

    /// **`Nova`** Spawns a Void organelle that consumes everything.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Void,
    /// **`Nova`** Opens a Void Rift at the specified location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    VoidRift,
    /// **`Nova`** Channels power from the nearest Void Rift.
    ///
    /// **Stack:** `[ ... ] -> [ ..., power ]`
    VoidCast,
    /// **`Nova`** Explodes the current strand, scattering genes onto the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Supernova,
    /// **`Nova`** Merges all strands into a single massive strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Singularity,

    /// **`Nova`** Increases local mutagen level.
    ///
    /// **Stack:** `[ ..., amount, radius ] -> [ ... ]`
    Irradiate,
    /// **`Nova`** Reads local mutagen level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., level ]`
    SenseMutagen,
    /// **`Nova`** Consumes local mutagen to gain energy.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    Devour,

    /// **`Nova`** Runs a cellular automaton step on the Grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Evolve,
    // Garden Features (Nova)
    /// **`Nova`** Defines a Cellular Automata rule for a species.
    ///
    /// **Stack:** `[ ..., rule_string, species_id ] -> [ ... ]`
    /// **Rule String:** e.g. "B3/S23" (Life), "B36/S23" (HighLife).
    Sow,
    /// **`Nova`** Harvests the grid pattern into a compressed string.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., rle_string ]`
    Harvest,
    /// **`Nova`** Randomly corrupts the Grid or Stack.
    ///
    /// **Stack:** `[ ..., severity ] -> [ ... ]`
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    /// Glitch
    Glitch,
    /// **`Nova`** Shuffles the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Scramble,

    /// **`Nova`** Pushes local entropy level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., entropy_level ]`
    Entropy,
    /// **`Nova`** Reduces local entropy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Cost:** Energy proportional to amount.
    Stabilize,
    /// **`Nova`** Increases global entropy and injects chaos into a random grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    EntropySurge,
    /// **`Nova`** Forcefully jumps the IP to a random gene in the current strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    QuantumTunnel,
    /// **`Nova`** Instantly converts a grid cell to high entropy.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Disintegrate,

    /// **`Nova`** Forks time into N timelines, selects one where the query is true.
    ///
    /// **Stack:** `[ ..., count, query_term ] -> [ ..., success ]`
    /// **Effect:** Replaces current VM state with the successful timeline.
    #[cfg(all(feature = "nova", feature = "oracle"))]
    Divergence,

    /// **`Nova`** Predicts if the current execution path leads to death within `ticks`.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ..., 1(Death)|0(Life) ]`
    Prophecy,

    /// **`Nova`** Toggles Logic Chemistry mode (Logos).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Enables/Disables active unification of grid cells based on Oracle rules.
    Logos,

    /// **`Nova`** Transmutes the current grid cell based on neighbors (Alchemy).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Alchemy,

    /// **`Nova`** Reads the grid geometry into a strand (Grid -> DNA).
    ///
    /// **Stack:** `[ ..., radius, start_y, start_x ] -> [ ..., new_strand_idx ]`
    AbsorbGeometry,

    /// **`Nova`** Projects a strand onto the grid geometry (DNA -> Grid).
    ///
    /// **Stack:** `[ ..., strand_idx, start_y, start_x ] -> [ ... ]`
    ProjectGeometry,

    /// **`Nova`** Triggers a metamorphic reboot based on a CA rule.
    ///
    /// **Stack:** `[ ..., rule_id ] -> [ ... ]`
    /// **Effect:** Replaces entire DNA with genes derived from the Grid state.
    Genesis,

    /// **`Nova`** Injects pure entropy into the system.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Effect:** Increases Havoc rate and local Entropy.
    Chaos,

    /// **[Mad Science]** Creates a new Virus using a Meme as payload and a Grammar for mutation.
    ///
    /// **Stack:** `[ ..., name_str, grammar_junction, meme_id ] -> [ ... ]`
    BioHack,

    /// **[Mad Science]** Replaces occurrences of an OpCode with another with a given probability.
    ///
    /// **Stack:** `[ ..., target_strand, probability, from_op_str, to_op_str ] -> [ ... ]`
    Mutagen,

    /// **[Mad Science]** Modifies the TUI state directly.
    ///
    /// **Stack:** `[ ..., value, mode ] -> [ ... ]`
    /// **Modes:** 0=Glitch, 1=Shake, 2=Message.
    TuiMod,

    /// **[Mad Science]** Writes the current genome to a new file (Viral Replication).
    ///
    /// **Stack:** `[ ... ] -> [ ..., success ]`
    SelfReplicate,

    /// **`Nova`** Interacts with the Genetic Codex (Spellbook).
    ///
    /// **Stack:** `[ ..., spell_id ] -> [ ... ]`
    Codex,

    // Verbum Features (The Word)
    /// **`Nova`** Forges a new Word from a strand.
    ///
    /// **Stack:** `[ ..., name_str, strand_idx ] -> [ ..., word_id ]`
    Forge,
    /// **`Nova`** Speaks a Word by name (Verbum).
    ///
    /// **Stack:** `[ ..., name_str ] -> [ ... ]`
    Speak,
    /// **`Nova`** Pushes the genes of a Word to the stack.
    ///
    /// **Stack:** `[ ..., name_str ] -> [ ..., gene_junction ]`
    Etymology,

    // Meta-Evolution Features
    /// **`Evo`** Pushes the current population size.
    ///
    /// **Stack:** `[ ... ] -> [ ..., size ]`
    EvoPopSize,
    /// **`Evo`** Loads a strand from the population by index.
    ///
    /// **Stack:** `[ ..., index ] -> [ ..., strand_id ]`
    /// **Effect:** Pushes an opaque ID (index) for the strand.
    EvoLoad,
    /// **`Evo`** Stores a strand from the stack (genes) into the buffer.
    ///
    /// **Stack:** `[ ..., gene_junction ] -> [ ... ]`
    EvoStore,
    /// **`Evo`** Calculates fitness of a strand.
    ///
    /// **Stack:** `[ ..., strand_id ] -> [ ..., fitness_score ]`
    EvoScore,
    /// **`Evo`** Breeds two strands (Crossover).
    ///
    /// **Stack:** `[ ..., strand_id_a, strand_id_b ] -> [ ..., new_strand_id ]`
    /// **Effect:** Creates child in buffer and returns its ID.
    EvoBreed,
    /// **`Evo`** Mutates a strand in place (in the buffer).
    ///
    /// **Stack:** `[ ..., strand_id ] -> [ ... ]`
    EvoMutate,
    /// **`Evo`** Replaces the main population with the buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    EvoReplace,
    /// **`Evo`** Clears the evolution buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    EvoClear,
    /// **`Evo`** Saves a strand from population to buffer (Clone).
    ///
    /// **Stack:** `[ ..., pop_idx ] -> [ ..., buf_idx ]`
    EvoSave,

    // Prologue Features (Rune Logic)
    /// **`Prologue`** Toggles Prologue Language mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Prologue,

    /// **`Prologue`** Toggles the PrologueEsolang Mad Scientist experiment mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Injects extreme genetic chaos and random Orca signal bursts.
    PrologueEsolang,

    /// **`PrologueEsolang`** GrayScott block execution
    GrayScott,
    /// **`PrologueEsolang`** Locus block execution
    Locus,
    /// **`PrologueEsolang`** Neuro block execution
    Neuro,
    /// **`PrologueEsolang`** Platter block execution
    Platter,
    /// **`Prologue`** Places a Rune on the grid.
    ///
    /// **Stack:** `[ ..., rune_char, y, x ] -> [ ... ]`
    Rune,

    /// **`Nova`** Triggers a Cambrian Explosion (Mass Speciation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Clears all organelles and spawns diverse new ones.
    Cambrian,

    // Cambrian Features (Evo-Devo)
    /// **`Nova`** Emits a chemical morphogen into the environment.
    ///
    /// **Stack:** `[ ..., channel, amount ] -> [ ... ]`
    Morphogen,
    /// **`Nova`** Switches execution based on local morphogen concentration (Hox Gene).
    ///
    /// **Stack:** `[ ..., channel, threshold, strand_idx ] -> [ ... ]`
    HoxSwitch,
    /// **`Nova`** Creates a structural bond with a neighbor (Tissue Formation).
    ///
    /// **Stack:** `[ ..., direction ] -> [ ..., success ]`
    Adhere,

    // Attractor Features (Nova - Chaos Dynamics)
    /// **`Nova`** Initializes the Strange Attractor.
    ///
    /// **Stack:** `[ ..., mode ] -> [ ... ]`
    /// **Mode:** 0=Lorenz, 1=Rossler, 2=Thomas.
    AttractorInit,
    /// **`Nova`** Steps the Strange Attractor simulation.
    ///
    /// **Stack:** `[ ..., dt ] -> [ ... ]`
    /// **dt:** Time delta (e.g. 0.01).
    AttractorStep,
    /// **`Nova`** Warps the IP based on the Attractor state.
    ///
    /// **Stack:** `[ ..., scale ] -> [ ... ]`
    /// **Effect:** Jumps to strand index derived from Attractor Z.
    AttractorSurf,
    /// **`Nova`** Maps the Attractor state to Grid variables.
    ///
    /// **Stack:** `[ ..., target ] -> [ ... ]`
    /// **Target:** 0=Entropy, 1=Mutation Rate, 2=Grid Warp.
    AttractorMap,

    // Holographic Features (Nova)
    /// **`Nova`** Encodes a strand into the Hologram Grid (Interference).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Interfere,
    /// **`Nova`** Decodes the Hologram Grid into a new strand (Refraction).
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    Refract,
    /// **`Nova`** Projects the Hologram intensity onto the main Grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Project,
    /// **`Nova`** Diffracts a strand into the Hologram Grid (Ghost/Split).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Diffract,
    /// **`Nova`** Toggles Holographic View/Mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Hologram,
    /// **`Nova`** Applies phase shift noise to the hologram grid (Mutation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    PhaseMutate,

    /// **`Nova`** Collapses the local Hologram wavefunction into a physical grid character.
    ///
    /// **Stack:** `[ ..., threshold ] -> [ ... ]`
    /// **Effect:** If `|H(y,x)| > threshold`, maps Phase(H) to an ASCII character and writes it to `Grid(y,x)`.
    QuantumScribe,

    /// **`Nova`** Encodes the local physical grid character into the Hologram.
    ///
    /// **Stack:** `[ ..., weight ] -> [ ... ]`
    /// **Effect:** Maps `Grid(y,x)` character to Phase, and adds `weight * e^(i*phase)` to `Hologram(y,x)`.
    QuantumScan,

    /// **`Nova`** Refracts the Hologram into a Grammar and parses a string with it.
    ///
    /// **Stack:** `[ ..., input_string ] -> [ ..., result_ast ]`
    HoloInvoke,

    /// **`Nova`** Refracts the Hologram into a Grammar and generates a string from it.
    ///
    /// **Stack:** `[ ... ] -> [ ..., output_string ]`
    HoloSpeak,

    /// **`Nova`** Converts an integer to a 1-character string (ASCII).
    ///
    /// **Stack:** `[ ..., int ] -> [ ..., string ]`
    Chr,

    /// **`Nova`** Converts the Hologram Grid state into MIDI events.
    ///
    /// **Stack:** `[ ..., threshold ] -> [ ... ]`
    /// **Effect:** Generates notes based on hologram intensity.
    HoloSonify,

    /// **`Nova`** Encodes the current Audio Snapshot (Cymatics) into the Hologram Grid.
    ///
    /// **Stack:** `[ ..., scale ] -> [ ... ]`
    /// **Effect:** Modifies hologram based on audio pressure.
    CymaticScan,

    /// **`Nova`** Toggles the Orca Signal Processing system on the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Orca,

    // Origami Features (Nova)
    /// **`Nova`** Folds a Miura-ori mesh using a given extension factor from the stack.
    ///
    /// **Stack:** `[ ..., extension_factor ] -> [ ... ]`
    Origami,

    // Catalyst Features (Nova)
    /// **`Nova`** Creates a new Catalyst from a strand.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., catalyst_id ]`
    Synthesize,
    /// **`Nova`** Applies a Catalyst to mutate a target strand.
    ///
    /// **Stack:** `[ ..., catalyst_id, target_strand_idx ] -> [ ... ]`
    Catalyze,

    // Chemistry Features (Nova)
    /// **`Nova`** Mixes neighbors into a solution in the current cell.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Mix,
    /// **`Nova`** Brews the current cell's mixture into a Potion.
    ///
    /// **Stack:** `[ ..., heat ] -> [ ... ]`
    Brew,
    /// **`Nova`** Splashes a Potion to a target location.
    ///
    /// **Stack:** `[ ..., radius, dy, dx ] -> [ ... ]`
    Splash,

    /// **`Nova`** Interacts with the Alchemical Crucible.
    ///
    /// **Stack:** `[ ..., mode ]`
    /// **Mode:** 0=Add (Pop), 1=Clear, 2=Transmute.
    Crucible,

    /// **`Nova`** Spreads the last executed instruction to a random strand (Memetics).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Meme,

    /// **`Nova`** Creates a Meme from the current strand's genes.
    ///
    /// **Stack:** `[ ..., len, virulence, fidelity ] -> [ ..., meme_id ]`
    Conceive,

    /// **`Nova`** Spreads a specific Meme to a target strand.
    ///
    /// **Stack:** `[ ..., meme_id, target_strand ] -> [ ... ]`
    Propagate,

    /// **`Nova`** Removes a Meme from the pool.
    ///
    /// **Stack:** `[ ..., meme_id ] -> [ ... ]`
    Forget,

    /// **`Nova`** Modifies the dialect of the current strand (OpCode Remapping).
    ///
    /// **Stack:** `[ ..., from_op_str, to_op_str ] -> [ ... ]`
    Shibboleth,

    /// **`Nova`** Spawns a linguistic virus at the current location.
    ///
    /// **Stack:** `[ ..., mutation_rate, pattern_str, name_str ] -> [ ... ]`
    Infect,

    /// **`Nova`** Triggers a viral outbreak step (spread & mutate).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Outbreak,

    /// **`Nova`** Clears viral infection in a radius.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Sanitize,

    /// **`Nova`** Randomly mutates the genome with a given probability (Linguistic Drift).
    ///
    /// **Stack:** `[ ..., probability ] -> [ ... ]`
    Drift,

    /// **`Nova`** Dynamic dispatch based on stack top type (Polymorphism).
    ///
    /// **Stack:** `[ val ]` (peeks) -> executes op1 (if Int) or op2 (if Str)
    /// **Args:** `[Nucleotide::String(op_int), Nucleotide::String(op_str)]`
    Poly,

    /// **`Nova`** Reshuffles the entire DNA based on the current Grid state.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Clears DNA, reads Grid as DNA, resets Energy to 50, IP to (0,0), Stack to [].
    Metamorphosis,

    /// **`Nova`** Executes the Grid Colors as a Piet program.
    ///
    /// **Stack:** `[ ..., steps ] -> [ ... ]`
    /// **Effect:** Runs a Piet interpreter on the ChromaGrid.
    /// **Interaction:** 'In' pops from Chimera Stack, 'Out' pushes to Chimera Stack.
    Piet,

    /// **`Nova`** Executes basic 2D Befunge logic based on string input on the stack.
    Befunge,

    /// **`Nova`** Freezes the environment and other organisms for a duration.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ... ]`
    /// **Cost:** 50 + ticks Energy.
    Chronostasis,

    /// **`Nova`** Sets the local time dilation factor in a radius.
    ///
    /// **Stack:** `[ ..., factor, radius ] -> [ ... ]`
    /// **Factor:** 0=Stasis, 1=Normal, >1=Accelerated.
    TimeWarp,

    /// **`Nova`** Reads the local time dilation factor at the current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., factor ]`
    Chronos,

    /// **`Nova`** Accesses the state of a grid cell from `ticks` ago.
    ///
    /// **Stack:** `[ ..., ticks, y, x ] -> [ ..., past_value ]`
    /// **Effect:** Allows looking back in time.
    Retroscope,

    /// **`Nova`** Reverts the Grid state to a previous point in time.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ... ]`
    /// **Effect:** Overwrites current grid with the state from `ticks` ago.
    Retrograde,

    // Egregore Features (Collective Consciousness)
    /// **`Nova`** Connects to the collective Egregore mind.
    ///
    /// **Stack:** `[ ..., channel_name ] -> [ ... ]`
    EgregoreLink,
    /// **`Nova`** Sacrifices energy (faith) to the Egregore.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    EgregoreTithe,
    /// **`Nova`** Sends a message to a shared Egregore channel.
    ///
    /// **Stack:** `[ ..., channel_name, value ] -> [ ... ]`
    EgregoreChannel,
    /// **`Nova`** Casts a vote on a global parameter.
    ///
    /// **Stack:** `[ ..., parameter_name, vote_value ] -> [ ... ]`
    EgregoreDictate,
    /// **`Nova`** Queries a value from the Egregore (channel or parameter).
    ///
    /// **Stack:** `[ ..., key ] -> [ ..., value ]`
    EgregoreQuery,
    /// **`Nova`** Attempts to summon a global effect using collective faith.
    ///
    /// **Stack:** `[ ..., ritual_name ] -> [ ... ]`
    EgregoreSummon,
    /// **`Nova`** Sacrifices the current strand to feed the Chaos of the Egregore.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Kills strand, shifts Alignment towards Chaos (-), gains Faith.
    Sacrifice,
    /// **`Nova`** Prays to the Egregore for Order.
    ///
    /// **Stack:** `[ ..., energy_amount ] -> [ ... ]`
    /// **Effect:** Consumes Energy, shifts Alignment towards Order (+), gains Faith.
    Pray,

    // String Theory (Nova)
    /// **`Nova`** Creates a vibrating Cosmic String.
    ///
    /// **Stack:** `[ ..., length, tension, y, x ] -> [ ... ]`
    /// **Effect:** Spawns a string starting at (x,y) extending in current direction.
    StringNew,
    /// **`Nova`** Plucks the nearest Cosmic String.
    ///
    /// **Stack:** `[ ..., force ] -> [ ... ]`
    /// **Effect:** Adds energy to the string's vibration.
    StringPluck,
    /// **`Nova`** Tunes the tension of the nearest Cosmic String.
    ///
    /// **Stack:** `[ ..., tension ] -> [ ... ]`
    StringTune,
    /// **`Nova`** Listens to the vibration of the nearest Cosmic String.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amplitude ]`
    StringListen,

    // Market Features
    /// **`Nova`** Places a Sell Order (Ask) on the Market.
    ///
    /// **Stack:** `[ ..., price, item ] -> [ ..., order_id ]`
    /// **Effect:** Adds item to market. If sold, funds are credited.
    Offer,
    /// **`Nova`** Places a Buy Order (Bid) on the Market.
    ///
    /// **Stack:** `[ ..., max_price, query ] -> [ ..., item, cost ]`
    /// **Effect:** Purchases item if available. Returns item and cost. If failed, returns 0.
    Buy,
    /// **`Nova`** Converts global Energy into local Credits.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Invest,
    /// **`Nova`** Converts local Credits into global Energy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Divest,
    /// **`Nova`** Pushes current Credit balance to stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., balance ]`
    Balance,
    /// **`Nova`** Pushes last trade price to stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., price ]`
    Ticker,

    // Relativity Features
    /// **`Nova`** Toggles General Relativity simulation (Time Dilation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Relativity,
    /// **`Nova`** Emits a gravity wave, increasing local mass.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Graviton,
    /// **`Nova`** Checks the local gravity field strength.
    ///
    /// **Stack:** `[ ... ] -> [ ..., gravity ]`
    EventHorizon,

    // Atmosphere Features
    /// **`Nova`** Sets the local wind vector.
    ///
    /// **Stack:** `[ ..., angle, strength ] -> [ ... ]`
    /// **Angle:** 0=N, 1=NE, 2=E, 3=SE, 4=S, 5=SW, 6=W, 7=NW.
    Aeolus,
    /// **`Nova`** Creates moisture (clouds/rain) at the current location.
    ///
    /// **Stack:** `[ ..., intensity, radius ] -> [ ... ]`
    Storm,
    /// **`Nova`** Reads the local wind vector.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx ]`
    SenseWind,
    /// **`Nova`** Reads the local moisture level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., moisture ]`
    SenseMoisture,
    /// **`Nova`** Creates a strong directional wind wave.
    ///
    /// **Stack:** `[ ..., power, direction ] -> [ ... ]`
    Tsunami,
    /// **`Nova`** Removes moisture from a circular area.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Dry,

    // Olfactory Features (Nova)
    /// **`Nova`** Emits a scent trail (Pheromone).
    ///
    /// **Stack:** `[ ..., intensity, signature_string ] -> [ ... ]`
    Emit,
    /// **`Nova`** Smells the strongest local scent.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx, intensity, signature ]`
    Smell,
    /// **`Nova`** Tracks a specific scent.
    ///
    /// **Stack:** `[ ..., signature ] -> [ ..., dy, dx ]`
    Track,

    /// **`Nova`** Changes the biome of the local area.
    ///
    /// **Stack:** `[ ..., biome_id, radius ] -> [ ... ]`
    /// **Biome IDs:** 0=Plains, 1=Swamp, 2=Desert, 3=Tundra, 4=Volcanic.
    Terraform,

    /// **`Nova`** Reads the current biome ID.
    ///
    /// **Stack:** `[ ... ] -> [ ..., biome_id ]`
    SenseBiome,

    /// **`Nova`** Remaps an OpCode to another OpCode at runtime.
    ///
    /// **Stack:** `[ ..., from_op_str, to_op_str ] -> [ ... ]`
    /// **Effect:** `from_op` will now behave like `to_op`.
    Remap,
    /// **`Nova`** Restores an OpCode to its original behavior.
    ///
    /// **Stack:** `[ ..., op_str ] -> [ ... ]`
    Restore,
    /// **`Nova`** Reverses the direction of execution.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Mirror,

    /// **`Nova`** Sets the foreground color of a grid cell (Chromatophores).
    ///
    /// **Stack:** `[ ..., r, g, b, y, x ] -> [ ... ]`
    Pigment,
    /// **`Nova`** Sets the character representation of a grid cell (Chromatophores).
    ///
    /// **Stack:** `[ ..., char_code, y, x ] -> [ ... ]`
    Glyph,
    /// **`Nova`** Reads the foreground color of a grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ..., r, g, b ]`
    SensePigment,
    /// **`Nova`** Reads the character representation of a grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ..., char_code ]`
    SenseGlyph,

    // Fungi Features (Mycelial Network)
    /// **`Nova`** Spawns a fungal node (Hyphae) at the current grid location.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Hyphae,
    /// **`Nova`** Connects the current Hyphae to another at target coordinates.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Connect,
    /// **`Nova`** Transport a value instantly to a connected Hyphae.
    ///
    /// **Stack:** `[ ..., val, y, x ] -> [ ... ]`
    Transport,
    /// **`Nova`** Release spores to randomly spawn Hyphae nearby.
    ///
    /// **Stack:** `[ ..., radius, density ] -> [ ... ]`
    SporeCloud,

    // Polyglot Features
    /// **`Nova`** Executes a string as Brainfuck code.
    ///
    /// **Stack:** `[ ..., bf_code_string, input_string ] -> [ ..., output_string ]`
    Brainfuck,

    /// **`PrologueEsolang`** Renders a TUI element or manipulates the grid based on a TUI instruction string.
    ///
    /// **Stack:** `[ ..., instruction_string ] -> [ ... ]`
    TuiDraw,

    /// **`PrologueEsolang`** Renders a native ratatui UI layout based on a mosaic instruction string.
    ///
    /// **Stack:** `[ ..., instruction_string ] -> [ ... ]`
    MosaicDraw,

    // Meta Features (Self-Definition)
    /// **`Nova`** Defines a new enzyme (OpCode) that calls a strand.
    ///
    /// **Stack:** `[ ..., name_str, strand_idx ] -> [ ... ]`
    Define,
    /// **`Nova`** Registers a custom Orca operator (Grid Glyph).
    ///
    /// **Stack:** `[ ..., char_str, strand_idx ] -> [ ... ]`
    Operator,
    /// **`Nova`** Removes a defined enzyme.
    ///
    /// **Stack:** `[ ..., name_str ] -> [ ... ]`
    Undefine,
    /// **`Nova`** Pushes a list of all defined custom enzymes.
    ///
    /// **Stack:** `[ ... ] -> [ ..., list_junction ]`
    Dictionary,

    // Akashic Features (Persistent Storage)
    /// **`Nova`** Writes a key-value pair to the persistent Akashic Record.
    ///
    /// **Stack:** `[ ..., key, value ] -> [ ... ]`
    AkashicWrite,
    /// **`Nova`** Reads a value from the persistent Akashic Record.
    ///
    /// **Stack:** `[ ..., key ] -> [ ..., value ]`
    AkashicRead,
    /// **`Nova`** Saves the entire VM state as a Memory in the Akashic Record.
    ///
    /// **Stack:** `[ ..., key_str ] -> [ ... ]`
    AkashicSave,
    /// **`Nova`** Loads a Memory from the Akashic Record, overwriting the current state.
    ///
    /// **Stack:** `[ ..., key_str ] -> [ ... ]`
    AkashicLoad,
    /// **`Nova`** Modifies the organism's Karma.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Karma,
    /// **`Nova`** Performs a Miracle using Karma.
    ///
    /// **Stack:** `[ ..., miracle_id ] -> [ ... ]`
    /// **IDs:** 0=Resurrection, 1=Terraform, 2=Wealth, 3=Cleanse, 4=Ascension.
    Miracle,

    /// **`Nova`** Dumps the flight recorder (blackbox) to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dump_string ]`
    Blackbox,

    // Ribozyme Features (Functional Programming)
    /// **`Nova`** Evaluates a string as code.
    ///
    /// **Stack:** `[ ..., code_string ] -> [ ... ]`
    Eval,
    /// **`Nova`** Evaluates a string as Lisp code.
    ///
    /// **Stack:** `[ ..., lisp_code_string ] -> [ ... ]`
    LispEval,
    /// **`Nova`** Concatenates two strands into a new function (Functional Composition).
    ///
    /// **Stack:** `[ ..., strand_g, strand_f ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates `new_strand = f + g` (f executes then g).
    Chain,
    /// **`Nova`** Partially applies a value to a strand.
    ///
    /// **Stack:** `[ ..., value, strand_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates `new_strand = [push(value)] + strand`.
    Curry,
    /// **`Nova`** Consumes the next instruction as a string literal.
    ///
    /// **Stack:** `[ ... ] -> [ ..., op_string ]`
    Quote,
    /// **`Nova`** Applies a function to each element of a Junction.
    ///
    /// **Stack:** `[ ..., junction, function ] -> [ ..., new_junction ]`
    Map,
    /// **`Nova`** Hashes a strand and adds it to the immune allowlist.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Vaccinate,
    /// **`Nova`** Verifies if a strand is in the allowlist.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., is_trusted ]`
    Verify,
    /// **`Nova`** Returns indices of all untrusted strands.
    ///
    /// **Stack:** `[ ... ] -> [ ..., junction_of_indices ]`
    Audit,
    /// **`Nova`** Invokes a magical Sigil based on a spatial pattern.
    ///
    /// **Stack:** `[ ..., sigil_name ] -> [ ... ]`
    /// **Effect:** Checks grid for pattern and applies effect.
    Invoke,
    /// **`Nova`** Inscribes a new Sigil from the local environment.
    ///
    /// **Stack:** `[ ..., strand_idx, radius, sigil_name ] -> [ ... ]`
    /// **Effect:** Learns a new Sigil pattern and binds it to the strand.
    Inscribe,
    /// **`Nova`** Toggles the Auto-Cast (Passive) state of a Sigil.
    ///
    /// **Stack:** `[ ..., sigil_name, state ] -> [ ... ]`
    /// **State:** 1=On, 0=Off.
    AutoCast,
    /// **`Nova`** Inscribes a Ward (trap) on the current grid cell.
    ///
    /// **Stack:** `[ ..., persistence, strand_idx ] -> [ ... ]`
    /// **Effect:** Writes a hidden trap that triggers the strand when stepped on.
    Ward,
    /// **`Nova`** Reduces a Junction to a single value.
    ///
    /// **Stack:** `[ ..., junction, init, function ] -> [ ..., result ]`
    Fold,
    /// **`Nova`** Filters a Junction based on a predicate.
    ///
    /// **Stack:** `[ ..., junction, predicate ] -> [ ..., new_junction ]`
    Filter,
    /// **`Nova`** Combines two Junctions into one.
    ///
    /// **Stack:** `[ ..., junction_a, junction_b ] -> [ ..., zipped_junction ]`
    Zip,
    /// **`Nova`** Matches a string against a pattern (wildcards supported).
    ///
    /// **Stack:** `[ ..., pattern, target ] -> [ ..., is_match ]`
    Match,

    // IPC Features (Ether Link)
    /// **`Nova`** Sends a value to an external Ether channel.
    ///
    /// **Stack:** `[ ..., channel, value ] -> [ ... ]`
    Signal,
    /// **`Nova`** Receives a value from an external Ether channel.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., value ]`
    Receive,

    // Bard Features (Music)
    /// **`Nova`** Adds a note to the musical score.
    ///
    /// **Stack:** `[ ..., velocity, duration, pitch ] -> [ ... ]`
    Note,
    /// **`Nova`** Adds a rest to the musical score.
    ///
    /// **Stack:** `[ ..., duration ] -> [ ... ]`
    Rest,
    /// **`Nova`** Sets the tempo of the composition.
    ///
    /// This instruction logs the tempo change as metadata for the score (e.g., for ABC export).
    /// It does **not** affect the VM's execution speed.
    ///
    /// **Stack:** `[ ..., bpm ] -> [ ... ]`
    Tempo,
    /// **`Nova`** Exports the score as ABC Notation to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., abc_string ]`
    Perform,

    /// **`Nova`** Converts the musical score into a new DNA strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Maps notes to OpCodes and creates a new strand.
    Compose,

    // Resonance Features (Audio Physics)
    /// **`Resonance`** Plucks the underlying physics grid at the current location.
    ///
    /// **Stack:** `[ ..., strength ] -> [ ... ]`
    Pluck,
    /// **`Resonance`** Continuously excites the grid at the current location with a sine wave.
    ///
    /// **Stack:** `[ ..., frequency, strength ] -> [ ... ]`
    /// **Effect:** Sets up an oscillator. If strength is 0, stops oscillation.
    Oscillate,
    /// **`Resonance`** Reads the amplitude of the physics grid at the current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amplitude ]`
    Hear,
    /// **`Resonance`** Emits a loud, physical shockwave.
    ///
    /// **Stack:** `[ ..., duration, strength ] -> [ ... ]`
    Scream,

    // Cymatics Features (Nova + Resonance)
    /// **`Cymatics`** Moves matter on the grid towards nodal points (low amplitude).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(all(feature = "nova", feature = "resonance"))]
    Sift,
    /// **`Cymatics`** Modifies walls (membranes) based on sound amplitude.
    ///
    /// **Stack:** `[ ..., threshold, mode ] -> [ ... ]`
    /// **Mode:** 0=Solidify (High Amp -> Wall), 1=Liquefy (High Amp -> No Wall).
    #[cfg(all(feature = "nova", feature = "resonance"))]
    Reshape,

    /// **`Cymatics`** Emits a resonant frequency and amplitude at the current location.
    ///
    /// **Stack:** `[ ..., frequency, amplitude ] -> [ ... ]`
    Resonate,
    /// **`Cymatics`** Claims territory if the local resonance matches the strand's frequency.
    ///
    /// **Stack:** `[ ..., target_frequency ] -> [ ... ]`
    SonicClaim,
    /// **`Cymatics`** Reduces resonance amplitude in an area.
    ///
    /// **Stack:** `[ ..., radius, amount ] -> [ ... ]`
    Dampen,
    /// **`Cymatics`** Registers a strand to trigger when a global frequency is dominant.
    ///
    /// **Stack:** `[ ..., frequency, strand_idx ] -> [ ... ]`
    ListenFreq,

    // Oracle Features (Logic Engine)
    /// **`Oracle`** Adds a fact or rule to the Knowledge Base.
    ///
    /// **Stack:** `[ ..., fact ] -> [ ... ]`
    Assert,
    /// **`Oracle`** Adds a rule to the Knowledge Base.
    ///
    /// **Stack:** `[ ..., rule_junction ] -> [ ... ]`
    Rule,
    /// **`Oracle`** Removes a fact or rule from the Knowledge Base.
    ///
    /// **Stack:** `[ ..., fact ] -> [ ... ]`
    Retract,
    /// **`Oracle`** Queries the Knowledge Base.
    ///
    /// **Stack:** `[ ..., query ] -> [ ..., success ]`
    /// **Effect:** Tries to prove the query. If successful, variables in the query structure may be unified.
    Query,

    /// **`Oracle`** Finds all solutions to a query.
    ///
    /// **Stack:** `[ ..., template, goal ] -> [ ..., result_list ]`
    FindAll,

    /// **`Oracle`** Registers an Omen (Trigger).
    ///
    /// **Stack:** `[ ..., condition, effect ] -> [ ... ]`
    Augury,

    /// **`Oracle`** Checks all registered Omens.
    ///
    /// **Stack:** `[ ... ] -> [ ..., triggered_count ]`
    Divinate,

    /// **`Oracle`** Searches for a strand satisfying a predicate and jumps to it.
    ///
    /// **Stack:** `[ ..., query ] -> [ ... ]`
    /// **Effect:** Jumps to the first matching strand.
    Seek,

    /// **`Oracle`** Applies a transformation to all states matching a query.
    ///
    /// **Stack:** `[ ..., query_template, transform_template ] -> [ ... ]`
    Manifest,

    /// **`Oracle`** Unifies two terms on the stack.
    ///
    /// **Stack:** `[ ..., term1, term2 ] -> [ ..., substitution_junction ]`
    /// **Effect:** Returns a list of variable bindings if successful, or 0 if failed.
    Unify,

    /// **`Oracle`** Executes a Prolog-style query string.
    ///
    /// **Stack:** `[ ..., query_string ] -> [ ..., result_junction ]`
    /// **Effect:** Parses and runs a query, returning bindings.
    PrologCall,

    /// **`Oracle`** Toggles the Regulatory Censor (Genetic Regulation via Logic).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** If enabled, every gene execution is checked against the Knowledge Base for `censor(Op)`.
    Censor,

    // Git Features (Repository Interaction)
    /// **`Git`** Pushes a list of recent commit hashes to the stack.
    ///
    /// **Stack:** `[ ..., count ] -> [ ..., n, hash_1, ..., hash_n ]`
    Ancestry,
    /// **`Git`** Gets history via git_associates.
    GitHistory,
    /// **`Git`** Gets diff via git_associates.
    GitDiffWorkspace,
    /// **`Git`** Reads the content of a file from a specific commit.
    ///
    /// **Stack:** `[ ..., hash_str, path_str ] -> [ ..., content_string ]`
    Excavate,
    /// **`Git`** Gets the diff of a commit.
    ///
    /// **Stack:** `[ ..., hash_str ] -> [ ..., diff_string ]`
    Evolution,

    // Phylogeny Features (Filesystem Genetics)
    /// **`Phylogeny`** Lists files in a directory.
    ///
    /// **Stack:** `[ ..., path_string ] -> [ ..., junction_of_filenames ]`
    Crawl,
    /// **`Phylogeny`** Reads a file's content (Sequencing).
    ///
    /// **Stack:** `[ ..., path_string ] -> [ ..., content_string ]`
    Sequencing,
    /// **`Phylogeny`** Writes content to a file (Synthesizing DNA).
    ///
    /// **Stack:** `[ ..., path_string, content_string ] -> [ ... ]`
    PhyloSynthesize,
    /// **`Phylogeny`** Appends content to a file (Infection).
    ///
    /// **Stack:** `[ ..., path_string, content_string ] -> [ ... ]`
    PhyloInfect,
    /// **`Phylogeny`** Executes a system command (Shell).
    ///
    /// **Stack:** `[ ..., command_string ] -> [ ..., output_string ]`
    Shell,

    // Geology Features (Nova)
    /// **`Nova`** Randomly shifts rows or columns of the grid (Plate Tectonics).
    ///
    /// **Stack:** `[ ..., intensity ] -> [ ... ]`
    Quake,
    /// **`Nova`** Reduces values in a circular area (Weathering).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Erode,
    /// **`Nova`** Increases values in a circular area (Deposition).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Sediment,
    /// **`Nova`** Shifts a rectangular block of the grid.
    ///
    /// **Stack:** `[ ..., dy, dx, h, w ] -> [ ... ]`
    Tectonics,
    /// **`Nova`** Erupts high values at the current location.
    ///
    /// **Stack:** `[ ..., power ] -> [ ... ]`
    Volcano,

    // Geomancy Features (Nova)
    /// **`Nova`** Detects the nearest Ley Node.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx, distance, power ]`
    LeySense,
    /// **`Nova`** Absorbs energy from a Ley Node.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    /// **Risk:** High power nodes can cause Overload damage.
    LeyTap,
    /// **`Nova`** Teleports to a connected Ley Node.
    ///
    /// **Stack:** `[ ..., target_node_idx ] -> [ ... ]`
    LeyWarp,
    /// **`Nova`** Moves a Ley Node to a new location.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    LeyShift,

    // Dimension Features (Nova)
    /// **`Nova`** Switches to a different Grid Dimension (Plane).
    ///
    /// **Stack:** `[ ..., dimension_id ] -> [ ... ]`
    /// **Effect:** Swaps the current grid with the target dimension's grid.
    Dimension,
    /// **`Nova`** Reads a value from a specific dimension.
    ///
    /// **Stack:** `[ ..., y, x, dimension_id ] -> [ ..., value ]`
    DRead,
    /// **`Nova`** Writes a value to a specific dimension.
    ///
    /// **Stack:** `[ ..., value, y, x, dimension_id ] -> [ ... ]`
    DWrite,
    /// **`Nova`** Merges a dimension into the current one.
    ///
    /// **Stack:** `[ ..., method, dimension_id ] -> [ ... ]`
    /// **Method:** 0=Add, 1=Max, 2=Overwrite.
    DMerge,
    /// **`Nova`** Pushes the current dimension ID to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dimension_id ]`
    DView,

    // Hive Features (Networking)
    /// **`Hive`** Binds a UDP port for receiving messages.
    ///
    /// **Stack:** `[ ..., port ] -> [ ... ]`
    HiveBind,
    /// **`Hive`** Sends a message to a target IP and Port.
    ///
    /// **Stack:** `[ ..., value, ip_string, port ] -> [ ... ]`
    HiveSend,
    /// **`Hive`** Receives a message from a bound port (non-blocking).
    ///
    /// **Stack:** `[ ..., port ] -> [ ..., value ]`
    /// **Effect:** Pushes received value or 0 if no message.
    HiveRecv,
    /// **`Hive`** Closes a bound UDP port.
    ///
    /// **Stack:** `[ ..., port ] -> [ ... ]`
    HiveClose,

    // Paleontology Features (Nova)
    /// **`Nova`** Fossilizes a strand into a compressed string on the grid.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Writes `Fossil:{tick}:{hash}:{dna}` to the grid.
    Fossilize,
    /// **`Nova`** Unearths a fossil from the grid, restoring the DNA strand.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., new_strand_idx ]`
    Unearth,
    /// **`Nova`** Carbon dates a fossil to determine its age.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., age ]`
    CarbonDate,

    // Crystallography Features (Nova)
    /// **`Nova`** Turns the current cell into a crystal seed.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Nucleate,
    /// **`Nova`** Grows the crystal by absorbing value from neighbors.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Accrete,
    /// **`Nova`** Explodes the crystal, scattering value to neighbors.
    ///
    /// **Stack:** `[ ..., force ] -> [ ... ]`
    Shatter,
    /// **`Nova`** Sorts the values in a local window (Annealing).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    Anneal,

    // Cartography Features (Nova)
    /// **`Nova`** Scans a circular area and returns a Junction of values.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., junction_of_values ]`
    Scan,
    /// **`Nova`** Pushes the current coordinates to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., y, x ]`
    Locate,
    /// **`Nova`** Writes a value to the persistent Cartography Map.
    ///
    /// **Stack:** `[ ..., value, y, x ] -> [ ... ]`
    Chart,
    /// **`Nova`** Reads a value from the persistent Cartography Map.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., value ]`
    Atlas,

    /// **`Nova`** Configures the Logistics Layer (Factory Automation).
    ///
    /// **Stack:** `[ ..., type, direction, y, x ] -> [ ... ]`
    /// **Type:** 0=Clear, 1=Belt, 2=Sorter.
    /// **Direction:** 0=N, 1=E, 2=S, 3=W.
    Logistics,

    /// **`Nova`** Compresses a grid area into a value on the stack.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., pocket_val ]`
    /// **Effect:** Clears the area.
    Pocket,
    /// **`Nova`** Decompresses a pocket value onto the grid.
    ///
    /// **Stack:** `[ ..., pocket_val ] -> [ ... ]`
    /// **Effect:** Overwrites the area.
    Unpocket,

    // Sovereignty Features (Territory)
    /// **`Nova`** Claims ownership of grid cells within a radius.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    /// **Cost:** 10 Energy per cell.
    Claim,
    /// **`Nova`** Renounces ownership of a grid cell.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    Cede,
    /// **`Nova`** Checks the owner of a grid cell.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., owner_id (-1 if none) ]`
    Sovereignty,
    /// **`Nova`** Sets the tax rate for the current strand's territory.
    ///
    /// **Stack:** `[ ..., rate ] -> [ ... ]`
    Tax,

    // Linguistics Features (Nova)
    /// **`Nova`** Calculates Levenshtein edit distance between two strings.
    ///
    /// **Stack:** `[ ..., s1, s2 ] -> [ ..., distance ]`
    Levenshtein,
    /// **`Nova`** Calculates Soundex phonetic hash.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., code ]`
    Soundex,
    /// **`Nova`** Checks if two strings are anagrams.
    ///
    /// **Stack:** `[ ..., s1, s2 ] -> [ ..., is_anagram ]`
    Anagram,
    /// **`Nova`** Applies Caesar cipher shift.
    ///
    /// **Stack:** `[ ..., shift, string ] -> [ ..., shifted_string ]`
    Cipher,
    /// **`Nova`** Checks if string is a pangram.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., is_pangram ]`
    Pangram,

    // Babel Features (Metalinguistics - Nova)
    /// **`Babel`** Constructs a parser object on the stack.
    ///
    /// **Stack:** `[ ..., type_str, ...args ] -> [ ..., parser_junction ]`
    /// Types: "Match", "Seq", "Alt", "Many", "Opt".
    Grammar,
    /// **`Babel`** Parses a string using a parser object.
    ///
    /// **Stack:** `[ ..., parser_junction, input_string ] -> [ ..., result_ast ]`
    Parse,
    /// **`Babel`** Creates a literal string matcher.
    ///
    /// **Stack:** `[ ..., pattern_string ] -> [ ..., parser_junction ]`
    ParserMatch,
    /// **`Babel`** Creates a regex matcher.
    ///
    /// **Stack:** `[ ..., regex_pattern ] -> [ ..., parser_junction ]`
    ParserRegex,
    /// **`Babel`** Creates a sequence parser (P1 then P2).
    ///
    /// **Stack:** `[ ..., p1, p2 ] -> [ ..., parser_junction ]`
    ParserSeq,
    /// **`Babel`** Creates an alternative parser (P1 or P2).
    ///
    /// **Stack:** `[ ..., p1, p2 ] -> [ ..., parser_junction ]`
    ParserAlt,
    /// **`Babel`** Creates a repetition parser (0 or more).
    ///
    /// **Stack:** `[ ..., p ] -> [ ..., parser_junction ]`
    ParserMany,
    /// **`Babel`** Creates an optional parser.
    ///
    /// **Stack:** `[ ..., p ] -> [ ..., parser_junction ]`
    ParserOpt,
    /// **`Babel`** Creates a variadic sequence parser.
    ///
    /// **Stack:** `[ ..., p1, p2, ..., pn, count ] -> [ ..., parser_junction ]`
    ParserSeqN,
    /// **`Babel`** Creates a variadic alternative parser.
    ///
    /// **Stack:** `[ ..., p1, p2, ..., pn, count ] -> [ ..., parser_junction ]`
    ParserAltN,
    /// **`Babel`** "Tongue of Madness": Parses input, mutates CST, regenerates string.
    ///
    /// **Stack:** `[ ..., grammar, input_string ] -> [ ..., corrupted_string ]`
    Tongue,
    /// **`Babel`** Generates a string from a Grammar.
    ///
    /// **Stack:** `[ ..., grammar_junction ] -> [ ..., generated_string ]`
    Generate,
    /// **`Babel`** Writes a string to the Tablet (Output Buffer).
    ///
    /// **Stack:** `[ ..., string ] -> [ ... ]`
    Scribe,

    /// **`Babel`** Defines a Grammar Rule manually.
    ///
    /// **Stack:** `[ ..., parser_junction, rule_name ] -> [ ... ]`
    DefineRule,

    /// **`Babel`** Compiles a Concrete Syntax Tree (CST) into a new Strand.
    ///
    /// **Stack:** `[ ..., cst, handler_strand_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates a strand that recreates the CST traversal, calling handler for each node.
    BabelCompile,

    /// **`Babel`** Increases linguistic chaos (Glossolalia).
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Glossolalia,

    /// **`Babel`** Reads a Grammar definition from the grid visually.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., grammar_junction ]`
    GridGrammar,
    /// **`Babel`** Restores linguistic integrity.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    Clarify,
    /// **`Babel`** Randomly reshuffles language mappings.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Confuse,

    /// **`Babel`** Runs a Live Parser on the grid.
    ///
    /// **Stack:** `[ ..., y, x, input_string ] -> [ ..., success_bool ]`
    BabelLive,

    /// **`Babel`** Rewrites the active grid-perception grammar.
    ///
    /// **Stack:** `[ ..., grammar_junction ] -> [ ... ]`
    /// **Effect:** Sets the VM's active grammar for Perception.
    SelfRewrite,

    /// **`Babel`** Perceives the grid as code using the active grammar.
    ///
    /// **Stack:** `[ ..., len ] -> [ ..., success_bool ]`
    /// **Effect:** Reads `len` cells from grid at current location, parses with Active Grammar, compiles, and calls the result.
    Perceive,

    /// **`Babel`** The Ouroboros Protocol: Self-consumption and rebirth.
    ///
    /// **Stack:** `[ ..., grammar_junction ] -> [ ... ]`
    /// **Effect:** Decompiles self, parses with grammar, mutates, recompiles, replaces self.
    Ouroboros,

    // Ballistics Features (Nova)
    /// **`Nova`** Fires a projectile with velocity and power.
    ///
    /// **Stack:** `[ ..., power, dy, dx ] -> [ ... ]`
    Fire,
    /// **`Nova`** Fires multiple projectiles in random directions.
    ///
    /// **Stack:** `[ ..., power, count ] -> [ ... ]`
    Salvo,

    // Optics Features (Nova)
    /// **`Nova`** Creates a reflective surface on the grid.
    ///
    /// **Stack:** `[ ..., orientation, y, x ] -> [ ... ]`
    /// **Orientation:** 0=|, 1=-, 2=/, 3=\.
    Reflector,
    /// **`Nova`** Creates a prism that splits projectiles.
    ///
    /// **Stack:** `[ ..., orientation, y, x ] -> [ ... ]`
    Prism,
    /// **`Nova`** Creates a lens that modifies projectile properties.
    ///
    /// **Stack:** `[ ..., power, y, x ] -> [ ... ]`
    Lens,

    // Sociology Features (Nova)
    /// **`Nova`** Manages Guild membership and treasury.
    ///
    /// **Stack:** `[ ..., action, guild_name, amount/arg ] -> [ ... ]`
    /// **Actions:** "Join", "Leave", "Deposit", "Withdraw", "Create".
    Guild,
    /// **`Nova`** Manages Guild policies (for founders/leaders).
    ///
    /// **Stack:** `[ ..., action, arg, guild_name ] -> [ ... ]`
    /// **Actions:** "Tax", "Kick", "Invite", "Motto".
    Charter,

    /// **`Nova`** Draws a Fate Card (Arcana) from the deck.
    ///
    /// **Stack:** `[ ... ] -> [ ..., card_id ]`
    Draw,
    /// **`Nova`** Checks the currently active Fate Card.
    ///
    /// **Stack:** `[ ... ] -> [ ..., card_id ]`
    Fate,
    /// **`Nova`** Shuffles the Fate Deck.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Shuffle,

    // Quipu Features (Topological Memory)
    /// **`Nova`** Ties a knot with a value on the current Quipu Cord.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    Knot,
    /// **`Nova`** Unties the last knot cluster from the current Quipu Cord.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    Unknot,
    /// **`Nova`** Runs a Quipu block.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Quipu,
    /// **`Nova`** Selects the active Quipu Cord.
    ///
    /// **Stack:** `[ ..., cord_idx ] -> [ ... ]`
    Cord,
    /// **`Nova`** Reads the value of the current Quipu Cord.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    ReadCord,
    /// **`Nova`** Entangles (adds) the value of another cord to the current one.
    ///
    /// **Stack:** `[ ..., other_cord_idx ] -> [ ... ]`
    Tangle,

    // Metazoa Features (Multicellularity)
    /// **`Nova`** Bonds with a neighbor to form a Tissue.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ..., tissue_id ]`
    Bond,
    /// **`Nova`** Severs the bond with a neighbor.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ... ]`
    Unbond,
    /// **`Nova`** Sends a signal to the entire Tissue.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    Signify,
    /// **`Nova`** Pushes the current Tissue ID.
    ///
    /// **Stack:** `[ ... ] -> [ ..., tissue_id ]`
    Tissue,

    Fluid,
    // Weaving Features (The Loom)
    /// **`Nova`** Weaves two strands together based on a pattern.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, pattern_strand ] -> [ ..., new_strand_idx ]`
    /// **Pattern:** 'A'=Take from A, 'B'=Take from B, 'X'=Random, '0'=Skip.
    Weave,
    /// **`Nova`** Unravels a strand, destroying it and reclaiming resources.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    Unravel,

    // BioMesh Features (Fungal Cybernetics)
    /// **`Nova`** Transforms the current cell into a Mesh Node.
    ///
    /// **Stack:** `[ ..., id ] -> [ ... ]`
    MeshNet,
    /// **`Nova`** Sends a packet to a target Node ID.
    ///
    /// **Stack:** `[ ..., target_id, value ] -> [ ... ]`
    MeshSend,
    /// **`Nova`** Receives a packet from the local Node buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    MeshRecv,
    /// **`Nova`** Connects the current Node to adjacent Nodes.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    MeshGrow,
    /// **`Nova`** Disconnects the current Node from all neighbors.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    MeshPrune,

    // Reactor Features (Logic Automata)
    /// **`Nova`** Toggles the Reactor (Logic Cellular Automata) mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    Reactor,
    /// **`Nova`** Registers a reaction rule.
    ///
    /// **Stack:** `[ ..., input_a, input_b, output ] -> [ ... ]`
    Reaction,

    // Scavenger Features (Mad Science)
    /// **`Scavenge`** Reads raw bytes from a file and converts them into DNA.
    ///
    /// **Stack:** `[ ..., path_string, len ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Reads `len` bytes from `path`, maps each byte to an OpCode, creates a new strand.
    Scavenge,
    /// **`Scavenge`** Reads raw bytes from the running executable itself (Self-Cannibalism).
    ///
    /// **Stack:** `[ ..., offset, len ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Reads `len` bytes from the binary at `offset`, maps to OpCodes, creates a new strand.
    Digest,

    // Semiotics Features (Meaning Mutation)
    /// **`Nova`** Converts a value into an abstract Symbol.
    ///
    /// **Stack:** `[ ..., value ] -> [ ..., symbol ]`
    Symbolize,
    /// **`Nova`** Resolves a Symbol to a value based on the current context.
    ///
    /// **Stack:** `[ ..., symbol ] -> [ ..., resolved_value ]`
    Interpret,
    /// **`Nova`** Shifts the semiotic context by XORing with a value.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    ContextShift,
    /// **`Nova`** Deconstructs a string into a junction of constituent Symbols.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., junction_of_symbols ]`
    Deconstruct,

    // Fractal Features (Chaos Visualization)
    /// **`Nova`** Sets the Fractal Mode to Mandelbrot.
    ///
    /// **Stack:** `[ ..., max_iterations ] -> [ ... ]`
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    /// Mandelbrot
    Mandelbrot,
    /// **`Nova`** Sets the Fractal Mode to Julia Set with constant c.
    ///
    /// **Stack:** `[ ..., c_re, c_im ] -> [ ... ]`
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    /// Julia
    Julia,
    /// **`Nova`** Adjusts the Fractal Zoom level.
    ///
    /// **Stack:** `[ ..., factor ] -> [ ... ]`
    /// **Effect:** Multiplies current zoom by factor.
    Zoom,
    /// **`Nova`** Pans the Fractal View center.
    ///
    /// **Stack:** `[ ..., dx, dy ] -> [ ... ]`
    Pan,
    /// **`Nova`** performs one iteration of z = z^2 + c.
    ///
    /// **Stack:** `[ ..., z_re, z_im, c_re, c_im ] -> [ ..., new_z_re, new_z_im ]`
    Iterate,
    /// **`Nova`** Computes escape time for a point.
    ///
    /// **Stack:** `[ ..., c_re, c_im, max_iter ] -> [ ..., escape_val ]`
    Escape,

    // Raku Features (Hyper-Operators)
    /// **`Raku`** Element-wise addition of two lists.
    ///
    /// **Stack:** `[ ..., list_a, list_b ] -> [ ..., list_result ]`
    HyperAdd,
    /// **`Raku`** Element-wise subtraction.
    ///
    /// **Stack:** `[ ..., list_a, list_b ] -> [ ..., list_result ]`
    HyperSub,
    /// **`Raku`** Element-wise multiplication.
    ///
    /// **Stack:** `[ ..., list_a, list_b ] -> [ ..., list_result ]`
    HyperMul,
    /// **`Raku`** Element-wise division.
    ///
    /// **Stack:** `[ ..., list_a, list_b ] -> [ ..., list_result ]`
    HyperDiv,
    /// **`Raku`** Reduces a list using an operator (Fold).
    ///
    /// **Stack:** `[ ..., list, op_str ] -> [ ..., result ]`
    Reduce,
    /// **`Raku`** Cross-product of two lists with an operator.
    ///
    /// **Stack:** `[ ..., list_a, list_b, op_str ] -> [ ..., list_result ]`
    Cross,
    /// **`Raku`** Zips two lists with an operator.
    ///
    /// **Stack:** `[ ..., list_a, list_b, op_str ] -> [ ..., list_result ]`
    ZipWith,
    Flock,
    /// **`Nova`** Executes hyperbolic geometry operations.
    Poincare,

    /// No Operation. Does nothing.
    Nop,

    /// Unknown or invalid instruction.
    #[strum(default)]
    Unknown(String),

    // Mad Scientist Experimental Opcodes
    /// Triggers the Miller Lattice esolang logic
    MillerLattice,
    /// Triggers the Hyper System esolang logic
    HyperSystem,
    /// Triggers the Physics PBD esolang logic
    PhysicsPbd,
    /// Triggers the Ferrous Core esolang logic
    FerrousCore,
    /// Triggers the Tardis logic
    Tardis,
    /// Triggers the Pachinko logic
    Pachinko,
    /// Triggers the Automaton logic
    Automaton,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Unknown(s) => write!(f, "{}", s),
            _ => write!(f, "{}", self.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_opcode_parsing_edge_cases() {
        assert_eq!(OpCode::from_str("g_read"), Ok(OpCode::GRead));
        assert_eq!(OpCode::from_str("s_len"), Ok(OpCode::SLen));
        assert_eq!(OpCode::from_str("brz_s"), Ok(OpCode::BrzS));
        assert_eq!(OpCode::from_str("helix_len"), Ok(OpCode::HelixLen));
        assert_eq!(
            OpCode::from_str("unknown_op"),
            Ok(OpCode::Unknown("unknown_op".to_string()))
        );
    }

    #[test]
    fn test_opcode_display_edge_cases() {
        assert_eq!(OpCode::GRead.to_string(), "g_read");
        assert_eq!(OpCode::SLen.to_string(), "s_len");
        assert_eq!(OpCode::Unknown("foo".to_string()).to_string(), "foo");
    }
}

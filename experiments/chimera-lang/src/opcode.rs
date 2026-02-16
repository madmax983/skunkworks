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
    /// **[Cortex]** Links two strands with a neural synapse.
    ///
    /// **Stack:** `[ ..., target_strand_idx ]` (uses current IP as source)
    #[cfg(feature = "cortex")]
    Link,
    /// **[Cortex]** Severs a synapse between the current strand and a target.
    ///
    /// **Stack:** `[ ..., target_strand_idx ]`
    #[cfg(feature = "cortex")]
    Sever,
    /// **[Cortex]** Fires a signal across all synapses from the current strand.
    ///
    /// **Stack:** `[ ..., amount ]`
    /// **Effect:** Increases activation level of target strands.
    #[cfg(feature = "cortex")]
    Spark,
    /// **[Cortex]** Reads the current strand's activation level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., activation_level ]`
    #[cfg(feature = "cortex")]
    Sense,
    /// **[Cortex]** Gates execution based on activation level.
    ///
    /// **Args:** `[Nucleotide::Number(threshold)]`
    /// **Effect:** Skips next instruction if `activation < threshold`.
    #[cfg(feature = "cortex")]
    Gate,

    // Biophysics Features
    /// **[Biophysics]** Spawns a Hodgkin-Huxley neuron at the specified grid location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    NeuroGenesis,
    /// **[Biophysics]** Injects current into a neuron.
    ///
    /// **Stack:** `[ ..., amount, y, x ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    Stimulate,
    /// **[Biophysics]** Reads the membrane potential (voltage) of a neuron.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., voltage ]`
    #[cfg(feature = "biophysics")]
    Dendrite,
    /// **[Biophysics]** Connects a neuron to another target (Synapse/Output).
    ///
    /// **Stack:** `[ ..., y_target, x_target, y_source, x_source ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    Axon,
    /// **[Biophysics]** Adds a chemical receptor to a neuron.
    ///
    /// **Stack:** `[ ..., channel, sensitivity, threshold, y, x ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    Receptor,
    /// **[Biophysics]** Sets the bidirectional coupling coefficient between Neuron and Grid Voltage.
    ///
    /// **Stack:** `[ ..., weight, y, x ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    NeuroCoupling,
    /// **[Biophysics]** Connects a neuron to a Cortex Strand.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    #[cfg(feature = "biophysics")]
    NeuroSynapse,

    // Silicon Features
    /// **[Silicon]** Runs one step of Wireworld on the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Conduct,
    /// **[Silicon]** Writes a conductor (1) to the grid.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Wire,
    /// **[Silicon]** Writes an electron head (2) to the grid (Pulse).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Pulse,
    /// **[Silicon]** Toggles automatic conduction mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Silicon,
    /// **[Silicon]** Constructs a logic gate on the grid.
    ///
    /// **Stack:** `[ ..., type, dir, y, x ] -> [ ... ]`
    /// **Types:** 0=AND, 1=OR, 2=XOR, 3=NAND, 4=NOT.
    /// **Dirs:** 0=N, 1=E, 2=S, 3=W.
    #[cfg(feature = "silicon")]
    Construct,
    /// **[Silicon]** A logic gate instruction (usually on grid, not in DNA).
    ///
    /// **Args:** `[Nucleotide::String(type), Nucleotide::Number(dir)]`
    #[cfg(feature = "silicon")]
    LogicGate,
    /// **[Silicon]** Creates an Input Pin (Reads from Stack -> Grid).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    PinIn,
    /// **[Silicon]** Creates an Output Pin (Reads from Grid -> Stack).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    PinOut,
    /// **[Silicon]** Creates an Emitter cell that pulses periodically.
    ///
    /// **Stack:** `[ ..., freq, y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Emitter,
    /// **[Silicon]** Creates a Receiver cell that triggers a strand when powered.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Receiver,
    /// **[Silicon]** Creates a stateful D-Latch on the grid.
    ///
    /// **Stack:** `[ ..., state, y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Latch,
    /// **[Silicon]** Reads 4-bit signal from neighbors and pushes value to stack (Digital-to-Analog).
    ///
    /// **Stack:** `[ ... ] -> [ ..., val ]`
    #[cfg(feature = "silicon")]
    DAC,
    /// **[Silicon]** Pops value and writes 4-bit signal to neighbors (Analog-to-Digital).
    ///
    /// **Stack:** `[ ..., val ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    ADC,
    /// **[Silicon]** Traces a connected circuit and compiles it into DNA (Biologize).
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "silicon")]
    Trace,
    /// **[Silicon]** Executes a strand as a construction blueprint (Siliconize).
    ///
    /// **Stack:** `[ ..., strand_idx, y, x ] -> [ ... ]`
    #[cfg(feature = "silicon")]
    Fabricate,

    // Havoc Features (Chaos Engineering)
    /// **[Havoc]** Sets the fault injection rate.
    ///
    /// **Stack:** `[ ..., rate ] -> [ ... ]`
    /// **Rate:** 0.0 to 1.0 (Probability per tick).
    HavocRate,
    /// **[Havoc]** Sets the scope of fault injection.
    ///
    /// **Stack:** `[ ..., mask ] -> [ ... ]`
    /// **Mask:** 1=Memory, 2=Stack, 4=Execution.
    HavocScope,

    // Elektra Features (Circuitry)
    /// **[Elektra]** Creates a Voltage Source (Battery) on the grid.
    ///
    /// **Stack:** `[ ..., voltage, y, x ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    Battery,
    /// **[Elektra]** Creates a Ground (0V Sink) on the grid.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    Ground,
    /// **[Elektra]** Reads the Voltage at a grid location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., voltage ]`
    #[cfg(feature = "elektra")]
    SenseVolt,
    /// **[Elektra]** Discharges built-up charge, damaging neighbors.
    ///
    /// **Stack:** `[ ..., power, radius ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    Shock,
    /// **[Elektra]** Creates a lightning strike effect.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    Lightning,

    /// **[Elektra]** Converts internal Energy to Voltage at the current location.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    Electrogenesis,

    /// **[Elektra]** Absorb Voltage at the current location to gain Energy.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amount ]`
    #[cfg(feature = "elektra")]
    Induction,

    /// **[Elektra]** Grows a wire (Value::Int(1)) in a direction.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    WireGrowth,

    /// **[Elektra]** Branches if Voltage at current location is > threshold.
    ///
    /// **Stack:** `[ ..., threshold, strand_idx ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    CircuitBreaker,

    /// **[Elektra]** Discharges high voltage to damage/charge nearby entities.
    ///
    /// **Stack:** `[ ..., power, radius ] -> [ ... ]`
    #[cfg(feature = "elektra")]
    TeslaCoil,

    /// **[Elektra + Nova]** Resurrects a dead strand using high voltage.
    ///
    /// **Stack:** `[ ..., graveyard_idx ] -> [ ..., new_strand_idx ]`
    #[cfg(all(feature = "elektra", feature = "nova"))]
    Galvanize,

    // Nova Features
    /// **[Nova]** Expands an L-System axiom using rules and iterations.
    ///
    /// **Stack:** `[ ..., axiom, rules, iterations ] -> [ ..., result_string ]`
    #[cfg(feature = "nova")]
    Morph,
    /// **[Nova]** Grows a structure on the grid using Turtle graphics commands.
    ///
    /// **Stack:** `[ ..., instruction_string, start_y, start_x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Grow,
    /// **[Nova]** Spawns a Seed organelle that grows procedurally over time (The Garden).
    ///
    /// **Stack:** `[ ..., rules, axiom ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Plant,

    /// **[Nova]** Creates a "time-travel" snapshot (Spore) of the VM state.
    ///
    /// **Stack:** `[ ... ] -> [ ..., spore_id ]`
    /// **Cost:** 50 Energy.
    #[cfg(feature = "nova")]
    Sporulate,
    /// **[Nova]** Restores the VM state from a Spore.
    ///
    /// **Stack:** `[ ..., spore_id ] -> [ ... ]`
    /// **Effect:** Reverts *everything* (Grid, DNA, Stack) to the spore's state.
    #[cfg(feature = "nova")]
    Germinate,
    /// **[Nova]** Creates a named Time Loop anchor.
    ///
    /// **Stack:** `[ ..., loop_id ] -> [ ... ]`
    /// **Effect:** Saves state to a specific slot.
    #[cfg(feature = "nova")]
    TimeLoop,
    /// **[Nova]** Triggers a Paradox, rewinding time but keeping a value.
    ///
    /// **Stack:** `[ ..., loop_id, value ] -> [ ..., value ]`
    /// **Effect:** Restores state from loop_id, but pushes value to stack.
    /// **Risk:** Increases Paradox counter. Too much Paradox causes issues.
    #[cfg(feature = "nova")]
    Paradox,
    /// **[Nova]** Splices a strand from a saved Spore (timeline) into the current genome.
    ///
    /// **Stack:** `[ ..., spore_id, strand_idx ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "nova")]
    ChronosSplice,
    /// **[Nova]** Creates a new DNA strand from a sequence of values on the grid.
    ///
    /// **Stack:** `[ ..., len, y, x ] -> [ ... ]`
    /// **Effect:** Reads `len` cells starting at `(x, y)` and compiles them into a new strand.
    #[cfg(feature = "nova")]
    Incubate,
    /// **[Nova]** Marks a gene as methylated (epigenetics).
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    /// **Effect:** Adds an epigenetic marker.
    #[cfg(feature = "nova")]
    Methylate,
    /// **[Nova]** Removes a methylation marker.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Demethylate,
    /// **[Nova]** Extends the lifespan (telomeres) of the current strand.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Cost:** 25 Energy.
    #[cfg(feature = "nova")]
    Telomerase,
    /// **[Nova]** Reads the remaining telomere length of the current strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., length ]`
    #[cfg(feature = "nova")]
    TLen,
    /// **[Nova]** Swaps the tails of two strands at a split point.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, split_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Recombine,
    /// **[Nova]** Slices and recombines two strands into a new one (Laboratory Splicing).
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, method ] -> [ ..., new_strand_idx ]`
    /// **Methods:** 0=Interleave, 1=Uniform Crossover, 2=Midpoint Split.
    #[cfg(feature = "nova")]
    Splice,
    /// **[Nova]** Stitches two strands together with high-voltage seams.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, stitches ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates a new strand by alternating chunks of A and B, inserting Sparks/Glitches at seams.
    #[cfg(feature = "nova")]
    Frankenstein,
    /// **[Nova]** Performs a single-point crossover at a random index.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b ] -> [ ..., new_strand_1, new_strand_2 ]`
    #[cfg(feature = "nova")]
    Crossover,
    /// **[Nova]** Pushes the index of the currently executing strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., current_strand_idx ]`
    #[cfg(feature = "nova")]
    SIndex,
    /// **[Nova]** Scans a target strand for a pattern matching a guide strand.
    ///
    /// **Stack:** `[ ..., target_idx, guide_idx ] -> [ ..., match_index ]`
    /// **Effect:** Returns index of first match or -1.
    #[cfg(feature = "nova")]
    CrisprScan,
    /// **[Nova]** Cuts a strand into two at a specific index.
    ///
    /// **Stack:** `[ ..., strand_idx, cut_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** The tail becomes a new strand.
    #[cfg(feature = "nova")]
    Cas9Cut,
    /// **[Nova]** Joins two strands together.
    ///
    /// **Stack:** `[ ..., recipient_idx, donor_idx ] -> [ ... ]`
    /// **Effect:** Appends donor genes to recipient. Donor becomes empty.
    #[cfg(feature = "nova")]
    Ligase,
    /// **[Nova]** Clones a strand perfectly.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Creates a new identical strand.
    #[cfg(feature = "nova")]
    Mitosis,
    /// **[Nova]** Destroys a strand.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Clears genes and epigenetics of the strand.
    #[cfg(feature = "nova")]
    Apoptosis,
    /// **[Nova]** Inserts a new gene into a strand.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx, op_name, arg ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Integrase,
    /// **[Nova]** Removes a gene from a strand.
    ///
    /// **Stack:** `[ ..., strand_idx, gene_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Excision,
    /// **[Nova]** Secretes hormones into the environment.
    ///
    /// **Stack:** `[ ..., channel, amount ] -> [ ... ]`
    /// **Effect:** Adds to hormone grid at current location.
    #[cfg(feature = "nova")]
    Secrete,
    /// **[Nova]** Detects hormone levels.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., intensity ]`
    #[cfg(feature = "nova")]
    Detect,
    /// **[Nova]** Absorbs hormones from the environment.
    ///
    /// **Stack:** `[ ..., channel, amount ] -> [ ..., absorbed_amount ]`
    #[cfg(feature = "nova")]
    Absorb,
    /// **[Nova]** Moves the execution context (spatial location).
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    /// **Cost:** 5 Energy. Blocked by walls/membranes.
    #[cfg(feature = "nova")]
    Migrate,
    /// **[Nova]** Cleans waste from the environment.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Detox,
    /// **[Nova]** Reads local waste level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., waste_level ]`
    #[cfg(feature = "nova")]
    WRead,
    /// **[Nova]** Calls another strand as a subroutine.
    ///
    /// **Args:** `[Nucleotide::Number(strand_idx)]`
    /// **Effect:** Pushes return address to `call_stack` and jumps.
    #[cfg(feature = "nova")]
    Call,
    /// **[Nova]** Calls a strand index popped from the stack.
    ///
    /// **Stack:** `[ ..., strand_idx ]`
    #[cfg(feature = "nova")]
    Exec,
    /// **[Nova]** Returns from a subroutine.
    ///
    /// **Effect:** Pops address from `call_stack` and jumps.
    #[cfg(feature = "nova")]
    Ret,
    /// **[Nova]** Binds a keyboard input to a strand.
    ///
    /// **Stack:** `[ ..., char_code, strand_idx ] -> [ ... ]`
    /// **Effect:** Pressing the key will trigger an interrupt on that strand.
    #[cfg(feature = "nova")]
    Bind,
    /// **[Nova]** Unbinds a keyboard input.
    ///
    /// **Stack:** `[ ..., char_code ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Unbind,
    /// **[Nova]** Registers an interrupt handler for internal events.
    ///
    /// **Stack:** `[ ..., event_id, strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Reflex,
    /// **[Nova]** Quantum entangles two strands.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b ] -> [ ... ]`
    /// **Effect:** Mutations/Transcriptions on one strand affect the other.
    #[cfg(feature = "nova")]
    Entangle,
    /// **[Nova]** Breaks quantum entanglement.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Decohere,
    /// **[Nova]** Moves the current gene to a new location in the strand (Transposon).
    ///
    /// **Stack:** `[ ..., offset ] -> [ ... ]`
    /// **Effect:** Moves current instruction `offset` genes away, replacing self with Nop. Jumps to new location.
    #[cfg(feature = "nova")]
    Transposon,
    /// **[Nova]** Writes a strand's code onto the grid physically.
    ///
    /// **Stack:** `[ ..., strand_idx, y, x, direction ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Conjugate,
    /// **[Nova]** Pulls items on the grid towards the center.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Gravitate,
    /// **[Nova]** Emits light into the environment.
    ///
    /// **Stack:** `[ ..., intensity, radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Lumine,
    /// **[Nova]** Senses light level at current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., intensity ]`
    #[cfg(feature = "nova")]
    SenseLight,
    /// **[Nova]** Sets the bioluminescent color and intensity at the current location.
    ///
    /// **Stack:** `[ ..., r, g, b, intensity ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Luciferin,
    /// **[Nova]** Emits the current location's bioluminescence to neighbors.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Photophore,
    /// **[Nova]** Spawns an Organelle (sub-process).
    ///
    /// **Stack:** `[ ..., strand_idx, type ] -> [ ... ]`
    /// **Types:** 1=Chloroplast, 2=Mitochondria, 3=Lysosome, 4=Ribosome.
    #[cfg(feature = "nova")]
    Spawn,
    /// **[Nova]** Runs a simulation of a strand in a sandboxed VM.
    ///
    /// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., result, energy, status ]`
    /// **Cost:** High energy cost.
    #[cfg(feature = "nova")]
    Simulate,
    /// **[Nova]** Simulates a mutated version of a strand; adopts if beneficial.
    ///
    /// **Stack:** `[ ..., strand_idx, ticks ] -> [ ..., success ]`
    #[cfg(feature = "nova")]
    Dream,
    /// **[Nova]** Reduces local entropy to prevent Nightmares.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Lucid,
    /// **[Nova]** Calculates direction towards highest chemical concentration.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., dy, dx ]`
    #[cfg(feature = "nova")]
    Chemotaxis,
    /// **[Nova]** Returns the identity (Organelle Type) of the current executor.
    ///
    /// **Stack:** `[ ... ] -> [ ..., type_id ]`
    #[cfg(feature = "nova")]
    Identity,
    /// **[Nova]** Changes the type of the current organelle.
    ///
    /// **Stack:** `[ ..., type_id ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Differentiate,
    /// **[Nova]** Scans a line on the grid for non-empty cells.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ..., distance, value ]`
    #[cfg(feature = "nova")]
    Sonar,
    /// **[Nova]** Opens a spatial portal between two points.
    ///
    /// **Stack:** `[ ..., y1, x1, y2, x2 ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Rift,
    /// **[Nova]** Closes a portal.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Seal,
    /// **[Nova]** Warps the topology of the grid (e.g., Torus, Klein Bottle).
    ///
    /// **Stack:** `[ ..., topology_type ] -> [ ... ]`
    /// **Cost:** 100 Energy.
    #[cfg(feature = "nova")]
    Shape,
    /// **[Nova]** Broadcasts a value to a global radio channel.
    ///
    /// **Stack:** `[ ..., channel, value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Broadcast,
    /// **[Nova]** Receives a value from a global radio channel.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    Tune,

    // Retina Features (Vision)
    /// **[Nova]** Draws a pixel to the Retina buffer.
    ///
    /// **Stack:** `[ ..., packed_color, char_code, y, x ] -> [ ... ]`
    /// **Packed Color:** `(R << 16) | (G << 8) | B`.
    #[cfg(feature = "nova")]
    RetinaDraw,
    /// **[Nova]** Clears the Retina buffer with a color.
    ///
    /// **Stack:** `[ ..., packed_color ] -> [ ... ]`
    #[cfg(feature = "nova")]
    RetinaClear,
    /// **[Nova]** Pushes the dimensions of the Retina buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., width, height ]`
    #[cfg(feature = "nova")]
    RetinaSize,
    /// **[Nova]** Reads a row of the Retina buffer.
    ///
    /// **Stack:** `[ ..., row_index ] -> [ ..., junction_of_pixels ]`
    #[cfg(feature = "nova")]
    Scanline,
    /// **[Nova]** Writes a Junction of values to the Retina buffer with glitch effects.
    ///
    /// **Stack:** `[ ..., y, x, junction, mode ] -> [ ... ]`
    /// **Mode:** 0=Linear, 1=Scatter, 2=XOR, 3=Sort.
    #[cfg(feature = "nova")]
    Rasterize,

    // Quantum Features (Superposition)
    /// **[Nova]** Instantly jumps to the entangled partner strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Sets IP to partner strand at current gene index.
    #[cfg(feature = "nova")]
    QuantumJump,
    /// **[Nova]** Creates a quantum superposition of the top two values.
    ///
    /// **Stack:** `[ ..., val_a, val_b ] -> [ ..., Ψ(val_a:0.5, val_b:0.5) ]`
    #[cfg(feature = "nova")]
    Superpose,
    /// **[Nova]** Collapses a superposition into a single value based on probability.
    ///
    /// **Stack:** `[ ..., superposition ] -> [ ..., collapsed_val ]`
    #[cfg(feature = "nova")]
    Collapse,
    /// **[Nova]** Observes the value, collapsing it and logging the result.
    ///
    /// **Stack:** `[ ..., superposition ] -> [ ..., collapsed_val ]`
    #[cfg(feature = "nova")]
    Observe,

    /// **[Nova]** Sings a note into the Chorus Buffer.
    ///
    /// **Stack:** `[ ..., note_string ] -> [ ... ]`
    /// **Effect:** Checks for Chords (magic spells).
    #[cfg(feature = "nova")]
    Sing,
    /// **[Nova]** Listens to the Chorus Buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., chorus_junction ]`
    #[cfg(feature = "nova")]
    Listen,
    /// **[Nova]** Registers a chord to trigger a strand.
    ///
    /// **Stack:** `[ ..., chord_junction, strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Harmonize,
    /// **[Nova]** Spawns a Choir organelle that sings a song.
    ///
    /// **Stack:** `[ ..., song_junction ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Choir,

    /// **[Nova]** Pauses execution and requests input from the Spirit (User).
    ///
    /// **Stack:** `[ ... ] -> [ ..., input_value ]`
    #[cfg(feature = "nova")]
    Spirit,

    /// **[Nova]** Shifts the organism's phase of matter.
    ///
    /// **Stack:** `[ ..., phase_id ] -> [ ... ]`
    /// **Modes:** 0=Corporeal, 1=Ethereal, 2=Crystalline, 3=Flux.
    /// **Cost:** 50 Energy.
    #[cfg(feature = "nova")]
    PhaseShift,
    /// **[Nova]** Toggles the organism's chirality (L-isomer <-> D-isomer).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Inverts arithmetic and control flow logic.
    #[cfg(feature = "nova")]
    Isomerize,
    /// **[Nova]** Modifies cellular membranes (walls) at current location.
    ///
    /// **Stack:** `[ ..., mask ] -> [ ... ]`
    /// **Mask:** 1=N, 2=S, 4=E, 8=W.
    #[cfg(feature = "nova")]
    Membrane,
    /// **[Nova]** Moves through membranes/walls at high cost.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Osmosis,
    /// **[Nova]** Merges an organelle back into the main organism (Symbiote).
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Symbiosis,
    /// **[Nova]** Ejects a Symbiote as a free-roaming Organelle.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Lysis,

    // Necromancy Features
    /// **[Nova]** Buries a strand in the graveyard (copies and clears).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Bury,
    /// **[Nova]** Exhumes a strand from the graveyard (restores it).
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "nova")]
    Exhume,
    /// **[Nova]** Executes a dead strand from the graveyard (ephemeral).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Seance,
    /// **[Nova]** Gains energy from the graveyard.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    #[cfg(feature = "nova")]
    Mourn,

    /// **[Nova]** Kills the target strand and spawns a mutated copy.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "nova")]
    Reincarnate,

    /// **[Nova]** Compiles a string into a strand.
    ///
    /// **Stack:** `[ ..., source_string ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Parses source and creates new strand.
    #[cfg(feature = "nova")]
    Compile,
    /// **[Nova]** Decompiles a strand into a string.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., source_string ]`
    #[cfg(feature = "nova")]
    Decompile,

    /// **[Nova]** Spawns a Void organelle that consumes everything.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Void,
    /// **[Nova]** Opens a Void Rift at the specified location.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    VoidRift,
    /// **[Nova]** Channels power from the nearest Void Rift.
    ///
    /// **Stack:** `[ ... ] -> [ ..., power ]`
    #[cfg(feature = "nova")]
    VoidCast,
    /// **[Nova]** Explodes the current strand, scattering genes onto the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Supernova,
    /// **[Nova]** Merges all strands into a single massive strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Singularity,

    /// **[Nova]** Increases local mutagen level.
    ///
    /// **Stack:** `[ ..., amount, radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Irradiate,
    /// **[Nova]** Reads local mutagen level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., level ]`
    #[cfg(feature = "nova")]
    SenseMutagen,
    /// **[Nova]** Consumes local mutagen to gain energy.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    #[cfg(feature = "nova")]
    Devour,

    /// **[Nova]** Runs a cellular automaton step on the Grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Evolve,
    // Garden Features (Nova)
    /// **[Nova]** Defines a Cellular Automata rule for a species.
    ///
    /// **Stack:** `[ ..., rule_string, species_id ] -> [ ... ]`
    /// **Rule String:** e.g. "B3/S23" (Life), "B36/S23" (HighLife).
    #[cfg(feature = "nova")]
    Sow,
    /// **[Nova]** Harvests the grid pattern into a compressed string.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., rle_string ]`
    #[cfg(feature = "nova")]
    Harvest,
    /// **[Nova]** Randomly corrupts the Grid or Stack.
    ///
    /// **Stack:** `[ ..., severity ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Glitch,
    /// **[Nova]** Shuffles the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Scramble,

    /// **[Nova]** Pushes local entropy level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., entropy_level ]`
    #[cfg(feature = "nova")]
    Entropy,
    /// **[Nova]** Reduces local entropy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Cost:** Energy proportional to amount.
    #[cfg(feature = "nova")]
    Stabilize,
    /// **[Nova]** Increases global entropy and injects chaos into a random grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EntropySurge,
    /// **[Nova]** Forcefully jumps the IP to a random gene in the current strand.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    QuantumTunnel,
    /// **[Nova]** Instantly converts a grid cell to high entropy.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Disintegrate,

    /// **[Nova]** Forks time into N timelines, selects one where the query is true.
    ///
    /// **Stack:** `[ ..., count, query_term ] -> [ ..., success ]`
    /// **Effect:** Replaces current VM state with the successful timeline.
    #[cfg(all(feature = "nova", feature = "oracle"))]
    Divergence,

    /// **[Nova]** Predicts if the current execution path leads to death within `ticks`.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ..., 1(Death)|0(Life) ]`
    #[cfg(feature = "nova")]
    Prophecy,

    /// **[Nova]** Toggles Logic Chemistry mode (Logos).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Enables/Disables active unification of grid cells based on Oracle rules.
    #[cfg(feature = "nova")]
    Logos,

    /// **[Nova]** Transmutes the current grid cell based on neighbors (Alchemy).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Alchemy,

    /// **[Nova]** Triggers a metamorphic reboot based on a CA rule.
    ///
    /// **Stack:** `[ ..., rule_id ] -> [ ... ]`
    /// **Effect:** Replaces entire DNA with genes derived from the Grid state.
    #[cfg(feature = "nova")]
    Genesis,

    /// **[Nova]** Injects pure entropy into the system.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    /// **Effect:** Increases Havoc rate and local Entropy.
    #[cfg(feature = "nova")]
    Chaos,

    /// **[Mad Science]** Replaces occurrences of an OpCode with another with a given probability.
    ///
    /// **Stack:** `[ ..., target_strand, probability, from_op_str, to_op_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Mutagen,

    /// **[Nova]** Triggers a Cambrian Explosion (Mass Speciation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Clears all organelles and spawns diverse new ones.
    #[cfg(feature = "nova")]
    Cambrian,

    // Attractor Features (Nova - Chaos Dynamics)
    /// **[Nova]** Initializes the Strange Attractor.
    ///
    /// **Stack:** `[ ..., mode ] -> [ ... ]`
    /// **Mode:** 0=Lorenz, 1=Rossler, 2=Thomas.
    #[cfg(feature = "nova")]
    AttractorInit,
    /// **[Nova]** Steps the Strange Attractor simulation.
    ///
    /// **Stack:** `[ ..., dt ] -> [ ... ]`
    /// **dt:** Time delta (e.g. 0.01).
    #[cfg(feature = "nova")]
    AttractorStep,
    /// **[Nova]** Warps the IP based on the Attractor state.
    ///
    /// **Stack:** `[ ..., scale ] -> [ ... ]`
    /// **Effect:** Jumps to strand index derived from Attractor Z.
    #[cfg(feature = "nova")]
    AttractorSurf,
    /// **[Nova]** Maps the Attractor state to Grid variables.
    ///
    /// **Stack:** `[ ..., target ] -> [ ... ]`
    /// **Target:** 0=Entropy, 1=Mutation Rate, 2=Grid Warp.
    #[cfg(feature = "nova")]
    AttractorMap,

    // Holographic Features (Nova)
    /// **[Nova]** Encodes a strand into the Hologram Grid (Interference).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Interfere,
    /// **[Nova]** Decodes the Hologram Grid into a new strand (Refraction).
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "nova")]
    Refract,
    /// **[Nova]** Projects the Hologram intensity onto the main Grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Project,
    /// **[Nova]** Diffracts a strand into the Hologram Grid (Ghost/Split).
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Diffract,
    /// **[Nova]** Toggles Holographic View/Mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Hologram,
    /// **[Nova]** Applies phase shift noise to the hologram grid (Mutation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    PhaseMutate,

    /// **[Nova]** Collapses the local Hologram wavefunction into a physical grid character.
    ///
    /// **Stack:** `[ ..., threshold ] -> [ ... ]`
    /// **Effect:** If `|H(y,x)| > threshold`, maps Phase(H) to an ASCII character and writes it to `Grid(y,x)`.
    #[cfg(feature = "nova")]
    QuantumScribe,

    /// **[Nova]** Encodes the local physical grid character into the Hologram.
    ///
    /// **Stack:** `[ ..., weight ] -> [ ... ]`
    /// **Effect:** Maps `Grid(y,x)` character to Phase, and adds `weight * e^(i*phase)` to `Hologram(y,x)`.
    #[cfg(feature = "nova")]
    QuantumScan,

    /// **[Nova]** Refracts the Hologram into a Grammar and parses a string with it.
    ///
    /// **Stack:** `[ ..., input_string ] -> [ ..., result_ast ]`
    #[cfg(feature = "nova")]
    HoloInvoke,

    /// **[Nova]** Refracts the Hologram into a Grammar and generates a string from it.
    ///
    /// **Stack:** `[ ... ] -> [ ..., output_string ]`
    #[cfg(feature = "nova")]
    HoloSpeak,

    /// **[Nova]** Converts an integer to a 1-character string (ASCII).
    ///
    /// **Stack:** `[ ..., int ] -> [ ..., string ]`
    #[cfg(feature = "nova")]
    Chr,

    /// **[Nova]** Converts the Hologram Grid state into MIDI events.
    ///
    /// **Stack:** `[ ..., threshold ] -> [ ... ]`
    /// **Effect:** Generates notes based on hologram intensity.
    #[cfg(feature = "nova")]
    HoloSonify,

    /// **[Nova]** Encodes the current Audio Snapshot (Cymatics) into the Hologram Grid.
    ///
    /// **Stack:** `[ ..., scale ] -> [ ... ]`
    /// **Effect:** Modifies hologram based on audio pressure.
    #[cfg(feature = "nova")]
    CymaticScan,

    /// **[Nova]** Toggles the Orca Signal Processing system on the grid.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Orca,

    // Catalyst Features (Nova)
    /// **[Nova]** Creates a new Catalyst from a strand.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., catalyst_id ]`
    #[cfg(feature = "nova")]
    Synthesize,
    /// **[Nova]** Applies a Catalyst to mutate a target strand.
    ///
    /// **Stack:** `[ ..., catalyst_id, target_strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Catalyze,

    // Chemistry Features (Nova)
    /// **[Nova]** Mixes neighbors into a solution in the current cell.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Mix,
    /// **[Nova]** Brews the current cell's mixture into a Potion.
    ///
    /// **Stack:** `[ ..., heat ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Brew,
    /// **[Nova]** Splashes a Potion to a target location.
    ///
    /// **Stack:** `[ ..., radius, dy, dx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Splash,

    /// **[Nova]** Interacts with the Alchemical Crucible.
    ///
    /// **Stack:** `[ ..., mode ]`
    /// **Mode:** 0=Add (Pop), 1=Clear, 2=Transmute.
    #[cfg(feature = "nova")]
    Crucible,

    /// **[Nova]** Spreads the last executed instruction to a random strand (Memetics).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Meme,

    /// **[Nova]** Creates a Meme from the current strand's genes.
    ///
    /// **Stack:** `[ ..., len, virulence, fidelity ] -> [ ..., meme_id ]`
    #[cfg(feature = "nova")]
    Conceive,

    /// **[Nova]** Spreads a specific Meme to a target strand.
    ///
    /// **Stack:** `[ ..., meme_id, target_strand ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Propagate,

    /// **[Nova]** Removes a Meme from the pool.
    ///
    /// **Stack:** `[ ..., meme_id ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Forget,

    /// **[Nova]** Modifies the dialect of the current strand (OpCode Remapping).
    ///
    /// **Stack:** `[ ..., from_op_str, to_op_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Shibboleth,

    /// **[Nova]** Spawns a linguistic virus at the current location.
    ///
    /// **Stack:** `[ ..., mutation_rate, pattern_str, name_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Infect,

    /// **[Nova]** Triggers a viral outbreak step (spread & mutate).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Outbreak,

    /// **[Nova]** Clears viral infection in a radius.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Sanitize,

    /// **[Nova]** Randomly mutates the genome with a given probability (Linguistic Drift).
    ///
    /// **Stack:** `[ ..., probability ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Drift,

    /// **[Nova]** Dynamic dispatch based on stack top type (Polymorphism).
    ///
    /// **Stack:** `[ val ]` (peeks) -> executes op1 (if Int) or op2 (if Str)
    /// **Args:** `[Nucleotide::String(op_int), Nucleotide::String(op_str)]`
    #[cfg(feature = "nova")]
    Poly,

    /// **[Nova]** Reshuffles the entire DNA based on the current Grid state.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Clears DNA, reads Grid as DNA, resets Energy to 50, IP to (0,0), Stack to [].
    #[cfg(feature = "nova")]
    Metamorphosis,

    /// **[Nova]** Executes the Grid Colors as a Piet program.
    ///
    /// **Stack:** `[ ..., steps ] -> [ ... ]`
    /// **Effect:** Runs a Piet interpreter on the ChromaGrid.
    /// **Interaction:** 'In' pops from Chimera Stack, 'Out' pushes to Chimera Stack.
    #[cfg(feature = "nova")]
    Piet,

    /// **[Nova]** Freezes the environment and other organisms for a duration.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ... ]`
    /// **Cost:** 50 + ticks Energy.
    #[cfg(feature = "nova")]
    Chronostasis,

    /// **[Nova]** Sets the local time dilation factor in a radius.
    ///
    /// **Stack:** `[ ..., factor, radius ] -> [ ... ]`
    /// **Factor:** 0=Stasis, 1=Normal, >1=Accelerated.
    #[cfg(feature = "nova")]
    TimeWarp,

    /// **[Nova]** Reads the local time dilation factor at the current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., factor ]`
    #[cfg(feature = "nova")]
    Chronos,

    /// **[Nova]** Accesses the state of a grid cell from `ticks` ago.
    ///
    /// **Stack:** `[ ..., ticks, y, x ] -> [ ..., past_value ]`
    /// **Effect:** Allows looking back in time.
    #[cfg(feature = "nova")]
    Retroscope,

    /// **[Nova]** Reverts the Grid state to a previous point in time.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ... ]`
    /// **Effect:** Overwrites current grid with the state from `ticks` ago.
    #[cfg(feature = "nova")]
    Retrograde,

    // Egregore Features (Collective Consciousness)
    /// **[Nova]** Connects to the collective Egregore mind.
    ///
    /// **Stack:** `[ ..., channel_name ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EgregoreLink,
    /// **[Nova]** Sacrifices energy (faith) to the Egregore.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EgregoreTithe,
    /// **[Nova]** Sends a message to a shared Egregore channel.
    ///
    /// **Stack:** `[ ..., channel_name, value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EgregoreChannel,
    /// **[Nova]** Casts a vote on a global parameter.
    ///
    /// **Stack:** `[ ..., parameter_name, vote_value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EgregoreDictate,
    /// **[Nova]** Queries a value from the Egregore (channel or parameter).
    ///
    /// **Stack:** `[ ..., key ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    EgregoreQuery,
    /// **[Nova]** Attempts to summon a global effect using collective faith.
    ///
    /// **Stack:** `[ ..., ritual_name ] -> [ ... ]`
    #[cfg(feature = "nova")]
    EgregoreSummon,
    /// **[Nova]** Sacrifices the current strand to feed the Chaos of the Egregore.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Effect:** Kills strand, shifts Alignment towards Chaos (-), gains Faith.
    #[cfg(feature = "nova")]
    Sacrifice,
    /// **[Nova]** Prays to the Egregore for Order.
    ///
    /// **Stack:** `[ ..., energy_amount ] -> [ ... ]`
    /// **Effect:** Consumes Energy, shifts Alignment towards Order (+), gains Faith.
    #[cfg(feature = "nova")]
    Pray,

    // String Theory (Nova)
    /// **[Nova]** Creates a vibrating Cosmic String.
    ///
    /// **Stack:** `[ ..., length, tension, y, x ] -> [ ... ]`
    /// **Effect:** Spawns a string starting at (x,y) extending in current direction.
    #[cfg(feature = "nova")]
    StringNew,
    /// **[Nova]** Plucks the nearest Cosmic String.
    ///
    /// **Stack:** `[ ..., force ] -> [ ... ]`
    /// **Effect:** Adds energy to the string's vibration.
    #[cfg(feature = "nova")]
    StringPluck,
    /// **[Nova]** Tunes the tension of the nearest Cosmic String.
    ///
    /// **Stack:** `[ ..., tension ] -> [ ... ]`
    #[cfg(feature = "nova")]
    StringTune,
    /// **[Nova]** Listens to the vibration of the nearest Cosmic String.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amplitude ]`
    #[cfg(feature = "nova")]
    StringListen,

    // Astrology Features (Nova)
    /// **[Nova]** Gazes at the sky to measure star intensity and color.
    ///
    /// **Stack:** `[ ... ] -> [ ..., intensity, color ]`
    #[cfg(feature = "nova")]
    Gaze,
    /// **[Nova]** Summons a meteor strike if a star is overhead.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    /// **Cost:** 50 Energy.
    #[cfg(feature = "nova")]
    Starfall,
    /// **[Nova]** Checks alignment with the nearest star.
    ///
    /// **Stack:** `[ ... ] -> [ ..., angle_to_nearest ]`
    #[cfg(feature = "nova")]
    Align,

    // Gastronomy Features (Nova)
    /// **[Nova]** Cooks stack items into a Dish.
    ///
    /// **Stack:** `[ ..., count, item_1, ..., item_n ] -> [ ..., dish_junction ]`
    #[cfg(feature = "nova")]
    Cook,
    /// **[Nova]** Adds a spice (modifier) to a Dish.
    ///
    /// **Stack:** `[ ..., dish_junction, spice_string ] -> [ ..., spiced_dish ]`
    #[cfg(feature = "nova")]
    Spice,
    /// **[Nova]** Consumes a Dish to gain Energy and Buffs.
    ///
    /// **Stack:** `[ ..., dish_junction ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Savor,
    /// **[Nova]** Cultivates a grid cell to increase its value/quality.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]` (Operates on current location)
    #[cfg(feature = "nova")]
    Cultivate,
    /// **[Nova]** Feeds neighbors with energy/healing.
    ///
    /// **Stack:** `[ ..., radius, amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Banquet,

    // Market Features
    /// **[Nova]** Places a Sell Order (Ask) on the Market.
    ///
    /// **Stack:** `[ ..., price, item ] -> [ ..., order_id ]`
    /// **Effect:** Adds item to market. If sold, funds are credited.
    #[cfg(feature = "nova")]
    Offer,
    /// **[Nova]** Places a Buy Order (Bid) on the Market.
    ///
    /// **Stack:** `[ ..., max_price, query ] -> [ ..., item, cost ]`
    /// **Effect:** Purchases item if available. Returns item and cost. If failed, returns 0.
    #[cfg(feature = "nova")]
    Buy,
    /// **[Nova]** Converts global Energy into local Credits.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Invest,
    /// **[Nova]** Converts local Credits into global Energy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Divest,
    /// **[Nova]** Pushes current Credit balance to stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., balance ]`
    #[cfg(feature = "nova")]
    Balance,
    /// **[Nova]** Pushes last trade price to stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., price ]`
    #[cfg(feature = "nova")]
    Ticker,

    // Relativity Features
    /// **[Nova]** Toggles General Relativity simulation (Time Dilation).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Relativity,
    /// **[Nova]** Emits a gravity wave, increasing local mass.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Graviton,
    /// **[Nova]** Checks the local gravity field strength.
    ///
    /// **Stack:** `[ ... ] -> [ ..., gravity ]`
    #[cfg(feature = "nova")]
    EventHorizon,

    // Atmosphere Features
    /// **[Nova]** Sets the local wind vector.
    ///
    /// **Stack:** `[ ..., angle, strength ] -> [ ... ]`
    /// **Angle:** 0=N, 1=NE, 2=E, 3=SE, 4=S, 5=SW, 6=W, 7=NW.
    #[cfg(feature = "nova")]
    Aeolus,
    /// **[Nova]** Creates moisture (clouds/rain) at the current location.
    ///
    /// **Stack:** `[ ..., intensity, radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Storm,
    /// **[Nova]** Reads the local wind vector.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx ]`
    #[cfg(feature = "nova")]
    SenseWind,
    /// **[Nova]** Reads the local moisture level.
    ///
    /// **Stack:** `[ ... ] -> [ ..., moisture ]`
    #[cfg(feature = "nova")]
    SenseMoisture,
    /// **[Nova]** Creates a strong directional wind wave.
    ///
    /// **Stack:** `[ ..., power, direction ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Tsunami,
    /// **[Nova]** Removes moisture from a circular area.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Dry,

    // Olfactory Features (Nova)
    /// **[Nova]** Emits a scent trail (Pheromone).
    ///
    /// **Stack:** `[ ..., intensity, signature_string ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Emit,
    /// **[Nova]** Smells the strongest local scent.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx, intensity, signature ]`
    #[cfg(feature = "nova")]
    Smell,
    /// **[Nova]** Tracks a specific scent.
    ///
    /// **Stack:** `[ ..., signature ] -> [ ..., dy, dx ]`
    #[cfg(feature = "nova")]
    Track,

    /// **[Nova]** Changes the biome of the local area.
    ///
    /// **Stack:** `[ ..., biome_id, radius ] -> [ ... ]`
    /// **Biome IDs:** 0=Plains, 1=Swamp, 2=Desert, 3=Tundra, 4=Volcanic.
    #[cfg(feature = "nova")]
    Terraform,

    /// **[Nova]** Reads the current biome ID.
    ///
    /// **Stack:** `[ ... ] -> [ ..., biome_id ]`
    #[cfg(feature = "nova")]
    SenseBiome,

    /// **[Nova]** Remaps an OpCode to another OpCode at runtime.
    ///
    /// **Stack:** `[ ..., from_op_str, to_op_str ] -> [ ... ]`
    /// **Effect:** `from_op` will now behave like `to_op`.
    #[cfg(feature = "nova")]
    Remap,
    /// **[Nova]** Restores an OpCode to its original behavior.
    ///
    /// **Stack:** `[ ..., op_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Restore,
    /// **[Nova]** Reverses the direction of execution.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Mirror,

    /// **[Nova]** Sets the foreground color of a grid cell (Chromatophores).
    ///
    /// **Stack:** `[ ..., r, g, b, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Pigment,
    /// **[Nova]** Sets the character representation of a grid cell (Chromatophores).
    ///
    /// **Stack:** `[ ..., char_code, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Glyph,
    /// **[Nova]** Reads the foreground color of a grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ..., r, g, b ]`
    #[cfg(feature = "nova")]
    SensePigment,
    /// **[Nova]** Reads the character representation of a grid cell.
    ///
    /// **Stack:** `[ ... ] -> [ ..., char_code ]`
    #[cfg(feature = "nova")]
    SenseGlyph,

    // Fungi Features (Mycelial Network)
    /// **[Nova]** Spawns a fungal node (Hyphae) at the current grid location.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Hyphae,
    /// **[Nova]** Connects the current Hyphae to another at target coordinates.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Connect,
    /// **[Nova]** Transport a value instantly to a connected Hyphae.
    ///
    /// **Stack:** `[ ..., val, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Transport,
    /// **[Nova]** Release spores to randomly spawn Hyphae nearby.
    ///
    /// **Stack:** `[ ..., radius, density ] -> [ ... ]`
    #[cfg(feature = "nova")]
    SporeCloud,

    // Polyglot Features
    /// **[Nova]** Executes a string as Brainfuck code.
    ///
    /// **Stack:** `[ ..., bf_code_string, input_string ] -> [ ..., output_string ]`
    #[cfg(feature = "nova")]
    Brainfuck,

    // Meta Features (Self-Definition)
    /// **[Nova]** Defines a new enzyme (OpCode) that calls a strand.
    ///
    /// **Stack:** `[ ..., name_str, strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Define,
    /// **[Nova]** Removes a defined enzyme.
    ///
    /// **Stack:** `[ ..., name_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Undefine,
    /// **[Nova]** Pushes a list of all defined custom enzymes.
    ///
    /// **Stack:** `[ ... ] -> [ ..., list_junction ]`
    #[cfg(feature = "nova")]
    Dictionary,

    // Akashic Features (Persistent Storage)
    /// **[Nova]** Writes a key-value pair to the persistent Akashic Record.
    ///
    /// **Stack:** `[ ..., key, value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    AkashicWrite,
    /// **[Nova]** Reads a value from the persistent Akashic Record.
    ///
    /// **Stack:** `[ ..., key ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    AkashicRead,

    /// **[Nova]** Dumps the flight recorder (blackbox) to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dump_string ]`
    #[cfg(feature = "nova")]
    Blackbox,

    // Ribozyme Features (Functional Programming)
    /// **[Nova]** Evaluates a string as code.
    ///
    /// **Stack:** `[ ..., code_string ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Eval,
    /// **[Nova]** Concatenates two strands into a new function (Functional Composition).
    ///
    /// **Stack:** `[ ..., strand_g, strand_f ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates `new_strand = f + g` (f executes then g).
    #[cfg(feature = "nova")]
    Chain,
    /// **[Nova]** Partially applies a value to a strand.
    ///
    /// **Stack:** `[ ..., value, strand_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates `new_strand = [push(value)] + strand`.
    #[cfg(feature = "nova")]
    Curry,
    /// **[Nova]** Consumes the next instruction as a string literal.
    ///
    /// **Stack:** `[ ... ] -> [ ..., op_string ]`
    #[cfg(feature = "nova")]
    Quote,
    /// **[Nova]** Applies a function to each element of a Junction.
    ///
    /// **Stack:** `[ ..., junction, function ] -> [ ..., new_junction ]`
    #[cfg(feature = "nova")]
    Map,
    /// **[Nova]** Hashes a strand and adds it to the immune allowlist.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Vaccinate,
    /// **[Nova]** Verifies if a strand is in the allowlist.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ..., is_trusted ]`
    #[cfg(feature = "nova")]
    Verify,
    /// **[Nova]** Returns indices of all untrusted strands.
    ///
    /// **Stack:** `[ ... ] -> [ ..., junction_of_indices ]`
    #[cfg(feature = "nova")]
    Audit,
    /// **[Nova]** Invokes a magical Sigil based on a spatial pattern.
    ///
    /// **Stack:** `[ ..., sigil_name ] -> [ ... ]`
    /// **Effect:** Checks grid for pattern and applies effect.
    #[cfg(feature = "nova")]
    Invoke,
    /// **[Nova]** Inscribes a new Sigil from the local environment.
    ///
    /// **Stack:** `[ ..., strand_idx, radius, sigil_name ] -> [ ... ]`
    /// **Effect:** Learns a new Sigil pattern and binds it to the strand.
    #[cfg(feature = "nova")]
    Inscribe,
    /// **[Nova]** Toggles the Auto-Cast (Passive) state of a Sigil.
    ///
    /// **Stack:** `[ ..., sigil_name, state ] -> [ ... ]`
    /// **State:** 1=On, 0=Off.
    #[cfg(feature = "nova")]
    AutoCast,
    /// **[Nova]** Inscribes a Ward (trap) on the current grid cell.
    ///
    /// **Stack:** `[ ..., persistence, strand_idx ] -> [ ... ]`
    /// **Effect:** Writes a hidden trap that triggers the strand when stepped on.
    #[cfg(feature = "nova")]
    Ward,
    /// **[Nova]** Reduces a Junction to a single value.
    ///
    /// **Stack:** `[ ..., junction, init, function ] -> [ ..., result ]`
    #[cfg(feature = "nova")]
    Fold,
    /// **[Nova]** Filters a Junction based on a predicate.
    ///
    /// **Stack:** `[ ..., junction, predicate ] -> [ ..., new_junction ]`
    #[cfg(feature = "nova")]
    Filter,
    /// **[Nova]** Combines two Junctions into one.
    ///
    /// **Stack:** `[ ..., junction_a, junction_b ] -> [ ..., zipped_junction ]`
    #[cfg(feature = "nova")]
    Zip,
    /// **[Nova]** Matches a string against a pattern (wildcards supported).
    ///
    /// **Stack:** `[ ..., pattern, target ] -> [ ..., is_match ]`
    #[cfg(feature = "nova")]
    Match,

    // IPC Features (Ether Link)
    /// **[Nova]** Sends a value to an external Ether channel.
    ///
    /// **Stack:** `[ ..., channel, value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Signal,
    /// **[Nova]** Receives a value from an external Ether channel.
    ///
    /// **Stack:** `[ ..., channel ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    Receive,

    // Bard Features (Music)
    /// **[Nova]** Adds a note to the musical score.
    ///
    /// **Stack:** `[ ..., velocity, duration, pitch ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Note,
    /// **[Nova]** Adds a rest to the musical score.
    ///
    /// **Stack:** `[ ..., duration ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Rest,
    /// **[Nova]** Sets the tempo of the composition.
    ///
    /// This instruction logs the tempo change as metadata for the score (e.g., for ABC export).
    /// It does **not** affect the VM's execution speed.
    ///
    /// **Stack:** `[ ..., bpm ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Tempo,
    /// **[Nova]** Exports the score as ABC Notation to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., abc_string ]`
    #[cfg(feature = "nova")]
    Perform,

    /// **[Nova]** Converts the musical score into a new DNA strand.
    ///
    /// **Stack:** `[ ... ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Maps notes to OpCodes and creates a new strand.
    #[cfg(feature = "nova")]
    Compose,

    // Resonance Features (Audio Physics)
    /// **[Resonance]** Plucks the underlying physics grid at the current location.
    ///
    /// **Stack:** `[ ..., strength ] -> [ ... ]`
    #[cfg(feature = "resonance")]
    Pluck,
    /// **[Resonance]** Continuously excites the grid at the current location with a sine wave.
    ///
    /// **Stack:** `[ ..., frequency, strength ] -> [ ... ]`
    /// **Effect:** Sets up an oscillator. If strength is 0, stops oscillation.
    #[cfg(feature = "resonance")]
    Oscillate,
    /// **[Resonance]** Reads the amplitude of the physics grid at the current location.
    ///
    /// **Stack:** `[ ... ] -> [ ..., amplitude ]`
    #[cfg(feature = "resonance")]
    Hear,
    /// **[Resonance]** Emits a loud, physical shockwave.
    ///
    /// **Stack:** `[ ..., duration, strength ] -> [ ... ]`
    #[cfg(feature = "resonance")]
    Scream,

    // Cymatics Features (Nova + Resonance)
    /// **[Cymatics]** Moves matter on the grid towards nodal points (low amplitude).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(all(feature = "nova", feature = "resonance"))]
    Sift,
    /// **[Cymatics]** Modifies walls (membranes) based on sound amplitude.
    ///
    /// **Stack:** `[ ..., threshold, mode ] -> [ ... ]`
    /// **Mode:** 0=Solidify (High Amp -> Wall), 1=Liquefy (High Amp -> No Wall).
    #[cfg(all(feature = "nova", feature = "resonance"))]
    Reshape,

    /// **[Cymatics]** Emits a resonant frequency and amplitude at the current location.
    ///
    /// **Stack:** `[ ..., frequency, amplitude ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Resonate,
    /// **[Cymatics]** Claims territory if the local resonance matches the strand's frequency.
    ///
    /// **Stack:** `[ ..., target_frequency ] -> [ ... ]`
    #[cfg(feature = "nova")]
    SonicClaim,
    /// **[Cymatics]** Reduces resonance amplitude in an area.
    ///
    /// **Stack:** `[ ..., radius, amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Dampen,
    /// **[Cymatics]** Registers a strand to trigger when a global frequency is dominant.
    ///
    /// **Stack:** `[ ..., frequency, strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    ListenFreq,

    // Oracle Features (Logic Engine)
    /// **[Oracle]** Adds a fact or rule to the Knowledge Base.
    ///
    /// **Stack:** `[ ..., fact ] -> [ ... ]`
    #[cfg(feature = "oracle")]
    Assert,
    /// **[Oracle]** Adds a rule to the Knowledge Base.
    ///
    /// **Stack:** `[ ..., rule_junction ] -> [ ... ]`
    #[cfg(feature = "oracle")]
    Rule,
    /// **[Oracle]** Removes a fact or rule from the Knowledge Base.
    ///
    /// **Stack:** `[ ..., fact ] -> [ ... ]`
    #[cfg(feature = "oracle")]
    Retract,
    /// **[Oracle]** Queries the Knowledge Base.
    ///
    /// **Stack:** `[ ..., query ] -> [ ..., success ]`
    /// **Effect:** Tries to prove the query. If successful, variables in the query structure may be unified.
    #[cfg(feature = "oracle")]
    Query,

    /// **[Oracle]** Finds all solutions to a query.
    ///
    /// **Stack:** `[ ..., template, goal ] -> [ ..., result_list ]`
    #[cfg(feature = "oracle")]
    FindAll,

    /// **[Oracle]** Registers an Omen (Trigger).
    ///
    /// **Stack:** `[ ..., condition, effect ] -> [ ... ]`
    #[cfg(feature = "oracle")]
    Augury,

    /// **[Oracle]** Checks all registered Omens.
    ///
    /// **Stack:** `[ ... ] -> [ ..., triggered_count ]`
    #[cfg(feature = "oracle")]
    Divinate,

    /// **[Oracle]** Searches for a strand satisfying a predicate and jumps to it.
    ///
    /// **Stack:** `[ ..., query ] -> [ ... ]`
    /// **Effect:** Jumps to the first matching strand.
    #[cfg(feature = "oracle")]
    Seek,

    /// **[Oracle]** Applies a transformation to all states matching a query.
    ///
    /// **Stack:** `[ ..., query_template, transform_template ] -> [ ... ]`
    #[cfg(feature = "oracle")]
    Manifest,

    /// **[Oracle]** Unifies two terms on the stack.
    ///
    /// **Stack:** `[ ..., term1, term2 ] -> [ ..., substitution_junction ]`
    /// **Effect:** Returns a list of variable bindings if successful, or 0 if failed.
    #[cfg(feature = "oracle")]
    Unify,

    /// **[Oracle]** Executes a Prolog-style query string.
    ///
    /// **Stack:** `[ ..., query_string ] -> [ ..., result_junction ]`
    /// **Effect:** Parses and runs a query, returning bindings.
    #[cfg(feature = "oracle")]
    PrologCall,

    // Git Features (Repository Interaction)
    /// **[Git]** Pushes a list of recent commit hashes to the stack.
    ///
    /// **Stack:** `[ ..., count ] -> [ ..., n, hash_1, ..., hash_n ]`
    #[cfg(feature = "git")]
    Ancestry,
    /// **[Git]** Reads the content of a file from a specific commit.
    ///
    /// **Stack:** `[ ..., hash_str, path_str ] -> [ ..., content_string ]`
    #[cfg(feature = "git")]
    Excavate,
    /// **[Git]** Gets the diff of a commit.
    ///
    /// **Stack:** `[ ..., hash_str ] -> [ ..., diff_string ]`
    #[cfg(feature = "git")]
    Evolution,

    // Phylogeny Features (Filesystem Genetics)
    /// **[Phylogeny]** Lists files in a directory.
    ///
    /// **Stack:** `[ ..., path_string ] -> [ ..., junction_of_filenames ]`
    #[cfg(feature = "phylogeny")]
    Crawl,
    /// **[Phylogeny]** Reads a file's content (Sequencing).
    ///
    /// **Stack:** `[ ..., path_string ] -> [ ..., content_string ]`
    #[cfg(feature = "phylogeny")]
    Sequencing,
    /// **[Phylogeny]** Writes content to a file (Synthesizing DNA).
    ///
    /// **Stack:** `[ ..., path_string, content_string ] -> [ ... ]`
    #[cfg(feature = "phylogeny")]
    Synthesize,
    /// **[Phylogeny]** Appends content to a file (Infection).
    ///
    /// **Stack:** `[ ..., path_string, content_string ] -> [ ... ]`
    #[cfg(feature = "phylogeny")]
    Infect,
    /// **[Phylogeny]** Executes a system command (Shell).
    ///
    /// **Stack:** `[ ..., command_string ] -> [ ..., output_string ]`
    #[cfg(feature = "phylogeny")]
    Shell,

    // Geology Features (Nova)
    /// **[Nova]** Randomly shifts rows or columns of the grid (Plate Tectonics).
    ///
    /// **Stack:** `[ ..., intensity ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Quake,
    /// **[Nova]** Reduces values in a circular area (Weathering).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Erode,
    /// **[Nova]** Increases values in a circular area (Deposition).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Sediment,
    /// **[Nova]** Shifts a rectangular block of the grid.
    ///
    /// **Stack:** `[ ..., dy, dx, h, w ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Tectonics,
    /// **[Nova]** Erupts high values at the current location.
    ///
    /// **Stack:** `[ ..., power ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Volcano,

    // Geomancy Features (Nova)
    /// **[Nova]** Detects the nearest Ley Node.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dy, dx, distance, power ]`
    #[cfg(feature = "nova")]
    LeySense,
    /// **[Nova]** Absorbs energy from a Ley Node.
    ///
    /// **Stack:** `[ ... ] -> [ ..., energy_gained ]`
    /// **Risk:** High power nodes can cause Overload damage.
    #[cfg(feature = "nova")]
    LeyTap,
    /// **[Nova]** Teleports to a connected Ley Node.
    ///
    /// **Stack:** `[ ..., target_node_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    LeyWarp,
    /// **[Nova]** Moves a Ley Node to a new location.
    ///
    /// **Stack:** `[ ..., dy, dx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    LeyShift,

    // Dimension Features (Nova)
    /// **[Nova]** Switches to a different Grid Dimension (Plane).
    ///
    /// **Stack:** `[ ..., dimension_id ] -> [ ... ]`
    /// **Effect:** Swaps the current grid with the target dimension's grid.
    #[cfg(feature = "nova")]
    Dimension,
    /// **[Nova]** Reads a value from a specific dimension.
    ///
    /// **Stack:** `[ ..., y, x, dimension_id ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    DRead,
    /// **[Nova]** Writes a value to a specific dimension.
    ///
    /// **Stack:** `[ ..., value, y, x, dimension_id ] -> [ ... ]`
    #[cfg(feature = "nova")]
    DWrite,
    /// **[Nova]** Merges a dimension into the current one.
    ///
    /// **Stack:** `[ ..., method, dimension_id ] -> [ ... ]`
    /// **Method:** 0=Add, 1=Max, 2=Overwrite.
    #[cfg(feature = "nova")]
    DMerge,
    /// **[Nova]** Pushes the current dimension ID to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., dimension_id ]`
    #[cfg(feature = "nova")]
    DView,

    // Hive Features (Networking)
    /// **[Hive]** Binds a UDP port for receiving messages.
    ///
    /// **Stack:** `[ ..., port ] -> [ ... ]`
    #[cfg(feature = "hive")]
    HiveBind,
    /// **[Hive]** Sends a message to a target IP and Port.
    ///
    /// **Stack:** `[ ..., value, ip_string, port ] -> [ ... ]`
    #[cfg(feature = "hive")]
    HiveSend,
    /// **[Hive]** Receives a message from a bound port (non-blocking).
    ///
    /// **Stack:** `[ ..., port ] -> [ ..., value ]`
    /// **Effect:** Pushes received value or 0 if no message.
    #[cfg(feature = "hive")]
    HiveRecv,
    /// **[Hive]** Closes a bound UDP port.
    ///
    /// **Stack:** `[ ..., port ] -> [ ... ]`
    #[cfg(feature = "hive")]
    HiveClose,

    // Paleontology Features (Nova)
    /// **[Nova]** Fossilizes a strand into a compressed string on the grid.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    /// **Effect:** Writes `Fossil:{tick}:{hash}:{dna}` to the grid.
    #[cfg(feature = "nova")]
    Fossilize,
    /// **[Nova]** Unearths a fossil from the grid, restoring the DNA strand.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., new_strand_idx ]`
    #[cfg(feature = "nova")]
    Unearth,
    /// **[Nova]** Carbon dates a fossil to determine its age.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., age ]`
    #[cfg(feature = "nova")]
    CarbonDate,

    // Crystallography Features (Nova)
    /// **[Nova]** Turns the current cell into a crystal seed.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Nucleate,
    /// **[Nova]** Grows the crystal by absorbing value from neighbors.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Accrete,
    /// **[Nova]** Explodes the crystal, scattering value to neighbors.
    ///
    /// **Stack:** `[ ..., force ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Shatter,
    /// **[Nova]** Sorts the values in a local window (Annealing).
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Anneal,

    // Cartography Features (Nova)
    /// **[Nova]** Scans a circular area and returns a Junction of values.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., junction_of_values ]`
    #[cfg(feature = "nova")]
    Scan,
    /// **[Nova]** Pushes the current coordinates to the stack.
    ///
    /// **Stack:** `[ ... ] -> [ ..., y, x ]`
    #[cfg(feature = "nova")]
    Locate,
    /// **[Nova]** Writes a value to the persistent Cartography Map.
    ///
    /// **Stack:** `[ ..., value, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Chart,
    /// **[Nova]** Reads a value from the persistent Cartography Map.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    Atlas,

    /// **[Nova]** Configures the Logistics Layer (Factory Automation).
    ///
    /// **Stack:** `[ ..., type, direction, y, x ] -> [ ... ]`
    /// **Type:** 0=Clear, 1=Belt, 2=Sorter.
    /// **Direction:** 0=N, 1=E, 2=S, 3=W.
    #[cfg(feature = "nova")]
    Logistics,

    /// **[Nova]** Compresses a grid area into a value on the stack.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ..., pocket_val ]`
    /// **Effect:** Clears the area.
    #[cfg(feature = "nova")]
    Pocket,
    /// **[Nova]** Decompresses a pocket value onto the grid.
    ///
    /// **Stack:** `[ ..., pocket_val ] -> [ ... ]`
    /// **Effect:** Overwrites the area.
    #[cfg(feature = "nova")]
    Unpocket,

    // Sovereignty Features (Territory)
    /// **[Nova]** Claims ownership of grid cells within a radius.
    ///
    /// **Stack:** `[ ..., radius ] -> [ ... ]`
    /// **Cost:** 10 Energy per cell.
    #[cfg(feature = "nova")]
    Claim,
    /// **[Nova]** Renounces ownership of a grid cell.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Cede,
    /// **[Nova]** Checks the owner of a grid cell.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., owner_id (-1 if none) ]`
    #[cfg(feature = "nova")]
    Sovereignty,
    /// **[Nova]** Sets the tax rate for the current strand's territory.
    ///
    /// **Stack:** `[ ..., rate ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Tax,

    // Linguistics Features (Nova)
    /// **[Nova]** Calculates Levenshtein edit distance between two strings.
    ///
    /// **Stack:** `[ ..., s1, s2 ] -> [ ..., distance ]`
    #[cfg(feature = "nova")]
    Levenshtein,
    /// **[Nova]** Calculates Soundex phonetic hash.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., code ]`
    #[cfg(feature = "nova")]
    Soundex,
    /// **[Nova]** Checks if two strings are anagrams.
    ///
    /// **Stack:** `[ ..., s1, s2 ] -> [ ..., is_anagram ]`
    #[cfg(feature = "nova")]
    Anagram,
    /// **[Nova]** Applies Caesar cipher shift.
    ///
    /// **Stack:** `[ ..., shift, string ] -> [ ..., shifted_string ]`
    #[cfg(feature = "nova")]
    Cipher,
    /// **[Nova]** Checks if string is a pangram.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., is_pangram ]`
    #[cfg(feature = "nova")]
    Pangram,

    // Babel Features (Metalinguistics - Nova)
    /// **[Babel]** Constructs a parser object on the stack.
    ///
    /// **Stack:** `[ ..., type_str, ...args ] -> [ ..., parser_junction ]`
    /// Types: "Match", "Seq", "Alt", "Many", "Opt".
    #[cfg(feature = "nova")]
    Grammar,
    /// **[Babel]** Parses a string using a parser object.
    ///
    /// **Stack:** `[ ..., parser_junction, input_string ] -> [ ..., result_ast ]`
    #[cfg(feature = "nova")]
    Parse,
    /// **[Babel]** Creates a literal string matcher.
    ///
    /// **Stack:** `[ ..., pattern_string ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserMatch,
    /// **[Babel]** Creates a regex matcher.
    ///
    /// **Stack:** `[ ..., regex_pattern ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserRegex,
    /// **[Babel]** Creates a sequence parser (P1 then P2).
    ///
    /// **Stack:** `[ ..., p1, p2 ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserSeq,
    /// **[Babel]** Creates an alternative parser (P1 or P2).
    ///
    /// **Stack:** `[ ..., p1, p2 ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserAlt,
    /// **[Babel]** Creates a repetition parser (0 or more).
    ///
    /// **Stack:** `[ ..., p ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserMany,
    /// **[Babel]** Creates an optional parser.
    ///
    /// **Stack:** `[ ..., p ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserOpt,
    /// **[Babel]** Creates a variadic sequence parser.
    ///
    /// **Stack:** `[ ..., p1, p2, ..., pn, count ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserSeqN,
    /// **[Babel]** Creates a variadic alternative parser.
    ///
    /// **Stack:** `[ ..., p1, p2, ..., pn, count ] -> [ ..., parser_junction ]`
    #[cfg(feature = "nova")]
    ParserAltN,
    /// **[Babel]** "Tongue of Madness": Parses input, mutates CST, regenerates string.
    ///
    /// **Stack:** `[ ..., grammar, input_string ] -> [ ..., corrupted_string ]`
    #[cfg(feature = "nova")]
    Tongue,
    /// **[Babel]** Generates a string from a Grammar.
    ///
    /// **Stack:** `[ ..., grammar_junction ] -> [ ..., generated_string ]`
    #[cfg(feature = "nova")]
    Generate,
    /// **[Babel]** Writes a string to the Tablet (Output Buffer).
    ///
    /// **Stack:** `[ ..., string ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Scribe,

    /// **[Babel]** Compiles a Concrete Syntax Tree (CST) into a new Strand.
    ///
    /// **Stack:** `[ ..., cst, handler_strand_idx ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Creates a strand that recreates the CST traversal, calling handler for each node.
    #[cfg(feature = "nova")]
    BabelCompile,

    /// **[Babel]** Increases linguistic chaos (Glossolalia).
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Glossolalia,

    /// **[Babel]** Reads a Grammar definition from the grid visually.
    ///
    /// **Stack:** `[ ..., y, x ] -> [ ..., grammar_junction ]`
    #[cfg(feature = "nova")]
    GridGrammar,
    /// **[Babel]** Restores linguistic integrity.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Clarify,
    /// **[Babel]** Randomly reshuffles language mappings.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Confuse,

    /// **[Babel]** Runs a Live Parser on the grid.
    ///
    /// **Stack:** `[ ..., y, x, input_string ] -> [ ..., success_bool ]`
    #[cfg(feature = "nova")]
    BabelLive,

    /// **[Babel]** Rewrites the active grid-perception grammar.
    ///
    /// **Stack:** `[ ..., grammar_junction ] -> [ ... ]`
    /// **Effect:** Sets the VM's active grammar for Perception.
    #[cfg(feature = "nova")]
    SelfRewrite,

    /// **[Babel]** Perceives the grid as code using the active grammar.
    ///
    /// **Stack:** `[ ..., len ] -> [ ..., success_bool ]`
    /// **Effect:** Reads `len` cells from grid at current location, parses with Active Grammar, compiles, and calls the result.
    #[cfg(feature = "nova")]
    Perceive,

    // Ballistics Features (Nova)
    /// **[Nova]** Fires a projectile with velocity and power.
    ///
    /// **Stack:** `[ ..., power, dy, dx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Fire,
    /// **[Nova]** Fires multiple projectiles in random directions.
    ///
    /// **Stack:** `[ ..., power, count ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Salvo,

    // Optics Features (Nova)
    /// **[Nova]** Creates a reflective surface on the grid.
    ///
    /// **Stack:** `[ ..., orientation, y, x ] -> [ ... ]`
    /// **Orientation:** 0=|, 1=-, 2=/, 3=\.
    #[cfg(feature = "nova")]
    Reflector,
    /// **[Nova]** Creates a prism that splits projectiles.
    ///
    /// **Stack:** `[ ..., orientation, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Prism,
    /// **[Nova]** Creates a lens that modifies projectile properties.
    ///
    /// **Stack:** `[ ..., power, y, x ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Lens,

    // Sociology Features (Nova)
    /// **[Nova]** Manages Guild membership and treasury.
    ///
    /// **Stack:** `[ ..., action, guild_name, amount/arg ] -> [ ... ]`
    /// **Actions:** "Join", "Leave", "Deposit", "Withdraw", "Create".
    #[cfg(feature = "nova")]
    Guild,
    /// **[Nova]** Manages Guild policies (for founders/leaders).
    ///
    /// **Stack:** `[ ..., action, arg, guild_name ] -> [ ... ]`
    /// **Actions:** "Tax", "Kick", "Invite", "Motto".
    #[cfg(feature = "nova")]
    Charter,

    // Bureaucracy Features (Nova)
    /// **[Nova]** Increases local bureaucracy (Red Tape), making actions cost more energy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    RedTape,
    /// **[Nova]** Files a Form to reduce local bureaucracy.
    ///
    /// **Stack:** `[ ..., amount ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Form,
    /// **[Nova]** Signs a Form on the stack, converting it to a Permit.
    ///
    /// **Stack:** `[ ..., form_str ] -> [ ..., permit_str ]`
    #[cfg(feature = "nova")]
    Sign,
    /// **[Nova]** Uses a Permit to gain temporary immunity to Red Tape.
    ///
    /// **Stack:** `[ ..., permit_str ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Permit,

    /// **[Nova]** Draws a Fate Card (Arcana) from the deck.
    ///
    /// **Stack:** `[ ... ] -> [ ..., card_id ]`
    #[cfg(feature = "nova")]
    Draw,
    /// **[Nova]** Checks the currently active Fate Card.
    ///
    /// **Stack:** `[ ... ] -> [ ..., card_id ]`
    #[cfg(feature = "nova")]
    Fate,
    /// **[Nova]** Shuffles the Fate Deck.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Shuffle,

    // Quipu Features (Topological Memory)
    /// **[Nova]** Ties a knot with a value on the current Quipu Cord.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Knot,
    /// **[Nova]** Unties the last knot cluster from the current Quipu Cord.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    Unknot,
    /// **[Nova]** Selects the active Quipu Cord.
    ///
    /// **Stack:** `[ ..., cord_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Cord,
    /// **[Nova]** Reads the value of the current Quipu Cord.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    ReadCord,
    /// **[Nova]** Entangles (adds) the value of another cord to the current one.
    ///
    /// **Stack:** `[ ..., other_cord_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Tangle,

    // Metazoa Features (Multicellularity)
    /// **[Nova]** Bonds with a neighbor to form a Tissue.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ..., tissue_id ]`
    #[cfg(feature = "nova")]
    Bond,
    /// **[Nova]** Severs the bond with a neighbor.
    ///
    /// **Stack:** `[ ..., direction ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Unbond,
    /// **[Nova]** Sends a signal to the entire Tissue.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Signify,
    /// **[Nova]** Pushes the current Tissue ID.
    ///
    /// **Stack:** `[ ... ] -> [ ..., tissue_id ]`
    #[cfg(feature = "nova")]
    Tissue,

    // Weaving Features (The Loom)
    /// **[Nova]** Weaves two strands together based on a pattern.
    ///
    /// **Stack:** `[ ..., strand_a, strand_b, pattern_strand ] -> [ ..., new_strand_idx ]`
    /// **Pattern:** 'A'=Take from A, 'B'=Take from B, 'X'=Random, '0'=Skip.
    #[cfg(feature = "nova")]
    Weave,
    /// **[Nova]** Unravels a strand, destroying it and reclaiming resources.
    ///
    /// **Stack:** `[ ..., strand_idx ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Unravel,

    // BioMesh Features (Fungal Cybernetics)
    /// **[Nova]** Transforms the current cell into a Mesh Node.
    ///
    /// **Stack:** `[ ..., id ] -> [ ... ]`
    #[cfg(feature = "nova")]
    MeshNet,
    /// **[Nova]** Sends a packet to a target Node ID.
    ///
    /// **Stack:** `[ ..., target_id, value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    MeshSend,
    /// **[Nova]** Receives a packet from the local Node buffer.
    ///
    /// **Stack:** `[ ... ] -> [ ..., value ]`
    #[cfg(feature = "nova")]
    MeshRecv,
    /// **[Nova]** Connects the current Node to adjacent Nodes.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    MeshGrow,
    /// **[Nova]** Disconnects the current Node from all neighbors.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    MeshPrune,

    // Reactor Features (Logic Automata)
    /// **[Nova]** Toggles the Reactor (Logic Cellular Automata) mode.
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Reactor,
    /// **[Nova]** Registers a reaction rule.
    ///
    /// **Stack:** `[ ..., input_a, input_b, output ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Reaction,

    // Scavenger Features (Mad Science)
    /// **[Scavenge]** Reads raw bytes from a file and converts them into DNA.
    ///
    /// **Stack:** `[ ..., path_string, len ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Reads `len` bytes from `path`, maps each byte to an OpCode, creates a new strand.
    #[cfg(feature = "nova")]
    Scavenge,
    /// **[Scavenge]** Reads raw bytes from the running executable itself (Self-Cannibalism).
    ///
    /// **Stack:** `[ ..., offset, len ] -> [ ..., new_strand_idx ]`
    /// **Effect:** Reads `len` bytes from the binary at `offset`, maps to OpCodes, creates a new strand.
    #[cfg(feature = "nova")]
    Digest,

    // Semiotics Features (Meaning Mutation)
    /// **[Nova]** Converts a value into an abstract Symbol.
    ///
    /// **Stack:** `[ ..., value ] -> [ ..., symbol ]`
    #[cfg(feature = "nova")]
    Symbolize,
    /// **[Nova]** Resolves a Symbol to a value based on the current context.
    ///
    /// **Stack:** `[ ..., symbol ] -> [ ..., resolved_value ]`
    #[cfg(feature = "nova")]
    Interpret,
    /// **[Nova]** Shifts the semiotic context by XORing with a value.
    ///
    /// **Stack:** `[ ..., value ] -> [ ... ]`
    #[cfg(feature = "nova")]
    ContextShift,
    /// **[Nova]** Deconstructs a string into a junction of constituent Symbols.
    ///
    /// **Stack:** `[ ..., string ] -> [ ..., junction_of_symbols ]`
    #[cfg(feature = "nova")]
    Deconstruct,

    // Fractal Features (Chaos Visualization)
    /// **[Nova]** Sets the Fractal Mode to Mandelbrot.
    ///
    /// **Stack:** `[ ..., max_iterations ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Mandelbrot,
    /// **[Nova]** Sets the Fractal Mode to Julia Set with constant c.
    ///
    /// **Stack:** `[ ..., c_re, c_im ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Julia,
    /// **[Nova]** Adjusts the Fractal Zoom level.
    ///
    /// **Stack:** `[ ..., factor ] -> [ ... ]`
    /// **Effect:** Multiplies current zoom by factor.
    #[cfg(feature = "nova")]
    Zoom,
    /// **[Nova]** Pans the Fractal View center.
    ///
    /// **Stack:** `[ ..., dx, dy ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Pan,
    /// **[Nova]** performs one iteration of z = z^2 + c.
    ///
    /// **Stack:** `[ ..., z_re, z_im, c_re, c_im ] -> [ ..., new_z_re, new_z_im ]`
    #[cfg(feature = "nova")]
    Iterate,
    /// **[Nova]** Computes escape time for a point.
    ///
    /// **Stack:** `[ ..., c_re, c_im, max_iter ] -> [ ..., escape_val ]`
    #[cfg(feature = "nova")]
    Escape,

    /// No Operation. Does nothing.
    Nop,

    /// Unknown or invalid instruction.
    #[strum(default)]
    Unknown(String),
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

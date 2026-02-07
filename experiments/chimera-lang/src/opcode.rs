use serde::{Deserialize, Serialize};
use std::fmt;
use strum_macros::{AsRefStr, EnumString};

/// Instructions for the Chimera Virtual Machine.
///
/// Each opcode represents a fundamental action that the organism can perform,
/// ranging from basic arithmetic to genetic engineering and inter-dimensional travel.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, EnumString, AsRefStr)]
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

    /// **[Nova]** Predicts if the current execution path leads to death within `ticks`.
    ///
    /// **Stack:** `[ ..., ticks ] -> [ ..., 1(Death)|0(Life) ]`
    #[cfg(feature = "nova")]
    Prophecy,

    /// **[Nova]** Transmutes the current grid cell based on neighbors (Alchemy).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Alchemy,

    /// **[Nova]** Spreads the last executed instruction to a random strand (Memetics).
    ///
    /// **Stack:** `[ ... ] -> [ ... ]`
    #[cfg(feature = "nova")]
    Meme,

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

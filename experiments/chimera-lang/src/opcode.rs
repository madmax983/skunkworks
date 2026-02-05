use std::fmt;
use std::str::FromStr;

/// Instructions for the Chimera Virtual Machine.
///
/// Each opcode represents a fundamental action that the organism can perform,
/// ranging from basic arithmetic to genetic engineering and inter-dimensional travel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    // Nova Features
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

    /// Unknown or invalid instruction.
    Unknown(String),
}

impl FromStr for OpCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "nova")]
            "remap" => Ok(OpCode::Remap),
            #[cfg(feature = "nova")]
            "restore" => Ok(OpCode::Restore),
            #[cfg(feature = "nova")]
            "mirror" => Ok(OpCode::Mirror),
            #[cfg(feature = "nova")]
            "eval" => Ok(OpCode::Eval),
            #[cfg(feature = "nova")]
            "map" => Ok(OpCode::Map),
            #[cfg(feature = "nova")]
            "fold" => Ok(OpCode::Fold),
            #[cfg(feature = "nova")]
            "filter" => Ok(OpCode::Filter),
            #[cfg(feature = "nova")]
            "zip" => Ok(OpCode::Zip),

            "push" => Ok(OpCode::Push),
            "add" => Ok(OpCode::Add),
            "sub" => Ok(OpCode::Sub),
            "mul" => Ok(OpCode::Mul),
            "div" => Ok(OpCode::Div),
            "dup" => Ok(OpCode::Dup),
            "swap" => Ok(OpCode::Swap),
            "drop" => Ok(OpCode::Drop),
            "print" => Ok(OpCode::Print),
            "jump" => Ok(OpCode::Jump),
            "brz" => Ok(OpCode::Brz),
            "photosynthesize" => Ok(OpCode::Photosynthesize),
            "consume" => Ok(OpCode::Consume),
            "g_read" => Ok(OpCode::GRead),
            "g_write" => Ok(OpCode::GWrite),
            "radiate" => Ok(OpCode::Radiate),
            "siphon" => Ok(OpCode::Siphon),
            "genome" => Ok(OpCode::Genome),
            "virus" => Ok(OpCode::Virus),
            "transcribe" => Ok(OpCode::Transcribe),
            "jump_s" => Ok(OpCode::JumpS),
            "brz_s" => Ok(OpCode::BrzS),
            "s_len" => Ok(OpCode::SLen),
            "helix_len" => Ok(OpCode::HelixLen),
            "gene_len" => Ok(OpCode::GeneLen),

            #[cfg(feature = "cortex")]
            "link" => Ok(OpCode::Link),
            #[cfg(feature = "cortex")]
            "sever" => Ok(OpCode::Sever),
            #[cfg(feature = "cortex")]
            "spark" => Ok(OpCode::Spark),
            #[cfg(feature = "cortex")]
            "sense" => Ok(OpCode::Sense),
            #[cfg(feature = "cortex")]
            "gate" => Ok(OpCode::Gate),

            #[cfg(feature = "nova")]
            "sporulate" => Ok(OpCode::Sporulate),
            #[cfg(feature = "nova")]
            "germinate" => Ok(OpCode::Germinate),
            #[cfg(feature = "nova")]
            "incubate" => Ok(OpCode::Incubate),
            #[cfg(feature = "nova")]
            "methylate" => Ok(OpCode::Methylate),
            #[cfg(feature = "nova")]
            "demethylate" => Ok(OpCode::Demethylate),
            #[cfg(feature = "nova")]
            "telomerase" => Ok(OpCode::Telomerase),
            #[cfg(feature = "nova")]
            "t_len" => Ok(OpCode::TLen),
            #[cfg(feature = "nova")]
            "recombine" => Ok(OpCode::Recombine),
            #[cfg(feature = "nova")]
            "s_index" => Ok(OpCode::SIndex),
            #[cfg(feature = "nova")]
            "crispr_scan" => Ok(OpCode::CrisprScan),
            #[cfg(feature = "nova")]
            "cas9_cut" => Ok(OpCode::Cas9Cut),
            #[cfg(feature = "nova")]
            "ligase" => Ok(OpCode::Ligase),
            #[cfg(feature = "nova")]
            "mitosis" => Ok(OpCode::Mitosis),
            #[cfg(feature = "nova")]
            "apoptosis" => Ok(OpCode::Apoptosis),
            #[cfg(feature = "nova")]
            "integrase" => Ok(OpCode::Integrase),
            #[cfg(feature = "nova")]
            "excision" => Ok(OpCode::Excision),
            #[cfg(feature = "nova")]
            "secrete" => Ok(OpCode::Secrete),
            #[cfg(feature = "nova")]
            "detect" => Ok(OpCode::Detect),
            #[cfg(feature = "nova")]
            "absorb" => Ok(OpCode::Absorb),
            #[cfg(feature = "nova")]
            "migrate" => Ok(OpCode::Migrate),
            #[cfg(feature = "nova")]
            "detox" => Ok(OpCode::Detox),
            #[cfg(feature = "nova")]
            "w_read" => Ok(OpCode::WRead),
            #[cfg(feature = "nova")]
            "call" => Ok(OpCode::Call),
            #[cfg(feature = "nova")]
            "ret" => Ok(OpCode::Ret),
            #[cfg(feature = "nova")]
            "bind" => Ok(OpCode::Bind),
            #[cfg(feature = "nova")]
            "unbind" => Ok(OpCode::Unbind),
            #[cfg(feature = "nova")]
            "reflex" => Ok(OpCode::Reflex),
            #[cfg(feature = "nova")]
            "entangle" => Ok(OpCode::Entangle),
            #[cfg(feature = "nova")]
            "decohere" => Ok(OpCode::Decohere),
            #[cfg(feature = "nova")]
            "conjugate" => Ok(OpCode::Conjugate),
            #[cfg(feature = "nova")]
            "gravitate" => Ok(OpCode::Gravitate),
            #[cfg(feature = "nova")]
            "lumine" => Ok(OpCode::Lumine),
            #[cfg(feature = "nova")]
            "sense_light" => Ok(OpCode::SenseLight),
            #[cfg(feature = "nova")]
            "spawn" => Ok(OpCode::Spawn),
            #[cfg(feature = "nova")]
            "simulate" => Ok(OpCode::Simulate),
            #[cfg(feature = "nova")]
            "dream" => Ok(OpCode::Dream),
            #[cfg(feature = "nova")]
            "chemotaxis" => Ok(OpCode::Chemotaxis),
            #[cfg(feature = "nova")]
            "identity" => Ok(OpCode::Identity),
            #[cfg(feature = "nova")]
            "differentiate" => Ok(OpCode::Differentiate),
            #[cfg(feature = "nova")]
            "sonar" => Ok(OpCode::Sonar),
            #[cfg(feature = "nova")]
            "rift" => Ok(OpCode::Rift),
            #[cfg(feature = "nova")]
            "seal" => Ok(OpCode::Seal),
            #[cfg(feature = "nova")]
            "shape" => Ok(OpCode::Shape),
            #[cfg(feature = "nova")]
            "broadcast" => Ok(OpCode::Broadcast),
            #[cfg(feature = "nova")]
            "tune" => Ok(OpCode::Tune),
            #[cfg(feature = "nova")]
            "membrane" => Ok(OpCode::Membrane),
            #[cfg(feature = "nova")]
            "osmosis" => Ok(OpCode::Osmosis),
            #[cfg(feature = "nova")]
            "symbiosis" => Ok(OpCode::Symbiosis),
            #[cfg(feature = "nova")]
            "lysis" => Ok(OpCode::Lysis),
            #[cfg(feature = "nova")]
            "compile" => Ok(OpCode::Compile),
            #[cfg(feature = "nova")]
            "irradiate" => Ok(OpCode::Irradiate),
            #[cfg(feature = "nova")]
            "sense_mutagen" => Ok(OpCode::SenseMutagen),
            #[cfg(feature = "nova")]
            "devour" => Ok(OpCode::Devour),
            #[cfg(feature = "nova")]
            "decompile" => Ok(OpCode::Decompile),

            _ => Ok(OpCode::Unknown(s.to_string())),
        }
    }
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::Push => write!(f, "push"),
            OpCode::Add => write!(f, "add"),
            OpCode::Sub => write!(f, "sub"),
            OpCode::Mul => write!(f, "mul"),
            OpCode::Div => write!(f, "div"),
            OpCode::Dup => write!(f, "dup"),
            OpCode::Swap => write!(f, "swap"),
            OpCode::Drop => write!(f, "drop"),
            OpCode::Print => write!(f, "print"),
            OpCode::Jump => write!(f, "jump"),
            OpCode::Brz => write!(f, "brz"),
            OpCode::Photosynthesize => write!(f, "photosynthesize"),
            OpCode::Consume => write!(f, "consume"),
            OpCode::GRead => write!(f, "g_read"),
            OpCode::GWrite => write!(f, "g_write"),
            OpCode::Radiate => write!(f, "radiate"),
            OpCode::Siphon => write!(f, "siphon"),
            OpCode::Genome => write!(f, "genome"),
            OpCode::Virus => write!(f, "virus"),
            OpCode::Transcribe => write!(f, "transcribe"),
            OpCode::JumpS => write!(f, "jump_s"),
            OpCode::BrzS => write!(f, "brz_s"),
            OpCode::SLen => write!(f, "s_len"),
            OpCode::HelixLen => write!(f, "helix_len"),
            OpCode::GeneLen => write!(f, "gene_len"),

            #[cfg(feature = "cortex")]
            OpCode::Link => write!(f, "link"),
            #[cfg(feature = "cortex")]
            OpCode::Sever => write!(f, "sever"),
            #[cfg(feature = "cortex")]
            OpCode::Spark => write!(f, "spark"),
            #[cfg(feature = "cortex")]
            OpCode::Sense => write!(f, "sense"),
            #[cfg(feature = "cortex")]
            OpCode::Gate => write!(f, "gate"),

            #[cfg(feature = "nova")]
            OpCode::Sporulate => write!(f, "sporulate"),
            #[cfg(feature = "nova")]
            OpCode::Germinate => write!(f, "germinate"),
            #[cfg(feature = "nova")]
            OpCode::Incubate => write!(f, "incubate"),
            #[cfg(feature = "nova")]
            OpCode::Methylate => write!(f, "methylate"),
            #[cfg(feature = "nova")]
            OpCode::Demethylate => write!(f, "demethylate"),
            #[cfg(feature = "nova")]
            OpCode::Telomerase => write!(f, "telomerase"),
            #[cfg(feature = "nova")]
            OpCode::TLen => write!(f, "t_len"),
            #[cfg(feature = "nova")]
            OpCode::Recombine => write!(f, "recombine"),
            #[cfg(feature = "nova")]
            OpCode::SIndex => write!(f, "s_index"),
            #[cfg(feature = "nova")]
            OpCode::CrisprScan => write!(f, "crispr_scan"),
            #[cfg(feature = "nova")]
            OpCode::Cas9Cut => write!(f, "cas9_cut"),
            #[cfg(feature = "nova")]
            OpCode::Ligase => write!(f, "ligase"),
            #[cfg(feature = "nova")]
            OpCode::Mitosis => write!(f, "mitosis"),
            #[cfg(feature = "nova")]
            OpCode::Apoptosis => write!(f, "apoptosis"),
            #[cfg(feature = "nova")]
            OpCode::Integrase => write!(f, "integrase"),
            #[cfg(feature = "nova")]
            OpCode::Excision => write!(f, "excision"),
            #[cfg(feature = "nova")]
            OpCode::Secrete => write!(f, "secrete"),
            #[cfg(feature = "nova")]
            OpCode::Detect => write!(f, "detect"),
            #[cfg(feature = "nova")]
            OpCode::Absorb => write!(f, "absorb"),
            #[cfg(feature = "nova")]
            OpCode::Migrate => write!(f, "migrate"),
            #[cfg(feature = "nova")]
            OpCode::Detox => write!(f, "detox"),
            #[cfg(feature = "nova")]
            OpCode::WRead => write!(f, "w_read"),
            #[cfg(feature = "nova")]
            OpCode::Call => write!(f, "call"),
            #[cfg(feature = "nova")]
            OpCode::Ret => write!(f, "ret"),
            #[cfg(feature = "nova")]
            OpCode::Bind => write!(f, "bind"),
            #[cfg(feature = "nova")]
            OpCode::Unbind => write!(f, "unbind"),
            #[cfg(feature = "nova")]
            OpCode::Reflex => write!(f, "reflex"),
            #[cfg(feature = "nova")]
            OpCode::Entangle => write!(f, "entangle"),
            #[cfg(feature = "nova")]
            OpCode::Decohere => write!(f, "decohere"),
            #[cfg(feature = "nova")]
            OpCode::Conjugate => write!(f, "conjugate"),
            #[cfg(feature = "nova")]
            OpCode::Gravitate => write!(f, "gravitate"),
            #[cfg(feature = "nova")]
            OpCode::Lumine => write!(f, "lumine"),
            #[cfg(feature = "nova")]
            OpCode::SenseLight => write!(f, "sense_light"),
            #[cfg(feature = "nova")]
            OpCode::Spawn => write!(f, "spawn"),
            #[cfg(feature = "nova")]
            OpCode::Simulate => write!(f, "simulate"),
            #[cfg(feature = "nova")]
            OpCode::Dream => write!(f, "dream"),
            #[cfg(feature = "nova")]
            OpCode::Chemotaxis => write!(f, "chemotaxis"),
            #[cfg(feature = "nova")]
            OpCode::Identity => write!(f, "identity"),
            #[cfg(feature = "nova")]
            OpCode::Differentiate => write!(f, "differentiate"),
            #[cfg(feature = "nova")]
            OpCode::Sonar => write!(f, "sonar"),
            #[cfg(feature = "nova")]
            OpCode::Rift => write!(f, "rift"),
            #[cfg(feature = "nova")]
            OpCode::Seal => write!(f, "seal"),
            #[cfg(feature = "nova")]
            OpCode::Shape => write!(f, "shape"),
            #[cfg(feature = "nova")]
            OpCode::Broadcast => write!(f, "broadcast"),
            #[cfg(feature = "nova")]
            OpCode::Tune => write!(f, "tune"),
            #[cfg(feature = "nova")]
            OpCode::Membrane => write!(f, "membrane"),
            #[cfg(feature = "nova")]
            OpCode::Osmosis => write!(f, "osmosis"),
            #[cfg(feature = "nova")]
            OpCode::Symbiosis => write!(f, "symbiosis"),
            #[cfg(feature = "nova")]
            OpCode::Lysis => write!(f, "lysis"),
            #[cfg(feature = "nova")]
            OpCode::Compile => write!(f, "compile"),
            #[cfg(feature = "nova")]
            OpCode::Decompile => write!(f, "decompile"),
            #[cfg(feature = "nova")]
            OpCode::Irradiate => write!(f, "irradiate"),
            #[cfg(feature = "nova")]
            OpCode::SenseMutagen => write!(f, "sense_mutagen"),
            #[cfg(feature = "nova")]
            OpCode::Devour => write!(f, "devour"),
            #[cfg(feature = "nova")]
            OpCode::Remap => write!(f, "remap"),
            #[cfg(feature = "nova")]
            OpCode::Restore => write!(f, "restore"),
            #[cfg(feature = "nova")]
            OpCode::Mirror => write!(f, "mirror"),
            #[cfg(feature = "nova")]
            OpCode::Eval => write!(f, "eval"),
            #[cfg(feature = "nova")]
            OpCode::Map => write!(f, "map"),
            #[cfg(feature = "nova")]
            OpCode::Fold => write!(f, "fold"),
            #[cfg(feature = "nova")]
            OpCode::Filter => write!(f, "filter"),
            #[cfg(feature = "nova")]
            OpCode::Zip => write!(f, "zip"),

            OpCode::Unknown(s) => write!(f, "{}", s),
        }
    }
}

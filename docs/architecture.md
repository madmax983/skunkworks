# System Architecture

## System Context (C4)

The **Skunkworks** repository is an experimental sandbox for creative coding projects in Rust.

```mermaid
C4Context
    title System Context diagram for Skunkworks

    Person(user, "Developer / Artist", "Explores creative coding experiments.")
    System(skunkworks, "Skunkworks Sandbox", "Collection of Rust experiments (Git Rhythm, Literary Boids, etc.)")
    System_Ext(git_repo, "Target Git Repository", "Any local git repository to be analyzed.")

    Rel(user, skunkworks, "Runs & Modifies", "Cargo CLI")
    Rel(skunkworks, git_repo, "Reads History", "libgit2")
```

## Shared Infrastructure

Common components used across multiple experiments to enforce consistency and reduce boilerplate.

### TUI Lifecycle (crates/tui-shared)

The `tui-shared` crate provides a RAII wrapper for Ratatui terminal initialization.

```mermaid
classDiagram
    direction LR
    class Tui {
        +Terminal terminal
        +init() Result~Self~
        +exit() Result
        +drop()
    }

    class NeuroTerminal

    NeuroTerminal ..> Tui : Uses

    note for Tui "Handles raw mode, alternate screen,\nand mouse capture automatically."
```

### Semantic Bridge (crates/tui-semantic)

The `tui-semantic` crate enables applications to expose their internal state as structured data for LLM agents.

```mermaid
classDiagram
    direction LR

    class Snapshot {
        +String app
        +u64 frame
        +Vec~Entity~ entities
        +Vec~Region~ regions
        +Vec~Action~ actions
        +to_json() String
    }

    class Entity {
        +String kind
        +String id
        +Vec2 position
        +HashMap~String, PropValue~ props
    }

    class Action {
        +String name
        +String description
        +String key
    }

    Snapshot *-- Entity : Contains
    Snapshot *-- Action : Contains
```

### Poincaré Disk (crates/poincare-disk)

The `poincare-disk` crate provides hyperbolic geometry primitives, shared by `hyperbolic-git`, `chimera-lang`, and others.

```mermaid
classDiagram
    direction LR

    class Point {
        <<Type Alias>>
        Complex~f64~
    }

    class Mobius {
        +Complex~f64~ a
        +Complex~f64~ b
        +Complex~f64~ c
        +Complex~f64~ d
        +apply(z: Point) Point
        +then(other: Mobius) Mobius
    }

    class TilingConsts {
        +f64 neighbor_offset
        +f64 vertex_offset
        +new_4_5() TilingConsts
    }

    Mobius ..> Point : Transforms
    TilingConsts ..> Point : Generates Offsets
```

### Resonance Audio (crates/resonance-audio)

The `resonance-audio` crate implements a 2D wave equation solver and audio state management, decoupled from any specific visualization.

```mermaid
classDiagram
    direction TB

    class AudioModel {
        +PhysicsGrid grid
        +HashMap oscillators
        +process(output: &mut [f32])
    }

    class PhysicsGrid {
        +Vec~f32~ u
        +step()
        +pluck(x, y, strength)
    }

    class AudioCommand {
        <<Enum>>
        +Pluck
        +Oscillate
        +AddWall
    }

    AudioModel *-- PhysicsGrid : Owns
    AudioModel ..> AudioCommand : Consumes
```

### Optional Audio Backend (ADR 005)

To support CI environments without audio drivers, all audio functionality is gated behind a `feature = "audio"` flag.

```mermaid
classDiagram
    direction TB
    class AudioFeature {
        <<Feature Flag>>
        +Enabled: bool
    }

    class Experiment {
        +run()
    }

    class AudioEngine {
        +play_sound()
    }

    Experiment ..> AudioEngine : Calls (if audio enabled)
    Experiment ..> AudioFeature : Checks
```

### Ghost Input Replay (crates/tui-shared)

The `tui-shared` crate includes a `ghost` feature for recording and replaying user input sessions, facilitating deterministic testing and demos.

```mermaid
classDiagram
    direction LR

    class GhostRecorder {
        +start()
        +record(event: Event)
        +to_json() String
    }

    class GhostReplayer {
        +start()
        +poll() Option~Event~
        +from_json(json: String) GhostReplayer
    }

    class GhostEvent {
        <<Serializable>>
        +Key(GhostKeyEvent)
        +Mouse(GhostMouseEvent)
    }

    GhostRecorder ..> GhostEvent : Serializes
    GhostReplayer ..> GhostEvent : Deserializes
```

### GPU Compute Architecture (ADR 045)

Standardized pattern for high-performance cellular automata using WGPU Compute Shaders.

```mermaid
classDiagram
    direction TB
    class State {
        +wgpu::Device device
        +wgpu::Queue queue
        +ComputePipeline pipeline
        +Uniforms uniforms
        +frame_count u64
        +update()
        +render()
    }

    class ComputePipeline {
        +wgpu::BindGroup bind_groups
        +wgpu::Buffer cell_buffers[2]
        +dispatch()
    }

    class Uniforms {
        +f32 time
        +u32 grid_size
        +f32 params
    }

    State *-- ComputePipeline : Owns
    State *-- Uniforms : Updates
    ComputePipeline o-- "2" Buffer : Ping-Pong
```

```mermaid
sequenceDiagram
    participant State
    participant GPU as WGPU Queue
    participant Ping as Buffer A
    participant Pong as Buffer B

    Note over State: Frame N (Even)
    State->>GPU: dispatch(Ping -> Pong)
    GPU->>Ping: Read State
    GPU->>Pong: Write Next State

    State->>GPU: render(Pong)
    GPU->>Pong: Vertex Pulling

    Note over State: Frame N+1 (Odd)
    State->>GPU: dispatch(Pong -> Ping)
    GPU->>Pong: Read State
    GPU->>Ping: Write Next State
```

### Storage Decoupling (ADR 006)

Refactoring to decouple storage from core logic to resolve circular dependencies.

#### Core vs Storage

```mermaid
classDiagram
  class Core
  class Storage
  Core --> Storage : Uses (Trait Bound)
  %% Removed the circular dependency arrow
```

#### Storage Flow

```mermaid
sequenceDiagram
    participant C as Core
    participant S as Storage

    Note over C,S: Decoupled via Trait (ADR 006)
    C->>S: save_state(data)
    S-->>C: Result<Ok>
```

## Shared Domain Logic

### Platter Field Logic (crates/platter)

Standardized scalar field simulation logic for magnetism, fluid density, and pheromones (ADR 062).

```mermaid
classDiagram
    direction TB
    class Platter {
        +Vec~f64~ magnetism
        +usize width
        +usize height
        +new(width, height)
        +magnetize(x, y, amount)
        +accumulate(x, y, amount)
        +decay(rate)
        +get(x, y) f64
    }

    note for Platter "magnetize = Clamped (1.0)\naccumulate = Unbounded\ndecay = Threshold (< 0.001 -> 0.0)"
```

Specialized libraries that encapsulate specific domain knowledge or data structures, reused across multiple experiments.

### Quipu Data Structures (crates/quipu)

Encapsulates Inca recording devices (`Knot`, `Cord`, `Quipu`) to ensure consistent representation and behavior (ADR 024).

```mermaid
classDiagram
    direction LR
    class Quipu {
        +Vec~Cord~ cords
        +display()
    }

    class Cord {
        +Vec~Vec~Knot~~ clusters
        +value() u64
        +add(Cord) Cord
        +sub(Cord) Cord
    }

    class Knot {
        <<Enum>>
        +Simple
        +Long(u8)
        +FigureEight
        +value() u8
    }

    Quipu *-- Cord : Contains
    Cord *-- Knot : Contains
```

### Locus Geometry (crates/locus)

Provides standard vector math (2D, 3D, 4D), topological wrapping logic, and spatial behaviors (flocking) (ADR 025, ADR 059).

```mermaid
classDiagram
    direction LR
    class Vec2 {
        +f64 x, y
        +add()
        +sub()
        +magnitude()
        +normalize()
    }
    class Vec3 {
        +f64 x, y, z
        +cross(Vec3) Vec3
    }
    class Vec4 {
        +f64 x, y, z, w
        +rotate_xw()
        +project_to_3d()
    }

    class Topology {
        <<Enum>>
        +Plane
        +Torus
        +KleinBottle
        +Mobius
        +normalize(y, x) Option~y, x~
    }

    class Flocking {
        <<Module>>
        +compute_force(agents, idx, params) Vec2
        +FlockingParams params
    }

    Topology ..> Vec2 : Complements
    Flocking ..> Vec2 : Uses
```

### Market Simulation (crates/market-sim)

Implements a Continuous Double Auction (CDA) using a physics-based particle system (ADR 026).

```mermaid
classDiagram
    direction TB
    class Grid {
        +Vec~Particle~ cells
        +update() Vec~TradeEvent~
    }

    class Particle {
        <<Enum>>
        +Bid(buyer_id)
        +Ask(seller_id)
        +Trade(age)
    }

    class TradeEvent {
        +usize buyer
        +usize seller
        +f32 price
    }

    Grid *-- Particle : Contains
    Grid ..> TradeEvent : Emits
```

### Synaptic Physics (crates/synaptic-physics)

Encapsulates the Izhikevich neuron model for biologically plausible neural simulations (ADR 027).

```mermaid
classDiagram
    class Izhikevich {
        +f32 v
        +f32 u
        +f32 tau
        +update(dt, current) (f32, bool)
        +inject(current)
        +random() Izhikevich
    }
```

### Git Associates (crates/git-associates)

Helper utilities for scanning and parsing Git history, used by `tectonic-git` and others (ADR 033).

```mermaid
classDiagram
    class GitAssociates {
        <<Library>>
    }
    class GitModel {
        +open(path)
        +history(limit)
        +diff_workdir()
    }

    class Commit {
        +String hash
        +String message
        +CommitStats stats
    }

    GitAssociates *-- GitModel : Exports
    GitModel ..> Commit : Produces
```


### Soroban Logic (crates/soroban)

The `soroban` crate encapsulates the logic of the Japanese Abacus, modeling state as physical bead positions rather than just integer values (ADR 034).

```mermaid
classDiagram
    direction TB
    class Soroban {
        +[Column; 13] columns
        +add(val: u64)
        +sub(val: u64)
        +value() u64
    }

    class Column {
        +bool upper_active
        +u8 lower_active
        +value() u8
    }

    class SorobanMarket {
        <<Experiment>>
        +render_tui(soroban)
    }

    class SorobanSpecter {
        <<Experiment>>
        +render_fluid(soroban)
    }

    Soroban *-- Column : Composes
    SorobanMarket ..> Soroban : Visualizes
    SorobanSpecter ..> Soroban : Visualizes
```

### Origami Logic (crates/origami)

Encapsulates Miura-ori folding geometry, separating mathematical vertex generation from rendering (ADR 040).

```mermaid
classDiagram
    direction TB
    class MiuraOri {
        +MiuraParams params
        +generate_mesh(extension) OrigamiMesh
        +generate_grid(extension) Vec~Vec3~
    }

    class MiuraParams {
        +f32 a
        +f32 b
        +f32 gamma
        +Orientation orientation
    }

    class OrigamiMesh {
        +Vec~OrigamiVertex~ vertices
        +Vec~u16~ indices
    }

    class Orientation {
        <<Enum>>
        +Horizontal
        +Vertical
    }

    MiuraOri *-- MiuraParams : Configured by
    MiuraOri ..> OrigamiMesh : Produces
```

### Gray-Scott Simulation (crates/gray-scott)

Provides a shared, optimized Gray-Scott reaction-diffusion simulation kernel with optional parallel updates (ADR 041).

```mermaid
classDiagram
    direction TB
    class GrayScott {
        +Vec~f32~ u
        +Vec~f32~ v
        +update(feed, kill, dt)
        +add_chemical(x, y, amt)
    }

    class Rayon {
        <<Library>>
        +par_iter_mut()
    }

    GrayScott ..> Rayon : Uses (if feature=parallel)
    note for GrayScott "Implements 3x3 Laplacian Convolution"
```

### Neuro Simulation (crates/neuro-sim)

Provides a high-level network simulation layer for spiking neural networks, managing synaptic connectivity and propagation delays (ADR 043).

```mermaid
classDiagram
    direction TB
    class Network {
        +Vec~Izhikevich~ neurons
        +Vec~Synapse~ synapses
        +Vec~bool~ spikes
        +step(external_inputs)
        +is_spiking(index) bool
    }

    class Synapse {
        +usize from
        +usize to
        +f32 weight
        +usize delay
    }

    class Izhikevich {
        <<Library: synaptic-physics>>
        +update()
    }

    Network *-- Synapse : Contains
    Network *-- Izhikevich : Contains
    Synapse ..> Izhikevich : Connects
```

### Physics PBD (crates/physics-pbd)

Implements a Position Based Dynamics engine for simulating physical constraints and particle interactions (ADR 044).

```mermaid
classDiagram
    direction TB
    class PbdSystem {
        +Vec~Particle~ particles
        +Vec~Constraint~ constraints
        +step(dt, iterations)
        +add_particle(pos, mass) usize
        +add_distance_constraint(p1, p2, stiffness)
    }

    class Particle {
        +Vec3 pos
        +Vec3 prev_pos
        +Vec3 vel
        +f32 inv_mass
    }

    class Constraint {
        <<Enum>>
        +Distance
        +Actuator
        +Pin
    }

    PbdSystem *-- Particle : Manages
    PbdSystem *-- Constraint : Enforces
    Constraint ..> Particle : Affects
```

### Hyper System (crates/hyper-system)

Provides system monitoring utilities for "Hyper" series experiments, re-exporting 4D math from `locus` (ADR 050, ADR 059).

```mermaid
classDiagram
    direction TB
    class SystemMonitor {
        +f32 cpu_usage
        +f32 mem_usage
        +f32 swap_usage
        +f32 load_avg
        +update_with_time(dt, now)
        +update()
    }

    note for SystemMonitor "Uses locus::Vec4\nInterpolates metrics for smooth visuals"
```

## Experiment: Git Harmony

**Git Harmony** (formerly Git Rhythm) generates music from git diffs ("Code Singing").

### Component Structure

```mermaid
classDiagram
    direction LR
    class DiffState {
        +Vec~FileDiff~ files
    }

    class VisualNote {
        +f32 pitch
        +f32 color_hue
        +String text
    }

    class Synthesizer {
        -DiffState diff
        -Vec~VisualNote~ active_notes
        +new(diff: DiffState)
        +tick()
        +play_event(event: LineChange)
    }

    class RodioSink {
        <<Optional Audio>>
        +append(source)
    }

    Synthesizer *-- DiffState : Iterates
    Synthesizer *-- VisualNote : Generates
    Synthesizer ..> RodioSink : Uses (if feature=audio)

    note for Synthesizer "Maps file hash -> Frequency\nMaps DiffType -> Color"
```


## Experiment: Chimera Lang (ADR 008)

**Chimera Lang** is a bio-inspired, stack-based esoteric programming language with an optional "Nova" expansion for advanced biological simulation.

### Lib/Bin Split

The project exposes its core modules to allow embedding in other experiments.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +Dna dna
        +PetriDish grid
        +Vec~Deque~ stack
        +step()
        +execute_gene()
    }

    class Dna {
        +Helix helix
    }

    class PetriDish {
        +[[Val; 16]; 16] cells
    }

    class ExternalApp {
        <<Consumer>>
    }

    ChimeraVM *-- Dna : Owns
    ChimeraVM *-- PetriDish : Owns
    ExternalApp ..> ChimeraVM : Embeds
```

### Nova Feature: Endocrine Cycle

When `feature = "nova"` is enabled, the VM simulates a hormone diffusion cycle.

```mermaid
sequenceDiagram
    participant App
    participant VM as ChimeraVM
    participant Endocrine as HormoneGrid
    participant Enzyme

    App->>VM: step()
    VM->>VM: process_metabolism()

    opt Nova Feature
        VM->>Endocrine: diffuse()
        VM->>Endocrine: decay()
    end

    VM->>Enzyme: execute(gene)

    opt Nova Feature
        Enzyme->>Endocrine: secrete(hormone)
        Enzyme->>Endocrine: detect(hormone)
    end
```

### Nova Feature: Time Travel (ADR 013)

The `Spore` struct enables deep state snapshots, allowing the VM to backtrack execution paths.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +Dna dna
        +Vec~Value~ stack
        +PetriDish grid
        +Vec~Spore~ spores
        +step()
        +sporulate()
        +germinate(id)
    }

    class Spore {
        <<Snapshot>>
        +Dna dna
        +Vec~Value~ stack
        +PetriDish grid
        +usize ip
    }

    ChimeraVM *-- Spore : Manages
    ChimeraVM ..> Spore : Creates (Sporulate)
    Spore ..> ChimeraVM : Restores (Germinate)
```

```mermaid
sequenceDiagram
    participant VM
    participant SporeStorage as Vec<Spore>

    VM->>VM: Execute OpCode::Sporulate
    VM->>SporeStorage: push(clone_state())
    SporeStorage-->>VM: return spore_id (0)
    VM->>VM: Continue Execution...
    VM->>VM: Encounter Hazard (Mutation/Death)

    opt If Failure Detected
        VM->>VM: Execute OpCode::Germinate(0)
        SporeStorage->>VM: restore_state(spore_0)
        VM->>VM: State Reverted (Time Travel)
    end
```

### Nova Feature: Prion Protocol (ADR 013)

Dynamic instruction remapping allows the environment or the program itself to alter the meaning of genes.

```mermaid
stateDiagram-v2
    [*] --> Normal
    Normal --> Remapped : OpCode::Remap(Add, Sub)
    Remapped --> Normal : OpCode::Restore(Add)

    state Normal {
        Add : Adds two numbers
    }

    state Remapped {
        Add : Subtracts two numbers (Prion)
    }
```

### Nova Feature: Chorus System (ADR 014)

The Chorus System allows the VM to trigger global effects by detecting specific sequences of "notes" (strings) in a sliding window buffer.

```mermaid
sequenceDiagram
    participant VM
    participant ChorusBuffer as VecDeque<String>
    participant State as Global State

    Note over VM: Program executes Sing("Do")
    VM->>ChorusBuffer: push_back("Do")
    VM->>VM: check_chorus_chords()

    Note over VM: Program executes Sing("Mi")
    VM->>ChorusBuffer: push_back("Mi")
    VM->>VM: check_chorus_chords()

    Note over VM: Program executes Sing("Sol")
    VM->>ChorusBuffer: push_back("Sol")
    VM->>VM: check_chorus_chords()

    rect rgb(200, 255, 200)
        Note right of VM: "Genesis" Chord Detected!
        VM->>State: Spawn(Worker)
        VM->>ChorusBuffer: clear()
    end
```

### Phylogeny Feature: Host Interaction (ADR 017)

The `phylogeny` feature enables the VM to escape its sandbox and interact with the host filesystem and shell.

```mermaid
sequenceDiagram
    participant VM as ChimeraVM
    participant Phy as Phylogeny Module
    participant OS as Host OS

    VM->>Phy: exec_phylogeny_op(Crawl, path)
    Phy->>OS: fs::read_dir(path)
    OS-->>Phy: Result<Entries>
    Phy-->>VM: push(Junction(files))

    VM->>Phy: exec_phylogeny_op(Shell, "cargo build")
    Phy->>OS: Command::new("cargo").arg("build")
    OS-->>Phy: Output(stdout, stderr)
    Phy-->>VM: push(String(stdout))
```


### Chimera Feature: Sovereignty (ADR 018)

The Sovereignty system enables organisms to claim territory and tax visitors.

```mermaid
sequenceDiagram
    participant VM
    participant SovGrid as Sovereignty Grid
    participant Market

    VM->>VM: step() calls process_territory()
    VM->>SovGrid: check owner at (x, y)

    alt If cell has Owner != Visitor
        VM->>VM: get tax_rate for Owner

        opt If tax_rate > 0
            VM->>VM: deduct tax from Visitor energy
            VM->>Market: credit(Owner, tax_amount)

            opt If Visitor energy <= 0
                VM->>VM: Kill Visitor (Starvation)
            end
        end
    end
```

### Nova Feature: Chemistry System (ADR 019)

The Chemistry system allows for the creation and manipulation of chemical solutions using recipes.

```mermaid
sequenceDiagram
    participant VM
    participant Grid
    participant Chemistry as Chemistry Module

    Note over VM: OpCode::Mix(radius)
    VM->>Grid: Collect ingredients in radius
    Grid-->>Chemistry: Ingredients
    Chemistry->>Grid: Set Cell = Dish(Ingredients)

    Note over VM: OpCode::Brew(heat)
    VM->>Grid: Get Dish at (x,y)
    Grid-->>Chemistry: Ingredients
    Chemistry->>Chemistry: Match Recipe(Ingredients, Heat)
    Chemistry->>Grid: Set Cell = Dish(Solution)

    Note over VM: OpCode::Splash(x, y, radius)
    VM->>Grid: Get Solution at (x,y)
    Chemistry->>Grid: Apply Effect(Solution, TargetArea)
    Note right of Grid: Acid: Destroy<br/>Elixir: Heal<br/>Mutagen: Mutate
```

### Nova Feature: Hive Networking (ADR 020)

The Hive system enables asynchronous UDP communication between Chimera VMs, facilitating distributed simulation and swarm behavior.

```mermaid
sequenceDiagram
    participant VM
    participant HiveSocket as UDP Socket (Bound)
    participant Ephemeral as UDP Socket (Temp)

    Note over VM: OpCode::HiveBind(8080)
    VM->>HiveSocket: bind("0.0.0.0:8080")
    HiveSocket-->>VM: Ok(Arc<Socket>)

    Note over VM: OpCode::HiveSend(msg, "1.2.3.4", 9090)
    VM->>Ephemeral: bind("0.0.0.0:0")
    Ephemeral->>Ephemeral: send_to(json(msg), "1.2.3.4:9090")
    Ephemeral-->>VM: Ok

    Note over VM: OpCode::HiveRecv(8080)
    VM->>HiveSocket: recv_from()
    alt Data Available
        HiveSocket-->>VM: Ok(payload, src_addr)
        VM->>VM: push(Junction(Dish, [src_port, src_ip, payload]))
    else WouldBlock
        HiveSocket-->>VM: Err(WouldBlock)
        VM->>VM: push(0)
    end
```

### Nova Feature: Oracle (ADR 021)

The Oracle module provides a Prolog-like inference engine, allowing the VM to query its own state and perform logical reasoning.

```mermaid
sequenceDiagram
    participant VM
    participant Oracle
    participant KB as Knowledge Base

    Note over VM: OpCode::Query(Goal)
    VM->>Oracle: solve(Goal)
    Oracle->>KB: fetch_facts()
    loop Unification
        Oracle->>Oracle: unify(Goal, Fact)
        alt Success
            Oracle-->>VM: push(Solution)
        else Failure
            Oracle->>Oracle: backtrack()
        end
    end

    Note over VM: OpCode::Seek(Predicate)
    VM->>Oracle: solve(has_feature(?Target, Predicate))
    Oracle-->>VM: bind(?Target = StrandIdx)
    VM->>VM: jump_to(StrandIdx)
```

### Nova Feature: Incubator (ADR 022)

The Incubator system enables the environment to influence the genome ("Abiogenesis") and allows organisms to edit their own DNA ("CRISPR").

```mermaid
stateDiagram-v2
    state Environment {
        GridCells
    }
    state Genome {
        Strand_A
        Strand_B
    }

    [*] --> Environment
    Environment --> Genome : Incubate(Grid -> Code)
    Genome --> Genome : CRISPR (Cut/Splice/Edit)
    Genome --> Offspring : Mitosis (Clone)
    Genome --> Offspring : Splice (Crossover)
    Offspring --> [*] : Apoptosis
```

### Nova Feature: Holographic Memory (ADR 028)

The Holographic Memory system enables the storage of genetic information as distributed interference patterns, allowing for fuzzy retrieval and resilience to local damage.

```mermaid
sequenceDiagram
    participant VM
    participant HologramGrid as ComplexGrid
    participant DNA

    Note over VM: OpCode::Interfere(StrandIdx)
    VM->>DNA: Get Genes
    loop Per Gene
        DNA-->>VM: Gene(Op, Arg)
        VM->>HologramGrid: Inverse DFT (Add Wave)
        Note right of HologramGrid: Accumulate Interference
    end

    Note over VM: OpCode::Refract
    VM->>HologramGrid: Forward DFT (Extract Frequencies)
    loop Per Frequency
        HologramGrid-->>VM: Magnitude & Phase
        alt Magnitude > Threshold
            VM->>VM: Phase -> OpCode
            VM->>VM: Amplitude -> Arg
            VM->>DNA: Append New Gene
        end
    end
```

### Nova Feature: Metazoa (ADR 029)

The Metazoa system enables multicellularity by allowing the VM to spawn independent `Organelle` agents that can bond into `Tissue` structures.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +Vec~Organelle~ organelles
        +HashMap~usize, Tissue~ tissues
        +step()
    }

    class Organelle {
        +usize id
        +Option~usize~ tissue_id
        +OrganelleType kind
        +Vec~Value~ stack
        +step()
    }

    class Tissue {
        +usize id
        +Vec~usize~ members
    }

    class OrganelleType {
        <<Enum>>
        +Worker
        +Chloroplast
        +Mitochondria
        +Lysosome
    }

    ChimeraVM *-- Organelle : Owns
    ChimeraVM *-- Tissue : Owns
    Tissue o-- Organelle : References
    Organelle ..> OrganelleType : Is-A
```

```mermaid
sequenceDiagram
    participant O1 as Organelle (A)
    participant O2 as Organelle (B)
    participant VM
    participant T as Tissue

    Note over O1: OpCode::Bond(East)
    O1->>VM: bond_with(O2)
    VM->>T: Create(A, B)
    T-->>O1: tissue_id = 1
    T-->>O2: tissue_id = 1

    Note over O1: OpCode::Signify("Help!")
    O1->>VM: broadcast(1, "Help!")
    VM->>O2: push("Help!")
```

### Nova Feature: Pandemonium Reactor (ADR 030)

The Pandemonium Reactor is an interactive TUI mode allowing for chaotic, hands-on manipulation of the genome via a spiral visualization.

```mermaid
sequenceDiagram
    participant User
    participant TUI
    participant P as VM::Pandemonium
    participant DNA

    Note over TUI: User selects Target (Reticle)
    User->>TUI: Press 'Space' (Apply Tool)
    TUI->>TUI: Map Cursor(x,y) -> GeneIdx

    alt Tool = Mutate
        TUI->>P: apply_mutation(vm, GeneIdx)
        P->>DNA: Randomize OpCode/Arg
    else Tool = Scramble
        TUI->>P: apply_scramble(vm, GeneIdx, Radius)
        P->>DNA: Shuffle Genes in Range
    else Tool = Storm
        TUI->>P: apply_storm(vm, GeneIdx, Radius)
        P->>DNA: Massive Randomization
    end

    DNA-->>TUI: Genome Updated
    TUI->>User: Render Spiral Visualization
```

### Nova Feature: Quantum Mechanics (ADR 031)

The Quantum system introduces non-local interaction between strands via Entanglement and probabilistic states via Superposition.

```mermaid
sequenceDiagram
    participant VM
    participant Strand_A
    participant Strand_B
    participant State

    Note over VM: OpCode::Entangle(A, B)
    VM->>State: Map A <-> B

    Note over Strand_A: OpCode::Transcribe(Gene_X, Arg_New)
    Strand_A->>VM: Modify Gene_X
    VM->>State: Check Entanglement(A)
    State-->>VM: Partner = B

    VM->>Strand_A: Update Gene_X
    VM->>Strand_B: Update Gene_X (Action at a distance)
```

### Nova Feature: Necromancy (ADR 031)

The Necromancy system allows for the persistence and recovery of "dead" code, enabling genetic memory and ghost execution.

```mermaid
stateDiagram-v2
    [*] --> Alive
    Alive --> Graveyard : Apoptosis / Bury
    Graveyard --> Alive : Exhume / Reincarnate
    Graveyard --> Ghost : Seance
    Ghost --> Graveyard : End Seance

    state Graveyard {
        [*] --> Stored
        Stored --> Consumed : Mourn (Energy Gain)
    }
```

### Nova Feature: Prologue System (ADR 042)

The Prologue system enables visual, grid-based logic execution, allowing for the creation of digital circuits and autonomous agents.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +PrologueState prologue_state
        +PetriDish grid
        +exec_prologue_tick()
    }

    class PrologueState {
        +bool active
        +HashSet runes
        +Vec~Vec~Option~Value~~~ signal_grid
        +Vec~Vec~Option~Value~~~ delayed_signals
        +HashMap~i64, Value~ teleport_channels
        +HashMap~String, Value~ library
        +Vec~Vec~EpigeneticMark~~ epigenetic_grid
        +f32 dream_intensity
        +HashMap~Pos, Deque~Value~~ history
        +Vec~PrologueAgent~ agents
    }

    class PrologueAgent {
        +usize x
        +usize y
        +Value state
        +Vec~Value~ stack
    }

    class PrologueModules {
        <<Namespace>>
        +topology
        +logic
        +math
        +io
        +quantum
        +alchemy
        +chronos
        +virology
        +narrative
        +memetics
        +epigenetics
        +hypnagogia
    }

    ChimeraVM *-- PrologueState : Owns
    PrologueState *-- PrologueAgent : Manages
    ChimeraVM ..> PrologueModules : Delegates to
```

```mermaid
sequenceDiagram
    participant VM as ChimeraVM
    participant State as PrologueState
    participant Modules as Prologue Submodules

    VM->>VM: exec_prologue_tick()
    VM->>State: scan_grid_rules()
    VM->>VM: prepare_signals()

    loop Propagation (Flood Fill)
        VM->>Modules: apply_topology_runes()
        VM->>Modules: apply_logic_runes()
        VM->>Modules: apply_quantum_runes()
        VM->>Modules: apply_alchemy_runes()
        Note right of Modules: ...and others
    end

    VM->>VM: process_sinks()
    VM->>VM: process_agents()
```

### Nova Feature: Ribozyme (ADR 052)

The Ribozyme system introduces functional programming capabilities and a Lisp parser.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +eval(code: String)
    }

    class LispParser {
        +parse(input: String) Result~Vec~SExpr~~
        +compile(exprs: Vec~SExpr~) Result~Dna~
    }

    class OpCode {
        <<Enum>>
        +Eval
        +Map
        +Fold
        +Filter
        +Zip
    }

    ChimeraVM ..> LispParser : Uses (for Eval)
    ChimeraVM ..> OpCode : Executes
```

### Nova Feature: Babel (ADR 053)

The Babel system implements parser combinators within the VM for dynamic grammar definition.

```mermaid
classDiagram
    direction TB
    class BabelState {
        +HashMap~String, Parser~ grammars
        +f32 integrity
        +parse(parser, input) Result~Ast~
    }

    class Parser {
        <<Enum>>
        +Match(String)
        +Regex(String)
        +Seq(Vec~Parser~)
        +Alt(Vec~Parser~)
        +Many(Box~Parser~)
    }

    class ChimeraVM {
        +BabelState babel_state
    }

    ChimeraVM *-- BabelState : Owns
    BabelState *-- Parser : Manages
```

### Nova Feature: Akashic Records (ADR 054)

The Akashic system provides persistent storage for the VM.

```mermaid
classDiagram
    direction TB
    class AkashicRecords {
        +PathBuf file_path
        +HashMap~String, Value~ memory
        +load()
        +save()
        +write(key, value)
        +read(key) Value
    }

    class ChimeraVM {
        +AkashicRecords akashic
    }

    ChimeraVM *-- AkashicRecords : Owns
    AkashicRecords ..> FileSystem : Persists to
```

### Nova Feature: Paradox System (ADR 055)

The Paradox system allows defining physics-like rules that trigger global or local effects based on environmental conditions.

### Class Structure

```mermaid
classDiagram
    class ChimeraVM {
        +Paradox paradox
        +step()
    }
    class Paradox {
        +Vec~Rule~ rules
        +tick(vm)
        +parse_rule(str)
    }
    class Rule {
        +Trigger trigger
        +Vec~Action~ actions
    }
    class Trigger {
        <<Enum>>
        +Always
        +Signal(String)
    }
    class Action {
        <<Enum>>
        +Log
        +Set
        +Glitch
    }
    ChimeraVM *-- Paradox : Owns
    Paradox *-- Rule : Contains
    Rule *-- Trigger : Uses
    Rule *-- Action : Uses
```

### Execution Cycle

```mermaid
sequenceDiagram
    participant VM
    participant Paradox

    VM->>VM: pre_tick_updates()
    VM->>Paradox: take()
    VM->>Paradox: tick(VM)
    loop Every Rule
        Paradox->>Paradox: Check Trigger
        opt Triggered
            Paradox->>VM: Apply Actions (Set/Glitch)
        end
    end
    Paradox-->>VM: return ownership
    VM->>VM: process_environment()
    VM->>VM: execute_dna()
```

### Nova Feature: BioMesh Network (ADR 056)

The BioMesh system provides a graph-based overlay network on top of the grid for efficient packet routing.

```mermaid
classDiagram
    direction TB
    class BioMeshState {
        +HashMap~Pos, BioMeshNode~ nodes
    }

    class BioMeshNode {
        +u64 id
        +VecDeque~Value~ buffer
        +Vec~Pos~ connections
    }

    class ChimeraVM {
        +BioMeshState biomesh
    }

    ChimeraVM *-- BioMeshState : Owns
    BioMeshState *-- BioMeshNode : Contains
```

```mermaid
sequenceDiagram
    participant VM
    participant Mesh as BioMeshState
    participant NodeA
    participant NodeB

    Note over VM: OpCode::MeshSend(TargetID, Value)
    VM->>Mesh: Find path (BFS) from NodeA to NodeB(TargetID)
    Mesh->>Mesh: Calculate Route...
    alt Path Found
        Mesh->>NodeB: buffer.push(Value)
        Mesh-->>VM: Success (0 Ticks)
    else No Path
        Mesh-->>VM: Failure
    end
```

### Nova Feature: Linguistics System (ADR 057)

The Linguistics system integrates string analysis algorithms directly into the VM.

```mermaid
classDiagram
    direction TB
    class ChimeraVM {
        +exec_gene()
    }

    class Linguistics {
        <<Module>>
        +levenshtein(s1, s2) usize
        +soundex(s) String
        +is_anagram(s1, s2) bool
        +is_pangram(s) bool
    }

    ChimeraVM ..> Linguistics : Calls via OpCodes
    note for Linguistics "Enforces MAX_COMPLEX_STRING_LEN"
```

### Nova Feature: Procedural Botany (ADR 058)

The Botany system implements L-System interpretation for procedural geometry generation via a specialized Organelle.

```mermaid
sequenceDiagram
    participant VM
    participant Seed as Organelle (Seed)
    participant Grid

    Note over VM: OpCode::Plant(Axiom, Rules)
    VM->>Seed: Spawn(Axiom, Rules)

    loop Every Tick
        Seed->>Seed: Expand String (L-System)
        Seed->>Seed: Interpret Chars (Turtle)

        alt Char = 'F' (Forward)
            Seed->>Grid: Write '#'
            Seed->>Seed: Move Position
        else Char = '+' (Turn)
            Seed->>Seed: Rotate Direction
        else Char = '[' (Branch)
            Seed->>Seed: Push State
        end
    end
```

### Constants Module (ADR 063)

The Constants module centralizes shared numerical values and parameters across the Chimera VM.

```mermaid
classDiagram
    direction TB
    class Constants {
        +GOLDEN_FREQUENCIES: [f32; 4]
    }
    class TuiViewsAudio {
        +render()
    }
    class VmNovaSignals {
        +process()
    }
    TuiViewsAudio --> Constants : Uses
    VmNovaSignals --> Constants : Uses
```

## Experiment: Tectonic Git (ADR 023)

**Tectonic Git** visualizes the repository history as geological strata, using code analysis to determine stability.

#### Geological Simulation

The simulation maps commits to layers and uses keyword density to generate stress and fissures.

```mermaid
classDiagram
    class World {
        +Vec~Strata~ strata
        +Vec~Fissure~ fissures
        +update()
    }
    class Strata {
        +CommitData commit
        +f64 stress
        +f64 offset_x
    }
    class Fissure {
        +Vec~Vec2~ points
        +f64 intensity
        +grow()
    }
    class GitScanner {
        +scan() Vec~CommitData~
    }

    World *-- Strata : Contains
    World *-- Fissure : Contains
    Strata ..> GitScanner : Created from
    note for Strata "Stress = keywords(TODO, FIXME, panic!)"
```

### Experiment: WASM Runes (ADR 035)

**WASM Runes** embeds executable WASM logic into procedurally generated "Rune" images, prioritized by a central spiral encoding.

#### Architecture

The system is composed of an embedding engine (`Stego`), a procedural generator (`Rune`), and a lightweight runner (`VM`).

```mermaid
classDiagram
    direction LR
    class Rune {
        +generate(hash) RgbaImage
        -draw_glyph()
    }

    class Stego {
        +embed(img, data)
        +extract(img) Vec~u8~
    }

    class SpiralIter {
        +next() (dx, dy)
        -step_size
        -turn_counter
    }

    class VM {
        +WasmEngine engine
        +run(binary)
    }

    Stego ..> SpiralIter : Uses
    Stego ..> Rune : Modifies Output
    VM ..> Stego : Consumes Payload
    note for VM "Uses wasmtime v14.0"
```

#### Lifecycle

```mermaid
sequenceDiagram
    participant User
    participant Stego
    participant Rune
    participant VM

    Note over User: User compiles Rust -> .wasm

    User->>Stego: embed(wasm_bytes)
    Stego->>Rune: generate(hash(wasm))
    Rune-->>Stego: RgbaImage
    Stego->>Stego: SpiralIter encode LSB
    Stego-->>User: "spell.png"

    Note over User: User shares image

    User->>VM: run("spell.png")
    VM->>Stego: extract("spell.png")
    Stego->>Stego: SpiralIter decode LSB
    Stego-->>VM: wasm_bytes
    VM->>VM: wasmtime::instantiate()
    VM-->>User: Output
```

## Experiment: Sandpile Scheduler (ADR 036)

**Sandpile Scheduler** simulates distributed task scheduling using the Abelian Sandpile Model to visualize load balancing and avalanches.

### Grid Architecture

The system uses a double-buffered grid and parallel iteration to handle large-scale updates.

```mermaid
classDiagram
    direction LR
    class Grid {
        +Vec~Cell~ cells
        +Vec~Cell~ next_cells
        +add_load(x, y, amount)
        +update(process_rate)
        +total_load() u64
    }

    class Cell {
        +u32 load
        +u64 processed
    }

    class Rayon {
        <<Library>>
        +par_iter_mut()
    }

    Grid *-- Cell : Contains (Double Buffer)
    Grid ..> Rayon : Uses for Update
    note for Grid "Topple Rule: Load >= 4 -> Distribute to neighbors"
```

## Experiment: Thermo-Defense (ADR 037)

**Thermo-Defense** simulates emergent defense strategies using swarm intelligence and heat diffusion.

### Swarm Architecture

The simulation uses a hybrid parallel architecture to support thousands of agents.

```mermaid
classDiagram
    direction LR
    class World {
        +Grid grid
        +Vec~Agent~ agents
        +update()
    }

    class Grid {
        +Vec~Cell~ cells
        +Vec~Cell~ next_cells
        +update_diffusion()
    }

    class Cell {
        +f32 heat
        +f32 pheromone_defense
        +f32 pheromone_attack
        +Material material
    }

    class Agent {
        +AgentType kind
        +Vec2 position
        +update(grid, rng) Option~GridAction~
    }

    class AgentType {
        <<Enum>>
        +Termite
        +Locust
    }

    World *-- Grid : Owns
    World *-- Agent : Owns
    Grid *-- Cell : Contains (Double Buffer)
    Agent ..> AgentType : Is-A
```

### Simulation Loop

To ensure determinism and performance, the simulation separates diffusion (Parallel), decision (Parallel), and mutation (Sequential).

```mermaid
sequenceDiagram
    participant World
    participant Grid
    participant Agents
    participant Actions

    Note over World: Update Step

    World->>Grid: update_diffusion()
    Grid->>Grid: Parallel Iter (Rayon)
    Grid-->>Grid: Swap Buffers

    World->>Agents: par_iter_mut()
    loop Parallel Decision
        Agents->>Grid: Read Cell (x,y)
        Agents->>Agents: Compute Logic
        Agents-->>Actions: Collect Option<Action>
    end

    World->>Actions: Iterate Results
    loop Sequential Resolution
        Actions->>Grid: Apply Mutation (Build/Destroy)
        Grid-->>Grid: Update Heat/Pheromone
    end
```

## Experiment: Crystal Defense (ADR 038)

**Crystal Defense** is a Tower Defense game played on a 3D projected Icosahedral Quasicrystal lattice.

### Hyper-dimensional Architecture

The game uses a 6D-to-3D projection to generate the game board, resulting in a non-periodic, highly symmetric graph.

```mermaid
classDiagram
    direction TB
    class State {
        +Dungeon dungeon
        +render()
    }

    class Dungeon {
        +World world
        +update()
    }

    class World {
        +Arc~Quasicrystal~ qc
        +Vec~Agent~ agents
        +Vec~f32~ heat_map
        +Vec~f32~ pheromones
    }

    class Quasicrystal {
        +Vec~Point3~ atoms
        +Vec~Edge~ edges
        +Vec~Vec~usize~~ adj
    }

    class Agent {
        +AgentType kind
        +usize current_node
        +Option~usize~ target_node
    }

    State *-- Dungeon : Owns
    Dungeon *-- World : Owns
    World o-- Quasicrystal : Shared (Arc)
    World *-- Agent : Manages
```

## Experiment: Chimera Circuit (ADR 039)

**Chimera Circuit** is a hybrid experiment that combines procedurally generated circuit boards with genetic algorithms.

### Hybrid Architecture

The experiment reuses logic from `circuit-sigil` via source adaptation and `chimera-lang` via library import.

```mermaid
classDiagram
    direction TB
    class ChimeraCircuit {
        +CircuitGenerator circuit
        +Vec~BioAgent~ agents
        +run()
    }

    class BioAgent {
        +ChimeraVM vm
        +Vec2 pos
        +update(circuit)
    }

    class CircuitGenerator {
        <<Adapted from circuit-sigil>>
        +generate(seed) (Image, Pads)
    }

    class ChimeraVM {
        <<Library: chimera-lang>>
        +execute()
    }

    ChimeraCircuit *-- CircuitGenerator : Owns
    ChimeraCircuit *-- BioAgent : Owns
    BioAgent *-- ChimeraVM : Wraps
    BioAgent ..> CircuitGenerator : Senses
```

## Experiment: Hyper Acoustics (ADR 049)

**Hyper Acoustics** simulates 4D acoustics via an FDTD solver where simulation parameters are modulated by system metrics.

### 4D Wave Simulation

The simulation runs on a 4D grid (x, y, z, w) and supports headless execution for audio generation.

```mermaid
classDiagram
    direction TB
    class AudioSystem {
        +cpal::Stream stream
        +thread::JoinHandle thread
    }

    class PhysicsGrid4D {
        +usize size
        +Vec~f32~ u
        +Vec~f32~ u_prev
        +Vec~f32~ u_next
        +step(c2, damping)
        +pluck(x, y, z, w, strength)
    }

    class AudioCommand {
        <<Enum>>
        +Pluck
        +SetParams
    }

    AudioSystem *-- PhysicsGrid4D : Owns (via Closure)
    AudioSystem ..> AudioCommand : Consumes
```

## Experiment: Chimera Hologram (ADR 028)

**Chimera Hologram** visualizes genetic information as holographic interference patterns using Fast Fourier Transforms (FFT).

### Holographic Memory

The system stores information in the frequency domain, allowing for distributed storage and fuzzy retrieval.

```mermaid
classDiagram
    direction TB
    class HolographicMemory {
        +usize width
        +usize height
        +Vec~Complex~ memory
        +record(object_grid, alpha)
        +reconstruct() Vec~f64~
    }

    class FftPlanner {
        <<Library: rustfft>>
        +plan_fft_forward()
        +plan_fft_inverse()
    }

    HolographicMemory ..> FftPlanner : Uses
    note for HolographicMemory "Stores accumulated interference patterns"
```

## Experiment: Process Canopy (ADR 051)

**Process Canopy** visualizes the OS process table as a procedurally generated forest, where CPU usage drives growth and scheduling affects sunlight exposure.

### Bio-Digital Isomorphism

The system maps process metrics to tree geometry and the scheduler to a moving sun.

```mermaid
classDiagram
    direction TB
    class Monitor {
        +sys: System
        +fetch_processes() Vec~Process~
    }
    class Tree {
        +ProcessStats stats
        +LSystem structure
        +draw(is_scheduled)
        +grow(cpu_usage)
    }
    class Sun {
        +ScheduleMode mode
        +update(dt)
        +is_shining_on(Tree) bool
    }
    class ScheduleMode {
        <<Enum>>
        +RoundRobin
        +Priority
    }

    Monitor --> Tree : Spawns from Process
    Sun --> Tree : Affects (Photosynthesis)
    Sun ..> ScheduleMode : Configured by
```

## Experiment: Sys Dance (ADR 051)

**Sys Dance** visualizes real-time system performance (RAM, CPU, Swap) as a procedurally animated dancer using Laban Movement Analysis parameters.

### Choreographic pipeline

System metrics are translated into Laban parameters (Effort, Space) which drive an Inverse Kinematics rig.

```mermaid
classDiagram
    direction LR
    class SystemMonitor {
        +sys: System
        +update()
    }
    class LabanState {
        +Effort effort
        +Space space
        +update(monitor)
    }
    class Choreographer {
        +plan_moves(laban) -> Pose
    }
    class IKSystem {
        +solve(limbs, target)
    }
    class Skeleton {
        +Torso
        +Limbs
    }

    SystemMonitor --> LabanState : Drives
    LabanState --> Choreographer : Informs
    Choreographer --> IKSystem : Targets
    IKSystem --> Skeleton : Animates
```

## Experiment: Ferrous Genesis (ADR 051)

**Ferrous Genesis** implements an "Amorphous Cellular Automaton" where particles executing ChimeraVM bytecode modulate their magnetism to self-organize in continuous space.

### Feedback Loop

Particles read the local magnetic field via the VM, execute logic to determine their desired state, and emit magnetism back into the field.

```mermaid
sequenceDiagram
    participant Universe
    participant Body
    participant VM as ChimeraVM
    participant Platter as MagneticField

    loop Physics Step
        Universe->>Body: step(dt)
        Body->>VM: execute_dna()
        VM->>Platter: read_magnetism(local_pos)
        VM->>VM: process_logic(Homeostasis)
        VM->>Platter: emit_magnetism(heat)
        Platter-->>Body: apply_force(magnetic)
        Body->>Universe: update_position(velocity)
    end
```

## Experiment: Syncopated Threads (ADR 065)

**Syncopated Threads** generates audio using multithreaded deterministic rhythms. It enforces a concrete, de-abstracted architecture for its audio system.

```mermaid
classDiagram
    direction TB
    class AudioCommand {
        <<Enum>>
        +Kick
        +Snare
        +Hat
        +Stop
    }

    class Drum {
        <<Enum>>
        +Kick
        +Snare
        +Hat
    }

    AudioCommand ..> Drum : Triggers
```

## Experiment: Miller Lattice Extraction (ADR 066)

**Miller Lattice Extraction** extracted common lattice logic into a shared crate to eliminate code duplication across filesystem, reaction, and file logic experiments.

### Extracted Lattice Architecture

```mermaid
classDiagram
    direction TB
    class MillerLattice {
        <<Library: miller-lattice>>
        +Crystal
        +Atom
        +LatticePoint
    }

    class MillerFS {
        <<Binary: miller-fs>>
    }

    class MillerReaction {
        <<Binary: miller-reaction>>
    }

    class FerroFile {
        <<Binary: ferro-file>>
    }

    MillerFS ..> MillerLattice : Uses
    MillerReaction ..> MillerLattice : Uses
    FerroFile ..> MillerLattice : Uses
```

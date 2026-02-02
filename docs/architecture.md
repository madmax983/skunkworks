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
    class AutomataWarfare

    NeuroTerminal ..> Tui : Uses
    AutomataWarfare ..> Tui : Uses

    note for Tui "Handles raw mode, alternate screen,\nand mouse capture automatically."
```

### Semantic Bridge (crates/tui-semantic)

The `tui-semantic` crate enables applications to expose their internal state as structured data for LLM agents.

```mermaid
classDiagram
    direction LR
    class SemanticState {
        <<trait>>
        +snapshot() Snapshot
    }

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

    SemanticState ..> Snapshot : Produces
    Snapshot *-- Entity : Contains
    Snapshot *-- Action : Contains
```

## Experiment: Git Rhythm

**Git Rhythm** sonifies and visualizes the history of a git repository.

### Component Structure

```mermaid
classDiagram
    direction LR
    class MusicalCommit {
        +String hash
        +String author
        +i64 timestamp
        +usize churn
    }

    class Harvester {
        +harvest_repo(path: &str) Result~Vec~MusicalCommit~~
    }

    class Synthesizer {
        -f32 phase
        -f32 mod_phase
        -u32 sample_rate
        +new(sample_rate: u32) Self
        +hash_to_freq(hash: &str) f32
        +generate_next_sample(commit: &MusicalCommit) f32
    }

    class VisualState {
        +Option~MusicalCommit~ current_commit
        +Vec~f32~ waveform_buffer
        +update(commit: MusicalCommit, sample: f32)
    }

    Harvester ..> MusicalCommit : Creates
    Synthesizer ..> MusicalCommit : Consumes (Modulates Sound)
    VisualState ..> MusicalCommit : Consumes (Updates Metadata)

    note for Synthesizer "FM Synthesis: Churn -> Modulation Index"
    note for VisualState "Renders TUI using Ratatui"
```

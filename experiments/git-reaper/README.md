# git-reaper 🧬

> "Garbage Collection of Codebase History."

A visual experiment combining `git-harmonograph` (git commit history parsing) with `myco-reaper` (fungal garbage collection on memory graphs).

## Concept
The repository's commit history is parsed and mapped into a spatial memory heap graph. Dangling branches or unreferenced commits act as dead memory, which is actively targeted and decomposed by the "reaper fungus" (a garbage collection visualizer). This turns the static history of a codebase into an active, decaying ecosystem where unreferenced nodes are organically consumed.

## Lineage
- From **`git-harmonograph`**: Parses the `git log` and retrieves historical commit hashes and messages to populate the graph nodes.
- From **`myco-reaper`**: Provides the structural memory graph (`heap.rs`) and the garbage collection modes (Manual, Reference Counting, Mark-and-Sweep) with fungal particle decay visualizations.

## Instructions
```bash
cargo run -p git-reaper --release
```

- **M**: Manual Mode
- **R**: Reference Counting Mode (Instantly cleans unreferenced nodes)
- **S**: Mark-and-Sweep Mode (Periodically traces from roots and collects)
- **A**: Toggle automatic random allocation of new nodes
- **Space**: Force allocate a new branch

# Chimera Fossil 🧬🦖

> "Jurassic Park for Code"

A hybrid experiment splicing `chimera-lang` (Genetic VM) with `repo-fossil` (Git Archaeology).

## Concept

We treat the git history of this repository as a geological record. Old commits are "deep strata". The text within files in those commits is "fossilized" (corrupted by entropy based on age).

**Chimera Fossil** allows you to:
1.  **Excavate**: Browse the git history and view fossilized file artifacts.
2.  **Sequence**: Attempt to extract viable genetic material (`OpCode` sequences) from the dead code.
3.  **Resurrect**: Inject the spliced DNA into a Chimera VM and watch it run.

## Lineage

- **Parent A**: `experiments/chimera-lang` (The Engine of Life)
- **Parent B**: `experiments/repo-fossil` (The Source of Material)
- **Novel Trait**: Paleo-genetic Resurrection. The behavior of the organism depends on the specific "era" of the codebase it was harvested from.

## Usage

```bash
cargo run -p chimera-fossil
```

### Controls

**Fossil View (Left Pane)**
- `j`/`k`: Navigate lists
- `l`/`Enter`: Select / Load / Excavate
- `h`/`Esc`: Back

**General**
- `Tab`: Switch between Fossil View and Chimera View
- `q`: Quit

**Chimera View (Right Pane)**
- `Space`: Step VM
- `m`: Force Mutation

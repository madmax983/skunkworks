# Arthropod Origami

This hybrid bridges the abstract immediate mode UI paradigms of `arthropod` with the continuous soft-body mesh generation of `origami`. It allows the user to interactively fold, expand, and contract the Miura-ori tessellation directly via GUI controls.

## Quick Start

```sh
# Run interactively (GUI)
cargo run -p arthropod-origami --release

# Run headlessly (CI/Non-interactive)
cargo run -p arthropod-origami --release -- --headless
```

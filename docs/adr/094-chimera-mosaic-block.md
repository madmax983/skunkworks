# 094. Chimera Mosaic Block

Date: 2026-05-04

## Status
Proposed

## Context
The esolang `chimera-lang` initially lacked a way to define user interfaces directly from within the language scripts. As the ecosystem evolved, experiments needed a more declarative approach to build terminal layouts without falling back to writing Rust wrapper code using the `ratatui` crate.

## Decision
The `prolouge` compiler within `chimera-lang` was evolved to include a `mosaic_block`. This block allows users to specify UI layout definitions directly in esolang scripts. These definitions are compiled to a string and pushed onto the VM's stack, followed by a new instruction `OpCode::MosaicDraw`, which allows the ChimeraVM to natively render the `ratatui` UI layout defined by the string block.

## Consequences

### Positive
*   **Declarative UI:** Scripts can now define their own interfaces, adhering to the "Documentation as Code" and infrastructure-as-code philosophies.
*   **Decoupling:** UI concerns are pushed down into the script level, reducing the need for hardcoded Rust UI facades for simple esolang scripts.

### Negative
*   **VM Complexity:** The virtual machine now needs to know how to interpret and parse a specific subset of UI layout commands, slightly increasing the coupling between the VM core and terminal rendering libraries like `ratatui`.
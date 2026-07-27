# hyper-neuro

**A Splice Surgeon Hybrid**

## Concept
Hyper-dimensional Neural Morphogenesis.

## Lineage
This experiment is a direct cross between:
1. **`crates/hyper-system`**: Provides the 4D vector mathematics (`Vec4`) and perspective projection logic required to render a rotating hypercube.
2. **`crates/neuro-sim`**: Provides the continuous biological simulation of an Izhikevich Spiking Neural Network (`Network`).

## Phenotype
The continuous 4D rotation parameters of the hypercube are driven entirely by the biological Spiking Neural Network. As neurons spike, they impart discrete "thoughts" or angular momentum to rotate the 4D projection planes (XW, YW, ZW). We are visually mapping cognitive load into 4D coordinate rotation.

## Execution
Run with `cargo run -p hyper-neuro`.

**CI Safety**: Supports `--headless` flag to bypass the macroquad UI loop and prevent X11 panics in headless environments.

# Chimera Runes 🧬🔮

> "The code has a face." - The Splice Surgeon

A hybrid experiment combining the procedural steganography of `wasm-runes` with the genetic programming of `chimera-lang`.

## Concept

In this lab, Chimera genomes (`Vec<u8>`) are visualized as "Runes" — procedural images generated deterministically from the hash of their DNA.

By breeding two Runes (Crossover), you create a new Child Rune with traits (genes) from both parents. Because the visual appearance is tied to the DNA hash, the Child Rune looks like a unique blend (or completely new mutation) of its parents.

## Lineage

-   **Parent A**: `experiments/wasm-runes`
    -   *Trait*: Procedural Rune Generation algorithm.
    -   *Allele*: Hash-seeded `ChaCha8Rng` driving drawing primitives.
-   **Parent B**: `experiments/chimera-lang`
    -   *Trait*: Genetic Code (OpCodes) and Virtual Machine execution.
    -   *Allele*: `ChimeraVM`, `OpCode`, `Gene`.

## Controls

-   **Space**: Breed (Crossover) parents to generate a new Child.
-   **M**: Mutate both parents (adds radiation).
-   **R**: Randomize parents (new stock).
-   **Enter**: Execute the Child's DNA in the Chimera VM.
-   **Q**: Quit.

## Emergent Behavior

-   **Visual Phenotypes**: Similar genomes often produce totally different runes because of the avalanche effect of cryptographic hashing. This highlights how small mutations can drastically alter the "identity" of an organism.
-   **Execution Visualization**: You can breed for a cool looking rune, then run it to see if it survives (has valid syntax/energy).

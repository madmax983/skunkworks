# market-lattice

A hybrid of `market-sim` and `miller-lattice`.

## Lineage
- **market-sim**: Provides the Continuous Double Auction particle system where trades happen when bids and asks collide.
- **miller-lattice**: Provides the 3D discrete hierarchical crystal structure of the directory tree.

## Concept
The structural crystal nodes of `miller-lattice` are injected directly into the continuous market grid of `market-sim`. The hierarchical file structures act as physical "walls" or constraints within the market, disrupting the flow of bids and asks. Structural branches force trades to navigate around the directory hierarchy, exploring how codebase architecture might constrain or channel financial pressure.

## Execution
```bash
# Normal visualization
cargo run -p market-lattice --features macroquad_run

# Headless CI mode
cargo run -p market-lattice --no-default-features -- --headless
```

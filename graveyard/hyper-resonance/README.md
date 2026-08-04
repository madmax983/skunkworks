# hyper-resonance

**Lineage:** `crates/hyper-system` × `crates/resonance-audio`

**Phenotype:** System acoustic volatility. The real-time CPU and memory stress from `hyper-system` drives acoustic plucks on a finite difference time domain (FDTD) grid from `resonance-audio`. The heavier the system load, the more violent the acoustic wave propagation.

## Execution

```bash
cargo run -p hyper-resonance
```

## Code

```rust
fn main() {
    println!("Hyper Resonance");
}
```

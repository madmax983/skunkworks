# Git Rhythm

Turn your git history into music and visuals.

## Story Mode (Nova)

> **⚠️ REQUIRES FEATURE `nova`**

To generate a narrative from your git history, use the Nova engine:

```rust
// examples/story_demo.rs
// Run with: cargo run --features nova --example story_demo
use git_rhythm::nova::NarrativeGenerator;

fn main() {
     let generator = NarrativeGenerator::new();
     println!("{}", generator.generate("start"));
}
```

Try it out:

```bash
cargo run --features nova --example story_demo
```

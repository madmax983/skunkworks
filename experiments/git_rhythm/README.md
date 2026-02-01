# Git Rhythm

Turn your git history into music and visuals.

## Story Mode (Nova)

> **⚠️ REQUIRES FEATURE `nova`**

To generate a narrative from your git history, use the Nova engine:

```rust
// examples/story_demo.rs
use git_rhythm::nova::NarrativeGenerator;

fn main() {
     let gen = NarrativeGenerator::new();
     println!("{}", gen.generate("start"));
}
```

Try it out:

```bash
cargo run --features nova --example story_demo
```

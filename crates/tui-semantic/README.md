# tui-semantic

Semantic state bridge for TUI applications - lets LLMs understand and interact with terminal UIs.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tui-semantic = { path = "crates/tui-semantic" }
# or if using workspace:
tui-semantic = { workspace = true }
```

## Usage

Here is a minimal example of how to use `Snapshot` and `Entity`:

```rust
use tui_semantic::{Entity, Snapshot};

struct MyApp {
    player: Player,
    score: i64,
}

struct Player {
    x: f64,
    y: f64,
    health: i64,
}

impl MyApp {
    fn snapshot(&self) -> Snapshot {
        Snapshot::new("my-app")
            .with_entity(
                Entity::new("player")
                    .at(self.player.x, self.player.y)
                    .with_prop("health", self.player.health),
            )
            .with_metric("score", self.score)
    }
}

fn main() {
    let app = MyApp {
        player: Player {
            x: 10.0,
            y: 20.0,
            health: 100,
        },
        score: 500,
    };
    let snap = app.snapshot();
    println!("{}", snap.to_json_pretty());
}
```

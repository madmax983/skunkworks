# Gastown (Rust)

A Rust implementation of the Gas Town AI orchestration tool.

## Usage

```bash
# Install workspace
cargo run -- install ./workspace

# Add a rig (project)
cargo run -- rig add myproject https://github.com/user/project

# Create a convoy (task list)
cargo run -- convoy create "Feature X" task1 task2

# Assign work (sling)
cargo run -- sling task1 myproject --agent claude

# List agents
cargo run -- agents
```

## Structure

- `src/main.rs`: Entry point and command handling.
- `src/core/config.rs`: Configuration and state management (JSON).
- `src/core/task.rs`: ID generation.
- `src/core/project.rs`: Git integration.

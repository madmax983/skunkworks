# Luminous Flock (Web)

A boids + firefly synchronization simulation running in the browser via WebAssembly.

Ported from the terminal version using [Ratzilla](https://github.com/ratatui/ratzilla).

## Local Development

```bash
# Install trunk if you haven't
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown

# Run dev server
trunk serve --open

# Build optimized release
trunk build --release
```

## Controls

- **r** - Reset simulation

## Deploy to Render

1. Create a new Static Site on Render
2. Connect your repository
3. Set build command: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && source $HOME/.cargo/env && rustup target add wasm32-unknown-unknown && cargo install trunk && cd experiments/luminous-flock-web && trunk build --release`
4. Set publish directory: `experiments/luminous-flock-web/dist`

Or use the `render.yaml` Blueprint for automatic deployment.

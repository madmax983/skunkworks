# 👺 Havoc: Segfault in `Button` rendering

### 🧨 The Trigger
The `Button` widget calculates the `text_area` for its label using its `inner_area`, which is derived from the widget's given `area`. It then attempts to draw this text using `buf.set_line(text_area.x + x_offset, text_area.y, ...)`.
Crucially, it does **not** intersect this area with the actual terminal buffer area (`buf.area`). If the parent layout inadvertently pushes the button off-screen (e.g. `x > 10` on a 10x10 terminal), `set_line` executes on out-of-bounds coordinates, triggering a deterministic panic and crashing the entire TUI application.

### 📉 The Stack Trace
```
thread 'main' panicked at /home/runner/.cargo/registry/src/index.crates.io-6f17d22bba15001f/ratatui-core-0.1.0/src/buffer/buffer.rs:360:9:
outside of buffer
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 🧪 Reproduction
Run the newly injected havoc harness:
```bash
cargo test -p tui-shared --test havoc_button_bounds
```

### 😈 Comment
You thought a button is just a button? You assumed the layout engine would mathematically guarantee every widget remains on screen? You were wrong. A single resizing artifact is all it takes to trigger an out-of-bounds panic and take down the entire system.

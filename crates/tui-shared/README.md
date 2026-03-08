# tui-shared 🎨

A robust shared library for initializing and managing Terminal User Interface (TUI) environments using `ratatui` and `crossterm`.

This crate provides a **RAII (Resource Acquisition Is Initialization)** wrapper around the terminal setup. It ensures that the terminal is correctly restored (raw mode disabled, cursor shown, alternate screen left) when the application exits or panics.

## Installation

### Option A: Internal Workspace Experiment (Recommended)

If you are adding a new experiment within this repository (e.g., in `experiments/my-cool-tui`), `tui-shared` is already available as a workspace dependency.

1.  Create your experiment crate:
    ```bash
    cargo new experiments/my-cool-tui
    ```
    *Note: The `experiments/` directory is already part of the workspace members in the root `Cargo.toml`.*

2.  Add dependencies to your experiment's `Cargo.toml`:
    ```toml
    [dependencies]
    tui-shared = { workspace = true }

    # You can access ratatui/crossterm via tui-shared re-exports,
    # or depend on them directly if you prefer:
    ratatui = { workspace = true }
    crossterm = { workspace = true }
    ```

### Option B: Standalone Project

If you are using this crate in a project *outside* of this workspace, you must point to the local path.

1.  Add to your `Cargo.toml`:
    ```toml
    [dependencies]
    # Replace path with the relative path from your project root to crates/tui-shared
    tui-shared = { path = "../../crates/tui-shared" }
    ratatui = "0.30"
    crossterm = "0.28"
    ```

## Usage

`tui-shared` re-exports `ratatui` and `crossterm` for convenience, ensuring version compatibility.

Here is a minimal example:

```rust
use std::{io, thread, time::Duration};
use tui_shared::Tui;
// Use the re-exported ratatui to ensure version alignment
use tui_shared::ratatui::{
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> io::Result<()> {
    // 1. Initialize the terminal
    // This enables raw mode, enters alternate screen, and captures mouse.
    let mut tui = Tui::init()?;

    // 2. Draw something to the terminal
    tui.terminal.draw(|f| {
        let size = f.area();
        let block = Block::default()
            .title(" My TUI App ")
            .borders(Borders::ALL);
        let p = Paragraph::new("Hello, World!")
            .block(block)
            .alignment(Alignment::Center);
        f.render_widget(p, size);
    })?;

    // 3. Application logic here...
    // In a real app, you would run an event loop here.
    thread::sleep(Duration::from_secs(3));

    // 4. Cleanup is automatic!
    // When `tui` goes out of scope, it automatically:
    // - Disables raw mode
    // - Leaves alternate screen
    // - Shows cursor
    Ok(())
}
```

## Testing

Because `Tui::init` modifies the global terminal state, it is not suitable for unit tests. Instead, use `ratatui::backend::TestBackend` to test your drawing logic without a real terminal.

```rust
use tui_shared::ratatui::{
    backend::TestBackend,
    Terminal,
    widgets::{Paragraph, Block, Borders}
};

#[test]
fn test_ui() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let p = Paragraph::new("Hello").block(Block::default().borders(Borders::ALL));
        f.render_widget(p, f.area());
    }).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer.get(1, 1).symbol(), "H");
}
```

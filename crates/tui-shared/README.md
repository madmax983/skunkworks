# tui-shared

A robust shared library for initializing and managing Terminal User Interface (TUI) environments using `ratatui` and `crossterm`.

This crate provides a RAII (Resource Acquisition Is Initialization) wrapper around the terminal setup, ensuring that the terminal is correctly restored (raw mode disabled, cursor shown, etc.) when the application exits or panics.

## Installation

Add this to your `Cargo.toml`. Note that you must also include `ratatui` as a dependency to use its widgets and layout types.

If you are working within the `skunkworks` workspace (e.g., adding a new experiment):

```toml
[dependencies]
tui-shared = { workspace = true }
ratatui = { workspace = true }
```

If you are using this crate in a standalone project, point to the local path:

```toml
[dependencies]
tui-shared = { path = "path/to/crates/tui-shared" } # e.g. "../../crates/tui-shared"
ratatui = "0.30"
```

## Usage

Here is a minimal example of how to use `Tui` to set up and tear down the terminal:

```rust
use tui_shared::Tui;
use std::io;
use ratatui::{widgets::{Block, Borders, Paragraph}, layout::Alignment};

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
    // std::thread::sleep(std::time::Duration::from_secs(3));

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
use ratatui::{backend::TestBackend, Terminal, widgets::{Paragraph, Block, Borders}};

#[test]
fn test_ui() {
    let backend = TestBackend::new(20, 10);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal.draw(|f| {
        let p = Paragraph::new("Hello").block(Block::default().borders(Borders::ALL));
        f.render_widget(p, f.area());
    }).unwrap();

    let buffer = terminal.backend().buffer();
    assert_eq!(buffer.get(1, 1).symbol, "H");
}
```

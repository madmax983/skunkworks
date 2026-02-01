# 003. Extract Shared TUI Lifecycle

## Status
Accepted

## Context
Several experiments (`neuro-terminal`, `automata-warfare`, etc.) require identical TUI initialization and teardown logic. This includes enabling raw mode, entering the alternate screen, enabling mouse capture, and creating a `Terminal` instance.

Duplicating this logic leads to:
1.  **Boilerplate:** High noise-to-signal ratio in `main.rs` files.
2.  **Inconsistency:** Risk of forgetting cleanup steps (e.g., leaving the terminal in raw mode) if the application panics or exits early.
3.  **Maintenance Burden:** Updates to `ratatui` or `crossterm` APIs must be propagated to every experiment manually.

## Decision
We decided to extract the TUI lifecycle management into a dedicated workspace crate: `crates/tui-shared`.

Key features of this abstraction:
*   **RAII Pattern:** A `Tui` struct handles initialization in its constructor (`Tui::init()`) and cleanup in its `Drop` implementation.
*   **Safe Defaults:** Automatically handles `crossterm` setup for raw mode, mouse capture, and alternate screens.
*   **Panic Safety:** By using `Drop`, we ensure the terminal is restored even if the application panics (provided the panic unwinds).

## Consequences
### Positive
*   **DRY (Don't Repeat Yourself):** Initialization logic is central.
*   **Safety:** Reduced risk of "broken" terminals after crashes.
*   **Velocity:** New experiments can spin up a TUI environment with a single line of code.

### Negative
*   **Coupling:** Experiments now depend on a shared crate, which might slightly complicate standalone builds if not handled via workspace dependencies.
*   **Rigidity:** The shared `Tui` struct enforces a specific setup (e.g., always enabling mouse capture) which might not fit every niche use case without configuration.

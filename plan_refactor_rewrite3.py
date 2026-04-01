with open("test_navigation_clean.rs", "r") as f:
    text = f.read()

# Remove duplicate imports
text = text.replace("use crate::tui::state::{AppState, InputMode, ViewMode};\nuse crate::vm::ChimeraVM;\nuse anyhow::Result;\nuse crossterm::event::KeyCode;\nuse crate::tui::state::{AppState, InputMode, ViewMode};\nuse crate::vm::ChimeraVM;\nuse anyhow::Result;\nuse crossterm::event::KeyCode;\n", "use crate::tui::state::{AppState, InputMode, ViewMode};\nuse crate::vm::ChimeraVM;\nuse anyhow::Result;\nuse crossterm::event::KeyCode;\n")

with open("test_navigation_clean.rs", "w") as f:
    f.write(text)
